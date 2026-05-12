use axum::{Json, extract::Path};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Serialize)]
pub struct RiskResponse {
    symbol: String,
    status: &'static str,
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

pub async fn risk_by_symbol(Path(symbol): Path<String>) -> Json<RiskResponse> {
    Json(RiskResponse {
        symbol,
        status: "not_configured",
    })
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
