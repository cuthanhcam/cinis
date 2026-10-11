# Protocol draft

Status: spike contract; encoding/schema not frozen.

Start with bounded length-prefixed TCP loopback frames. Declare version, message type, and payload length. Reject unknown versions, malformed frames, and oversized payloads before allocation or mutation. Spike decisions: byte order, length encoding, maximum frame size, canonical payload hashing, and C#/Rust numeric serialization.

Command envelope: version, account ID, command ID, command type, payload. Initial commands: synthetic funding, place limit, cancel, market-state change. Gateway owns authentication; direct engine access is restricted to trusted local services.

Result: command/account ID, sequence where applicable, status, stable error code, order ID where applicable, event IDs. Errors include invalid input, insufficient funds, unauthorized owner, market paused, capacity exhausted, overflow, duplicate conflict, and execution unavailable.

Events use execution sequence plus event ordinal for stable identity. Include version, command ID, kind, and required financial/order fields. Consumers follow sequence/ordinal. Market-data streams define a separate cursor and consistent snapshot boundary.

Timeout/disconnect gives unknown outcome: retry the same ID/payload. Bound queues and timeouts; overloaded ingress rejects before acceptance. Queries expose applied sequence to show projection lag. Gaps trigger catch-up or snapshot resync.
