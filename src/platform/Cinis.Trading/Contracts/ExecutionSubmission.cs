namespace Cinis.Trading.Contracts;

/// <summary>Transport certainty is independent from financial success.</summary>
public enum SubmissionStatus
{
    /// <summary>A validated execution result is known, including a possible business/admission rejection.</summary>
    KnownResult,
    /// <summary>The adapter knows that application submission did not start.</summary>
    UnavailableBeforeSend,
    /// <summary>Submission may have reached execution; retry requires the original identity and payload.</summary>
    UnknownOutcome
}

/// <summary>Application-level transport classification. Business result encoding awaits the spike.</summary>
/// <param name="Status">Evidence about the submission; this alone does not mean financial success.</param>
/// <param name="Sequence">Known admitted decision sequence, or null when none is established.</param>
/// <param name="Code">Local application diagnostic code; not a public wire schema.</param>
public sealed record ExecutionSubmission(SubmissionStatus Status, ulong? Sequence, string Code);
