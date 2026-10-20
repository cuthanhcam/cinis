namespace Cinis.Access.Contracts;

/// <summary>A trusted local grant, not an authenticated public credential.</summary>
/// <param name="PrincipalId">Case-sensitive identity supplied by trusted application code.</param>
/// <param name="AccountId">Nonzero account identity; no financial state is stored in a grant.</param>
/// <param name="Action">The exact permitted action, without inherited privileges.</param>
public sealed record AccountGrant(
    string PrincipalId,
    ulong AccountId,
    AccountAction Action);
