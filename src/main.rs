use clorinde::deadpool_postgres::{Config, Pool, Runtime, SslMode};
use clorinde::tokio_postgres::NoTls;

use axum::{Router, response::IntoResponse, routing::get};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use miette::{IntoDiagnostic, Result};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;

#[derive(Clone)]
struct AppState {
    pool: clorinde::deadpool_postgres::Pool,
}

fn app() -> Result<Router> {
    let pool = create_pool(
        std::env::var("DATABASE_URL")
            .expect("No DATABASE_URL env var")
            .into(),
    )?;

    Ok(Router::new()
        .nest_service("/assets", ServeDir::new("./web/assets"))
        .route("/", get(index))
        .with_state(AppState { pool })
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .route("/health", get(health)))
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ =
        init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers().into_diagnostic()?;

    let app = app()?;
    let addr = &"0.0.0.0:3000".parse::<SocketAddr>().into_diagnostic()?;

    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .into_diagnostic()?;

    axum::serve(listener, app.into_make_service())
        .await
        .into_diagnostic()?;

    Ok(())
}

pub fn create_pool(connection: String) -> Result<Pool> {
    let mut cfg = Config::new();
    cfg.url = Some(connection);
    cfg.ssl_mode = Some(SslMode::Prefer);
    Ok(cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .into_diagnostic()?)
}

async fn health() -> impl IntoResponse {
    "up"
}

#[tracing::instrument]
async fn index() -> impl IntoResponse {
    let trace_id = find_current_trace_id();
    format!("{}", trace_id.unwrap())
}
