namespace Cinis.Platform.Middleware;

/// <summary>Applies the browser response-header policy for the current JSON-only API host.</summary>
/// <param name="next">Remaining HTTP pipeline.</param>
/// <remarks>
/// Headers are set just before transmission, including translated errors. This policy does not implement
/// authentication, TLS or proxy trust. Future HTML/documentation routes require their own reviewed CSP policy.
/// </remarks>
public sealed class SecurityHeadersMiddleware(RequestDelegate next)
{
    /// <summary>Registers response protection before invoking downstream components.</summary>
    /// <param name="context">Current HTTP request and response.</param>
    public Task InvokeAsync(HttpContext context)
    {
        // Register first so this callback runs last: endpoint code cannot accidentally weaken the host policy.
        context.Response.OnStarting(() =>
        {
            var headers = context.Response.Headers;
            headers.XContentTypeOptions = "nosniff";
            headers.XFrameOptions = "DENY";
            headers["Referrer-Policy"] = "no-referrer";
            headers.ContentSecurityPolicy = "default-src 'none'; frame-ancestors 'none'";
            return Task.CompletedTask;
        });
        return next(context);
    }
}
