using Cinis.Trading.Contracts;

namespace Cinis.Trading.Features.CancelOrder;

/// <summary>Classifies local handling without overriding the execution owner's decision.</summary>
public enum CancelOrderStatus
{
    /// <summary>No execution call was made because an intent identity was invalid.</summary>
    InvalidIntent,
    /// <summary>No execution call was made because account access was denied.</summary>
    Forbidden,
    /// <summary>The adapter returned evidence; inspect it rather than assuming that a command was sent.</summary>
    ExecutionOutcome
}

/// <summary>Combines local validation/authorization with optional execution evidence.</summary>
/// <param name="Status">Local handling classification.</param>
/// <param name="Submission">Adapter evidence only when execution was invoked; otherwise null.</param>
public sealed record CancelOrderResult(
    CancelOrderStatus Status,
    ExecutionSubmission? Submission);
