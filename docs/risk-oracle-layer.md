# Risk Oracle Layer

The Risk Oracle Layer is Layer 1 of the Stablecoin Panic Control Protocol.

Its job is to detect stablecoin panic, compute risk, and publish a machine-readable risk feed that the Circuit Breaker Layer consumes.

## Components

1. Data Ingestion Layer

Collects raw Solana and oracle data from RPC, DEX pools, stablecoin accounts, price feeds, bridge flows, and whale activity.

2. Market State Builder

Converts raw events into current stablecoin market state, such as price, liquidity depth, pool imbalance, slippage, volume, and flow direction.

3. Signal Engine

Computes normalized risk signals from market state. Examples include peg deviation, oracle divergence, pool imbalance, whale exit velocity, liquidity depth drop, and bridge outflow velocity.

4. Stress Score Engine

Combines risk signals into a single `0-100` stress score for each supported stablecoin.

5. Liquidity Health Engine

Classifies liquidity condition as `Healthy`, `Watch`, `Stressed`, `Critical`, or `Severe`.

6. Depeg Probability Engine

Estimates near-term peg instability risk using rule-based bands first, with room for historical and ML-based models later.

7. Panic Classification Engine

Labels the type of panic event, such as liquidity panic, oracle dislocation, whale exit event, bridge outflow event, or systemic market stress.

8. Alert Engine

Creates risk alerts for protocols, dashboards, SDK clients, and off-chain notification channels.

9. Oracle Publisher

Takes the computed risk output and submits it to the on-chain Risk Oracle Program.

10. Risk Oracle Program

Stores the latest canonical risk state on Solana so integrated protocols can read it directly.

## Production Flow

```txt
Data Ingestion
  -> Market State Builder
  -> Signal Engine
  -> Stress Score / Liquidity Health / Depeg Probability
  -> Panic Classification
  -> Oracle Publisher
  -> On-Chain Risk Oracle Program
  -> Circuit Breaker Layer / Protocol Integrations
```
