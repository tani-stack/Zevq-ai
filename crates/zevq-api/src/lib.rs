pub mod config;
pub mod error;
pub mod routes;
pub mod state;

use axum::{
    routing::{get, post},
    Router,
};

use state::AppState;

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(routes::health::health_check))
        .route("/metrics", get(routes::health::metrics))
        .route(
            "/v1/stress-test",
            post(routes::stress_test::stress_test_handler),
        )
        .route("/v1/certify", post(routes::certify::certify_handler))
        .route("/v1/score/:id", get(routes::score::get_score))
        .route("/v1/traps/stream", get(routes::score::stream_traps))
        .with_state(state)
}
