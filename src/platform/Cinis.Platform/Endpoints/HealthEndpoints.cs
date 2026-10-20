using Cinis.Platform.Filters;

namespace Cinis.Platform.Endpoints;

internal static class HealthEndpoints
{
    internal static void MapPlatformHealth(this WebApplication app)
    {
        var health = app.MapGroup("/health").AddEndpointFilter<NoStoreEndpointFilter>();
        health.MapGet("/live", () => Results.Ok(new { status = "alive", mode = "scaffold" }));
        health.MapGet("/ready", () => Results.Json(new { status = "not_ready", reason = "execution_not_configured" }, statusCode: 503));
    }
}
