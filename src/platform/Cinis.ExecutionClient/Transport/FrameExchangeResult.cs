namespace Cinis.ExecutionClient.Transport;

/// <summary>Immutable transport evidence returned by one channel invocation.</summary>
/// <remarks>A received body still needs version, identity and semantic validation before it is an execution result.</remarks>
public sealed class FrameExchangeResult
{
    internal FrameExchangeResult(FrameExchangeStatus status, FrameExchangeFailure failure, byte[]? reply = null)
    {
        Status = status;
        Failure = failure;
        Reply = reply ?? ReadOnlyMemory<byte>.Empty;
    }

    /// <summary>Gets whether a frame arrived, nothing was sent, or the outcome remains uncertain.</summary>
    public FrameExchangeStatus Status { get; }
    /// <summary>Gets a local failure category without exposing exception messages or peer data.</summary>
    public FrameExchangeFailure Failure { get; }
    /// <summary>Gets the received body for ReplyReceived; otherwise an empty memory region.</summary>
    public ReadOnlyMemory<byte> Reply { get; }
}
