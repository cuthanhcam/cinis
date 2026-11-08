using Cinis.ProtocolProbe;

if (args.Length != 2) throw new ArgumentException("Usage: protocol-probe <repository-root> <Rust-probe-path>");
var root = Path.GetFullPath(args[0]);
var executable = Path.GetFullPath(args[1]);
if (OperatingSystem.IsWindows() && !executable.EndsWith(".exe", StringComparison.OrdinalIgnoreCase)) executable += ".exe";
if (!File.Exists(executable)) throw new FileNotFoundException("Build cinis-wire-probe first.", executable);
var passed = 0;
void Check(bool condition, string name)
{
    if (!condition) throw new InvalidOperationException("Probe assertion failed: " + name);
    passed++;
    Console.WriteLine("PASS " + name);
}
byte[] Fixture(string name) => Convert.FromHexString(string.Concat(
    File.ReadAllText(Path.Combine(root, "tests", "fixtures", "protocol", name + ".hex")).Split((char[]?)null,
        StringSplitOptions.RemoveEmptyEntries)));

var request = new FundingRequest(0x0102030405060708, 9_007_199_254_740_993, ulong.MaxValue, 1, 17);
foreach (var offset in new[] { 0, 2, 10, 18, 28, 37 })
{
    var invalid = Fixture("funding_reply")[4..];
    invalid[offset] ^= 0xff;
    var rejected = false;
    try { FundingReply.Decode(invalid, request); }
    catch (InvalidDataException) { rejected = true; }
    Check(rejected, "typed mapper rejects contradictory schema/identity/effects at " + offset);
}
var shortRejected = false;
try { FundingReply.Decode(Fixture("funding_reply")[4..^1], request); }
catch (InvalidDataException) { shortRejected = true; }
Check(shortRejected, "typed mapper rejects wrong reply body length");
using (var frame = new MemoryStream())
{
    await OwnedPeer.Codec.WriteAsync(frame, request.Encode());
    Check(frame.ToArray().SequenceEqual(Fixture("funding_request")), "shared request golden bytes and exact u64");
}
await using (var peer = new OwnedPeer(executable))
{
    await peer.SendAsync(request.Encode(), fragmented: true);
    var body = await peer.ReadAsync();
    Check(body.SequenceEqual(Fixture("funding_reply").AsSpan(4).ToArray()), "shared reply golden bytes over fragmented writes");
    var first = FundingReply.Decode(body, request);
    Check(first is { Kind: 2, Source: 1, Sequence: 1, QuoteAvailable: 17 }, "real Rust funding success");
    var retry = await peer.ExchangeAsync(request with { Correlation = 2 });
    Check(retry is { Kind: 2, Source: 2, Sequence: 1, QuoteAvailable: 17 }, "new correlation preserves retained command identity");
    var conflict = await peer.ExchangeAsync(request with { Correlation = 3, Amount = 18 });
    Check(conflict is { Kind: 1, Code: 19, Source: 0, Sequence: 0, QuoteAvailable: 17 }, "changed payload has no new decision");
    var otherAccount = await peer.ExchangeAsync(request with { Correlation = 4, Account = 2 });
    Check(otherAccount is { Kind: 2, Source: 1, Sequence: 2, QuoteAvailable: 17 }, "command key is account scoped");
    var invalidAccount = await peer.ExchangeAsync(request with { Correlation = 5, Account = 0 });
    Check(invalidAccount is { Kind: 1, Code: 16, Sequence: 0 }, "invalid account denied before admission");
    var invalidCommand = await peer.ExchangeAsync(request with { Correlation = 6, Command = 0 });
    Check(invalidCommand is { Kind: 1, Code: 17, Sequence: 0 }, "invalid command denied before admission");
    var zero = request with { Correlation = 7, Command = 1, Amount = 0 };
    Check(await peer.ExchangeAsync(zero) is { Kind: 3, Code: 32, Source: 1, Sequence: 3, Events: 0, Postings: 0 },
        "business rejection consumes sequence without effects");
    Check(await peer.ExchangeAsync(zero with { Correlation = 8 }) is { Kind: 3, Code: 32, Source: 2, Sequence: 3 },
        "business rejection is retained");
    Check(await peer.ExchangeAsync(request with { Correlation = 9, Command = 2, Amount = 3 })
        is { Kind: 2, Source: 1, Sequence: 4, QuoteAvailable: 20 }, "subsequent funding is exact");
    Check(await peer.ExchangeAsync(request with { Correlation = 10 })
        is { Kind: 2, Source: 2, Sequence: 1, QuoteAvailable: 20 }, "historical receipt distinct from current diagnostic balance");
    Check(await peer.ExchangeAsync(request with { Correlation = 11, Command = 3, Amount = ulong.MaxValue })
        is { Kind: 3, Code: 32, Sequence: 5, QuoteAvailable: 20 }, "u64 max amount arrives intact and is business rejected");
    Check(await peer.ExchangeAsync(request with { Correlation = 12, Account = 3, Amount = 1_000_000_000_000_000_000 })
        is { Kind: 2, Sequence: 6, QuoteAvailable: 1_000_000_000_000_000_000 }, "accepted exact account balance boundary");
    Check(await peer.ExchangeAsync(request with { Correlation = 13, Account = 3, Command = 1, Amount = 1 })
        is { Kind: 3, Code: 33, Sequence: 7, QuoteAvailable: 1_000_000_000_000_000_000 }, "balance-limit rejection preserves state");
    foreach (var (offset, value, expectedCode) in new (int, byte, byte)[] { (0, 2, 1), (1, 9, 2), (26, 2, 3) })
    {
        var invalid = request.Encode(); invalid[offset] = value;
        await peer.SendAsync(invalid);
        Check(FundingReply.Decode(await peer.ReadAsync(), request) is { Kind: 4, Sequence: 0 } rejected
            && rejected.Code == expectedCode, "protocol field rejected before admission " + offset);
    }
    var noCorrelation = request with { Correlation = 0 };
    Check(await peer.ExchangeAsync(noCorrelation) is { Kind: 4, Code: 4, Sequence: 0 }, "zero correlation rejected");
    await peer.SendAsync(request.Encode()[..34]);
    Check(FundingReply.Decode(await peer.ReadAsync(), request) is { Kind: 4, Code: 5, Sequence: 0 }, "wrong body shape rejected");
    Check(await peer.ExchangeAsync(request with { Correlation = 14, Command = 4, Asset = 0, Amount = 5 })
        is { Kind: 2, Sequence: 8, BaseAvailable: 5, QuoteAvailable: 20 }, "protocol errors did not consume execution sequence");
    await peer.FinishAsync();
}
await using (var peer = new OwnedPeer(executable, dropFirst: true))
{
    var lost = request with { Correlation = 1 };
    await peer.SendAsync(lost.Encode());
    // Deliberately omit the first read: the fault peer drops that reply. Explicit same-ID retry is a probe action,
    // not an automatic retry policy or evidence of network timeout/reconnection behavior.
    var retry = await peer.ExchangeAsync(lost with { Correlation = 2 });
    Check(retry is { Kind: 2, Source: 2, Sequence: 1, QuoteAvailable: 17 }, "lost reply followed by explicit retry has one funding effect");
    await peer.FinishAsync();
}
await using (var restarted = new OwnedPeer(executable))
{
    Check(await restarted.ExchangeAsync(request) is { Kind: 2, Source: 1, Sequence: 1, QuoteAvailable: 17 },
        "fresh process does not retain prior receipts or balances");
    await restarted.FinishAsync();
}
await using (var silent = new OwnedPeer(executable, dropFirst: true, budget: TimeSpan.FromSeconds(1)))
{
    var expired = false;
    try
    {
        await silent.SendAsync(request.Encode());
        await silent.ReadAsync();
    }
    catch (OperationCanceledException) { expired = true; }
    Check(expired, "silent peer expires the bounded session without inventing a rejection");
}
foreach (var frame in new byte[][] { [0, 0, 0, 0], [0, 0, 0, 129], [0, 0], [0, 0, 0, 35, 1] })
{
    await using var peer = new OwnedPeer(executable);
    await peer.InvalidFrameAsync(frame);
    Check(true, "invalid/truncated framing terminates the owned peer");
}
Console.WriteLine($"Protocol research passed: {passed} assertions; volatile synthetic funding over owned stdio only.");
