namespace Cinis.Trading.Contracts;

/// <summary>Transport certainty is independent from financial success.</summary>
public enum SubmissionStatus { KnownResult, UnavailableBeforeSend, UnknownOutcome }

/// <summary>Application-level transport classification. Business result encoding awaits the spike.</summary>
public sealed record ExecutionSubmission(SubmissionStatus Status, ulong? Sequence, string Code);
