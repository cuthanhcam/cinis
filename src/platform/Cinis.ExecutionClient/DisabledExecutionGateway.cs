using Cinis.Trading.Contracts;
using Cinis.Trading.Ports;

namespace Cinis.ExecutionClient;

/// <summary>Explicit scaffold mode: sends nothing and cannot report a financial success.</summary>
public sealed class DisabledExecutionGateway : IExecutionGateway
{
    /// <summary>Reports that no transport is configured without contacting a peer.</summary>
    /// <param name="intent">Non-null application intent; no bytes are sent.</param>
    /// <param name="cancellationToken">Pre-send cancellation, propagated without any submission.</param>
    /// <returns>An unavailable-before-send result with no execution sequence.</returns>
    /// <exception cref="ArgumentNullException">The intent is absent.</exception>
    /// <exception cref="OperationCanceledException">Cancellation was requested before any work.</exception>
    public Task<ExecutionSubmission> CancelAsync(CancelOrderIntent intent, CancellationToken cancellationToken)
    {
        ArgumentNullException.ThrowIfNull(intent);
        cancellationToken.ThrowIfCancellationRequested();
        return Task.FromResult(new ExecutionSubmission(SubmissionStatus.UnavailableBeforeSend, null, "execution_not_configured"));
    }
}
