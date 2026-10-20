namespace Cinis.Trading.Contracts;

/// <summary>Application intent, not a frozen Rust wire schema.</summary>
/// <param name="AccountId">Nonzero owning account identity.</param>
/// <param name="CommandId">Nonzero account-scoped idempotency identity, preserved across retries.</param>
/// <param name="OrderId">Nonzero order whose remaining quantity is requested for cancellation.</param>
public sealed record CancelOrderIntent(ulong AccountId, ulong CommandId, ulong OrderId);
