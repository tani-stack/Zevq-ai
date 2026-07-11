use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zevq_api::{app_router, config::AppConfig, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::from_env();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.log_level.clone()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--self-test") {
        println!("Zevq AI DASE Engine v0.1.0 self-test");
        let traps = zevq_core::generator::quick_generate(10, 42);
        assert_eq!(traps.len(), 10);
        let report = zevq_core::StressTestReport::new(zevq_core::verifier::verify_all(
            traps,
            "fn safe(x: Option<i32>){ if let Some(v)=x { let _=v; } }",
            42,
        ));
        println!(
            "Self-test OK: score={} grade={}",
            report.dase_score, report.grade
        );
        return Ok(());
    }

    let state = AppState::new(config.clone());
    let app = app_router(state);
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(
        "Zevq AI listening on {} max_traps={}",
        addr,
        config.max_traps
    );
    axum::serve(listener, app).await?;
    Ok(())
}
