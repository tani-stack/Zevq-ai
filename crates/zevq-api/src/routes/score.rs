use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::{error::AppError, state::AppState};

#[derive(Deserialize)]
pub struct StreamParams {
    pub seed: Option<u64>,
    pub count: Option<usize>,
}

pub async fn get_score(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let reports = s.reports.read().await;
    match reports.get(&id) {
        Some(r) => Ok(Json(
            serde_json::to_value(r).map_err(|e| AppError::Internal(e.to_string()))?,
        )),
        None => Err(AppError::NotFound(format!(
            "Report {id} not found. Run POST /v1/stress-test first"
        ))),
    }
}

pub async fn stream_traps(
    Query(p): Query<StreamParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let seed = p.seed.unwrap_or_else(rand::random);
    let count = p.count.unwrap_or(20).clamp(1, 1000);
    let traps = zevq_core::generator::quick_generate(count, seed);
    Ok(Json(serde_json::json!({
        "seed": seed,
        "count": count,
        "traps": traps,
        "note": "Infinite combinatorial stream - change seed for billions of unique traps"
    })))
}
