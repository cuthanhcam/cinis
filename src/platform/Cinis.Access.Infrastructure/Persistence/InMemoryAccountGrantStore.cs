using Cinis.Access.Contracts;
using Cinis.Access.Ports;

namespace Cinis.Access.Infrastructure.Persistence;

/// <summary>Immutable, explicitly provisioned local grants. No key authentication or persistence.</summary>
public sealed class InMemoryAccountGrantStore : IAccountGrantStore
{
    private readonly HashSet<AccountGrant> _grants;

    /// <summary>Copies explicit grants into a private set that is never mutated after construction.</summary>
    /// <param name="grants">Trusted bootstrap data; an empty collection denies all callers.</param>
    /// <exception cref="ArgumentNullException">The collection or an entry is absent.</exception>
    /// <exception cref="ArgumentException">An entry has an invalid identity or action.</exception>
    /// <remarks>Read-only concurrent lookups are safe; caller collection changes cannot alter this snapshot.</remarks>
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

    /// <inheritdoc />
    public ValueTask<bool> HasGrantAsync(
        AccountGrant grant,
        CancellationToken cancellationToken)
    {
        ArgumentNullException.ThrowIfNull(grant);
        cancellationToken.ThrowIfCancellationRequested();
        return ValueTask.FromResult(_grants.Contains(grant));
    }
}
