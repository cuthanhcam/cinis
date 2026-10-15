# Durability and recovery

Status: intended M4 contract. M1-M3 prototypes must label themselves non-durable.

1. Assign sequence and append a versioned, framed command record with integrity metadata.
2. Cross the configured storage synchronization boundary.
3. Apply the deterministic transition in execution order.
4. Return final result and stable event IDs; preserve history sufficient to reproduce results.

Received means ingress receipt only. A final durable result means recoverable under the tested storage policy. Journal failure stops admission; never apply unrecorded commands or return durable success. A crash after journaling but before response can commit an operation the client has not observed; retry the same key.

Snapshots include sequence, schema version, book, balances, holds, ledger state or replay reference, deduplication results, and counters. Publish atomically after verification; retain required journal history. A failed snapshot must not invalidate the previous recoverable baseline.

| Crash point | Required outcome |
| --- | --- |
| Before complete durable record | No durable success promised; retry resolves outcome |
| After durable record, before apply | Replay applies once |
| After apply, before response | Retry returns original result |
| After response, before publish | Consumers recover events from durable history |
| During snapshot creation | Previous valid snapshot plus journal recovers |
| Incomplete trailing record | Discard only proven uncommitted tail under format rules |
| Corruption within committed history | Fail closed; preserve files and investigate |

Power-loss guarantees depend on synchronization, filesystem, and hardware. Document and test the exact policy before claiming it. Disk loss, replication, and multi-host failover are outside v0.1. Journal format, sync APIs, snapshot layout, and retention need ADRs before M4.

## Current matching contract

The M1 core is volatile. Accepted means the in-memory transition completed; process exit loses everything. Do not issue or infer a durable ACK. ADR-0002 accepts this starting guarantee.

## M2 extension

ExecutionCore now retains ledger, balances, holds, market state, and original command outcomes in memory. None survives process exit. Its committed response means atomic in-memory completion only; M2 does not upgrade the durability guarantee. A future snapshot/journal must include financial and deduplication state, not merely the book.
