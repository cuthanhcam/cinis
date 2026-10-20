using Cinis.Access.Contracts;
using Cinis.Access.Ports;

namespace Cinis.Access.Features.AuthorizeAccount;

/// <summary>Checks explicit account/action grants; absent and malformed grants deny.</summary>
/// <param name="store">Module-owned grant reader; persistence failures must propagate rather than grant access.</param>
public sealed class AccountAccessService(IAccountGrantStore store)
{
    private readonly IAccountGrantStore _store = store ?? throw new ArgumentNullException(nameof(store));

    /// <summary>Checks one exact grant without treating a principal string as authentication.</summary>
    /// <param name="principalId">Trusted, case-sensitive principal identity.</param>
    /// <param name="accountId">Target account identity.</param>
    /// <param name="action">Specific required permission.</param>
    /// <param name="cancellationToken">Cancellation of the grant lookup.</param>
    /// <returns>False for malformed/absent grants; otherwise the store's current decision.</returns>
    /// <exception cref="OperationCanceledException">Lookup cancellation was requested.</exception>
    /// <remarks>No authorization decision is cached here; future revocation/caching needs an explicit policy.</remarks>
    public ValueTask<bool> IsAllowedAsync(string principalId, ulong accountId, AccountAction action,
        CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        if (string.IsNullOrWhiteSpace(principalId) || accountId == 0 || !Enum.IsDefined(action))
            return ValueTask.FromResult(false);
        return _store.HasGrantAsync(new AccountGrant(principalId, accountId, action), cancellationToken);
    }
}
