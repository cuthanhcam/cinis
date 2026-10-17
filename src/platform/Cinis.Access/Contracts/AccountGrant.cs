namespace Cinis.Access.Contracts;

/// <summary>A trusted local grant, not an authenticated public credential.</summary>
public sealed record AccountGrant(string PrincipalId, ulong AccountId, AccountAction Action);
