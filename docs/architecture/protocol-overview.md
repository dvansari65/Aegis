# Protocol Overview

The Stablecoin Panic Control Protocol is designed as two separate but connected layers.

## Layer 1: Risk Oracle

Layer 1 observes the market and produces trusted risk state.

Responsibilities:
- collect stablecoin market data
- build current market state
- compute risk signals
- score panic severity
- estimate depeg probability
- publish risk feeds on-chain
- notify off-chain consumers

Layer 1 does not control liquidity or user actions. It only detects and publishes risk.

## Layer 2: Circuit Breaker

Layer 2 consumes risk oracle output and turns it into defensive actions.

Responsibilities:
- activate adaptive fees
- throttle risky withdrawals
- coordinate liquidity routing
- suppress toxic arbitrage patterns
- trigger emergency liquidity policies
- expose integration modules for DEXs, lending markets, bridges, and stablecoin pools

Layer 2 should depend on Layer 1 outputs, but Layer 1 should not depend on Layer 2.

## Data Flow

```txt
Solana / Oracle / DEX / Bridge Data
  -> Layer 1 Risk Oracle
  -> On-Chain Risk Feed
  -> Layer 2 Circuit Breaker
  -> Integrated Protocol Actions
```

