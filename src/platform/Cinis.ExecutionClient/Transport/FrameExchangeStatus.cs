namespace Cinis.ExecutionClient.Transport;

/// <summary>Describes transport evidence without assigning a financial outcome.</summary>
public enum FrameExchangeStatus
{
    /// <summary>A complete bounded response frame arrived; its business schema is not yet validated.</summary>
    ReplyReceived,
    /// <summary>No application-frame write was attempted, so this invocation did not submit a command.</summary>
    NotSent,
    /// <summary>A write was attempted, but no complete response arrived; the peer may have applied the intent.</summary>
    UnknownOutcome
}
