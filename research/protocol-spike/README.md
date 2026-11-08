# Cross-language synthetic-funding probe

Status: Implemented research experiment. Date: 2026-11-08. Scope: real .NET/Rust process interoperability over owned stdio; proposed financial wire/trust decisions remain open.

## Question and hypothesis

Can the existing .NET bounded frame codec exchange exact typed identities/results with a separate Rust process using the actual ExecutionCore, preserving admission failure, retained business rejection and same-ID retry semantics?

The hypothesis is deliberately narrower than a network gateway: a fixed-width binary candidate should preserve u64 values and command identity without JSON/decimal conversion. A successful experiment is evidence for evaluating TD-01/03, not acceptance of this schema over Protobuf or of an engine service trust model.

## Boundary and organization

- [Rust research binary](../../src/engine/src/bin/wire_probe.rs) owns one ExecutionCore and fixed ADMIN context for this synthetic parent-owned experiment. It has no network listener or permissions field. The matching/ledger/execution library remains synchronous and unchanged.
- [Private Rust codec](../../src/engine/src/bin/wire_probe/protocol.rs) validates framing/schema before execution and checks full owner invariants after each request.
- Cinis.ProtocolProbe is an independent executable harness outside the production platform solution. It references the actual ExecutionClient frame codec but is not registered in the host or an IExecutionGateway implementation.
- [Golden fixtures](../../tests/fixtures/protocol/README.md) are shared by both languages. No additional NuGet/Cargo package is introduced. Commit the research project lockfile.

Owned pipes avoid exposing privileged unauthenticated funding while TD-02 is unresolved. This is a first serialization/process experiment, not a replacement for the planned loopback TCP evidence. No funds or receipts survive peer restart.

## Reproduction

From repository root, using the pinned toolchains:

```sh
cargo build --locked --manifest-path src/engine/Cargo.toml --bin cinis-wire-probe
dotnet restore research/protocol-spike/Cinis.ProtocolProbe.csproj --locked-mode
dotnet format research/protocol-spike/Cinis.ProtocolProbe.csproj whitespace --no-restore --verify-no-changes
dotnet run --project research/protocol-spike/Cinis.ProtocolProbe.csproj --no-restore -c Release -- . src/engine/target/debug/cinis-wire-probe
```

The harness appends .exe on Windows. It starts only the supplied owned binary with --stdio-research, drains its bounded static stderr, enforces a 30-second session budget, and closes stdin for clean EOF. Disposal waits three seconds before terminating only the owned child tree. A protocol/I/O timeout fails the experiment; it is not translated to a financial rejection or automatically retried. Direct binary startup requires the explicit research switch. stdout is framed bytes only; human diagnostics use stderr. The peer is not a production supervisor and a hard-killed harness can bypass cooperative cleanup.

Platform CI runs the harness on Linux and Windows, after locked Rust build, .NET restore/format/build and normal platform tests. Research runs are separate from the platform's .NET test count. Local logs/output can be redirected under ignored artifacts/; no benchmark, latency or zero-allocation claim follows from a pass.

## Provisional binary schema

All multibyte integers are unsigned big-endian. Every message has a four-byte body length; nonempty body maximum is 128 bytes, checked before allocation. Clean EOF between frames ends the peer; zero/oversized/truncated framing terminates with exit 2 and no response. Structurally bounded but invalid bodies produce a protocol failure without admission. Per instance: eight accounts, 32 retained commands, 32 postings, four events per decision, zero orders, at most 64 frames. Exhaustion terminates or produces the corresponding core admission/business failure; no eviction occurs.

Request body (35 bytes):

| Offset | Width | Meaning |
| --- | --- | --- |
| 0 | 1 | Version 1 |
| 1 | 1 | Message kind 1, funding |
| 2 / 10 / 18 | 8 each | Correlation / account / command IDs |
| 26 | 1 | Asset: base 0, quote 1 |
| 27 | 8 | Amount in asset atoms |

Correlation is nonzero and distinct from account-scoped command identity. Only the latter participates in core idempotency. Payload equality is the decoded typed asset/amount, with no hash or float canonicalization.

Reply body (77 bytes):

| Offset | Width | Meaning |
| --- | --- | --- |
| 0 / 1 | 1 each | Version 1 / reply kind 2 |
| 2 / 10 / 18 | 8 each | Echoed correlation / account / command |
| 26 / 27 / 28 | 1 each | Decision kind / code / source |
| 29 | 8 | Original decision sequence, or zero when not admitted |
| 37 / 41 | 4 each | Original event count / posting count |
| 45 / 53 / 61 / 69 | 8 each | Current base available/reserved, quote available/reserved |

Decision kinds: admission failure 1, success 2, business rejection 3, protocol failure 4. Sources: none 0, new 1, retained 2. Protocol codes: version 1, kind 2, asset 3, zero correlation 4, body shape 5. If fewer than 26 bytes are available, response identities are zero. Such malformed responses cannot correlate a typed financial result.

Admission codes 16..22 map respectively to InvalidAccountId, InvalidCommandId, Unauthorized, DuplicateConflict, CommandCapacity, SequenceExhausted and InvalidLimits. Business codes 32..38 map to InvalidAmount, BalanceLimit, SupplyLimit, AccountCapacity, LedgerCapacity, ArithmeticOverflow and InvariantViolation. Code 63 is an explicit unmapped non-funding rejection bucket; this is not a full financial error taxonomy. Wire-selected permissions and other financial commands are intentionally absent.

Success has one event and one posting; business rejection has sequence/source but zero effects. Admission/protocol failure has no sequence/source/effects. The harness validates schema, correlation/account/command, source/status consistency and effect counts before interpreting outcomes. Retained receipt sequence/counts describe historical effects; diagnostic balances describe current state and can change after later commands. They are not a historical account snapshot or a projection contract. Source=retained must never instruct a consumer to republish effects. No event publisher exists in this experiment.

## Evidence and limitations

The harness passes 35 assertions checking golden bytes including identities above 2^53 and ulong.MaxValue, fragmented byte writes, multiple frames per process, correlation changes on same-ID retry, account-scoped identity, changed-payload conflict, invalid account/command IDs, retained zero-amount rejection, exact funding/balance boundaries, unsupported protocol fields, malformed body size, lost first reply followed by explicit retry, fresh-process reset and four fatal framing cases. Rust unit tests independently check golden replies, framing boundaries and malformed payload non-mutation.

--drop-first-reply is an explicit research fault switch: the first admitted decision executes but its response is omitted. The harness sends a same-ID retry without an outstanding read and observes a retained result and one funding effect. A separate silent-peer test exercises bounded await expiry and disposal of the abandoned pipe session; it does not reuse that stream. This does not prove socket disconnect/reconnect, native pipe read cancellation or general financial timeout recovery. Child termination after framing failure likewise does not prove server-side TCP stream recovery.

Local verification records the actual installed/resolved toolchains. On 2026-11-08, Windows had SDK 10.0.402 but lacked the repository's 10.0.301; the pinned SDK was installed separately under ignored artifacts/toolchains for validation without changing global.json. Hosted results are required on the exact integration SHA before publication.

Remaining M0-04 work: actual loopback Rust peer/trust boundary, disconnect/timeout/correlation under competing callers, full command/result/event-batch representation, process/source identity and alternative encoding comparison. ARCH-02 and NET-05 remain open. This experiment authorizes neither public funding nor production ADMIN access. Actual focused engineering effort was not measured.
