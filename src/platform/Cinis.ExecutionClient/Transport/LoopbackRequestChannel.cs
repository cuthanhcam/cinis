using System.Net.Sockets;
using Cinis.ExecutionClient.Protocol;

namespace Cinis.ExecutionClient.Transport;

/// <summary>Performs a bounded, single-flight framed exchange against a numeric loopback peer.</summary>
/// <remarks>
/// Each call owns a fresh connection, preventing a timed-out response from being reused by another request.
/// There is no automatic retry or internal waiting queue. This is transport groundwork, not an authenticated Rust protocol.
/// </remarks>
public sealed class LoopbackRequestChannel
{
    private readonly LoopbackChannelOptions _options;
    private readonly BoundedFrameCodec _codec;
    private int _inFlight;

    /// <summary>Creates a channel with one exchange slot and validated resource budgets.</summary>
    /// <param name="options">Immutable endpoint and resource limits.</param>
    /// <exception cref="ArgumentNullException">Options are absent.</exception>
    public LoopbackRequestChannel(LoopbackChannelOptions options)
    {
        ArgumentNullException.ThrowIfNull(options);
        _options = options;
        _codec = new BoundedFrameCodec(options.MaxPayloadBytes);
    }

    /// <summary>Attempts one write/read exchange without retrying or assigning a business outcome.</summary>
    /// <param name="request">Body bytes. The caller must not mutate their backing storage until completion.</param>
    /// <param name="cancellationToken">Cancellation of this invocation, not reversal of a peer's command.</param>
    /// <returns>Transport evidence including pre-send denial versus post-write uncertainty.</returns>
    /// <remarks>
    /// Cancellation before a write attempt returns NotSent. Once the first write may have begun, cancellation,
    /// malformed replies, EOF and timeout return UnknownOutcome. A complete reply wins a later cancellation race.
    /// Unexpected programming/runtime exceptions are not converted to successful or business-rejected results.
    /// </remarks>
    public async Task<FrameExchangeResult> ExchangeAsync(ReadOnlyMemory<byte> request,
        CancellationToken cancellationToken = default)
    {
        if (request.IsEmpty || request.Length > _options.MaxPayloadBytes)
            return NotSent(FrameExchangeFailure.InvalidPayload);
        if (cancellationToken.IsCancellationRequested) return NotSent(FrameExchangeFailure.Cancelled);
        // Reject overload immediately. Waiting on a semaphore here would hide an unbounded ingress queue.
        if (Interlocked.CompareExchange(ref _inFlight, 1, 0) != 0) return NotSent(FrameExchangeFailure.Busy);
        try
        {
            using var client = new TcpClient(_options.Address.AddressFamily) { NoDelay = true };
            using (var connect = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken))
            {
                connect.CancelAfter(_options.ConnectTimeout);
                try
                {
                    await client.ConnectAsync(_options.Address, _options.Port, connect.Token).ConfigureAwait(false);
                }
                catch (OperationCanceledException)
                {
                    return NotSent(cancellationToken.IsCancellationRequested
                        ? FrameExchangeFailure.Cancelled : FrameExchangeFailure.ConnectTimedOut);
                }
                catch (SocketException) { return NotSent(FrameExchangeFailure.ConnectFailed); }
            }

            if (cancellationToken.IsCancellationRequested) return NotSent(FrameExchangeFailure.Cancelled);
            using var exchange = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
            exchange.CancelAfter(_options.ExchangeTimeout);
            var stream = client.GetStream();
            try
            {
                // Conservatively cross the uncertainty boundary BEFORE calling WriteAsync: an exception cannot prove
                // that no header/body bytes escaped. Never turn a lost reply into a definite financial rejection.
                await _codec.WriteAsync(stream, request, exchange.Token).ConfigureAwait(false);
                var reply = await _codec.ReadAsync(stream, exchange.Token).ConfigureAwait(false);
                return new(FrameExchangeStatus.ReplyReceived, FrameExchangeFailure.None, reply);
            }
            catch (OperationCanceledException)
            {
                return Unknown(cancellationToken.IsCancellationRequested
                    ? FrameExchangeFailure.Cancelled : FrameExchangeFailure.ExchangeTimedOut);
            }
            catch (InvalidDataException) { return Unknown(FrameExchangeFailure.InvalidFrame); }
            catch (IOException) { return Unknown(FrameExchangeFailure.Disconnected); }
            catch (SocketException) { return Unknown(FrameExchangeFailure.Disconnected); }
        }
        finally
        {
            // Connection and timers are disposed before releasing the slot, including every early-return path.
            Volatile.Write(ref _inFlight, 0);
        }
    }

    private static FrameExchangeResult NotSent(FrameExchangeFailure failure) => new(FrameExchangeStatus.NotSent, failure);
    private static FrameExchangeResult Unknown(FrameExchangeFailure failure) => new(FrameExchangeStatus.UnknownOutcome, failure);
}
