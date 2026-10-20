using Cinis.Access.Contracts;
using Cinis.Access.Features.AuthorizeAccount;
using Cinis.Access.Infrastructure.Persistence;

namespace Cinis.Access.Tests;

public sealed class AccountAccessTests
{
    [Fact]
    public void MissingStore_FailsAtCompositionBoundary() =>
        Assert.Throws<ArgumentNullException>(() => new AccountAccessService(null!));

    [Fact]
    public async Task ExplicitGrant_DifferentPrincipalAccountOrAction_Denies()
    {
        var access = new AccountAccessService(new InMemoryAccountGrantStore([new("alice", 1, AccountAction.Read)]));
        Assert.True(await access.IsAllowedAsync("alice", 1, AccountAction.Read));
        Assert.False(await access.IsAllowedAsync("bob", 1, AccountAction.Read));
        Assert.False(await access.IsAllowedAsync("alice", 2, AccountAction.Read));
        Assert.False(await access.IsAllowedAsync("alice", 1, AccountAction.Trade));
        Assert.False(await access.IsAllowedAsync("alice", 1, AccountAction.Administer));
    }

    [Fact]
    public async Task EmptyStore_AllValidRequests_Deny()
    {
        var access = new AccountAccessService(new InMemoryAccountGrantStore([]));
        Assert.False(await access.IsAllowedAsync("alice", 1, AccountAction.Trade));
    }

    [Theory]
    [InlineData("", 1UL, AccountAction.Read)]
    [InlineData(" ", 1UL, AccountAction.Read)]
    [InlineData("alice", 0UL, AccountAction.Read)]
    [InlineData("alice", 1UL, (AccountAction)999)]
    public async Task InvalidRequest_Denies(
        string principal,
        ulong account,
        AccountAction action)
    {
        var access = new AccountAccessService(new InMemoryAccountGrantStore([]));
        Assert.False(await access.IsAllowedAsync(principal, account, action));
    }

    [Fact]
    public async Task Construction_CopiesTrustedGrants_WithoutMutableExternalAlias()
    {
        List<AccountGrant> grants = [new("alice", 1, AccountAction.Trade)];
        var store = new InMemoryAccountGrantStore(grants);
        grants.Clear();
        Assert.True(await store.HasGrantAsync(new("alice", 1, AccountAction.Trade), default));
        Assert.Throws<ArgumentException>(() => new InMemoryAccountGrantStore([new("alice", 0, AccountAction.Read)]));
    }

    [Fact]
    public async Task CancelledCheck_PropagatesCancellation()
    {
        var access = new AccountAccessService(new InMemoryAccountGrantStore([]));
        await Assert.ThrowsAnyAsync<OperationCanceledException>(async () =>
            await access.IsAllowedAsync("alice", 1, AccountAction.Read, new CancellationToken(true)));
    }
}
