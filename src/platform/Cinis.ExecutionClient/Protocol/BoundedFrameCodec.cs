using System.Buffers.Binary;

namespace Cinis.ExecutionClient.Protocol;

/// <summary>Experimental four-byte big-endian length framing. Not an accepted financial wire schema.</summary>
public sealed class BoundedFrameCodec
{
    public int MaxPayloadBytes { get; }

    public BoundedFrameCodec(int maxPayloadBytes)
    {
        if (maxPayloadBytes <= 0) throw new ArgumentOutOfRangeException(nameof(maxPayloadBytes));
        MaxPayloadBytes = maxPayloadBytes;
    }

    public async Task<byte[]> ReadAsync(Stream stream, CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(stream);
        var header = new byte[4];
        await stream.ReadExactlyAsync(header, cancellationToken);
        var length = BinaryPrimitives.ReadUInt32BigEndian(header);
        if (length == 0 || length > MaxPayloadBytes)
            throw new InvalidDataException("Frame size is outside the configured bound.");
        var payload = new byte[(int)length];
        await stream.ReadExactlyAsync(payload, cancellationToken);
        return payload;
    }

    public async Task WriteAsync(Stream stream, ReadOnlyMemory<byte> payload,
        CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(stream);
        if (payload.IsEmpty || payload.Length > MaxPayloadBytes)
            throw new InvalidDataException("Frame size is outside the configured bound.");
        var header = new byte[4];
        BinaryPrimitives.WriteUInt32BigEndian(header, (uint)payload.Length);
        // One owner must serialize writes. A failed write leaves connection state uncertain.
        await stream.WriteAsync(header, cancellationToken);
        await stream.WriteAsync(payload, cancellationToken);
    }
}
