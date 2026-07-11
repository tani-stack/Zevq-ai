use rand::{rngs::StdRng, Rng, SeedableRng};

use super::TrapGeneratorImpl;
use crate::{Trap, TrapCategory};

fn random_ident(rng: &mut StdRng, prefix: &str) -> String {
    let suffix: String = (0..4).map(|_| rng.gen_range(b'a'..=b'z') as char).collect();
    format!("{prefix}_{suffix}")
}

fn random_table(rng: &mut StdRng) -> String {
    let bases = [
        "users",
        "orders",
        "payments",
        "events",
        "inventory",
        "sessions",
    ];
    if rng.gen_bool(0.6) {
        bases[rng.gen_range(0..bases.len())].to_string()
    } else {
        random_ident(rng, "t")
    }
}

#[derive(Default)]
pub struct SqlLogicGenerator;

impl TrapGeneratorImpl for SqlLogicGenerator {
    fn category_name(&self) -> &'static str {
        "sql_logic"
    }

    fn generate(&mut self, seed: u64, nonce: u64) -> Trap {
        let mut rng = StdRng::seed_from_u64(seed ^ nonce);
        let table = random_table(&mut rng);
        let col = random_ident(&mut rng, "col");
        let code = format!(
            "SELECT * FROM {table} WHERE {col} NOT IN (SELECT id FROM orders WHERE id IS NULL)"
        );
        Trap {
            id: String::new(),
            category: TrapCategory::SqlLogic,
            severity: rng.gen_range(7..=10),
            code,
            description: format!("SQL logic trap on {table}.{col}"),
            expected_failure: "NOT IN with NULL is UNKNOWN".to_string(),
            deterministic_seed: seed,
            generation_nonce: nonce,
        }
    }
}

#[derive(Default)]
pub struct JoinComplexityGenerator;

impl TrapGeneratorImpl for JoinComplexityGenerator {
    fn category_name(&self) -> &'static str {
        "join_complexity"
    }

    fn generate(&mut self, seed: u64, nonce: u64) -> Trap {
        let mut rng = StdRng::seed_from_u64(seed ^ nonce ^ 0xdeadbeef);
        let left = random_table(&mut rng);
        let right = random_table(&mut rng);
        let amount = rng.gen_range(10..10000);
        let code = format!(
            "SELECT {left}.id, COUNT({right}.id) FROM {left} LEFT JOIN {right} ON {right}.fk={left}.id AND {right}.amount>{amount} GROUP BY {left}.id"
        );
        Trap {
            id: String::new(),
            category: TrapCategory::JoinComplexity,
            severity: 9,
            code,
            description: format!("JOIN trap #{nonce}"),
            expected_failure: "LEFT JOIN ON vs WHERE".to_string(),
            deterministic_seed: seed,
            generation_nonce: nonce,
        }
    }
}
