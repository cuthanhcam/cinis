# ADR-0009: Bounded experimental request channel and documented public APIs

Date: 2026-10-20. Status: Accepted implementation baseline for .NET transport groundwork; financial wire/trust policies remain open.

## Context and alternatives

The platform had a bounded byte codec and disabled financial adapter but no real TCP transport behavior. Pooling, multiplexing and automatic retries would require correlation and unknown-outcome contracts before a Rust wire schema exists. Comments described intent but the build did not enforce API documentation.

## Decision

Add a numeric-loopback-only request channel with one active invocation and immediate Busy rejection, a fresh connection per call, separate finite connection and combined write/read budgets, and a configurable 1-byte to 1-MiB body bound. Reject bad inputs before connecting. Distinguish NotSent, UnknownOutcome and ReplyReceived; a complete byte frame is not a validated financial result. After a write attempt, cancellation, malformed replies and transport loss remain uncertain. Perform no automatic retry and close every connection after its invocation.

Generate production XML documentation through the ordinary SDK build and retain warnings-as-errors for missing/broken public documentation. Add detailed contract and rationale comments without line-by-line narration. Test projects explicitly disable API XML generation. Guard constructor dependencies early and keep host logging in UTC with request scopes.

## Consequences and evidence

This favors understandable failure semantics over connection throughput. Single-flight is per instance; a future host still needs bounded aggregate ingress and trusted service identity. Loopback is not authentication. Caller memory stays owned by the caller and must not change during an exchange. The host's disabled execution registration and false trading readiness remain unchanged.

Real loopback tests prove framing, cancellation/timeout uncertainty, busy rejection, malformed replies, refusal and subsequent exchange after failure. Architecture tests inspect generated production XML, and a local negative compilation probe confirms missing comments fail with CS1591. No Rust interop, financial protocol acceptance, authentication or durability is established. Complete M0-04/ARCH-02 before exposing financial endpoints.

Revisit connection pooling/multiplexing only with a versioned identity/correlation contract and measured need. The detailed [transport/comment guide](../../development/platform-transport.md) owns current behavior, examples, failure categories and evidence limits.
