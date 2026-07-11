use std::{collections::HashMap, sync::Arc, time::Instant};

use tokio::sync::RwLock;
use zevq_core::StressTestReport;

#[derive(Clone)]
pub struct AppState {
    pub reports: Arc<RwLock<HashMap<String, StressTestReport>>>,
    pub start_time: Instant,
    pub config: crate::config::AppConfig,
}

impl AppState {
    pub fn new(config: crate::config::AppConfig) -> Self {
        Self {
            reports: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
            config,
        }
    }

    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}
