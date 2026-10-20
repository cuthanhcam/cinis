namespace Cinis.Trading.Ports;

/// <summary>Consumer-owned access boundary that every trading entry point must invoke.</summary>
public interface IAccountAuthorizer
{
    /// <summary>Checks current permission to trade for one account.</summary>
    /// <param name="principalId">Identity already established by trusted caller code.</param>
    /// <param name="accountId">Target account, not an account selected from a balance projection.</param>
    /// <param name="cancellationToken">Cancellation of authorization work.</param>
    /// <returns>True only when this account/action is authorized.</returns>
    /// <exception cref="OperationCanceledException">Authorization was cancelled.</exception>
    ValueTask<bool> CanTradeAsync(
        string principalId,
        ulong accountId,
        CancellationToken cancellationToken);
}
