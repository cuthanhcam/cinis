# Changelog

## Unreleased

- Added repeatable Latest service logs with exclusive writer locking, Archive opt-in, child run identity and ignored diagnostic outputs.
- Added host-owned request middleware, scoped JSON formatter, generic correlated Problem Details, health no-store filter and localhost defaults; expanded to 60 .NET and 21 ops tests.

- Added an experimental bounded single-flight loopback TCP channel with conservative send certainty, deadlines, fresh connections and real socket tests; financial Rust integration remains pending.
- Enforced public production XML documentation during compilation, documented contracts and failure semantics, added dependency guards and UTC scoped JSON logging.

- Added the .NET 10 platform .slnx with five production/five test projects, central locked package/build policy, 37 tests, health-only host, local grant/cancel application slice and experimental bounded framing.
- Added Platform Linux/Windows CI as a required fifth workflow, logged platform Verify/Run/Serve tasks, deny-by-default bootstrap documentation and architecture approval scope.

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

Financial execution is volatile. Authenticated network trading, database projections and durable recovery remain undelivered; no release is claimed.
