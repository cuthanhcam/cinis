using System.Net;
using System.Net.Sockets;
using Cinis.ExecutionClient.Protocol;
using Cinis.ExecutionClient.Transport;

namespace Cinis.ExecutionClient.Tests;

public sealed class LoopbackChannelTests
{
    private static readonly BoundedFrameCodec Codec = new(128);

    // Each test owns an ephemeral loopback listener. Cancellation closes only this peer; no fixed ports or sleeps
    // are used to guess when a request arrived. Assertions in the peer task are observed during async disposal.
    private sealed class Peer : IAsyncDisposable
    {
        private readonly TcpListener _listener = new(IPAddress.Loopback, 0);
        private readonly CancellationTokenSource _stop = new();
        public Task Completion { get; }
        public int Port => ((IPEndPoint)_listener.LocalEndpoint).Port;

        public Peer(Func<NetworkStream, CancellationToken, Task> handler, int connections = 1)
        {
            _listener.Start();
            Completion = ServeAsync(handler, connections);
        }

        private async Task ServeAsync(Func<NetworkStream, CancellationToken, Task> handler, int connections)
        {
            for (var index = 0; index < connections; index++)
            {
                using var client = await _listener.AcceptTcpClientAsync(_stop.Token);
                await handler(client.GetStream(), _stop.Token);
            }
        }

        public LoopbackRequestChannel Channel(TimeSpan? timeout = null) => new(new(
            IPAddress.Loopback, Port, 128, TimeSpan.FromSeconds(5), timeout ?? TimeSpan.FromSeconds(5)));

        public async ValueTask DisposeAsync()
        {
            await _stop.CancelAsync();
            _listener.Stop();
            try { await Completion.WaitAsync(TimeSpan.FromSeconds(5)); }
            catch (OperationCanceledException) when (_stop.IsCancellationRequested) { }
            catch (SocketException) when (_stop.IsCancellationRequested) { }
            finally { _stop.Dispose(); }
        }
    }

    [Fact]
    public async Task CompleteFrame_ReturnsBytesWithoutClaimingFinancialSuccess()
    {
        byte[] request = [255, 0, 1];
        byte[] reply = [0, 255, 4, 8];
        await using var peer = new Peer(async (stream, token) =>
        {
            Assert.Equal(request, await Codec.ReadAsync(stream, token));
            using var encoded = new MemoryStream();
            await Codec.WriteAsync(encoded, reply, token);
            foreach (var value in encoded.ToArray()) await stream.WriteAsync(new byte[] { value }, token);
        });
        var result = await peer.Channel().ExchangeAsync(request);
        Assert.Equal(FrameExchangeStatus.ReplyReceived, result.Status);
        Assert.Equal(FrameExchangeFailure.None, result.Failure);
        Assert.Equal(reply, result.Reply.ToArray());
    }

    [Theory]
    [InlineData("10.0.0.1")]
    [InlineData("0.0.0.0")]
    [InlineData("::")]
    public void NonLoopbackPeer_IsRejected(string address) =>
        Assert.Throws<ArgumentException>(() => new LoopbackChannelOptions(IPAddress.Parse(address), 12345, 128,
            TimeSpan.FromSeconds(1), TimeSpan.FromSeconds(1)));

    [Fact]
    public void AddressConfiguration_IsIndependentFromMutableCallerCopies()
    {
        var address = IPAddress.Parse("::1");
        var options = new LoopbackChannelOptions(address, 1, 128, TimeSpan.FromSeconds(1), TimeSpan.FromSeconds(1));
        address.ScopeId = 17;
        var returned = options.Address;
        returned.ScopeId = 23;
        Assert.Equal(IPAddress.IPv6Loopback, options.Address);
    }

    [Fact]
    public void InvalidResourceBudgets_AreRejected()
    {
        Assert.Throws<ArgumentOutOfRangeException>(() => new LoopbackChannelOptions(IPAddress.Loopback, 0, 128,
            TimeSpan.FromSeconds(1), TimeSpan.FromSeconds(1)));
        Assert.Throws<ArgumentOutOfRangeException>(() => new LoopbackChannelOptions(IPAddress.Loopback, 1, 1_048_577,
            TimeSpan.FromSeconds(1), TimeSpan.FromSeconds(1)));
        Assert.Throws<ArgumentOutOfRangeException>(() => new LoopbackChannelOptions(IPAddress.Loopback, 1, 128,
            TimeSpan.Zero, TimeSpan.FromSeconds(1)));
        Assert.Throws<ArgumentOutOfRangeException>(() => new LoopbackChannelOptions(IPAddress.Loopback, 1, 128,
            TimeSpan.FromSeconds(1), Timeout.InfiniteTimeSpan));
    }

    [Theory]
    [InlineData(0)]
    [InlineData(129)]
    public async Task InvalidPayload_DoesNotContactPeer(int size)
    {
        await using var peer = new Peer((_, _) => throw new InvalidOperationException("No connection expected."));
        var result = await peer.Channel().ExchangeAsync(new byte[size]);
        Assert.Equal(FrameExchangeStatus.NotSent, result.Status);
        Assert.Equal(FrameExchangeFailure.InvalidPayload, result.Failure);
    }

    [Fact]
    public async Task AlreadyCancelled_DoesNotContactPeer()
    {
        await using var peer = new Peer((_, _) => throw new InvalidOperationException("No connection expected."));
        var result = await peer.Channel().ExchangeAsync(new byte[] { 1 }, new CancellationToken(true));
        Assert.Equal(FrameExchangeStatus.NotSent, result.Status);
        Assert.Equal(FrameExchangeFailure.Cancelled, result.Failure);
    }

    [Fact]
    public async Task ConnectionRefused_IsDefinitelyNotSent()
    {
        // Reserve but do not listen on the port, preventing another test from becoming its peer.
        using var bound = new Socket(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);
        bound.Bind(new IPEndPoint(IPAddress.Loopback, 0));
        var channel = new LoopbackRequestChannel(new(IPAddress.Loopback, ((IPEndPoint)bound.LocalEndPoint!).Port,
            128, TimeSpan.FromSeconds(5), TimeSpan.FromSeconds(5)));
        var result = await channel.ExchangeAsync(new byte[] { 1 });
        Assert.Equal(FrameExchangeStatus.NotSent, result.Status);
        Assert.Equal(FrameExchangeFailure.ConnectFailed, result.Failure);
    }

    [Fact]
    public async Task ConcurrentCaller_IsRejectedWithoutWaiting_AndPostSendCancellationIsUnknown()
    {
        var received = new TaskCompletionSource(TaskCreationOptions.RunContinuationsAsynchronously);
        await using var peer = new Peer(async (stream, token) =>
        {
            await Codec.ReadAsync(stream, token);
            received.SetResult();
            await Task.Delay(Timeout.InfiniteTimeSpan, token);
        });
        var channel = peer.Channel();
        using var cancel = new CancellationTokenSource();
        var first = channel.ExchangeAsync(new byte[] { 1 }, cancel.Token);
        await received.Task.WaitAsync(TimeSpan.FromSeconds(5));
        var busy = await channel.ExchangeAsync(new byte[] { 2 });
        Assert.Equal(FrameExchangeStatus.NotSent, busy.Status);
        Assert.Equal(FrameExchangeFailure.Busy, busy.Failure);
        await cancel.CancelAsync();
        var result = await first.WaitAsync(TimeSpan.FromSeconds(5));
        Assert.Equal(FrameExchangeStatus.UnknownOutcome, result.Status);
        Assert.Equal(FrameExchangeFailure.Cancelled, result.Failure);
    }

    [Fact]
    public async Task ReplyBudgetExpires_WithoutInventingBusinessRejection()
    {
        await using var peer = new Peer(async (stream, token) =>
        {
            await Codec.ReadAsync(stream, token);
            await Task.Delay(Timeout.InfiniteTimeSpan, token);
        });
        var result = await peer.Channel(TimeSpan.FromMilliseconds(300)).ExchangeAsync(new byte[] { 1 });
        Assert.Equal(FrameExchangeStatus.UnknownOutcome, result.Status);
        Assert.Equal(FrameExchangeFailure.ExchangeTimedOut, result.Failure);
        Assert.True(result.Reply.IsEmpty);
    }

    [Fact]
    public async Task PeerClosesAfterRead_OutcomeRemainsUnknown()
    {
        await using var peer = new Peer(async (stream, token) => { await Codec.ReadAsync(stream, token); });
        var result = await peer.Channel().ExchangeAsync(new byte[] { 1 });
        Assert.Equal(FrameExchangeStatus.UnknownOutcome, result.Status);
        Assert.Equal(FrameExchangeFailure.Disconnected, result.Failure);
    }

    [Fact]
    public async Task MalformedReply_ClosesConnectionAndReleasesSlotForFreshExchange()
    {
        var call = 0;
        await using var peer = new Peer(async (stream, token) =>
        {
            await Codec.ReadAsync(stream, token);
            if (++call == 1) await stream.WriteAsync(new byte[4], token); // Invalid zero-length response.
            else await Codec.WriteAsync(stream, new byte[] { 42 }, token);
        }, connections: 2);
        var channel = peer.Channel();
        var failed = await channel.ExchangeAsync(new byte[] { 1 });
        Assert.Equal(FrameExchangeStatus.UnknownOutcome, failed.Status);
        Assert.Equal(FrameExchangeFailure.InvalidFrame, failed.Failure);
        var next = await channel.ExchangeAsync(new byte[] { 2 });
        Assert.Equal(FrameExchangeStatus.ReplyReceived, next.Status);
        Assert.Equal(new byte[] { 42 }, next.Reply.ToArray());
        Assert.Equal(2, call);
    }
}
