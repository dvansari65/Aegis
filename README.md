<table align="center" cellpadding="28" cellspacing="0">
  <tr>
    <td align="center" bgcolor="#F8FAFC">
      <img src="docs/public/aegis-logo.svg" alt="Aegis" width="280" />
    </td>
  </tr>
</table>

<h1 align="center">Aegis</h1>

<p align="center">
  <strong>Real-time stablecoin risk control and circuit breaker infrastructure on Solana.</strong>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-workspace-orange?logo=rust&logoColor=white" alt="Rust" /></a>
  <a href="https://solana.com/"><img src="https://img.shields.io/badge/Solana-programs-9945FF?logo=solana&logoColor=white" alt="Solana" /></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License MIT" /></a>
</p>

---

## Overview

**Aegis** detects panic-driven instability in stablecoin markets before liquidity stress spreads across DeFi. During withdrawals, oracle dislocations, and bridge outflows, markets can move faster than human operators can react.

Aegis is built as three cooperating roles:

| Role | What it does |
|------|----------------|
| **Risk oracle** | Ingests live market and on-chain signals, scores stress, and publishes machine-readable risk state. |
| **Stress engine** | Combines signals into liquidity health, depeg probability, and a normalized stress score. |
| **Circuit breaker** | Maps severe risk state into defensive controls (fees, throttles, routing) for integrated protocols. |

---

## Repository layout

| Path | Description |
|------|-------------|
| `layers/risk-oracle/crates/core` | Deterministic risk math (stress, liquidity health, depeg probability). |
| `layers/risk-oracle/crates/ingestion` | Data collection and normalization. |
| `layers/risk-oracle/apps/oracle-publisher` | Off-chain publisher that updates on-chain risk state. |
| `layers/risk-oracle/apps/api` | HTTP API for off-chain consumers (see that crate’s README for routes). |
| `layers/risk-oracle/programs/risk-oracle` | Solana program storing canonical risk state (`pinocchio`). |
| `layers/circuit-breaker/programs/circuit-breaker` | Solana program for circuit breaker control state. |
| `layers/circuit-breaker/apps/keeper` | Off-chain automation for policy execution. |
| `docs/` | Next.js marketing and documentation site. |

---

## Architecture (high level)

```text
Market / oracle / DEX / bridge data
  → Risk oracle (off-chain + ingestion + core)
  → On-chain risk oracle program
  → Circuit breaker program + keeper
  → Integrated protocol actions
```

---

## Getting started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)

### Build on-chain programs

Programs use `pinocchio` and the Solana SBF toolchain:

```bash
cargo build-sbf
```

### Run tests

```bash
cargo test --workspace
```

### Run the oracle publisher (Layer 1 pipeline)

```bash
cargo run -p risk-oracle-publisher
```

### Documentation site (optional)

```bash
cd docs
npm install
npm run dev
```

---

## License

MIT
