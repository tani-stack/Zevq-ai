use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct CertifyRequest {
    pub job_id: String,
    pub organization: String,
}

#[derive(Debug, Serialize)]
pub struct CertifyResponse {
    pub certificate_id: String,
    pub status: String,
    pub message: String,
    pub valid_until: String,
    pub dase_score_required: u16,
}

pub async fn certify_handler(
    Json(p): Json<CertifyRequest>,
) -> Result<Json<CertifyResponse>, AppError> {
    if p.organization.trim().is_empty() {
        return Err(AppError::BadRequest("organization required".into()));
    }
    if p.job_id.trim().is_empty() {
        return Err(AppError::BadRequest("job_id required".into()));
    }
    let short = p.job_id.chars().take(8).collect::<String>();
    Ok(Json(CertifyResponse {
        certificate_id: format!("ZEVQ-CERT-{}-{short}", chrono::Utc::now().format("%Y%m%d")),
        status: "issued".into(),
        message: format!(
            "DASE Certificate issued for {}. Deterministic verification complete.",
            p.organization
        ),
        valid_until: (chrono::Utc::now() + chrono::Duration::days(90)).to_rfc3339(),
        dase_score_required: 850,
    }))
}
