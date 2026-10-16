# Architecture and business decision register

Purpose: turn uncertainty into explicit review choices before implementation grows.
Status: review register, not acceptance of new policies. Owner/approver: project maintainer; architect prepares options/evidence. Reviewed: 2026-10-15.

Existing Accepted decisions stay in their canonical ADRs. New entries below are Proposed or Deferred. Closing an entry requires the maintainer's explicit decision recorded in an ADR or canonical contract, updated acceptance cases, and links back here. Proposed recommendations guide discussion; they are not silent business-policy changes. [Backlog](../../delivery/backlog.md) owns implementation tasks and estimates.

## Existing accepted baseline

| Area | Accepted decision / evidence | Revisit trigger |
| --- | --- | --- |
| Product boundary | Synthetic single-market spot lab; [ADR-0001](../decisions/0001-project-boundaries.md) | Real assets, real venue connectivity or production SLA |
| Matching | Limit GTC, price-time, resting price, incoming self-trade rejection, no amendment; [ADR-0002](../decisions/0002-matching-baseline.md) | New order types, fees or market constraints |
| Financial ownership | One Rust owner, checked integers, atomic reserve/settle/release, zero fees; [ADR-0005](../decisions/0005-financial-execution.md) | Multi-market account sharing or transaction optimization |
| Idempotency | Account/command key; retain original success/rejection; recheck permissions; same typed payload | Wire equality, retention, recovery and permission lifecycle |
| Numeric/operational bounds | Explicit caps; no eviction; whole-state cloning/full checks; [ledger](../ledger.md) | Usability under capacity, larger histories or performance research |
| Current guarantee | Volatile in-memory completion only | M4 journal/recovery evidence |
| Repository/integration | English docs, language-aware naming, docs/operations + ops automation; exact hosted-SHA gate | Real service/toolchain additions |

## Priority 1: before the cross-language spike and M3 gateway

| ID | Decision question / business impact | Recommended direction (Proposed) | Options / evidence and closure location |
| --- | --- | --- | --- |
| BD-01 | Who can own/control an account, and is v0.1 single-tenant? Wrong mapping enables unauthorized trades | One sandbox namespace with explicit principal-to-account grants; distinguish operator, funder, trader and reader | Single-owner versus delegated grants; examples of shared account/read-only/revoked key. Close in requirements and security ADR |
| BD-02 | What does revocation do to in-flight commands, retries and outcome queries? | Check permission at admission; admitted work completes; reauthorized retry follows current core behavior. Separately authorize status reads | Define cutoff/observable errors, cache lifetime and admin recovery of unknown outcomes. No retroactive reversal of fills. Close security/protocol |
| BD-03 | What is the externally visible instrument and precision? Demo atom units are not real decimal market conventions | Keep one immutable synthetic base/quote instrument for v0.1 and publish exact ticks/lots/caps | Choose symbols, display scales, min/max notional and configuration lifecycle. No float conversion/implicit rounding. Close domain contract |
| TD-01 | What exact frame/encoding/version contract connects C# and Rust? | Bounded length-prefixed loopback TCP; compare Protobuf payloads against explicit binary schema | Fix byte order, frame maximum, partial reads, invalid version, u64 encoding, canonical payload equality, schema compatibility. Spike report + protocol ADR |
| TD-02 | How does a trusted service prove authority to submit an account/action? | Restricted engine listener plus gateway-derived trusted context; never accept client Permissions | OS/process access limitations versus service token/mTLS; fail-closed access-store policy and replay admission facts. Close threat model/ADR before listener exposure |
| TD-03 | What are client command identity, timeout and result-query semantics? | Stable account-scoped command ID, same payload retry, exact numeric transport; explicit unknown-outcome status | HTTP schema/errors, deadlines, duplicate conflict, retained reject, multi-connection correlation, JS-safe decimal strings. Close protocol and contract fixtures |
| TD-04 | How do projections detect completeness and progress when a sequence has no events? | Complete per-command committed batches with explicit sequence/progress; transactional cursor | Event ordinal completeness, reject/no-event batches, duplicate and gap recovery. Close event-feed ADR and consumer tests |
| TD-13 | Which .NET architecture patterns and dependency boundaries should govern long-term implementation? | Proposed modular monolith, consumer-owned ports/adapters, vertical slices and CQRS; Rust remains financial authority | Review [platform plan](../platform-architecture.md), project-reference allowlist, worker/scaling/extraction gates and [ADR-0007](../decisions/0007-platform-architecture.md) before long-lived .NET projects. No source delivered |
| TD-05 | Which supported SDK/database/package versions are pinned? | .NET 10 LTS family and supported PostgreSQL major; exact pins with real projects | Host OS compatibility, supported patch, migrations/Npgsql/EF version alignment, Linux/Windows CI. Close setup/CI contracts |

## Priority 2: before M4 durability implementation

| ID | Decision question / business impact | Recommended direction (Proposed) | Evidence and closure |
| --- | --- | --- | --- |
| BD-04 | What exactly does a durable result promise? | One-host process-crash recovery first; state storage-sync policy precisely; no disk-loss/HA promise | Distinguish receipt, volatile result and durable result. Agree fault model and tolerated loss/availability. Close durability ADR |
| BD-05 | How should capacity exhaustion affect cancel and sandbox lifecycle? Current accepted caps can block a fresh cancel | Preserve current behavior until reviewed; evaluate reserved emergency/cancel budget and explicit reset/archive workflow | Demonstrate max-results/max-ledger cases, admin procedure, duplicate history effects. Any change needs ADR-0005 amendment and tests |
| TD-06 | Journal commands, outcomes or atomic deltas; custom file or embedded storage? | Spike versioned reproducible command records with admission/configuration facts and deterministic outcome verification | Partial writes, rejection replay, schema/semantic upgrades, checksum versus authenticity, sync failure, events after response loss. Close storage ADR |
| TD-07 | How are snapshots published and journal/receipt history retained? | Verified snapshot plus compatible suffix; persist dedup/counters and preserve previous recoverable baseline | Atomic publication semantics on supported OS, retention/rebuild horizon, migration, backup/restore and corruption drill. No premature receipt expiry |
| TD-08 | How can consumers access committed history without coupled private-file parsing? | Engine-owned versioned export/catch-up contract with cursors | Feed bounds, replay from snapshot, slow consumer, retention exhaustion, publisher crash, no double live publication on retry. Close feed ADR |

## Priority 3: before v0.1 completion or expansion

| ID | Decision question | Recommended direction / status | Closure trigger |
| --- | --- | --- | --- |
| BD-06 | Fees, rounding and asset issuance limits beyond the synthetic demo | Zero fees stays Accepted; nonzero fee model Deferred | Separate fee account, rounding beneficiary, reserve fee, partial fills and conservation fixtures before adding fees |
| BD-07 | Multi-market portfolio and account risk authority | Deferred; initially one owner across shared assets | Decide partition/reservation coordination before parallel market engines |
| TD-09 | Market-data cursor, snapshot cut and client resync | Proposed M5: bounded clients, consistent snapshot + ordered deltas | Explicit gap/resync/slow-client test and operational limits |
| TD-10 | Service supervision, readiness, shutdown, secrets and backups | Proposed M3/M4: native tasks then optional Compose with actual services | Process ownership, storage paths, crash drill, restore evidence and secret redaction |
| TD-11 | Performance targets and optimization data structures | Deferred acceptance of numeric targets; keep baseline until measured | Record workload and core/network/durable timing boundaries before optimization ADR |
| TD-12 | Replication, leader election, multi-host failover | Deferred beyond v0.1 | Agreed availability/RPO/RTO, fencing/consensus and split-brain tests |
| BD-08 | Aegis or blockchain/real-funds expansion | Deferred, optional identity adapter only in current direction | New business/security charter and finality/custody model; do not block standalone Cinis |

## Review sequence and record format

First review BD-01/02/03 and TD-01/02/03/04 together: business access and retry rules constrain the wire contract. Run the small cross-language spike, record observed failures and alternatives, then accept specific contracts. Review BD-04/05 and TD-06/07/08 before writing a journal. The remainder gates later delivery or a new scope.

For each closure record: decision ID, chosen option, status/date/approver, why alternatives were rejected, affected requirement/rule IDs, failure scenarios, compatibility/migration impact, test evidence, and revisit trigger. Keep a decision Proposed if the evidence is incomplete. Publishing this document does not close M0, M3 or M4.
