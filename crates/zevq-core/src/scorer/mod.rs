use std::collections::HashMap;

use crate::{TrapCategory, TrapResult};

#[derive(Debug, Clone)]
pub struct ScoringConfig {
    pub weights: HashMap<TrapCategory, f64>,
    pub exponent: f64,
    pub s_tier: u16,
    pub a_tier: u16,
}

impl Default for ScoringConfig {
    fn default() -> Self {
        let mut weights = HashMap::new();
        for cat in TrapCategory::all() {
            weights.insert(cat.clone(), cat.weight());
        }
        Self {
            weights,
            exponent: 0.85,
            s_tier: 900,
            a_tier: 850,
        }
    }
}

pub fn calculate_dase_score(results: &[TrapResult]) -> u16 {
    calculate_with_config(results, &ScoringConfig::default())
}

pub fn calculate_with_config(results: &[TrapResult], config: &ScoringConfig) -> u16 {
    if results.is_empty() {
        return 1000;
    }
    let total: f64 = results.iter().map(|r| r.trap.severity as f64).sum();
    if total == 0.0 {
        return 1000;
    }
    let failed: f64 = results
        .iter()
        .filter(|r| !r.passed)
        .map(|r| {
            let w = config.weights.get(&r.trap.category).copied().unwrap_or(1.0);
            r.trap.severity as f64 * w * (1.5 - f64::from(r.confidence))
        })
        .sum();
    let ratio = (failed / total).clamp(0.0, 1.0);
    (1000.0 * (1.0 - ratio.powf(config.exponent))).clamp(0.0, 1000.0) as u16
}

pub fn grade_from_score(score: u16) -> String {
    grade_with_config(score, &ScoringConfig::default())
}

pub fn grade_with_config(score: u16, config: &ScoringConfig) -> String {
    if score >= config.s_tier {
        format!("S-Tier (Production Ready) [{score}/1000]")
    } else if score >= config.a_tier {
        format!("A-Tier (Certifiable) [{score}/1000]")
    } else if score >= 700 {
        format!("B-Tier (Needs Hardening) [{score}/1000]")
    } else if score >= 500 {
        format!("C-Tier (Risky) [{score}/1000]")
    } else {
        format!("F-Tier (Catastrophic) [{score}/1000]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_empty() {
        assert_eq!(calculate_dase_score(&[]), 1000);
    }

    #[test]
    fn grade_bounds() {
        assert!(grade_from_score(950).contains("S-Tier"));
    }
}
