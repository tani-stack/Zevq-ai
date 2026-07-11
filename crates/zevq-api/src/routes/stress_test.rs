use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, state::AppState};
use zevq_core::{generator, verifier};

#[derive(Debug, Deserialize)]
pub struct StressRequest {
    pub target_code: String,
    pub language: Option<String>,
    pub trap_count: Option<usize>,
    pub seed: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct StressResponse {
    pub job_id: String,
    pub dase_score: u16,
    pub grade: String,
    pub traps_generated: usize,
    pub traps_failed: usize,
    pub certificate_eligible: bool,
    pub telemetry_hash: String,
    pub report_url: String,
    pub generation_seed: u64,
}

pub async fn stress_test_handler(
    State(state): State<AppState>,
    Json(payload): Json<StressRequest>,
) -> Result<Json<StressResponse>, AppError> {
    if payload.target_code.trim().is_empty() {
        return Err(AppError::BadRequest("target_code cannot be empty".into()));
    }
    if payload.target_code.len() > 200_000 {
        return Err(AppError::BadRequest(
            "target_code too large, max 200KB".into(),
        ));
    }

    let requested = payload.trap_count.unwrap_or(state.config.default_traps);
    let count = requested.clamp(1, state.config.max_traps);
    let seed = payload.seed.unwrap_or_else(rand::random);

    let traps = generator::quick_generate(count, seed);
    let results = verifier::verify_all(traps, &payload.target_code, seed);
    let report = zevq_core::StressTestReport::new(results);

    let response = StressResponse {
        job_id: report.job_id.to_string(),
        dase_score: report.dase_score,
        grade: report.grade.clone(),
        traps_generated: report.traps_generated,
        traps_failed: report.traps_failed,
        certificate_eligible: report.certificate_eligible,
        telemetry_hash: report.telemetry_hash.clone(),
        report_url: format!("/v1/score/{}", report.job_id),
        generation_seed: seed,
    };

    state
        .reports
        .write()
        .await
        .insert(report.job_id.to_string(), report);
    Ok(Json(response))
}
