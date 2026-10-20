using System.Buffers.Binary;

namespace Cinis.ExecutionClient.Protocol;

/// <summary>Experimental four-byte big-endian length framing. Not an accepted financial wire schema.</summary>
/// <remarks>The caller owns/disposes the stream and serializes operations. A partial write/read invalidates connection framing.</remarks>
public sealed class BoundedFrameCodec
{
    /// <summary>Gets the maximum body size in bytes, excluding the four-byte header.</summary>
    public int MaxPayloadBytes { get; }

    /// <summary>Creates a codec with an explicit trusted allocation limit.</summary>
    /// <param name="maxPayloadBytes">Positive maximum body length in bytes.</param>
    /// <exception cref="ArgumentOutOfRangeException">The size is not positive.</exception>
    public BoundedFrameCodec(int maxPayloadBytes)
    {
        if (maxPayloadBytes <= 0) throw new ArgumentOutOfRangeException(nameof(maxPayloadBytes));
        MaxPayloadBytes = maxPayloadBytes;
    }

    /// <summary>Reads exactly one bounded frame, tolerating fragmented stream reads.</summary>
    /// <param name="stream">Readable stream, left open on completion/failure.</param>
    /// <param name="cancellationToken">Read budget/cancellation supplied by the connection owner.</param>
    /// <returns>A newly allocated complete body; never a partial frame.</returns>
    /// <exception cref="ArgumentNullException">The stream is absent.</exception>
    /// <exception cref="InvalidDataException">The advertised body length is zero or exceeds the bound.</exception>
    /// <exception cref="EndOfStreamException">The header or body is incomplete.</exception>
    /// <exception cref="IOException">Stream reading fails.</exception>
    /// <exception cref="OperationCanceledException">The read is cancelled.</exception>
    public async Task<byte[]> ReadAsync(
        Stream stream,
        CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(stream);
        var header = new byte[4];
        await stream.ReadExactlyAsync(header, cancellationToken).ConfigureAwait(false);
        var length = BinaryPrimitives.ReadUInt32BigEndian(header);
        if (length == 0 || length > MaxPayloadBytes)
            throw new InvalidDataException("Frame size is outside the configured bound.");
        // Validate untrusted length before payload allocation; checked length also makes the int conversion safe.
        var payload = new byte[(int)length];
        await stream.ReadExactlyAsync(payload, cancellationToken).ConfigureAwait(false);
        return payload;
    }

    /// <summary>Writes one header/body pair; an invalid body is rejected before the first write.</summary>
    /// <param name="stream">Writable stream, left open and unflushed for its owner.</param>
    /// <param name="payload">Body bytes that the caller must keep unchanged until completion.</param>
    /// <param name="cancellationToken">Write budget/cancellation supplied by the owner.</param>
    /// <returns>A task completing only after header and body writes complete.</returns>
    /// <exception cref="ArgumentNullException">The stream is absent.</exception>
    /// <exception cref="InvalidDataException">The body is empty or oversized.</exception>
    /// <exception cref="IOException">A write fails; some bytes may already have been transmitted.</exception>
    /// <exception cref="OperationCanceledException">The write is cancelled; sent bytes are not rolled back.</exception>
    public async Task WriteAsync(
        Stream stream,
        ReadOnlyMemory<byte> payload,
        CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(stream);
        if (payload.IsEmpty || payload.Length > MaxPayloadBytes)
            throw new InvalidDataException("Frame size is outside the configured bound.");
        var header = new byte[4];
        BinaryPrimitives.WriteUInt32BigEndian(header, (uint)payload.Length);
        // One owner must serialize writes. A failed write leaves connection state uncertain.
        await stream.WriteAsync(header, cancellationToken).ConfigureAwait(false);
        await stream.WriteAsync(payload, cancellationToken).ConfigureAwait(false);
    }
}
