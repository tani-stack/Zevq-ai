use rand::{rngs::StdRng, Rng, SeedableRng};

use super::TrapGeneratorImpl;
use crate::{Trap, TrapCategory};

#[derive(Default)]
pub struct NullSafetyGenerator;

impl TrapGeneratorImpl for NullSafetyGenerator {
    fn category_name(&self) -> &'static str {
        "null_safety"
    }

    fn generate(&mut self, seed: u64, nonce: u64) -> Trap {
        let mut rng = StdRng::seed_from_u64(seed ^ nonce ^ 0x1234);
        let fields = [
            "email",
            "phone",
            "profile.avatar_url",
            "user.name",
            "meta.id",
            "payload.data",
        ];
        let field = fields[rng.gen_range(0..fields.len())];
        let id = rng.gen_range(100..999);
        let code = format!("let d_{id} = {field}.split('@').collect::<Vec<_>>()[1];");
        Trap {
            id: String::new(),
            category: TrapCategory::NullHandling,
            severity: 10,
            code,
            description: format!("Null trap on {field}"),
            expected_failure: "Missing None check".to_string(),
            deterministic_seed: seed,
            generation_nonce: nonce,
        }
    }
}

#[derive(Default)]
pub struct BoundaryGenerator;

impl TrapGeneratorImpl for BoundaryGenerator {
    fn category_name(&self) -> &'static str {
        "boundary"
    }

    fn generate(&mut self, seed: u64, nonce: u64) -> Trap {
        let mut rng = StdRng::seed_from_u64(seed ^ nonce ^ 0xabcd);
        let n = rng.gen_range(1..32);
        let code = format!(
            "fn check(arr: &[i32]) -> bool {{ for i in 0..arr.len()-{n} {{ if arr[i]==arr[i+{n}] {{ return true; }} }} false }}"
        );
        Trap {
            id: String::new(),
            category: TrapCategory::TypeCoercion,
            severity: 8,
            code,
            description: format!("Boundary off-by-{n}"),
            expected_failure: "len()-N underflows".to_string(),
            deterministic_seed: seed,
            generation_nonce: nonce,
        }
    }
}
