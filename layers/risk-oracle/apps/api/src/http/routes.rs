use axum::{
    Json,
    extract::Path,
    extract::State,
    http::{HeaderValue, header},
    response::IntoResponse,
};
use serde::Serialize;

use crate::monitor_resolve;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Serialize)]
pub struct AlertsResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct OracleStatusResponse {
    status: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "risk-oracle-api",
    })
}

pub async fn risk_by_symbol(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> impl IntoResponse {
    let body = monitor_resolve::resolve_monitor(&state, &symbol).await;
    let mut res = Json(body).into_response();
    res.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, must-revalidate"),
    );
    res
}

pub async fn alerts() -> Json<AlertsResponse> {
    Json(AlertsResponse {
        status: "not_configured",
    })
}

pub async fn oracle_status() -> Json<OracleStatusResponse> {
    Json(OracleStatusResponse {
        status: "not_configured",
    })
}
