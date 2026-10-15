# Solution architecture

Purpose: define how Cinis stays financially coherent while growing into a researchable Rust/.NET system.
Status: existing ownership and matching policies Accepted; target processes and new extension recommendations Proposed unless explicitly marked Deferred.
Owner: project maintainer. Reviewed: 2026-10-15. Baseline: 6cf0828.

## Architectural thesis

Cinis is a Rust/.NET trading infrastructure lab for deterministic order execution, financial correctness, durable recovery, and reproducible performance research. v0.1 is a single-market spot sandbox using synthetic funds. The architecture optimizes first for explainable decisions and reproducible evidence. It has three product capabilities: a Rust execution authority, a .NET access/management platform, and a research toolkit. These are responsibility boundaries; they do not imply three independently deployed services today.

The core rule is one writer for a coherent financial state. Matching, reservations, settlement, balances, ledger transfers, and retained command outcomes belong to one Rust owner. A gateway or PostgreSQL balance row cannot authorize spending. Adding a message broker or database does not create this correctness property.

## Actors and value paths

| Actor | Primary journey | Boundary |
| --- | --- | --- |
| Developer/researcher | Run fixtures, inspect events and ledger, replay a seed, compare implementations | Local tools; reproducibility manifests |
| Strategy client | Submit/cancel synthetic orders; query outcomes and balances; consume book updates | Future authenticated gateway; no direct state mutation |
| Sandbox administrator | Provision account access, issue synthetic funds, pause trading | Management metadata plus privileged sequenced engine commands |
| Operator | Start/stop, inspect readiness/logs, recover and reconcile | Restricted ops interfaces; no editing balances or journal to make recovery pass |
| Aegis or external identity system | Optional identity/access integration | Adapter contract; Cinis must run independently |

The target acceptance journey is fund -> rest sell -> partial buy -> same-ID retry -> restart/recover -> cancel remainder -> reconcile. The existing demo covers the in-memory portions only. Real deposits, withdrawal, custody, derivatives, routing to real venues, and blockchain finality require a separate business/security scope.

## Bounded contexts and responsibilities

| Context | Responsibilities | Owns | Must not do | Delivery |
| --- | --- | --- | --- | --- |
| Execution | Admit trusted commands; deduplicate; sequence; preflight; match; reserve/settle/release; reconcile | Authoritative book, financial state, market pause, receipts/counters | Ask asynchronous projections whether funds are available | Implemented in memory |
| Access and management | Authenticate keys, map principal to accounts/scopes, manage revocation and descriptive metadata | Identity/access records | Let clients supply trusted engine permissions; mutate financial tables | Proposed M3 |
| Execution transport | Bounded framing, correlation, reconnect, deadlines, backpressure | Connection/queue state only | Treat socket receipt or timeout as final order outcome | Proposed M0 spike/M3 |
| Persistence/recovery | Journal append/sync; snapshots; deterministic replay; compatibility checks | Durable execution history under Rust authority | Publish durable success before tested storage boundary | Intended M4 contract |
| Query projection | Apply versioned events idempotently; expose cursor/lag; rebuild | Derived PostgreSQL read models | Correct engine state from a read model | Proposed M3, recoverable in M4 |
| Market data | Consistent snapshot plus ordered deltas; gap recovery; bounded subscriptions | Derived book and stream cursor | Block execution behind a slow client | Proposed M5 |
| Research | Seeded workloads, oracle comparisons, environment manifests, benchmark reports | Fixtures/experiments/artifacts | Present matching-only results as end-to-end durable throughput | Partly implemented; tools expand later |
| Operations | Logged tasks, process ownership, health/readiness, recovery drills | Runtime lifecycle and diagnostics | Confuse diagnostic logs with a financial journal | Runner implemented; services planned |

## Current execution boundary

ExecutionCore is a synchronous safe-Rust library with private matcher and ledger state. The public matching Engine is a research primitive that bypasses financial admission; a future service must expose ExecutionCore, not the raw matcher. Permissions are trusted local flags, not authentication.

For a fresh authorized request, the core checks identity/result capacity, allocates a global execution sequence, clones tentative state, applies the complete financial transition, reconciles invariants, and swaps state only on success. Business rejections retain a sequence and original result while leaving financial/book/market state unchanged. Admission failures do not consume sequence. Same account/command ID plus equal typed payload returns the original response after permissions are checked again. A changed payload conflicts. Returning historical events is not publishing them again.

The accepted-place priority ordinal is distinct from the global command sequence. Funding and business rejections consume execution sequence but do not acquire order priority. IDs and event/posting ordinals are stable identities; timestamps are diagnostics and never matching priority.

This design copies state and reconciles full history. Its clarity is deliberate; it is not an allocation-free or low-latency implementation. It loses book, funds, receipts, and history on process exit.

## Target request and event contracts

External clients use an authenticated management/gateway API. Proposed HTTP operations separate commands from queries: submit/cancel/fund/pause return a correlated execution result; queries return a read model with applied cursor. Endpoint paths, response codes, request limits, and long-running status behavior remain protocol decisions, not frozen APIs here.

Gateway flow: authenticate -> authorize account and action -> validate bounded shape -> forward a trusted envelope. Rust repeats ownership/business checks and alone decides reserve/settle. A local socket is not an authentication mechanism: restrict service access, bind loopback, and define trusted caller identity before exposing a listener. Remote engine links require an explicit service-authentication/TLS design.

The existing direction is bounded length-prefixed loopback TCP. The spike must choose byte order, maximum frame size, version handshake, encoding, canonical payload equality, overflow handling, and reconnect behavior. No unrestricted client may set Permissions. Cross-language u64 values need exact handling; public JSON should use documented decimal strings for IDs/atoms if JavaScript clients are supported. That representation is Proposed, not a change to the in-process API.

A timeout means unknown outcome. Retry the same identity and semantic payload; never automatically mint a new command ID. A gateway must not turn a transport failure into a business rejection. Scope revocation can deny a cached result, consistent with current authorization-before-dedup behavior; a separately authorized status-query policy needs review.

Events carry schema version, execution sequence and ordinal, command/account correlation, and financial/order facts. Consumers must not infer a missing execution sequence means a lost event: business rejects can produce no events. Define complete command-batch envelopes or explicit progress records for durable cursor advancement. A ledger posting ID and an event ID are separate domains even when their numeric pairs coincide.

## Authority and consistency

Management storage and execution storage have separate transaction boundaries. Account access provisioning does not imply funding. Management-to-engine actions use correlated, retryable commands and observable status; no distributed transaction is assumed. If access creation succeeds and funding fails, the account remains unfunded and the administrator can retry the same funding intent.

PostgreSQL has logically separate management and projection schemas/roles. The initial platform may use one database instance for simplicity, but projection writes cannot alter management permissions, and neither role writes execution authority. Apply a complete event batch and consumer cursor in one database transaction. A unique event identity plus idempotent application prevents duplicate effects. A crash rolls back both rows and cursor. Rebuild into a new projection version and publish only at a consistent cursor. Stale reads expose lag; they are not valid pre-trade risk checks.

Current responses commit in memory. The intended M4 order is journal -> tested sync boundary -> deterministic apply -> reply; durable history also lets consumers recover events after a lost publication. Persist enough configuration, receipts, rejections, counters, and format/semantic versions to reproduce exactly the same results. Wall clocks, random iteration order, external queries, and current permission databases must not alter replay. Journaled trusted admission facts and changes to business semantics require an ADR before implementation.

## Deployment and operations

Today: CLI plus Rust library in one process, with versioned fixtures and logged Windows tasks. No listener, database, .NET host, or container orchestration exists.

Proposed first integrated topology: one host running a .NET modular application (management, gateway, projection worker, later market-data adapter), one Rust execution process, and PostgreSQL. Separate components can share the .NET host initially; each box is not automatically a microservice. Journal/snapshots belong on a restricted engine-owned persistent volume. Diagnostic logs go to ignored artifacts locally and a selected sink later. Database storage, execution history, secrets, and logs have independent lifecycle/backup policies. Docker Compose is a candidate reproducible local topology once actual services exist; native logged tasks remain useful for debugging. Kubernetes is Deferred.

Readiness must distinguish liveness from ability to accept trading: engine recovery completed, invariants checked, storage writable, ingress policy established. Projection lag degrades queries but need not stop financially safe execution; access-store failure should deny new authorization by default unless an explicit bounded cache/revocation policy is accepted. Journal failure stops new admission. Graceful shutdown stops ingress, drains accepted work under a bounded policy, syncs history, and records the final cursor. Force-stop recovery must be tested separately.

## Growth without changing authority accidentally

| Extension | Safe seam | Required gate |
| --- | --- | --- |
| New order types/fees | Versioned deterministic command/state transition | Business price/priority/rounding rules, fixtures, replay compatibility |
| More markets | Initially keep one owner across shared account assets | Define global reservation authority before parallel market writers |
| More throughput | Measured collection/state-transaction changes | Differential equivalence, financial invariants, reproducible benchmark |
| Independent consumers | Recoverable event feed and isolated cursors | Schema evolution, retention, replay and slow-consumer policy |
| Replication/failover | One fenced active financial leader | Consensus/commit semantics, split-brain tests, recovery and RPO/RTO decisions |
| Aegis integration | Replaceable identity adapter | Principal/account mapping, scopes, revocation, independent sandbox mode |
| Blockchain settlement | Separate external settlement workflow/state | Finality/reorg/custody model; trading fill is not chain finality |

Do not shard only by market while allowing each shard to spend the same account balance independently. Do not run two active execution processes against the same journal. Read replicas and message delivery guarantees cannot substitute for a single financial authority.

See [diagrams](diagrams.md), [technology choices](technology-and-algorithms.md), [decision register](decision-register.md), and [quality gates](quality-and-gates.md).
