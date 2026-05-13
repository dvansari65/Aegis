pub mod monitor;
mod routes;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(routes::health))
        .route("/v1/risk/{symbol}", get(routes::risk_by_symbol))
        .route("/v1/alerts", get(routes::alerts))
        .route("/v1/oracle/status", get(routes::oracle_status))
        .with_state(state)
}
