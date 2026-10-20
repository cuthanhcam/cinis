using Cinis.Trading.Contracts;

namespace Cinis.Trading.Ports;

/// <summary>Adapters classify failures before send versus uncertain outcomes after send.
/// Cancellation after send must not be presented as a business rejection or retried with a new identity.</summary>
public interface IExecutionGateway
{
    /// <summary>Submits the same typed cancel intent once, without inventing a new command identity.</summary>
    /// <param name="intent">Validated intent whose identity/payload must remain stable on any later retry.</param>
    /// <param name="cancellationToken">Caller cancellation; it cannot reverse a completed peer transition.</param>
    /// <returns>Known execution evidence, definitely unavailable-before-send, or unknown outcome.</returns>
    /// <exception cref="ArgumentNullException">The intent is absent.</exception>
    /// <remarks>Adapters must preserve post-send uncertainty, including cancellation; callers must not infer rejection from a lost reply.</remarks>
    Task<ExecutionSubmission> CancelAsync(
        CancelOrderIntent intent,
        CancellationToken cancellationToken);
}
