<div align="center">
  <img src="layers/risk-oracle/apps/frontend/public/aegis-logo.svg" alt="Aegis Logo" width="300" />

  <h1>Aegis</h1>
  <p><strong>A Real-Time Stablecoin Risk-Control & Circuit Breaker Protocol on Solana</strong></p>
</div>

---

## 🛡️ Overview

**Aegis** is designed to detect panic-driven instability in stablecoin markets before liquidity collapse spreads across DeFi protocols. Stablecoins are the foundation of decentralized finance, but during market panic, rapid withdrawals and arbitrage exhaustion can lead to systemic failures.

Aegis acts as:
1. **A Real-Time Risk Oracle**: Ingesting live, on-chain state to measure market panic.
2. **A Market Stress Engine**: Mathematically computing liquidity health and depeg probabilities.
3. **An Automated Stabilization Layer**: Halting withdrawals or scaling fees via an on-chain Circuit Breaker.

## 🏗️ Architecture

Aegis is composed of two primary layers:

### Layer 1: Risk Oracle
A real-time data pipeline and risk engine that calculates system panic scores (0-100).
- **`core`**: Pure Rust crate containing the mathematical formulas for Depeg Probability, Liquidity Health, and Stress Scoring.
- **`ingestion`**: Async workers polling Pyth Price feeds, DEX Spl-Token pools, and tracking Whale exits.
- **`oracle-publisher`**: The off-chain worker that securely pipes ingestion data into the core scoring engine, and subsequently updates the Solana smart contract.
- **`risk-oracle` (On-Chain)**: Highly optimized Solana program built with `pinocchio` (zero-allocator) that persistently stores the `RiskState` so DeFi protocols can compose with it.

### Layer 2: Circuit Breaker (In Progress)
The defensive infrastructure layer. Protocols will route their withdraw/deposit actions through this layer. If the Risk Oracle reports a critical Stress Score (>80), Aegis will automatically trip the breaker, halting outflows to preserve protocol insolvency.

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)

### Building the Smart Contracts
Aegis smart contracts are optimized using `pinocchio`. To build the programs:
```bash
cargo build-sbf
```

### Running the Test Suite
The core risk engine contains comprehensive unit tests for mathematical boundary verifications:
```bash
cargo test --workspace
```

### Running the Oracle Publisher
To simulate the end-to-end Layer 1 pipeline:
```bash
cargo run -p risk-oracle-publisher
```

## 📜 License
MIT
