using Cinis.Access.Contracts;

namespace Cinis.Access.Ports;

/// <summary>Reads grant state without exposing a database context or persistence entity.</summary>
public interface IAccountGrantStore
{
    /// <summary>Determines whether the exact principal/account/action grant currently exists.</summary>
    /// <param name="grant">Validated, non-null grant identity to inspect.</param>
    /// <param name="cancellationToken">Cancellation of the lookup.</param>
    /// <returns>True only for an explicitly present grant.</returns>
    /// <exception cref="ArgumentNullException">The grant is absent.</exception>
    /// <exception cref="OperationCanceledException">Lookup cancellation was requested.</exception>
    /// <remarks>Storage errors propagate; implementations must not convert unavailable storage into permission.</remarks>
    ValueTask<bool> HasGrantAsync(AccountGrant grant, CancellationToken cancellationToken);
}
