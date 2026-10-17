using Cinis.Access.Contracts;
using Cinis.Access.Features.AuthorizeAccount;
using Cinis.Trading.Ports;

namespace Cinis.Platform.Adapters;

internal sealed class AccessAuthorizationAdapter(AccountAccessService access) : IAccountAuthorizer
{
    public ValueTask<bool> CanTradeAsync(string principalId, ulong accountId, CancellationToken cancellationToken) =>
        access.IsAllowedAsync(principalId, accountId, AccountAction.Trade, cancellationToken);
}
