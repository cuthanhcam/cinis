using System.Collections.Concurrent;
using System.Net;
using System.Text.Json;
using Cinis.Platform.Composition;
using Cinis.Platform.Diagnostics;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.TestHost;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;

namespace Cinis.Platform.IntegrationTests;

public sealed class DiagnosticsTests
{
    [Fact]
    public void ConsoleFormatter_WritesOneJsonLineAndDropsRawFrameworkScopesAndExceptionObjects()
    {
        var scopes = new LoggerExternalScopeProvider();
        using var scope = scopes.Push(new Dictionary<string, object?>
        { ["TraceId"] = "trace", ["RequestPath"] = "/private", ["Authorization"] = "secret" });
        var state = new Dictionary<string, object?> { ["StatusCode"] = 500 };
        var entry = new LogEntry<Dictionary<string, object?>>(LogLevel.Error, "Cinis.Test", new EventId(1001, "Failure"),
            state, new Exception("secret-exception"), (_, _) => "safe\nmessage");
        using var writer = new StringWriter();
        new CinisJsonConsoleFormatter(new("Test", "1.0", 1, "run")).Write(entry, scopes, writer);
        var output = writer.ToString();
        Assert.Single(output.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries));
        using var parsed = JsonDocument.Parse(output);
        Assert.Equal("safe\nmessage", parsed.RootElement.GetProperty("Message").GetString());
        Assert.EndsWith("+00:00", parsed.RootElement.GetProperty("Timestamp").GetString());
        Assert.Equal(500, parsed.RootElement.GetProperty("Fields").GetProperty("StatusCode").GetInt32());
        Assert.DoesNotContain("/private", output);
        Assert.DoesNotContain("secret", output);
    }

    // These test-only routes exercise the same composition methods as Program without shipping fault endpoints.
    private static async Task<WebApplication> StartAsync(CaptureProvider logs)
    {
        var builder = WebApplication.CreateBuilder(new WebApplicationOptions { EnvironmentName = "Production" });
        builder.WebHost.UseTestServer();
        builder.AddPlatformHttp();
        builder.Logging.AddFilter("Microsoft.AspNetCore", LogLevel.Warning);
        builder.Logging.AddProvider(logs);
        var app = builder.Build();
        app.UsePlatformHttp();
        app.MapGet("/items/{id}", () => "ok");
        app.MapGet("/failure", () => Task.FromException<string>(new InvalidOperationException("secret-exception-message")));
        app.MapGet("/weaken-headers", (HttpContext context) =>
        {
            context.Response.Headers.XFrameOptions = "SAMEORIGIN";
            context.Response.Headers.XContentTypeOptions = "unsafe";
            return "ok";
        });
        app.MapGet("/abort", (HttpContext context) =>
        {
            context.RequestAborted = new CancellationToken(true);
            throw new OperationCanceledException(context.RequestAborted);
        });
        await app.StartAsync();
        return app;
    }

    [Fact]
    public async Task SecurityHeaders_CoverSuccessErrorsAndPreventEndpointWeakening()
    {
        await using var app = await StartAsync(new CaptureProvider());
        using var client = app.GetTestClient();
        foreach (var (route, status) in new[] { ("/items/1", 200), ("/unknown", 404), ("/failure", 500), ("/weaken-headers", 200) })
        {
            using var response = await client.GetAsync(route);
            Assert.Equal(status, (int)response.StatusCode);
            Assert.Equal("nosniff", response.Headers.GetValues("X-Content-Type-Options").Single());
            Assert.Equal("DENY", response.Headers.GetValues("X-Frame-Options").Single());
            Assert.Equal("no-referrer", response.Headers.GetValues("Referrer-Policy").Single());
            Assert.Equal("default-src 'none'; frame-ancestors 'none'", response.Headers.GetValues("Content-Security-Policy").Single());
            Assert.False(response.Headers.Contains("Strict-Transport-Security"));
        }
    }

    [Fact]
    public async Task RequestIds_AreGeneratedAndCompletionExcludesSensitiveInputs()
    {
        var logs = new CaptureProvider();
        await using var app = await StartAsync(logs);
        using var client = app.GetTestClient();
        using var request = new HttpRequestMessage(HttpMethod.Get, "/items/private-account?key=secret-query");
        request.Headers.Add("X-Request-ID", "untrusted-request-id");
        request.Headers.Add("Authorization", "Bearer secret-credential");
        request.Headers.Add("traceparent", "00-0123456789abcdef0123456789abcdef-0123456789abcdef-01");
        using var response = await client.SendAsync(request);
        Assert.Equal(HttpStatusCode.OK, response.StatusCode);
        Assert.True(Guid.TryParseExact(response.Headers.GetValues("X-Request-ID").Single(), "N", out _));
        Assert.Equal("0123456789abcdef0123456789abcdef", response.Headers.GetValues("X-Trace-ID").Single());
        var completion = Assert.Single(logs.Events, item => item.Id == 1000);
        Assert.Contains("/items/{id}", completion.Message);
        Assert.True(completion.Scope.ContainsKey("RunId"));
        Assert.Equal("Cinis.Platform", completion.Scope["Service"]);
        var text = JsonSerializer.Serialize(logs.Events);
        foreach (var secret in new[] { "private-account", "secret-query", "secret-credential", "untrusted-request-id" })
            Assert.DoesNotContain(secret, text);
    }

    [Fact]
    public async Task Exception_IsSanitizedAndHasCorrelatedProblemAndSingleErrorEvent()
    {
        var logs = new CaptureProvider();
        await using var app = await StartAsync(logs);
        using var client = app.GetTestClient();
        using var response = await client.GetAsync("/failure");
        Assert.Equal(HttpStatusCode.InternalServerError, response.StatusCode);
        Assert.Equal("application/problem+json", response.Content.Headers.ContentType?.MediaType);
        var body = await response.Content.ReadAsStringAsync();
        using var problem = JsonDocument.Parse(body);
        Assert.Equal("internal_error", problem.RootElement.GetProperty("code").GetString());
        Assert.Equal(response.Headers.GetValues("X-Request-ID").Single(), problem.RootElement.GetProperty("requestId").GetString());
        Assert.Equal(response.Headers.GetValues("X-Trace-ID").Single(), problem.RootElement.GetProperty("traceId").GetString());
        Assert.DoesNotContain("secret-exception-message", body);
        Assert.DoesNotContain("secret-exception-message", JsonSerializer.Serialize(logs.Events));
        Assert.Single(logs.Events, item => item.Id == 1001);
        Assert.Contains("500", Assert.Single(logs.Events, item => item.Id == 1000).Message);
    }

    [Fact]
    public async Task UnknownRoute_ReturnsCorrelatedProblemWithoutEchoingPath()
    {
        await using var app = await StartAsync(new CaptureProvider());
        using var client = app.GetTestClient();
        using var response = await client.GetAsync("/private-unknown?token=secret");
        Assert.Equal(HttpStatusCode.NotFound, response.StatusCode);
        var body = await response.Content.ReadAsStringAsync();
        Assert.Contains(response.Headers.GetValues("X-Request-ID").Single(), body);
        Assert.DoesNotContain("private-unknown", body);
    }

    [Fact]
    public async Task ClientAbort_DoesNotBecomeInternalServerError()
    {
        var logs = new CaptureProvider();
        await using var app = await StartAsync(logs);
        using var client = app.GetTestClient();
        using var response = await client.GetAsync("/abort");
        Assert.Equal(499, (int)response.StatusCode);
        Assert.DoesNotContain(logs.Events, item => item.Id == 1001);
        Assert.Contains("499", Assert.Single(logs.Events, item => item.Id == 1000).Message);
    }

    private sealed record LogEvent(
        int Id,
        string Message,
        Dictionary<string, object?> Scope);

    private sealed class CaptureProvider : ILoggerProvider, ISupportExternalScope
    {
        private IExternalScopeProvider _scopes = new LoggerExternalScopeProvider();
        internal ConcurrentQueue<LogEvent> Events { get; } = new();
        public void SetScopeProvider(IExternalScopeProvider scopeProvider) => _scopes = scopeProvider;
        public ILogger CreateLogger(string categoryName) => new CaptureLogger(this);
        public void Dispose() { }

        private sealed class CaptureLogger(CaptureProvider owner) : ILogger
        {
            public bool IsEnabled(LogLevel logLevel) => true;
            public IDisposable? BeginScope<TState>(TState state) where TState : notnull => owner._scopes.Push(state);
            public void Log<TState>(
                LogLevel logLevel,
                EventId eventId,
                TState state,
                Exception? exception,
                Func<TState, Exception?, string> formatter)
            {
                var scope = CinisJsonConsoleFormatter.CaptureScope(owner._scopes);
                owner.Events.Enqueue(new(eventId.Id, formatter(state, exception), scope));
            }
        }
    }
}
