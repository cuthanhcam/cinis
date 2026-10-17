namespace Cinis.Trading.Contracts;

/// <summary>Application intent, not a frozen Rust wire schema.</summary>
public sealed record CancelOrderIntent(ulong AccountId, ulong CommandId, ulong OrderId);
