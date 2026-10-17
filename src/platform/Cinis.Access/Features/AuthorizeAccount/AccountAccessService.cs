using Cinis.Access.Contracts;
using Cinis.Access.Ports;

namespace Cinis.Access.Features.AuthorizeAccount;

/// <summary>Checks explicit account/action grants; absent and malformed grants deny.</summary>
public sealed class AccountAccessService(IAccountGrantStore store)
{
    public ValueTask<bool> IsAllowedAsync(string principalId, ulong accountId, AccountAction action,
        CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        if (string.IsNullOrWhiteSpace(principalId) || accountId == 0 || !Enum.IsDefined(action))
            return ValueTask.FromResult(false);
        return store.HasGrantAsync(new AccountGrant(principalId, accountId, action), cancellationToken);
    }
}
