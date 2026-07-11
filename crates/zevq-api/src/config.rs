use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub max_traps: usize,
    pub default_traps: usize,
    pub request_timeout_secs: u64,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8080),
            max_traps: env::var("MAX_TRAPS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10_000),
            default_traps: env::var("DEFAULT_TRAPS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50),
            request_timeout_secs: env::var("REQUEST_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
            log_level: env::var("RUST_LOG")
                .unwrap_or_else(|_| "zevq_api=info,zevq_core=info".to_string()),
        }
    }
}
