# Layer 2: Circuit Breaker

The Circuit Breaker Layer consumes Risk Oracle output and activates protective market controls.

This layer answers:

```txt
What should integrated protocols do when stablecoin panic is detected?
```

Core responsibilities:
- adaptive fee policies
- withdrawal throttling policies
- liquidity routing policies
- emergency liquidity policies
- toxic arbitrage response policies
- cross-protocol coordination hooks
- recovery mode after stress decreases

This layer should not compute the canonical risk score. It reads risk state from Layer 1 and maps that risk state into defensive actions.

## Wiring (Risk Oracle → Circuit Breaker)

The **circuit-breaker keeper** (`apps/keeper`) polls Solana JSON-RPC, decodes the risk oracle state account into a `RiskOracleSnapshot`, reads the current circuit breaker `ProtectionMode` from the breaker state account, runs `CircuitBreakerPolicyEngine` (+ fee/throttle engines), and logs the resulting **`UpdatePolicy`** instruction payload (hex). See `apps/keeper/README.md` for environment variables.

The **oracle publisher** can append an 8-byte little-endian `updated_at_slot` to `UpdateRisk` when `SOLANA_RPC_URL` is set so the risk oracle program persists `RiskState.updated_at_slot` for recovery cooldown math.

## Folder Map

- `crates/core`: deterministic circuit breaker policy logic
- `programs/circuit-breaker`: Pinocchio-based Solana control state program
- `apps/keeper`: off-chain automation service for policy execution
- `packages/sdk`: integration SDKs for protocols using circuit breaker controls

