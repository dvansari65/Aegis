# Risk Oracle Crate

Core domain crate for the Risk Oracle Layer.

This crate should contain deterministic business logic only:
- market state construction
- signal computation
- stress scoring
- liquidity health classification
- depeg probability estimation
- panic classification
- alert generation
- explainability metadata

It should not own network polling, database writes, or Solana transactions. Those responsibilities stay in ingestion services, storage services, and publisher apps.

