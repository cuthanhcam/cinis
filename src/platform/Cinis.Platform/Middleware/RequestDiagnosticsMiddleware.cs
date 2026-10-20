using System.Diagnostics;
using Cinis.Platform.Diagnostics;
using Microsoft.AspNetCore.Routing;

namespace Cinis.Platform.Middleware;

/// <summary>Correlates HTTP requests and emits one bounded completion event without request payloads.</summary>
/// <param name="next">Remaining pipeline, including exception handling.</param>
/// <param name="logger">Structured request-event sink.</param>
/// <param name="identity">Immutable host/run identity.</param>
public sealed class RequestDiagnosticsMiddleware(
    RequestDelegate next,
    ILogger<RequestDiagnosticsMiddleware> logger,
    ServiceIdentity identity)
{
    /// <summary>Assigns a local request ID and records the final outcome under a diagnostic scope.</summary>
    /// <param name="context">Current request; incoming request-ID headers are not trusted or echoed.</param>
    /// <remarks>W3C trace context is correlation data, never an authorization identity.</remarks>
    public async Task InvokeAsync(HttpContext context)
    {
        context.TraceIdentifier = Guid.NewGuid().ToString("N");
        var traceId = Activity.Current?.TraceId.ToString() ?? context.TraceIdentifier;
        using var scope = logger.BeginScope(new Dictionary<string, object?>
        {
            ["Service"] = "Cinis.Platform",
            ["Environment"] = identity.EnvironmentName,
            ["Version"] = identity.Version,
            ["ProcessId"] = identity.ProcessId,
            ["RunId"] = identity.RunId,
            ["RequestId"] = context.TraceIdentifier,
            ["TraceId"] = traceId,
            ["SpanId"] = Activity.Current?.SpanId.ToString()
        });
        context.Response.OnStarting(() =>
        {
            context.Response.Headers["X-Request-ID"] = context.TraceIdentifier;
            context.Response.Headers["X-Trace-ID"] = traceId;
            return Task.CompletedTask;
        });
        var started = Stopwatch.GetTimestamp();
        var outcome = "Completed";
        try { await next(context); }
        catch (OperationCanceledException) when (context.RequestAborted.IsCancellationRequested)
        {
            outcome = "ClientAborted";
            if (!context.Response.HasStarted) context.Response.StatusCode = 499;
        }
        catch
        {
            outcome = "Unhandled";
            throw;
        }
        finally
        {
            // Route templates avoid recording account IDs, credentials in query strings, or arbitrary unknown paths.
            var route = (context.GetEndpoint() as RouteEndpoint)?.RoutePattern.RawText ?? "unmatched";
            var status = outcome == "Unhandled" ? 500 : context.Response.StatusCode;
            var level = status >= 500 ? LogLevel.Error : status >= 400 ? LogLevel.Warning : LogLevel.Information;
            logger.Log(level, new EventId(1000, "RequestCompleted"),
                "HTTP {Method} {Route} completed {StatusCode} in {ElapsedMilliseconds} ms ({Outcome})",
                context.Request.Method, route, status, Stopwatch.GetElapsedTime(started).TotalMilliseconds, outcome);
        }
    }
}
