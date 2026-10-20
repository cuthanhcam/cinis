# Backlog

This file is the single work-item source. GitHub Issues are not maintained in parallel. Milestones describe outcomes and point here. Status: Todo / In progress / Done / Deferred; Done requires linked evidence.

| ID | Work item | Dependencies | Acceptance / evidence | Estimate | Status |
| --- | --- | --- | --- | --- | --- |
| M0-01 | Review definition and decisions | none | M0 decision register and ADR-0002 | 2-4 h | Done (specification) |
| M0-02 | Requirements and ownership | M0-01 | FR/QR acceptance and sole state owners | 3-6 h | Done (specification) |
| M0-03 | Structured matching examples | M0-01 | Initial state, commands, events, final state in shared fixtures | 3-6 h | Done: 17 shared MC fixtures |
| M0-04 | Cross-language spike | M0-01 | Framing/disconnect/correlation report; resolve protocol questions | 4-8 h | Todo |
| M1-01 | Rust crate and pinned toolchain | M0-01 | Build, rustfmt, Clippy, tests, CI workflow | 2-4 h | Done: crate, pin, lockfile, workflow; local checks pass |
| M1-02 | First matching flow | M1-01, M0-03 | Sell 10, buy 4, cancel 6; demo and invariant checks | 4-8 h | Done: fixture tests and CLI verified locally |
| M1-03 | Matching edge cases | M1-02 | Priority/FIFO/multifill/cancel/invalid/self-trade/capacity fixtures | 4-8 h | Done: fixture tests and CLI verified locally |
| M1-04 | Independent reference and generated streams | M1-03 | differential.rs: 8,192 generated commands plus targeted reference test; verification report | 4-8 h | Done (local evidence) |
| M1-05 | M1 review and estimate refresh | M1-04 | m1-review.md records local evidence/limits; maintainer review and effort calibration pending | 2-4 h | In progress |
| M2-01 | Ledger and financial limits | M1-03 | architecture/ledger.md and ADR-0005 | 3-6 h | Done (specification) |
| M2-02 | Financial state and command deduplication | M2-01 | financial.rs, financial_properties.rs, core/demo, financial-testing.md; volatile INV-001/002/004/006 | Effort not measured | Done (in-memory implementation; hosted gate required for publication) |
| M3-01 | Gateway/auth/query integration | M0-04, M2-02 | FR-004 and end-to-end orders; independent from Aegis | Refine at M3 | Todo |
| M4-01 | Durability and crash recovery | M2-02 | Storage ADRs and crash matrix | Refine at M4 | Todo |
| M5-01 | Market data and operations | M4-01 | Gap/resync, bounded clients, verified runbooks | Refine at M5 | Todo |
| M6-01 | Full demo and baseline report | M5-01 | Full scope journey; benchmark-environment.md plus raw evidence | Refine at M6 | Todo |

Actual focused effort has not been measured; do not substitute elapsed assistant runtime for engineering estimates. Financial scenarios now have in-memory evidence; durability and network scenarios remain pending.

M1-01 CI workflow is committed; local success is recorded in testing.md. Remote CI execution must be checked separately. M1-04 has local evidence; M1-05 remains open for maintainer review and effort calibration.

## Developer operations

| ID | Work item | Evidence | Status |
| --- | --- | --- | --- |
| OPS-01 | Local task runner and diagnostic logs | ops/scripts, seven integration tests; Verify/Run pass locally | Done |
| OPS-02 | Structure, documentation standard, runbooks, Windows CI | ADR-0003, docs/operations and ops workflow; hosted result tracked separately | Done (implementation) |
| OPS-03 | Persistent service lifecycle, readiness, shutdown, rotation | Add with actual gateway/execution service; docs/operations/service-lifecycle.md | Deferred to M3/M4 |
| OPS-04 | Language-aware naming and unified automation paths | ADR-0004, naming-conventions.md, check_naming.py, updated task/CI callers | Done (implementation; local verification recorded with change) |
| OPS-05 | Hosted CI repair and comprehensive repository scaffold | Host-native module imports, CI regression/gate tests, workflow/scaffold validation, GitHub/community/config enhancements | Implementation complete; hosted exact-SHA gate required before develop |

## Architecture review before further implementation

| ID | Work item | Dependencies | Acceptance / evidence | Estimate | Status |
| --- | --- | --- | --- | --- | --- |
| ARCH-01 | High-level solution architecture package | Existing M2 contracts | [Blueprint](../architecture/blueprint/README.md): current/target diagrams, offline Atlas, technology/algorithm options, authority/failure boundaries | Effort not measured | Done (documentation; new recommendations Proposed) |
| ARCH-02 | Accept business/access and wire-contract choices | ARCH-01, M0-04 spike evidence | Close BD-01/02/03 and TD-01/02/03/04 in [decision register](../architecture/blueprint/decision-register.md); canonical contracts/ADRs and acceptance cases updated | Refine after review | Todo; gates expanded M3 implementation |
| ARCH-03 | Accept durability, capacity and feed choices | ARCH-01, M2-02 | Close BD-04/05 and TD-06/07/08 with storage/feed spike and crash acceptance matrix | Refine after review | Todo; gates M4 implementation |

The architecture register records decision options/status; this backlog remains the only implementation work-item tracker. Publishing the package does not accept new business or technical policies.

## Rust engine maintainability and adapter preparation

| ID | Work item | Dependencies | Acceptance / evidence | Estimate | Status |
| --- | --- | --- | --- | --- | --- |
| CORE-01 | Cohesive private Rust modules and one-way dependency flow | M2-02, ARCH-01 | [Rust engine guide](../development/rust-engine.md), ADR-0006; private ledger state/errors; existing 56 tests unchanged | Effort not measured | Done (implementation; hosted publication gate required) |
| CORE-02 | New/retained invocation metadata for future adapters | CORE-01 | execute_with_outcome, five outcomes.rs tests, compiled example, financial demo; existing execute behavior preserved | Effort not measured | Done (in-process only; no publisher/outbox) |

The next functional gate remains M0-04/ARCH-02 before the full M3 gateway. These internal/API changes do not accept unresolved business policies or claim network/durable behavior.

## .NET architecture planning

| ID | Work item | Dependencies | Acceptance / evidence | Estimate | Status |
| --- | --- | --- | --- | --- | --- |
| NET-01 | Define platform patterns, boundaries and evolution plan | ARCH-01, CORE-01/02 | [Platform architecture](../architecture/platform-architecture.md), dependency/flow diagrams, [phase gates](../architecture/platform-implementation-plan.md), ADR-0007 | Effort not measured | Done (plan; architecture accepted separately) |
| NET-02 | Review platform architecture baseline | NET-01 | Close TD-13 explicitly in ADR-0007; agree module/project graph, transaction authority, worker roles and extraction criteria | Refine after review | Done: maintainer authorized solution/modules on 2026-10-17; ADR-0007 accepted |
| NET-03 | Implement actual .NET baseline and architecture checks | NET-02; bootstrap ordering authorized before M0-04/ARCH-02 | [Platform guide](../development/platform.md), .slnx, 37 tests, locked dependencies, health/smoke/foreground logs, Platform CI | Effort not measured | Done for fail-closed bootstrap; full P2/P3 protocol/auth gates remain open |

P0-P5 describe implementation order and acceptance gates; this table and existing M0/M3/M4 work items remain the single tracker. The maintainer authorized the scaffold first on 2026-10-17; ADR-0008 records this bootstrap ordering without accepting business/wire policies.

| ID | Next implementation | Dependencies | Acceptance | Status |
| --- | --- | --- | --- | --- |
| NET-04 | Trusted local grant/cancel slice and experimental bounded codec | NET-03 | Access/Trading/ExecutionClient tests; denied caller never reaches execution; exact intent and transport certainty preserved; no financial HTTP route | Done (in-process only) |
| NET-05 | Actual Rust transport and authenticated endpoint | M0-04, ARCH-02, NET-04 | Accepted protocol/trust/access contracts, round-trip/disconnect/retry evidence and real credential/account isolation | Todo |

## Platform transport groundwork

| ID | Work item | Dependencies | Acceptance evidence | Status |
| --- | --- | --- | --- | --- |
| NET-06 | Bounded experimental loopback channel | NET-04 | Single-flight admission, bounded frames/deadlines, conservative send certainty, fresh-connection recovery and real socket tests; [guide](../development/platform-transport.md) | Done; does not close NET-05 |
| NET-07 | Maintainable documented platform contracts | NET-04 | Compiler-enforced production XML documentation, generated-file architecture check, constructor guards and UTC scoped JSON logging | Done |

| NET-08 | HTTP diagnostic foundation | NET-07 | Host middleware/filter/error handling, allowlisted JSON scopes, correlated 500/404/cancellation and no-store tests; [pipeline](../development/platform-http-pipeline.md) | Done for health scaffold |
| OPS-LOG-01 | Repeatable service logs | Existing runner | Latest overwrite, exclusive locking, Archive opt-in and child run correlation; Windows regression tests | Done for local debugging |
