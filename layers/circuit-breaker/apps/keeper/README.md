# Circuit Breaker Keeper

Rust worker for circuit breaker automation.

Responsibilities:
- read latest risk state from the Risk Oracle Program
- evaluate circuit breaker policy decisions
- submit authorized control updates
- move protocols from panic protection into recovery mode when stress decreases
- log all actions for audits and incident review

Initial runtime shape:
- loads keeper configuration
- starts a polling loop
- evaluates active risk state
- submits eligible circuit breaker updates
- handles shutdown cleanly
