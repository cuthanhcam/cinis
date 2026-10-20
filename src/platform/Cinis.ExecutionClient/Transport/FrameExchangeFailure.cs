namespace Cinis.ExecutionClient.Transport;

/// <summary>Stable local diagnostic categories; these are not versioned wire or business error codes.</summary>
public enum FrameExchangeFailure
{
    /// <summary>A complete frame arrived without an operational failure.</summary>
    None,
    /// <summary>The channel already owns one exchange; no waiting request was queued.</summary>
    Busy,
    /// <summary>The request body was empty or exceeded the configured bound.</summary>
    InvalidPayload,
    /// <summary>The caller requested cancellation; status still records whether a write was attempted.</summary>
    Cancelled,
    /// <summary>A TCP connection could not be established.</summary>
    ConnectFailed,
    /// <summary>The connection budget elapsed before application-frame writing.</summary>
    ConnectTimedOut,
    /// <summary>The combined write/read budget elapsed after writing could have begun.</summary>
    ExchangeTimedOut,
    /// <summary>The peer closed or the stream failed before a complete response.</summary>
    Disconnected,
    /// <summary>The peer sent an invalid response length.</summary>
    InvalidFrame
}
