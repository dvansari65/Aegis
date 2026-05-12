# Oracle Publisher

Rust worker that publishes finalized risk outputs to the on-chain Risk Oracle Program.

Responsibilities:
- read risk outputs from the Risk Oracle Layer
- validate publisher authority
- submit Solana transactions
- retry failed updates safely
- record publish status for operators

This app should not compute risk scores. It should only publish already computed oracle state.

Initial runtime shape:
- loads publisher configuration
- starts a polling loop
- reads pending risk updates
- publishes eligible updates to Solana
- handles shutdown cleanly
