pub mod analyzer;
pub mod generator;
pub mod scorer;
pub mod verifier;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TrapCategory {
    SqlLogic,
    NullHandling,
    TypeCoercion,
    JoinComplexity,
    RecursionDepth,
    ArithmeticEdge,
}

impl TrapCategory {
    pub fn weight(&self) -> f64 {
        match self {
            Self::NullHandling => 1.5,
            Self::SqlLogic => 1.4,
            Self::ArithmeticEdge => 1.3,
            Self::JoinComplexity => 1.2,
            Self::TypeCoercion => 1.1,
            Self::RecursionDepth => 1.0,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::SqlLogic,
            Self::NullHandling,
            Self::TypeCoercion,
            Self::JoinComplexity,
            Self::RecursionDepth,
            Self::ArithmeticEdge,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trap {
    pub id: String,
    pub category: TrapCategory,
    pub severity: u8,
    pub code: String,
    pub description: String,
    pub expected_failure: String,
    pub deterministic_seed: u64,
    pub generation_nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrapResult {
    pub trap: Trap,
    pub passed: bool,
    pub failure_reason: Option<String>,
    pub execution_time_micros: u64,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressTestReport {
    pub job_id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub traps_generated: usize,
    pub traps_failed: usize,
    pub dase_score: u16,
    pub grade: String,
    pub results: Vec<TrapResult>,
    pub certificate_eligible: bool,
    pub telemetry_hash: String,
}

impl StressTestReport {
    pub fn new(results: Vec<TrapResult>) -> Self {
        let traps_failed = results.iter().filter(|r| !r.passed).count();
        let score = scorer::calculate_dase_score(&results);
        let grade = scorer::grade_from_score(score);
        let certificate_eligible = score >= 850 && traps_failed == 0;
        let mut hasher = Sha256::new();
        hasher.update(format!("{score}:{traps_failed}:{}", results.len()).as_bytes());
        let hash = hex::encode(hasher.finalize());
        Self {
            job_id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            traps_generated: results.len(),
            traps_failed,
            dase_score: score,
            grade,
            results,
            certificate_eligible,
            telemetry_hash: hash,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub max_traps: usize,
    pub default_traps: usize,
    pub enable_z3: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_traps: 10_000,
            default_traps: 50,
            enable_z3: false,
        }
    }
}
