# Technology and algorithm decisions

Purpose: recommend a small baseline stack and explain when more complex techniques become justified.
Status: implemented choices are distinguished from Proposed/Deferred candidates. Owner: project maintainer. Reviewed: 2026-10-15.

## Technology portfolio

| Area | Baseline / recommendation | Status and rationale | Alternative / revisit trigger |
| --- | --- | --- | --- |
| Execution language | Safe Rust; pinned 1.95.0, Cargo.lock committed | Implemented. Ownership and checked integers support a deterministic synchronous core | Upgrade through CI/equivalence evidence; do not upgrade just because reference docs show a newer compiler |
| Core runtime | Synchronous library, one mutable owner; no async inside matching/ledger | Implemented. Explicit ordering and testable atomic transitions | Async network adapter outside core in M3; Tokio is a candidate, not a required core dependency |
| Management host | C# / ASP.NET Core on .NET 10 LTS; modular application first | Proposed M3. HTTP, policy authorization, background workers and ecosystem fit existing direction. Pin exact SDK in global.json only with actual projects | Recheck supported servicing SDK/OS at implementation; separate services only when independent scaling/failure evidence warrants it |
| Management/projection data | PostgreSQL; separate schemas/roles; Npgsql with EF Core for metadata and explicit transactional SQL where projection semantics benefit | Proposed M3. Relational constraints, migrations, transactionally applied effects/cursor | Choose supported major and exact dependencies with spike; do not use DB balances as engine risk authority |
| Gateway to Rust | Bounded length-prefixed loopback TCP | Existing draft direction; framing and wire encoding still Proposed. Simple cross-platform starting point, including Windows | Compare schema-defined Protobuf payloads with explicit bounded binary encoding; gRPC is a useful comparison spike, not a second mandatory protocol |
| External API | HTTPS commands/queries; later WebSocket snapshot/deltas | Proposed. Separate authoritative results from lagging queries | Define schemas, errors, request sizes, scopes and cursor behavior before publishing SDKs |
| Auth/access | High-entropy API keys, verifiable secret hashes, account/action policies and revocation | Proposed; M2 local permissions are not authentication. Hash choice, key format, storage and revocation SLA remain security decisions | Aegis/OIDC adapter later. Avoid requiring an external identity system to run a synthetic sandbox |
| Execution persistence | Engine-owned append-only journal plus verified snapshots | Intended M4, not implemented. One recovery authority for all financial state and receipts | Library/custom record layout decision needs crash spike; PostgreSQL WAL protects PostgreSQL, not separately held Rust state |
| Event delivery | Recoverable execution history -> idempotent consumers; no broker in first baseline | Proposed. Removes an unnecessary additional durability boundary | NATS/RabbitMQ/Kafka only after measured fan-out/retention/operational needs; none replaces engine journal or cursor contracts |
| Observability | Existing redacted per-run logs; proposed structured service logs, metrics and trace correlation | Logs implemented; OpenTelemetry/Collector optional later. Instrument outside deterministic transition where practical | Add Prometheus/Grafana or another backend with actual telemetry needs; telemetry outage must not corrupt execution |
| Research/tools | Rust fixtures/oracles; Python standard-library orchestration/reporting initially | Partly implemented. Seeds, manifests, raw output and revision IDs make results inspectable | Add statistical/plotting libraries only with an experiment; no Python financial authority in the live path |
| Local deployment | Native logged tasks now; Docker Compose candidate once services exist | Current runner implemented; Compose Proposed, Kubernetes Deferred | Multi-host orchestration only with deployment/HA requirements and operator capacity |
| CI | Existing GitHub Actions Linux/Windows, pinned actions, exact integration-SHA gate | Implemented. All four workflows must pass before develop moves | Add .NET restore/build/test and SDK pin together with real solution; do not create passing placeholder jobs |

.NET 10 is currently listed as LTS through November 2028 by [Microsoft support documentation](https://learn.microsoft.com/en-us/dotnet/core/releases-and-support). This is a version-family recommendation, not an installed .NET project. Patch/SDK/OS selection remains an implementation gate.

ASP.NET Core supplies [policy-based authorization](https://learn.microsoft.com/en-us/aspnet/core/security/authorization/policies?view=aspnetcore-10.0); Cinis must still define its own account/resource policies and test them. [Microsoft's gRPC guidance](https://learn.microsoft.com/en-us/aspnet/core/grpc/performance?view=aspnetcore-10.0) discusses connection/channel reuse and HTTP/2 constraints. It informs a transport comparison, not proof that gRPC will be faster for Cinis.

## Matching structures and complexity

Let N be retained orders (including terminal records in the current bounded design), L active price levels, Q orders at a price level, F fills produced, H ledger-transfer history, A accounts, and R retained command results. Complexity below describes individual structures, not measured command latency.

| Technique | Current or candidate behavior | Trade-off / gate |
| --- | --- | --- |
| Vec-backed order store and candidate sort | Implemented. Scan retained orders, build crossing candidates, sort by price/accepted priority; conservative O(N log N) matching planning upper bound plus fills and other validation | Easy inspection and reference testing; repeated scans, allocation and terminal retention grow work. Do not claim O(log N) execution |
| BTreeMap<price, price-level> plus FIFO queue | Proposed first optimization. Tree lookup/insert/remove O(log L); queue endpoints amortized O(1), with costs for traversed levels/fills | Standard safe collections; preserves explicit price order. A plain VecDeque arbitrary cancel/removal can be O(Q), not O(1) |
| Order-ID index | Proposed alongside indexed book; HashMap expected O(1) lookup or BTreeMap O(log N) | Lookup alone does not make unlink/cancel constant-time. Keep terminal ID semantics; never depend on hash iteration order for outcomes |
| Arena with generation handles and linked level membership | Deferred until measurement. Can support direct unlink with validated handles | More invariants/free-list complexity; stale handle protection and safe Rust design required. No automatic zero-allocation guarantee |
| Binary heap | Not preferred baseline for a cancel-heavy limit book | Best-priority access fits a heap, but arbitrary removal, stale entries and FIFO maintenance need extra machinery |
| Whole-state tentative clone and full reconciliation | Implemented financial atomicity baseline; work grows with orders, accounts and history; full core checks also inspect receipts | Replacing it needs staged deltas/undo or preflight+infallible commit proof and failure injection. Changing the book alone will not remove this dominant work |
| Ledger aggregate and hold equality | Implemented checked postings plus independent per-asset aggregation and order-hold checks | Optimize incrementally only while retaining a separate full audit oracle and deterministic evidence |
| Receipt map keyed by (account, command) | Implemented BTreeMap, O(log R) key lookup, stored original typed payload/result | No expiry today. Payload/result copies and global checks add cost; M4 must preserve receipts across restart |

The standard library documents [collection behavior and operation costs](https://doc.rust-lang.org/std/collections/index.html). The costs above are architectural analysis for Cinis, not benchmark results. Actual financial transitions perform additional cloning, scanning, posting and validation. Inspect [matching.rs](../../../src/engine/src/matching.rs) and [execution](../../../src/engine/src/execution/mod.rs) before interpreting a microbenchmark.

## Deterministic algorithms and financial rules

1. Price-time matching: best contra price, then accepted-place ordinal; execute at resting price. The same ordered input/state must produce the same IDs, fills and final state. Client clocks do not resolve ties.
2. Atomic self-trade prevention: preflight the actual fill path and reject the whole incoming order if it would self-match. No earlier fill survives that rejection; scanning irrelevant liquidity must not create a false reject.
3. Reservation: buy holds limit-price notional in quote; sell holds remaining base. Use checked integer products and configured caps. Never use floating-point amounts.
4. Settlement: transfer filled base seller-reserved -> buyer-available and quote buyer-reserved -> seller-available. Release buy price improvement; cancel releases the remaining hold. Fees are explicitly zero in the accepted baseline.
5. Reconciliation: ledger source/destination movements balance per asset; postings reproduce available/reserved balances; active order requirements equal holds; issued supply equals account totals. Synthetic issuance is an explicit ledger counterparty.
6. Idempotency: authorization before receipt lookup; equal semantic payload returns the original decision; changed payload conflicts. Rejected business outcomes remain original even after later funding. Transport redelivery is allowed; repeated financial effect is not.
7. Persistence/replay (intended M4): framed versioned journal, validated sequence/integrity, tested sync, deterministic replay, snapshot plus suffix. Restore receipts/counters/configuration as well as book/balances. Checksums are integrity checks, not access control.
8. Projection (Proposed): consume complete committed batches, deduplicate event IDs, apply effects and advance cursor atomically. Market-data snapshots need a consistent cut and an independently defined delta cursor; gaps trigger catch-up/resync.

Canonical units, caps, permission behavior and business semantics remain in [trading rules](../../domain/trading-rules.md), [ledger](../ledger.md) and [ADR-0005](../decisions/0005-financial-execution.md). This list does not change them.

## Persistence alternatives and failure reasoning

A custom journal exposes recovery research directly but requires framing, integrity, partial-write handling, schema/version compatibility, sync/error semantics, snapshots, retention and crash tests. An embedded storage library can reduce some file mechanics but still needs atomic execution-state/receipt semantics and a deterministic recovery contract. A relational authoritative engine design is another architecture, not a transparent swap of an asynchronous projection database. Choose with a bounded spike and ADR before M4.

Rust [File::sync_all](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all) attempts to synchronize file content and metadata; it does not justify an untested end-to-end power-loss guarantee. [PostgreSQL reliability documentation](https://www.postgresql.org/docs/current/wal-reliability.html) explains the hardware/storage dependencies of durable writes. Cinis must record OS, filesystem, storage settings, sync APIs, record boundaries and tested fault model. Group commit is a later candidate: results wait until their commit group crosses the promised boundary; batching is not permission to reply early.

## Research portfolio and evidence boundaries

| Question | Candidate experiment | Required report |
| --- | --- | --- |
| Is book lookup the real bottleneck? | Vec baseline versus price-level BTreeMap/FIFO on identical seeded commands | Equivalent outputs/invariants, N/L/F distributions, allocation/memory and latency distributions |
| Can transaction cost be reduced safely? | Whole-state clone versus staged delta implementation | Differential financial results, injected failures, retained rejection/retry behavior |
| What does the gateway cost? | Core-only versus framed cross-process round-trip; optional gRPC comparator | Same workload, encoding/copies/queues, end-to-end versus core timing boundaries |
| What does durability cost? | Per-command sync versus bounded group commit after correctness | Sync policy, crash outcomes, response durability, hardware and p50/p95/p99 with raw data |
| Can consumers recover under load? | Delayed worker, duplicate batches, cursor gaps, slow WebSocket client | No duplicate effects, bounded queues, lag/resync behavior, recovery time with workload size |

Avoid invented numeric targets. Agree workload distributions, duration, warm-up, concurrency, seeds, measurement points, and acceptable latency/capacity only after a reproducible baseline. [OpenTelemetry Collector](https://opentelemetry.io/docs/collector/) can receive/process/export telemetry; it is not the execution journal. Instrumentation overhead belongs in the measured environment.

## Dependency adoption rule

Before adding a crate/package/service: identify the requirement, maintenance/security/license implications, failure boundary, exact version pinning/lock policy, compatibility tests and removal path. Favor the smallest stack that answers the current experiment. Advanced IPC, lock-free queues, SIMD, unsafe Rust, Redis, message brokers and Kubernetes are not default requirements simply because Cinis is a trading lab.
