use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::state::AppState;

pub async fn health_check(State(s): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "zevq-ai",
        "engine": "DASE v0.1.0",
        "uptime_secs": s.uptime_secs(),
        "config": {
            "max_traps": s.config.max_traps,
            "default_traps": s.config.default_traps
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn metrics(State(s): State<AppState>) -> Json<Value> {
    let count = s.reports.read().await.len();
    Json(json!({
        "reports_stored": count,
        "uptime_secs": s.uptime_secs(),
        "version": "0.1.0",
        "engine": "deterministic, no hardcoded limits"
    }))
}
