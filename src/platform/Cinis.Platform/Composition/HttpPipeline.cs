using System.Diagnostics;
using System.Reflection;
using System.Text.RegularExpressions;
using Cinis.Platform.Diagnostics;
using Cinis.Platform.Errors;
using Cinis.Platform.Middleware;
using Microsoft.Extensions.Logging.Console;

namespace Cinis.Platform.Composition;

internal static class HttpPipeline
{
    internal static void AddPlatformHttp(this WebApplicationBuilder builder)
    {
        builder.Logging.ClearProviders();
        builder.Logging.Configure(options => options.ActivityTrackingOptions =
            ActivityTrackingOptions.TraceId | ActivityTrackingOptions.SpanId | ActivityTrackingOptions.ParentId);
        builder.Logging.AddConsole(options => options.FormatterName = "cinis-json")
            .AddConsoleFormatter<CinisJsonConsoleFormatter, ConsoleFormatterOptions>();
        var candidate = Environment.GetEnvironmentVariable("CINIS_RUN_ID") ?? "";
        var runId = Regex.IsMatch(candidate, @"\A\d{8}T\d{9}Z-[a-f0-9]{8}\z", RegexOptions.CultureInvariant)
            ? candidate : Guid.NewGuid().ToString("N");
        builder.Services.AddSingleton(new ServiceIdentity(builder.Environment.EnvironmentName,
            typeof(HttpPipeline).Assembly.GetCustomAttribute<AssemblyInformationalVersionAttribute>()?.InformationalVersion ?? "unknown",
            Environment.ProcessId, runId));
        builder.Services.AddProblemDetails(options => options.CustomizeProblemDetails = context =>
        {
            context.ProblemDetails.Extensions["requestId"] = context.HttpContext.TraceIdentifier;
            context.ProblemDetails.Extensions["traceId"] = Activity.Current?.TraceId.ToString() ?? context.HttpContext.TraceIdentifier;
        });
        builder.Services.AddExceptionHandler<PlatformExceptionHandler>();
    }

    internal static void UsePlatformHttp(this WebApplication app)
    {
        // Routing precedes diagnostics so events use route templates. Diagnostics wraps error handling so completion
        // sees the translated status; the exception handler and problem writer share the same request scope.
        app.UseRouting();
        app.UseMiddleware<RequestDiagnosticsMiddleware>();
        app.UseExceptionHandler();
        app.UseStatusCodePages(async context => await Results.Problem(statusCode: context.HttpContext.Response.StatusCode)
            .ExecuteAsync(context.HttpContext));
    }
}
