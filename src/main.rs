use axum::routing::{MethodFilter, on};
use axum::{Extension, middleware};
use axum::{Router, response::IntoResponse, routing::get};
use axum_login::{
    AuthManagerLayerBuilder, permission_required,
    tower_sessions::{Expiry, SessionManagerLayer, cookie::time::Duration},
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use clorinde::deadpool_postgres::{Config, Runtime, SslMode};
use clorinde::tokio_postgres::NoTls;
use juniper::{Context, EmptyMutation, EmptySubscription};
use juniper_axum::graphiql;
use miette::{IntoDiagnostic, Result};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_sessions_redis_store::{
    RedisStore,
    fred::prelude::{ClientLike, Config as ValkeyConfig, Pool as ValkeyPool},
};

mod api;
mod common;
mod components;
mod routes;
mod statics;
#[cfg(test)]
#[cfg(feature = "test.integration")]
mod tests;

use api::Schema;
use common::session::Backend;
pub use common::tracer;

#[derive(Clone)]
pub struct AppState {
    pool: clorinde::deadpool_postgres::Pool,
    valkey: ValkeyPool,
}

impl Context for AppState {}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard =
        init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers().into_diagnostic()?;

    let addr = &"0.0.0.0:3000".parse::<SocketAddr>().into_diagnostic()?;
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .into_diagnostic()?;

    //
    // Postgres pool
    //

    let mut cfg = Config::new();
    cfg.ssl_mode = Some(SslMode::Prefer);
    cfg.url = Some(
        std::env::var("DATABASE_URL")
            .expect("No DATABASE_URL env var")
            .into(),
    );

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .into_diagnostic()?;

    //
    // Valkey pool
    //

    let valkey = ValkeyPool::new(
        ValkeyConfig::from_url(&std::env::var("VALKEY_URL").expect("No VALKEY_URL env var"))
            .into_diagnostic()?,
        None,
        None,
        None,
        6,
    )
    .into_diagnostic()?;
    let valkey_conn = valkey.connect();
    valkey.wait_for_connect().await.into_diagnostic()?;

    //
    // Auth service
    //

    let session_store = RedisStore::new(valkey.clone());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    let backend = Backend::new(pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    //
    // Routes/layers
    //

    let state = AppState { pool, valkey };

    let protected_routes = Router::new()
        // Basic user
        .route("/tp", get(test_protect))
        .route_layer(permission_required!(
            Backend,
            login_url = "/login",
            String::from("protected.read"),
        ));

    let schema = Schema::new(
        api::query::Query,
        EmptyMutation::new(),
        EmptySubscription::new(),
    );

    let api_routes = Router::new()
        .route(
            "/graphql",
            on(MethodFilter::GET.or(MethodFilter::POST), api::route),
        )
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            api::middleware,
        ))
        .layer(Extension(Arc::new(schema)))
        .layer(Extension(state.clone()));

    let app = Router::new()
        .merge(protected_routes)
        .merge(routes::auth::router())
        .merge(api_routes)
        .route("/health", get(routes::health))
        .route("/static/{*path}", get(statics::route::static_path))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(auth_layer)
        .with_state(state);

    axum::serve(listener, app.into_make_service())
        .await
        .into_diagnostic()?;

    valkey_conn.await.into_diagnostic()?.into_diagnostic()?;

    Ok(())
}

async fn test_protect() -> impl IntoResponse {
    "up"
}
