# Circuit Breaker Program

Pinocchio-based Solana program for circuit breaker control state.

Planned responsibilities:
- store protection mode
- store active control policies
- enforce authorized policy updates
- expose state that integrated protocols can read before executing risky actions

The program should consume risk output from the Risk Oracle Program, either directly or through authorized keeper updates.

