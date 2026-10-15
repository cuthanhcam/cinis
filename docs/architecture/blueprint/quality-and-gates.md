# Quality attributes and architecture gates

Purpose: define how the architecture is evaluated, including failure behavior, security, measurement and evolution.
Status: quality scenarios Proposed where not already required; implemented evidence linked explicitly. Owner: project maintainer. Reviewed: 2026-10-15.

Canonical quality requirements are [QR-001 through QR-006](../../product/requirements.md), and detailed financial properties are in [invariants](../../verification/invariants.md). This document links those expectations to architecture boundaries; it does not invent a production SLA.

## Quality scenario matrix

| Attribute | Stimulus / boundary | Required response and observable evidence | Current status |
| --- | --- | --- | --- |
| Financial correctness | Concurrent clients attempt to spend the same asset | Single sequencer makes ordered decisions; holds prevent overspend; every successful transition reconciles; rejection leaves no partial effects | In-process authority implemented; network concurrency test pending |
| Determinism | Replay same ordered inputs/configuration | Same decisions, IDs, ledger and final state; no client clocks/random iteration/external DB queries in transition | Matching differential and seeded financial tests implemented; durable replay pending |
| Idempotency | Reply lost; retry accepted or rejected command | Return original result after current authorization; no extra transfers or live publication | In-memory results implemented; transport/restart evidence pending |
| Durability | Crash around append/sync/apply/reply/publish/snapshot | Recover exactly committed history and receipts under stated storage fault model; no false durable ACK | Intended M4 only |
| Integrity | Committed history corrupted or incompatible | Fail closed, preserve evidence, no silent reset/truncation | Contract only |
| Access isolation | Read key trades; trader funds; wrong account cancels; revoked key retries | Deny unauthorized action before mutation; redact secrets; define in-flight cutoff | Trusted local flags/ownership implemented; real auth pending |
| Capacity | Queues/history/event/ledger bounds exhausted | Bounded memory/admission, explicit error and operational stop/recovery policy; no silent receipt eviction | Core caps implemented; accepted cancel-at-cap limitation requires review |
| Projection consistency | Worker crashes between effects and cursor; duplicate batch arrives | Atomic DB transaction; idempotent identities; cursor explains lag; no financial authority leak | Proposed M3/M4 |
| Market-data isolation | Slow client or gap | Bounded queue; detach/resync at consistent snapshot; no execution wait on WebSocket | Proposed M5 |
| Operability | Process starts, stalls, exits, is force-stopped | Owned process lifecycle; meaningful readiness; correlated redacted logs; tested restart/recovery runbook | Logged one-shot runner implemented; service behavior pending |
| Performance research | Workload or data structure changes | Reproducible raw measurements, environment, seed/revision, latency distribution and error counts; equivalence before speed claims | Correctness corpus present; performance baseline not established |
| Portability | Supported Linux/Windows build or storage behavior differs | Pinned builds/tests and explicit host-specific sync/snapshot evidence | Engine CI both hosts; Windows ops; persistence not tested |

## Failure policy by boundary

| Failure | Execution behavior | Client/operator consequence |
| --- | --- | --- |
| Invalid/oversized/unsupported frame | Reject before mutation and bounded allocation | Stable protocol error/connection policy; no business outcome inferred |
| Gateway disconnect after forwarding | Engine may have completed | Unknown outcome; retry same command ID/payload |
| Access store unavailable | Proposed deny new authorization unless bounded accepted cache policy | Explicit unavailable/unauthorized policy; never infer privileges |
| Business reject after admission | Retain original rejected result and sequence | New intent needs new ID; retry does not reevaluate after funding |
| Projection database down | Owner remains financially authoritative; feed retention bounds apply | Queries unavailable/stale with explicit cursor; catch up after recovery |
| Telemetry backend down | Do not corrupt execution; bound diagnostic buffers | Dropped-log/metric counters and operator signal, not unbounded queue |
| Journal append/sync error | Intended M4 stops new admission; never durable-success unrecorded work | Halt and investigate storage; preserve uncertain outcome for recovery |
| Committed corruption or invariant failure | Fail closed | Evidence-preserving recovery investigation; no automatic balance reset |
| Host/disk lost | Outside initial one-host durability guarantee | Restore from defined backup if available; no claimed HA/RPO without tests |

## Security and threat boundaries

Threat model starts at an untrusted client, proceeds through authenticated account/action admission, then a restricted service boundary to Rust. Length/version/numeric bounds are enforced at each parser. Gateway authentication does not remove engine ownership checks. Public requests cannot carry trusted permission flags. Engine/storage/database ports are not public endpoints in the proposed first topology.

Secret handling must cover key generation, display-once, verification hash, rotation/revocation, transport protection, least-privilege database roles and logs. Correlation IDs are not secrets; bearer keys and environment dumps must never be logged. Denial-of-service controls include request/frame limits, per-principal admission policy, bounded queues and finite retained state. Precise values and revocation/cache semantics require TD-02/BD-02 decisions.

Replay integrity and authentication differ: a checksum does not prove who wrote a record. Restrict storage access and define operator privileges/backup handling. A replay process restores accepted facts, rather than asking today's permission store to reinterpret yesterday's history.

## Measurement boundaries

Report separate stages: client-to-gateway; auth/admission; gateway-to-engine framing/queue; core apply; journal append/sync; reply; event export; projection apply; market-data delivery. Never combine a core-only in-memory number with a durable end-to-end claim. Record queue depth, rejects, retained-state size, allocations/memory, storage policy, warm-up, duration, concurrency, workload distribution and host/toolchain/runtime settings.

Use p50/p95/p99 and worst observed samples with sample counts/raw data, not only average throughput. No numeric latency, capacity, recovery-time or availability target is accepted by this blueprint. Establish a workload baseline, then agree budgets per boundary. Current default caps are safety bounds, not measured sustainable capacity. See [benchmark guidance](../../verification/benchmarks.md) and [experiment template](../../../research/experiment-template.md).

## Gate chart

| Gate | Required artifacts and evidence | Blocks |
| --- | --- | --- |
| A: business/authority review | Accepted account/access/instrument rules; explicit sole financial owner; review current capacity semantics | Expanded gateway/business behavior |
| B: protocol spike | C#/Rust exact numeric round-trip; partial/oversized/malformed frame, disconnect, correlation, retry/conflict; schemas/version policy | Real gateway/listener implementation contract |
| C: integrated access | Auth/account isolation/revocation tests; management/projection role isolation; commands and cursored queries; actual SDK pin/CI | Claim of authenticated end-to-end sandbox |
| D: durable execution | Accepted storage/feed ADRs; full-state/receipt replay; crash matrix; committed-corruption halt; snapshot/backup restore evidence | Durable ACK and full v0.1 restart scenario |
| E: operational/market data | Readiness/shutdown/recovery drills; bounded subscriptions; snapshot/gap/resync; log redaction | Market-data and service-operability completion |
| F: reproducible baseline | Clean-start funding/partial-fill/retry/restart/cancel/reconcile; workload/environment/raw reports; limitations | v0.1 review and optimization claims |
| G: expansion | Business need and measured constraint; financial-authority/failure model; migration/replay compatibility | Sharding, replication, fees, external settlement |

The chart is a review dependency model, not a new milestone numbering system. [Delivery backlog](../../delivery/backlog.md) remains canonical. On any implementation change use existing format/lint/tests, document checks and four-workflow exact-SHA integration gate; this architecture package does not relax CI.

## Current evidence and limits

The baseline includes 56 Rust tests, 17 matching fixtures, an 8,192-command matching differential corpus and 1,024 fresh generated financial attempts plus setup/retry/conflict checks. Financial tests compare deterministic cores and independently aggregate financial properties; they are not a complete independent financial-engine oracle. See [financial verification](../../verification/financial-testing.md) and [matching differential verification](../../verification/differential-testing.md).

The offline Atlas is documentation, not a runtime control plane or verified topology discovery tool. Its state/labels are maintained with the Markdown views. Browser checks validate readability and interactions; they do not validate execution architecture.
