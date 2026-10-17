using Cinis.Trading.Contracts;

namespace Cinis.Trading.Ports;

/// <summary>Adapters classify failures before send versus uncertain outcomes after send.
/// Cancellation after send must not be presented as a business rejection or retried with a new identity.</summary>
public interface IExecutionGateway
{
    Task<ExecutionSubmission> CancelAsync(CancelOrderIntent intent, CancellationToken cancellationToken);
}
