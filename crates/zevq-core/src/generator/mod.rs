pub mod logic_traps;
pub mod math_traps;
pub mod sql_traps;

use rand::{rngs::StdRng, Rng, SeedableRng};
use uuid::Uuid;

use crate::{EngineConfig, Trap};

pub trait TrapGeneratorImpl: Send {
    fn category_name(&self) -> &'static str;
    fn generate(&mut self, seed: u64, nonce: u64) -> Trap;
}

pub struct DynamicTrapEngine {
    generators: Vec<Box<dyn TrapGeneratorImpl>>,
    rng: StdRng,
    base_seed: u64,
    nonce: u64,
}

impl DynamicTrapEngine {
    pub fn new(seed: u64) -> Self {
        let mut engine = Self {
            generators: Vec::new(),
            rng: StdRng::seed_from_u64(seed),
            base_seed: seed,
            nonce: 0,
        };
        engine.register_defaults();
        engine
    }

    pub fn with_config(seed: u64, _config: &EngineConfig) -> Self {
        Self::new(seed)
    }

    fn register_defaults(&mut self) {
        self.generators
            .push(Box::new(sql_traps::SqlLogicGenerator));
        self.generators
            .push(Box::new(sql_traps::JoinComplexityGenerator));
        self.generators
            .push(Box::new(logic_traps::NullSafetyGenerator));
        self.generators
            .push(Box::new(logic_traps::BoundaryGenerator));
        self.generators
            .push(Box::new(math_traps::ArithmeticEdgeGenerator));
        self.generators
            .push(Box::new(math_traps::OverflowGenerator));
    }

    pub fn next_trap(&mut self) -> Trap {
        if self.generators.is_empty() {
            panic!("No generators registered");
        }
        let idx = self.rng.gen_range(0..self.generators.len());
        let trap_seed = self
            .base_seed
            .wrapping_add(self.nonce)
            .wrapping_mul(0x9e3779b97f4a7c15);
        let mut trap = self.generators[idx].generate(trap_seed, self.nonce);
        trap.id = Uuid::new_v4().to_string();
        trap.deterministic_seed = trap_seed;
        trap.generation_nonce = self.nonce;
        self.nonce = self.nonce.wrapping_add(1);
        trap
    }

    pub fn generate_n(&mut self, count: usize) -> Vec<Trap> {
        (0..count).map(|_| self.next_trap()).collect()
    }

    pub fn iter(self) -> impl Iterator<Item = Trap> {
        let mut engine = self;
        std::iter::from_fn(move || Some(engine.next_trap()))
    }
}

pub fn quick_generate(count: usize, seed: u64) -> Vec<Trap> {
    let config = EngineConfig::default();
    let mut engine = DynamicTrapEngine::with_config(seed, &config);
    engine.generate_n(count)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_deterministic() {
        let a = quick_generate(20, 12345);
        let b = quick_generate(20, 12345);
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x.code, y.code);
        }
    }

    #[test]
    fn test_unique() {
        let traps = quick_generate(100, 999);
        let codes: HashSet<_> = traps.iter().map(|t| &t.code).collect();
        assert!(codes.len() > 80);
    }
}
