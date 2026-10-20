using Cinis.Access.Contracts;
using Cinis.Access.Features.AuthorizeAccount;
using Cinis.Trading.Ports;

namespace Cinis.Platform.Adapters;

/// <summary>Bridges Trading's consumer-owned authorization port to Access's public use case.</summary>
/// <remarks>Keeping the bridge in the host prevents a Trading-to-Access storage dependency.</remarks>
internal sealed class AccessAuthorizationAdapter(AccountAccessService access) : IAccountAuthorizer
{
    public ValueTask<bool> CanTradeAsync(string principalId, ulong accountId, CancellationToken cancellationToken) =>
        access.IsAllowedAsync(principalId, accountId, AccountAction.Trade, cancellationToken);
}
