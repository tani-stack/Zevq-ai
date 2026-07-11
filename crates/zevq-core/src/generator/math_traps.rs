use rand::{rngs::StdRng, Rng, SeedableRng};

use super::TrapGeneratorImpl;
use crate::{Trap, TrapCategory};

#[derive(Default)]
pub struct ArithmeticEdgeGenerator;

impl TrapGeneratorImpl for ArithmeticEdgeGenerator {
    fn category_name(&self) -> &'static str {
        "arithmetic"
    }

    fn generate(&mut self, seed: u64, nonce: u64) -> Trap {
        let mut rng = StdRng::seed_from_u64(seed ^ nonce ^ 0x55aa);
        let a = rng.gen_range(-5000..5000);
        let b = rng.gen_range(1..10);
        let name = format!("calc_{}", rng.gen_range(100..999));
        let code = format!("fn {name}(x: i64) -> i64 {{ (x*{b}) / (x - {a}) }}");
        Trap {
            id: String::new(),
            category: TrapCategory::ArithmeticEdge,
            severity: 9,
            code,
            description: format!("Div zero root {a}"),
            expected_failure: "No zero guard".to_string(),
            deterministic_seed: seed,
            generation_nonce: nonce,
        }
    }
}

#[derive(Default)]
pub struct OverflowGenerator;

impl TrapGeneratorImpl for OverflowGenerator {
    fn category_name(&self) -> &'static str {
        "overflow"
    }

    fn generate(&mut self, seed: u64, nonce: u64) -> Trap {
        let mut rng = StdRng::seed_from_u64(seed ^ nonce ^ 0x99ff);
        let shift = rng.gen_range(0..70);
        let code = format!("let v = 1u64 << {shift};");
        Trap {
            id: String::new(),
            category: TrapCategory::RecursionDepth,
            severity: 8,
            code,
            description: format!("Shift overflow {shift}"),
            expected_failure: "Unchecked shift".to_string(),
            deterministic_seed: seed,
            generation_nonce: nonce,
        }
    }
}
