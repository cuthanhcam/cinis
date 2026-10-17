using Cinis.Access.Contracts;
using Cinis.Access.Ports;

namespace Cinis.Access.Infrastructure.Persistence;

/// <summary>Immutable, explicitly provisioned local grants. No key authentication or persistence.</summary>
public sealed class InMemoryAccountGrantStore : IAccountGrantStore
{
    private readonly HashSet<AccountGrant> _grants;

    public InMemoryAccountGrantStore(IEnumerable<AccountGrant> grants)
    {
        ArgumentNullException.ThrowIfNull(grants);
        _grants = [];
        foreach (var grant in grants)
        {
            ArgumentNullException.ThrowIfNull(grant);
            if (string.IsNullOrWhiteSpace(grant.PrincipalId) || grant.AccountId == 0 || !Enum.IsDefined(grant.Action))
                throw new ArgumentException("Grants need a principal, nonzero account and known action.", nameof(grants));
            _grants.Add(grant);
        }
    }

    public ValueTask<bool> HasGrantAsync(AccountGrant grant, CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return ValueTask.FromResult(_grants.Contains(grant));
    }
}
