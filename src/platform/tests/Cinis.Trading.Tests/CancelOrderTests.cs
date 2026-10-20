using Cinis.Trading.Contracts;
using Cinis.Trading.Features.CancelOrder;
using Cinis.Trading.Ports;

namespace Cinis.Trading.Tests;

public sealed class CancelOrderTests
{
    [Fact]
    public void MissingDependency_FailsBeforeHandlingAnIntent()
    {
        var gateway = new Gateway(new(SubmissionStatus.UnavailableBeforeSend, null, "test"));
        Assert.Throws<ArgumentNullException>(() => new CancelOrderHandler(null!, gateway));
        Assert.Throws<ArgumentNullException>(() => new CancelOrderHandler(new Authorizer(true), null!));
    }

    private sealed class Authorizer(bool allowed) : IAccountAuthorizer
    {
        public ValueTask<bool> CanTradeAsync(string principalId, ulong accountId, CancellationToken cancellationToken) =>
            ValueTask.FromResult(allowed);
    }

    private sealed class Gateway(ExecutionSubmission result) : IExecutionGateway
    {
        public List<CancelOrderIntent> Received { get; } = [];
        public Task<ExecutionSubmission> CancelAsync(CancelOrderIntent intent, CancellationToken cancellationToken)
        {
            Received.Add(intent);
            return Task.FromResult(result);
        }
    }

    [Theory]
    [InlineData(0UL, 1UL, 1UL)]
    [InlineData(1UL, 0UL, 1UL)]
    [InlineData(1UL, 1UL, 0UL)]
    public async Task InvalidIntent_NeverForwards(ulong account, ulong command, ulong order)
    {
        var gateway = new Gateway(new(SubmissionStatus.KnownResult, 1, "test"));
        var result = await new CancelOrderHandler(new Authorizer(true), gateway).HandleAsync("alice", new(account, command, order));
        Assert.Equal(CancelOrderStatus.InvalidIntent, result.Status);
        Assert.Empty(gateway.Received);
    }

    [Fact]
    public async Task Forbidden_NeverContactsExecution()
    {
        var gateway = new Gateway(new(SubmissionStatus.KnownResult, 1, "test"));
        var result = await new CancelOrderHandler(new Authorizer(false), gateway).HandleAsync("alice", new(1, 1, 1));
        Assert.Equal(CancelOrderStatus.Forbidden, result.Status);
        Assert.Null(result.Submission);
        Assert.Empty(gateway.Received);
    }

    [Theory]
    [InlineData(SubmissionStatus.KnownResult)]
    [InlineData(SubmissionStatus.UnknownOutcome)]
    [InlineData(SubmissionStatus.UnavailableBeforeSend)]
    public async Task Authorized_PreservesIdentityAndOutcome_WithoutRetry(SubmissionStatus status)
    {
        var submission = new ExecutionSubmission(status, status == SubmissionStatus.KnownResult ? 17UL : null, "test");
        var gateway = new Gateway(submission);
        var intent = new CancelOrderIntent(ulong.MaxValue, ulong.MaxValue, ulong.MaxValue);
        var result = await new CancelOrderHandler(new Authorizer(true), gateway).HandleAsync("alice", intent);
        Assert.Equal(CancelOrderStatus.ExecutionOutcome, result.Status);
        Assert.Equal(submission, result.Submission);
        Assert.Equal(intent, Assert.Single(gateway.Received));
    }
}
