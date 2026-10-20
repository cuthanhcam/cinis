namespace Cinis.Platform.Filters;

/// <summary>Prevents caches from serving stale operational endpoint responses.</summary>
public sealed class NoStoreEndpointFilter : IEndpointFilter
{
    /// <inheritdoc />
    public ValueTask<object?> InvokeAsync(
        EndpointFilterInvocationContext context,
        EndpointFilterDelegate next)
    {
        context.HttpContext.Response.Headers.CacheControl = "no-store";
        return next(context);
    }
}
