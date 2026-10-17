namespace Cinis.Trading.Ports;

public interface IAccountAuthorizer
{
    ValueTask<bool> CanTradeAsync(string principalId, ulong accountId, CancellationToken cancellationToken);
}
