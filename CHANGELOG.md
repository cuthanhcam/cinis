# Changelog

## Unreleased

- Added ExecutionCore as the atomic financial entry point with synthetic funding, checked account/supply limits, reservations, settlement/release, balanced ledger, market pause, and account-scoped command idempotency.
- Added 24 financial scenarios, four fixed-seed financial streams, two execution boundary tests, ledger/ADR/API documentation, and a funded default CLI with a preserved matching-only binary.
- Split matching, ledger, execution contracts/state, and sequencing ownership into cohesive Rust modules; all state remains volatile.

- Fixed the hosted pwsh-to-Windows-PowerShell module-resolution failure and added shadowed-module regression coverage.
- Run required workflows on all pushed branches, test Rust on Linux/Windows, pin current Node 24 actions, and bound/consolidate CI runs.
- Added pinned actionlint, scaffold/ignore checks, and a read-only exact-commit CI integration gate with regression tests.
- Expanded onboarding/contribution/security guidance, GitHub ownership/intake/dependency configuration, shared editor settings, and language-aware ignore/text policy.

- Restored docs/operations/ while retaining ops/ for automation; consolidated portable checks under ops/scripts/ and updated CI/task callers.
- Added a language-aware naming standard and Git-inventory path/case-collision checks.

- Added an independent map-based matching oracle, eight fixed-seed generated tests (8,192 commands), and targeted cancellation/terminal differential checks.

- Added ops/ Windows automation for build/test/run, per-run diagnostic logs and metadata, timeout/owned-process cleanup, and seven process integration tests.
- Added Windows ops CI, repository/documentation conventions, templates, and detailed local operations/debugging guides.

- Added Cinis project definition, domain baseline, architecture, verification strategy, and delivery plan.
- Added repository conventions, contribution templates, documentation checks, and local branch workflow.

- Added requirements, state ownership, M0 decision review, explicit matching policy, verification strategy, and a single backlog.
- Added a pinned safe-Rust matching-only crate, CLI demo, 17 shared executable scenario fixtures, and engine CI.

No financial, gateway, or recovery implementation and no release has been delivered.
