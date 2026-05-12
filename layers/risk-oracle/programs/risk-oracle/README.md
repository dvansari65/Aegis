# Risk Oracle Program

Pinocchio-based Solana program that stores canonical stablecoin risk state.

Planned account state:
- stablecoin mint
- stress score
- liquidity health
- depeg probability
- panic classification
- confidence value
- last updated slot
- authorized publisher

The Circuit Breaker Layer will eventually read this program before activating protective controls.

