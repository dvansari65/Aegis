use crate::chain_monitor;
use crate::http::monitor;
use crate::state::AppState;

pub async fn resolve_monitor(state: &AppState, symbol: &str) -> monitor::MonitorResponse {
    if let Some(cfg) = &state.chain {
        match chain_monitor::fetch_chain_monitor(&state.http, cfg, symbol).await {
            Ok(m) => return m,
            Err(e) => {
                tracing::warn!(error = %e, "chain-backed monitor failed; falling back to demo payload");
            }
        }
    }
    monitor::build_monitor(symbol)
}
