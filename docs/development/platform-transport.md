# Platform transport and code-documentation baseline

Purpose: explain the implemented request-channel experiment and the comment/API contracts that maintainers must preserve.
Status: Implemented .NET transport groundwork, verified locally on Windows 2026-10-20. Rust wire integration and authentication remain pending.

## Delivered behavior

[LoopbackRequestChannel](../../src/platform/Cinis.ExecutionClient/Transport/LoopbackRequestChannel.cs) owns one exchange slot, creates one connection per request, writes one bounded frame, reads one bounded reply, then disposes the connection and timers. No automatic reconnect/retry or hidden waiting queue exists. A concurrent caller receives Busy/NotSent immediately. The current host still registers DisabledExecutionGateway, so trading readiness remains false and no financial endpoint is exposed.

[LoopbackChannelOptions](../../src/platform/Cinis.ExecutionClient/Transport/LoopbackChannelOptions.cs) validates a numeric loopback address, port, body bound and finite connect/exchange budgets at construction. Addresses are snapshotted before validation and getters return independent copies. DNS resolution and remote peers are outside this experiment. The transport body limit is configurable from 1 byte through 1 MiB; it is a local resource limit, not a frozen financial-message size. Time budgets are positive whole-millisecond-compatible values bounded by signed millisecond timers.

Connection and exchange budgets are separate. The exchange budget covers both request writing and response reading, so a stalled writer cannot escape a read-only timeout. Each accepted call has at most one live connection; this is a per-channel-instance limit, not a global ingress policy. Reuse one instance for one peer in a future adapter, rather than bypassing admission by allocating a channel per caller. The caller owns request memory and must keep its bytes unchanged until completion.

## Certainty boundary

| Event | Transport result | Meaning |
| --- | --- | --- |
| Empty/oversized request | NotSent / InvalidPayload | No connection or application write attempted |
| Already-cancelled caller | NotSent / Cancelled | No connection/application write attempted |
| Slot occupied | NotSent / Busy | No waiting request queued |
| Connection fails/expires | NotSent / ConnectFailed or ConnectTimedOut | No application-frame write attempted; TCP handshake traffic is not a command |
| Caller cancels after connect, before writing | NotSent / Cancelled | No application-frame write attempted |
| Any write attempt followed by cancellation/timeout/EOF/stream failure | UnknownOutcome | The peer may have received/applied a request |
| Invalid response length | UnknownOutcome / InvalidFrame | Peer input is invalid, but that does not prove the command was rejected |
| Complete response body | ReplyReceived / None | Only framing is known; version, correlation, authorization and business semantics still need validation |

The uncertainty boundary is crossed conservatively before invoking the first write. A thrown write cannot prove that no header/body bytes reached the peer. Cancelling a request is not financial rollback. Never turn UnknownOutcome into insufficient funds, rejection or a new command ID. Future typed retries must preserve account/command identity and semantic payload; this channel performs none automatically.

ReplyReceived is deliberately different from Trading's KnownResult. The future wire mapper must verify schema version, message type, correlation/identities and error/result meaning before returning a typed execution outcome. It must not deserialize arbitrary response bytes directly into trusted application permissions. A complete received reply is retained even if caller cancellation races after completion.

Fresh connections prevent leftover timed-out replies from satisfying later requests. This costs connection establishment and is an explainable experiment baseline, not a latency optimization. Pooling/multiplexing needs an explicit correlation and connection-state contract, benchmarks, and failure tests first.

## Error and lifetime policy

The channel converts expected cancellation/socket/stream/framing failures into bounded diagnostic enums; it does not expose raw exception messages or peer bytes as error codes. Unexpected programming/runtime failures propagate rather than becoming successful or business-rejected results. Resource disposal happens before releasing the single-flight slot, including error and early-return paths. There is no external Dispose race because no connection or timer survives an invocation.

BoundedFrameCodec leaves streams open for their owner and uses ReadExactlyAsync for partial reads. It validates the untrusted length before payload allocation. A failed partial write/read invalidates the connection; the channel closes it rather than trying to recover framing by guessing. The [ReadExactlyAsync contract](https://learn.microsoft.com/en-us/dotnet/api/system.io.stream.readexactlyasync?view=net-10.0) and [cancellable TcpClient connection API](https://learn.microsoft.com/en-us/dotnet/api/system.net.sockets.tcpclient.connectasync?view=net-10.0) inform the implementation. No TLS/service authentication is implied by loopback restriction.

## XML comments and maintainable code

All production projects now generate XML documentation next to their assemblies. Existing TreatWarningsAsErrors means missing public comments (CS1591), malformed XML, bad parameter references and invalid cref references fail the build. Test projects explicitly disable API XML generation because test method names/assertions are their behavior specification; production warnings are not suppressed. New test projects should follow that property convention.

Follow the [C# XML documentation guidance](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/xmldoc/recommended-tags):

- summary: observable purpose, using a complete sentence.
- param/returns: identity, units, memory ownership, nullability and outcome semantics where relevant.
- exception: expected public failure conditions, especially cancellation and partial I/O.
- remarks: concurrency, lifetime, trust, retry, durability and protocol limits that callers must preserve.
- inheritdoc: reuse an implemented interface contract when it describes the exact implementation; add implementation-specific limits separately.

Internal comments explain why a choice is necessary, such as checking length before allocation, placing the host authorization bridge outside Trading, or crossing uncertainty before a write. Avoid narrating assignments, commented-out code, empty regions, decorative separators or repeating the same contract in several files. Keep comments in English, update them with behavior, and place business rules in their canonical specs/ADRs rather than embedding an alternate rulebook in comments.

Application constructors now reject missing store/authorizer/gateway dependencies immediately. Access checks still do not cache or authenticate a principal string. Cancel handling forwards once and preserves adapter uncertainty; library awaits avoid capturing a caller synchronization context. Host JSON logging includes UTC round-trip timestamps and request scopes, without adding frames/credentials/environment dumps.

## Evidence and remaining gates

Current total: 61 .NET cases: Access 9, Trading 8, ExecutionClient 25, host integration 12, architecture 7. New loopback tests use ephemeral listeners and task-completion signals rather than sleeps to guess request arrival. They exercise complete byte replies, refused connections, malformed/truncated responses, post-send cancellation, exchange timeout, concurrent rejection, fresh exchange after failure, non-loopback/configuration validation and address ownership. The existing codec tests retain length/fragmentation/u64 coverage. There is no claim of exhaustive connection-timer or every operating-system socket failure injection.

Architecture tests verify the five production XML outputs exist and parse. An ignored standalone negative probe with an undocumented public class was compiled locally and failed with CS1591 as expected; this verified the actual compiler policy rather than merely inspecting a project property. Normal solution build succeeds with zero warnings/errors. Test fixtures and compiler documentation generation run in existing Linux/Windows Platform CI; exact hosted outcomes are recorded per integration SHA.

Still required: a Rust peer, versioned financial payload/schema, service trust, correlation/multiplexing policy, exact numeric cross-language fixtures and retained-result/disconnect evidence. A local .NET test peer does not close M0-04 or NET-05. Authentication, database, durable feed and recovery also remain pending. See [protocol](../architecture/protocol.md), [platform guide](platform.md), [ADR-0009](../architecture/decisions/0009-platform-transport-channel.md) and [backlog](../delivery/backlog.md).
