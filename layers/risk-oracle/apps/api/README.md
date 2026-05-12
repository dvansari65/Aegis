# Risk Oracle API

Rust/Axum API for protocols, bots, dashboards, and institutional users.

Responsibilities:
- expose latest stablecoin risk state
- expose historical risk timelines
- serve active panic alerts
- provide integration-friendly endpoints for off-chain consumers

The canonical protocol state should still be the on-chain Risk Oracle Program.

Initial routes:
- `GET /health`
- `GET /v1/risk/:symbol`
- `GET /v1/alerts`
- `GET /v1/oracle/status`
