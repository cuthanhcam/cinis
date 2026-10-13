# Shared verification

[Matching fixtures](fixtures/matching/README.md) define initial state/configuration, commands, expected events/rejections, and complete final order state. The Rust integration runner in src/engine/tests/scenarios.rs consumes these exact files.

[Full-system acceptance scenarios](../docs/verification/scenarios.md) still include planned financial, gateway, and recovery behavior. Passing matching fixtures does not complete those scenarios.
