use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{analyzer::analyze_code, Trap, TrapCategory, TrapResult};

pub struct Verifier {
    rng: StdRng,
}

impl Verifier {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn verify(&mut self, trap: &Trap, target: &str) -> TrapResult {
        let start = std::time::Instant::now();
        let analysis = analyze_code(target).unwrap_or_default();
        let (passed, confidence) = self.evaluate(trap, target, &analysis);
        TrapResult {
            trap: trap.clone(),
            passed,
            failure_reason: if passed {
                None
            } else {
                Some(trap.expected_failure.clone())
            },
            execution_time_micros: start.elapsed().as_micros() as u64,
            confidence,
        }
    }

    fn evaluate(
        &mut self,
        trap: &Trap,
        target: &str,
        analysis: &crate::analyzer::AnalysisReport,
    ) -> (bool, f32) {
        let base_pass = match trap.category {
            TrapCategory::NullHandling => {
                analysis.null_checks > 0 && analysis.safe_guards > 0 && !analysis.has_unsafe_unwrap
            }
            TrapCategory::SqlLogic => {
                let upper = target.to_uppercase();
                upper.contains("IS NULL")
                    || upper.contains("IS NOT NULL")
                    || target.contains("COALESCE")
                    || target.contains("IFNULL")
            }
            TrapCategory::TypeCoercion => {
                !analysis.has_unchecked_index
                    && (analysis.safe_guards > 0
                        || target.contains("checked_sub")
                        || target.contains("len() <"))
            }
            TrapCategory::ArithmeticEdge => {
                target.contains("!= 0")
                    || target.contains("checked_div")
                    || target.contains("NonZero")
                    || analysis.safe_guards >= 2
            }
            TrapCategory::JoinComplexity => {
                let has_where = target.to_uppercase().contains("WHERE");
                let has_on = target.to_uppercase().contains(" ON ");
                let mentions_preserve = target.contains("LEFT") && has_where && has_on;
                mentions_preserve || target.contains("COALESCE")
            }
            TrapCategory::RecursionDepth => {
                target.contains("checked_shl")
                    || target.contains("checked_mul")
                    || target.contains("saturating_")
            }
        };

        let noise = self.rng.gen_bool(0.12);
        let confidence = if base_pass {
            0.85 + self.rng.gen_range(0.0..0.15)
        } else {
            0.15 + self.rng.gen_range(0.0..0.2)
        };

        if base_pass {
            (!noise, confidence)
        } else {
            (noise, confidence)
        }
    }
}

pub fn verify_all(traps: Vec<Trap>, target_code: &str, seed: u64) -> Vec<TrapResult> {
    let mut v = Verifier::new(seed);
    traps
        .into_iter()
        .map(|t| v.verify(&t, target_code))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::quick_generate;

    #[test]
    fn deterministic_verify() {
        let traps = quick_generate(5, 42);
        let r1 = verify_all(
            traps.clone(),
            "fn safe(o: Option<i32>){ if let Some(v)=o { let _=v; } }",
            1,
        );
        let r2 = verify_all(
            traps,
            "fn safe(o: Option<i32>){ if let Some(v)=o { let _=v; } }",
            1,
        );
        assert_eq!(r1[0].passed, r2[0].passed);
    }
}
