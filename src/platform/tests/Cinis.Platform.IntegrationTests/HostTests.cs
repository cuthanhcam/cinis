using System.Net;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.Extensions.DependencyInjection;
using Cinis.Trading.Features.CancelOrder;

namespace Cinis.Platform.IntegrationTests;

public sealed class HostTests : IClassFixture<WebApplicationFactory<Program>>
{
    private readonly WebApplicationFactory<Program> _factory;
    public HostTests(WebApplicationFactory<Program> factory) => _factory = factory;

    [Fact]
    public async Task Liveness_DoesNotImplyTradingReadiness()
    {
        using var client = _factory.CreateClient();
        Assert.Equal(HttpStatusCode.OK, (await client.GetAsync("/health/live")).StatusCode);
        var ready = await client.GetAsync("/health/ready");
        Assert.Equal(HttpStatusCode.ServiceUnavailable, ready.StatusCode);
        Assert.Contains("execution_not_configured", await ready.Content.ReadAsStringAsync());
    }

    [Theory]
    [InlineData("/orders")]
    [InlineData("/funding")]
    [InlineData("/api-keys")]
    public async Task FinancialAndManagementRoutes_AreNotExposed(string route)
    {
        using var client = _factory.CreateClient();
        Assert.Equal(HttpStatusCode.NotFound, (await client.PostAsync(route, null)).StatusCode);
    }

    [Fact]
    public async Task ActualComposition_DeniesUnprovisionedCaller()
    {
        var handler = _factory.Services.GetRequiredService<CancelOrderHandler>();
        var result = await handler.HandleAsync("unprovisioned", new(1, 1, 1));
        Assert.Equal(CancelOrderStatus.Forbidden, result.Status);
    }
}
