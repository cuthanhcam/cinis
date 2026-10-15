# Cinis repository instructions

Read [AGENTS.md](../AGENTS.md), [documentation index](../docs/README.md), and the affected specifications. The current code is a volatile, matching-only Rust prototype. Do not imply financial, gateway, or recovery guarantees exist.

Use the naming policy and single state owner. Repository automation belongs under ops/scripts/; operational documentation belongs under docs/operations/. Keep the backlog canonical, preserve ignored init.md, and never commit generated logs or secrets.

Run relevant local checks and verify all required GitHub workflows on the exact integration commit before updating develop. Local success is insufficient hosted evidence. Do not disable a failing check or use continue-on-error to make integration appear green. Publication follows the user's authorization and the Git workflow.
