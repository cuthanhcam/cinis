using Cinis.Trading.Contracts;
using Cinis.Trading.Ports;

namespace Cinis.Trading.Features.CancelOrder;

/// <summary>Validates/authorizes a trusted caller and forwards once, preserving command identity.</summary>
/// <param name="authorizer">Required account/action check, independent from HTTP middleware.</param>
/// <param name="gateway">Execution adapter responsible for transport certainty.</param>
public sealed class CancelOrderHandler(IAccountAuthorizer authorizer, IExecutionGateway gateway)
{
    private readonly IAccountAuthorizer _authorizer = authorizer ?? throw new ArgumentNullException(nameof(authorizer));
    private readonly IExecutionGateway _gateway = gateway ?? throw new ArgumentNullException(nameof(gateway));

    /// <summary>Validates, authorizes, then invokes execution at most once.</summary>
    /// <param name="principalId">Trusted principal; this method does not authenticate credentials.</param>
    /// <param name="intent">Nonzero account/command/order identities.</param>
    /// <param name="cancellationToken">Cancellation of local work; the adapter classifies cancellation after send.</param>
    /// <returns>Validation/access denial or unchanged execution-adapter evidence.</returns>
    /// <exception cref="ArgumentNullException">The intent is absent.</exception>
    /// <exception cref="OperationCanceledException">Cancellation occurs before execution takes ownership, or a dependency violates the outcome contract.</exception>
    /// <remarks>There is no financial mutation, idempotency cache or retry policy in this use case.</remarks>
    public async Task<CancelOrderResult> HandleAsync(string principalId, CancelOrderIntent intent,
        CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(intent);
        cancellationToken.ThrowIfCancellationRequested();
        if (string.IsNullOrWhiteSpace(principalId) || intent.AccountId == 0 || intent.CommandId == 0 || intent.OrderId == 0)
            return new(CancelOrderStatus.InvalidIntent, null);
        if (!await _authorizer.CanTradeAsync(principalId, intent.AccountId, cancellationToken).ConfigureAwait(false))
            return new(CancelOrderStatus.Forbidden, null);
        // The adapter owns send certainty. Never catch a post-send failure and invent a rejection/retry.
        var submission = await _gateway.CancelAsync(intent, cancellationToken).ConfigureAwait(false);
        return new(CancelOrderStatus.ExecutionOutcome, submission);
    }
}
