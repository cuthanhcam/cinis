using Cinis.Platform.Composition;
using Cinis.Platform.Endpoints;
using Cinis.Trading.Contracts;
using Cinis.Trading.Features.CancelOrder;
using Cinis.Trading.Ports;

var smokeTest = args.Contains("--smoke-test", StringComparer.Ordinal);
var builder = WebApplication.CreateBuilder(args.Where(a => a != "--smoke-test").ToArray());
if (builder.Configuration["urls"] is null) builder.WebHost.UseUrls("http://localhost:5080");
builder.AddPlatformHttp();
builder.Services.AddPlatformModules();
var app = builder.Build();

app.UsePlatformHttp();
app.MapPlatformHealth();
// No financial or management endpoints until authentication and the wire contract are implemented.

if (smokeTest)
{
    var intent = new CancelOrderIntent(1, 1, 1);
    var denied = await app.Services.GetRequiredService<CancelOrderHandler>().HandleAsync("smoke", intent);
    var unavailable = await app.Services.GetRequiredService<IExecutionGateway>().CancelAsync(intent, default);
    if (denied.Status != CancelOrderStatus.Forbidden || unavailable.Status != SubmissionStatus.UnavailableBeforeSend)
        throw new InvalidOperationException("Scaffold must deny unprovisioned access and never simulate execution.");
    Console.WriteLine("Cinis platform smoke passed: empty grants deny; execution disabled; trading readiness false.");
    await app.DisposeAsync();
    return;
}

await app.RunAsync();

/// <summary>Entry point exposed for ASP.NET Core integration hosting.</summary>
public partial class Program;
