using Cinis.Trading.Contracts;
using Cinis.Trading.Ports;

namespace Cinis.ExecutionClient;

/// <summary>Explicit scaffold mode: sends nothing and cannot report a financial success.</summary>
public sealed class DisabledExecutionGateway : IExecutionGateway
{
    public Task<ExecutionSubmission> CancelAsync(CancelOrderIntent intent, CancellationToken cancellationToken)
    {
        ArgumentNullException.ThrowIfNull(intent);
        cancellationToken.ThrowIfCancellationRequested();
        return Task.FromResult(new ExecutionSubmission(SubmissionStatus.UnavailableBeforeSend, null, "execution_not_configured"));
    }
}
