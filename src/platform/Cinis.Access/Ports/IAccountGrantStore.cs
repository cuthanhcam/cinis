using Cinis.Access.Contracts;

namespace Cinis.Access.Ports;

public interface IAccountGrantStore
{
    ValueTask<bool> HasGrantAsync(AccountGrant grant, CancellationToken cancellationToken);
}
