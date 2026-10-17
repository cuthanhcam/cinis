using Cinis.Trading.Contracts;
using Cinis.Trading.Ports;

namespace Cinis.Trading.Features.CancelOrder;

/// <summary>Validates/authorizes a trusted caller and forwards once, preserving command identity.</summary>
public sealed class CancelOrderHandler(IAccountAuthorizer authorizer, IExecutionGateway gateway)
{
    public async Task<CancelOrderResult> HandleAsync(string principalId, CancelOrderIntent intent,
        CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(intent);
        cancellationToken.ThrowIfCancellationRequested();
        if (string.IsNullOrWhiteSpace(principalId) || intent.AccountId == 0 || intent.CommandId == 0 || intent.OrderId == 0)
            return new(CancelOrderStatus.InvalidIntent, null);
        if (!await authorizer.CanTradeAsync(principalId, intent.AccountId, cancellationToken))
            return new(CancelOrderStatus.Forbidden, null);
        // The adapter owns send certainty. Never catch a post-send failure and invent a rejection/retry.
        var submission = await gateway.CancelAsync(intent, cancellationToken);
        return new(CancelOrderStatus.ExecutionOutcome, submission);
    }
}
