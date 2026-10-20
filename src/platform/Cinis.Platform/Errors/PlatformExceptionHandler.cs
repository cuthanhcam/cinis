using Microsoft.AspNetCore.Diagnostics;

namespace Cinis.Platform.Errors;

/// <summary>Converts unexpected HTTP failures to a stable, non-sensitive problem response.</summary>
/// <param name="logger">Diagnostic sink; exception messages may contain secrets and are excluded.</param>
public sealed class PlatformExceptionHandler(ILogger<PlatformExceptionHandler> logger) : IExceptionHandler
{
    /// <inheritdoc />
    public async ValueTask<bool> TryHandleAsync(HttpContext httpContext, Exception exception,
        CancellationToken cancellationToken)
    {
        // .NET 10 suppresses framework diagnostics for handled exceptions. Emit one explicit event here.
        // Stack frames aid local debugging; messages, Data and nested exception text are deliberately excluded.
        logger.LogError(new EventId(1001, "RequestFailed"),
            "Unhandled HTTP failure {ExceptionType}; stack {StackTrace}", exception.GetType().FullName, exception.StackTrace);
        await Results.Problem(statusCode: 500, title: "An unexpected error occurred.",
            extensions: new Dictionary<string, object?> { ["code"] = "internal_error" }).ExecuteAsync(httpContext);
        return true;
    }
}
