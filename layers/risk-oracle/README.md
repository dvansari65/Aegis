# Layer 1: Risk Oracle

The Risk Oracle Layer detects stablecoin panic and publishes machine-readable risk state.

This layer answers:

```txt
How dangerous is the current stablecoin market condition?
```

Core responsibilities:
- ingest real-time market data
- build stablecoin market state
- compute normalized panic signals
- produce stress scores
- classify liquidity health
- estimate depeg probability
- classify panic type
- publish risk state on-chain
- serve SDK, API, dashboard, and alert consumers

This layer must stay independent from circuit breaker execution. It can recommend or publish risk, but it should not directly mutate another protocol's controls.

## Folder Map

- `crates/ingestion`: raw data collection and event normalization
- `crates/core`: deterministic risk logic
- `programs/risk-oracle`: Pinocchio-based Solana risk feed program
- `apps/oracle-publisher`: off-chain publisher for on-chain risk updates
- `apps/api`: public API for off-chain consumers
- `apps/dashboard`: operator and ecosystem dashboard
- `packages/sdk`: integration SDKs

