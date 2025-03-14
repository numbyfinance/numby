use axum::{Json, extract::State, response::IntoResponse};
use tower_sessions_redis_store::fred::prelude::ClientLike;

use crate::AppState;

#[derive(serde::Serialize)]
struct HealthStatus {
    status: String,
    postgres: ServiceStatus,
    valkey: ServiceStatus,
    timestamp: String,
}

#[derive(serde::Serialize)]
struct ServiceStatus {
    status: String,
    error: Option<String>,
}

pub async fn handler(State(state): State<AppState>) -> impl IntoResponse {
    let timestamp = chrono::Utc::now().to_rfc3339();

    let postgres = match state.pool.get().await {
        Ok(_) => ServiceStatus {
            status: "up".to_string(),
            error: None,
        },
        Err(e) => {
            tracing::error!("Health check failed: Postgres error: {}", e);
            ServiceStatus {
                status: "down".to_string(),
                error: Some(e.to_string()),
            }
        }
    };

    let valkey = if state.valkey.is_connected() {
        ServiceStatus {
            status: "up".to_string(),
            error: None,
        }
    } else {
        ServiceStatus {
            status: "down".to_string(),
            error: Some("Valkey not connected".to_string()),
        }
    };

    let status = if postgres.status == "up" && valkey.status == "up" {
        "healthy".to_string()
    } else {
        "unhealthy".to_string()
    };

    let health_status = HealthStatus {
        status,
        postgres,
        valkey,
        timestamp,
    };

    Json(health_status)
}
