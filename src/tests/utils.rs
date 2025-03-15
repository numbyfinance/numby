use async_lazy::Lazy;
use axum::{
    Router,
    body::Body,
    http::{Request, Response, StatusCode},
};
use axum_login::{
    AuthManagerLayerBuilder,
    tower_sessions::{MemoryStore, SessionManagerLayer},
};
use clorinde::deadpool_postgres::{Config, Runtime, SslMode};
use clorinde::tokio_postgres::NoTls;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};
use tower::ServiceExt;
use tower_sessions_redis_store::fred::prelude::{
    ClientLike, Config as ValkeyConfig, Pool as ValkeyPool,
};

use crate::session::Backend;
use crate::{AppState, session::Credentials};

static INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);
static SHARED_ENV: Lazy<TestEnv> = Lazy::new(|| Box::pin(async { TestEnv::new().await }));

#[derive(Debug)]
pub struct SseEvent {
    pub event_type: String,
    pub data: String,
}

impl SseEvent {
    pub fn new(event_type: &str, data: &str) -> Self {
        Self {
            event_type: event_type.to_string(),
            data: data.to_string(),
        }
    }
}

pub struct TextContext {
    pub _state: AppState,
    app: Router,
    cookie: Option<String>,
}

impl TextContext {
    pub async fn new(router: Router<AppState>) -> Self {
        INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst);

        let env = tokio::spawn(async { SHARED_ENV.force().await.clone() })
            .await
            .unwrap();

        // NOTE: using MemoryStore instead of Valkey to prevent flakiness
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

        let backend = Backend::new(env.state.pool.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        Self {
            app: Router::new()
                .merge(router)
                .layer(auth_layer)
                .with_state(env.state.clone()),
            cookie: None,
            _state: env.state.clone(),
        }
    }

    pub async fn login(mut self) -> Self {
        let credentials = Credentials {
            email: "topaz@ipc.org".to_string(),
            password: "topaz".to_string(),
        };

        let request = Request::builder()
            .method("POST")
            .uri("/login")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&credentials).unwrap()))
            .unwrap();

        let response = self.send_request(request).await;

        let cookie_header = response
            .headers()
            .get("set-cookie")
            .unwrap()
            .to_str()
            .unwrap();

        let id_cookie = cookie_header.split(';').next().unwrap().to_string();

        assert!(
            self.assert_sse_contains(
                response,
                "datastar-execute-script",
                "window.location.assign"
            )
            .await,
            "Expected redirect but didn't find it"
        );

        self.cookie = Some(id_cookie);

        self
    }

    pub fn build_request(&self) -> axum::http::request::Builder {
        Request::builder().header(
            "Cookie",
            self.cookie
                .as_ref()
                .expect("No cookie, call `.login()` first"),
        )
    }

    pub async fn send_request(&self, request: Request<Body>) -> Response<Body> {
        self.app.clone().oneshot(request).await.unwrap()
    }

    /// Parse SSE events from a response body
    pub async fn parse_sse_events(response: Response<Body>) -> Vec<SseEvent> {
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body();
        let body_bytes = http_body_util::BodyExt::collect(body)
            .await
            .unwrap()
            .to_bytes();

        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        body_str
            .split("\n\n")
            .filter(|chunk| !chunk.is_empty())
            .map(|event_chunk| {
                // Extract event type
                let event_type = event_chunk
                    .lines()
                    .find(|line| line.starts_with("event: "))
                    .map(|line| line.trim_start_matches("event: ").trim())
                    .unwrap_or("message")
                    .to_string();

                // Collect all data lines
                let data_lines: Vec<String> = event_chunk
                    .lines()
                    .filter(|line| line.starts_with("data: "))
                    .map(|line| line.trim_start_matches("data: ").trim().to_string())
                    .collect();

                // Join multiple data lines with newlines if there are multiple
                let data = if data_lines.len() > 1 {
                    data_lines.join("\n")
                } else {
                    data_lines.first().cloned().unwrap_or_default()
                };

                SseEvent::new(&event_type, &data)
            })
            .collect()
    }

    /// Check if SSE response contains a specific message
    pub async fn assert_sse_contains(
        &self,
        response: Response<Body>,
        event_type: &str,
        content: &str,
    ) -> bool {
        let events = Self::parse_sse_events(response).await;
        events
            .iter()
            .any(|event| event.event_type == event_type && event.data.contains(content))
    }
}

impl Drop for TextContext {
    /// Cleanup containers when there are no tests running anymore
    fn drop(&mut self) {
        let remaining = INSTANCE_COUNT.fetch_sub(1, Ordering::SeqCst) - 1;
        if remaining == 0 {
            let _ = tokio::task::spawn_blocking(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let env = tokio::spawn(async { SHARED_ENV.force().await.clone() })
                        .await
                        .unwrap();

                    if let Err(e) = tokio::process::Command::new("podman")
                        .args(["rm", "-f", "-v", &env.postgres.container.id()])
                        .output()
                        .await
                    {
                        eprintln!("Error removing postgres container: {}", e);
                    }

                    if let Err(e) = tokio::process::Command::new("podman")
                        .args(["rm", "-f", "-v", &env.valkey.container.id()])
                        .output()
                        .await
                    {
                        eprintln!("Error removing valkey container: {}", e);
                    }
                });
            });
        }
    }
}

#[derive(Clone)]
pub struct PostgresContainer {
    container: Arc<ContainerAsync<GenericImage>>,
}

impl PostgresContainer {
    pub async fn new() -> Self {
        let container = GenericImage::new("postgres", "17")
            .with_exposed_port(5432.tcp())
            .with_wait_for(WaitFor::message_on_stdout(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "numby")
            .with_env_var("POSTGRES_DB", "numby_test")
            .start()
            .await
            .unwrap();

        Self {
            container: Arc::new(container),
        }
    }

    pub async fn get_connection_string(&self) -> String {
        let host = self
            .container
            .get_host()
            .await
            .expect("Couldn't get postgres host");

        let host_port = self
            .container
            .get_host_port_ipv4(5432)
            .await
            .expect("Couldn't get postgres port");

        format!("postgres://postgres:numby@{host}:{host_port}/numby_test")
    }
}

#[derive(Clone)]
pub struct ValkeyContainer {
    container: Arc<ContainerAsync<GenericImage>>,
}

impl ValkeyContainer {
    pub async fn new() -> Self {
        let container = GenericImage::new("valkey/valkey", "8")
            .with_exposed_port(6379.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
            .start()
            .await
            .unwrap();

        Self {
            container: Arc::new(container),
        }
    }

    pub async fn get_connection_string(&self) -> String {
        let host = self
            .container
            .get_host()
            .await
            .expect("Couldn't get valkey host");

        let host_port = self
            .container
            .get_host_port_ipv4(6379)
            .await
            .expect("Couldn't get valkey port");

        format!("valkey://{host}:{host_port}")
    }
}

pub struct TestEnv {
    pub postgres: PostgresContainer,
    pub valkey: ValkeyContainer,
    pub state: AppState,
}

impl TestEnv {
    pub async fn new() -> Self {
        let postgres = PostgresContainer::new().await;
        let valkey = ValkeyContainer::new().await;

        let state = Self::create_state(&postgres, &valkey).await;
        let env = Self {
            postgres,
            valkey,
            state,
        };

        // wait for connections
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        env.init_database().await;
        env
    }

    async fn create_state(postgres: &PostgresContainer, valkey: &ValkeyContainer) -> AppState {
        let mut pg_config = Config::new();
        pg_config.ssl_mode = Some(SslMode::Prefer);
        pg_config.url = Some(postgres.get_connection_string().await);

        let pool = pg_config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Failed to create database pool");

        let val_pool = ValkeyPool::new(
            ValkeyConfig::from_url(&valkey.get_connection_string().await)
                .expect("Failed to create Valkey config"),
            None,
            None,
            None,
            6,
        )
        .expect("Failed to create Valkey pool");

        let _ = val_pool.connect();
        val_pool.wait_for_connect().await.unwrap();

        AppState {
            pool,
            valkey: val_pool,
        }
    }

    async fn init_database(&self) {
        let database_url = self.postgres.get_connection_string().await;

        let create_output = tokio::process::Command::new("sqlx")
            .arg("database")
            .arg("create")
            .arg("--database-url")
            .arg(&database_url)
            .output()
            .await
            .expect("Failed to execute sqlx database create command");

        if !create_output.status.success() {
            let stderr = String::from_utf8_lossy(&create_output.stderr);
            if !stderr.contains("database exists") {
                println!("Database creation note: {}", stderr);
            }
        }

        let migrate_output = tokio::process::Command::new("sqlx")
            .arg("migrate")
            .arg("run")
            .arg("--database-url")
            .arg(&database_url)
            .output()
            .await
            .expect("Failed to execute sqlx migrate run command");

        if !migrate_output.status.success() {
            let stderr = String::from_utf8_lossy(&migrate_output.stderr);
            panic!("Failed to run migrations: {}", stderr);
        }
    }
}

impl Clone for TestEnv {
    fn clone(&self) -> Self {
        Self {
            postgres: PostgresContainer {
                container: self.postgres.container.clone(),
            },
            valkey: ValkeyContainer {
                container: self.valkey.container.clone(),
            },
            state: self.state.clone(),
        }
    }
}
