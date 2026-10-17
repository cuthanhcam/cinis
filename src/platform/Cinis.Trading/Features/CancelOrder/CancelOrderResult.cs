using Cinis.Trading.Contracts;

namespace Cinis.Trading.Features.CancelOrder;

public enum CancelOrderStatus { InvalidIntent, Forbidden, ExecutionOutcome }

public sealed record CancelOrderResult(CancelOrderStatus Status, ExecutionSubmission? Submission);
