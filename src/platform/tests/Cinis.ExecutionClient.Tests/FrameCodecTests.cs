using System.Buffers.Binary;
using Cinis.ExecutionClient.Protocol;
using Cinis.Trading.Contracts;

namespace Cinis.ExecutionClient.Tests;

public sealed class FrameCodecTests
{
    private sealed class FragmentedStream(byte[] bytes) : MemoryStream(bytes)
    {
        public override ValueTask<int> ReadAsync(
            Memory<byte> buffer,
            CancellationToken cancellationToken = default) =>
            base.ReadAsync(buffer[..Math.Min(buffer.Length, 1)], cancellationToken);
    }

    [Fact]
    public async Task FragmentedRead_PreservesExactUnsignedPayload()
    {
        var codec = new BoundedFrameCodec(8);
        var payload = new byte[8];
        BinaryPrimitives.WriteUInt64BigEndian(payload, ulong.MaxValue);
        using var wire = new MemoryStream();
        await codec.WriteAsync(wire, payload);
        Assert.Equal(new byte[] { 0, 0, 0, 8 }, wire.ToArray()[..4]);
        using var fragments = new FragmentedStream(wire.ToArray());
        Assert.Equal(ulong.MaxValue, BinaryPrimitives.ReadUInt64BigEndian(await codec.ReadAsync(fragments)));
    }

    [Theory]
    [InlineData(0U)]
    [InlineData(9U)]
    [InlineData(uint.MaxValue)]
    public async Task InvalidLength_RejectsBeforePayloadRead(uint length)
    {
        var header = new byte[4];
        BinaryPrimitives.WriteUInt32BigEndian(header, length);
        using var wire = new MemoryStream(header);
        await Assert.ThrowsAsync<InvalidDataException>(() => new BoundedFrameCodec(8).ReadAsync(wire));
        Assert.Equal(4, wire.Position);
    }

    [Theory]
    [InlineData(0)]
    [InlineData(3)]
    [InlineData(5)]
    public async Task TruncatedFrame_Rejects(int bytes)
    {
        using var wire = new MemoryStream(new byte[] { 0, 0, 0, 2, 1, 2 }[..bytes]);
        await Assert.ThrowsAsync<EndOfStreamException>(() => new BoundedFrameCodec(8).ReadAsync(wire));
    }

    [Theory]
    [InlineData(0)]
    [InlineData(9)]
    public async Task InvalidWrite_DoesNotWriteHeader(int size)
    {
        using var wire = new MemoryStream();
        await Assert.ThrowsAsync<InvalidDataException>(() => new BoundedFrameCodec(8).WriteAsync(wire, new byte[size]));
        Assert.Equal(0, wire.Length);
    }

    [Fact]
    public async Task ConsecutiveFrames_HaveIndependentBoundaries()
    {
        var codec = new BoundedFrameCodec(8);
        using var wire = new MemoryStream();
        await codec.WriteAsync(wire, new byte[] { 1 });
        await codec.WriteAsync(wire, new byte[] { 2, 3 });
        wire.Position = 0;
        Assert.Equal(new byte[] { 1 }, await codec.ReadAsync(wire));
        Assert.Equal(new byte[] { 2, 3 }, await codec.ReadAsync(wire));
    }

    [Fact]
    public async Task DisabledAdapter_NeverClaimsSubmission()
    {
        var result = await new DisabledExecutionGateway().CancelAsync(new(1, 1, 1), default);
        Assert.Equal(SubmissionStatus.UnavailableBeforeSend, result.Status);
        Assert.Null(result.Sequence);
    }
}
