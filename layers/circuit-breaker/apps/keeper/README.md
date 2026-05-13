# Circuit Breaker Keeper

Rust worker that **wires the Risk Oracle on-chain state into the Circuit Breaker policy core** and emits the on-chain `UpdatePolicy` instruction bytes for an authority to submit.

## Flow

```txt
Risk Oracle state account (RPC)
  → RiskOracleSnapshot (`shock_absorber_circuit_breaker::risk_oracle_feed`)
  → CircuitBreakerPolicyEngine + fee/throttle engines
  → hex-encoded UpdatePolicy instruction data (logs)
```

On-chain transaction submission (signer + program id + accounts) is **not** automated here yet; the keeper logs a structured decision and `update_policy_ix_hex` so you can integrate signing with Solana SDK or ops tooling.

## Environment

| Variable | Required | Description |
|----------|----------|-------------|
| `RISK_ORACLE_STATE_PUBKEY` | yes | Base58 pubkey of the risk oracle state account. |
| `CIRCUIT_BREAKER_STATE_PUBKEY` | yes | Base58 pubkey of the circuit breaker state account. |
| `SOLANA_RPC_URL` | no | JSON-RPC HTTP endpoint (default `http://127.0.0.1:8899`). |
| `CIRCUIT_BREAKER_KEEPER_INTERVAL_MS` | no | Poll interval (default `2000`). |
| `AEGIS_ORACLE_CONFIDENCE` | no | 0–100 confidence passed into policy snapshot (default `90`). |

## Run

```bash
export RISK_ORACLE_STATE_PUBKEY='...'
export CIRCUIT_BREAKER_STATE_PUBKEY='...'
export SOLANA_RPC_URL='http://127.0.0.1:8899'

cargo run -p circuit-breaker-keeper
```

## Responsibilities (target)

- read latest risk state from the Risk Oracle Program
- evaluate circuit breaker policy decisions
- submit authorized control updates
- move protocols from panic protection into recovery mode when stress decreases
- log all actions for audits and incident review
