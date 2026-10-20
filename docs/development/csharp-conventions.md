# C# coding conventions

Status: Accepted repository style, 2026-10-20. Applies to production and test C# under src/platform.

## Declaration layout

For declarations with two or more parameters, place the opening parenthesis after the declaration name, start each parameter on its own line, indent one level (four spaces), and keep the closing parenthesis after the last parameter. Apply this consistently to positional records, primary constructors, ordinary constructors, methods, interface methods, delegates and local functions. A zero/one-parameter declaration can stay on one line. Preserve parameter attributes, default values, nullability and XML comments.

```csharp
public sealed record CancelOrderIntent(
    ulong AccountId,
    ulong CommandId,
    ulong OrderId);

public Task<FrameExchangeResult> ExchangeAsync(
    ReadOnlyMemory<byte> request,
    CancellationToken cancellationToken = default)
{
    // Implementation follows the documented ownership and cancellation contract.
}
```

The method above illustrates layout rather than a compilable implementation. Short call sites and lambdas may remain compact; declaration layout does not require every invocation or lambda to expand. Prioritize readable handler bodies and avoid wrapping unrelated statements during a signature edit.

The maintainer's generic-result example uses the same layout:

```csharp
public sealed record CinisOperationResult<T>(
    bool IsSuccess,
    T? Data,
    string? ErrorCode,
    string? Message,
    IReadOnlyCollection<CinisError> Errors);
```

This is a proposed contract illustration, not an implemented shared type. The [foundation review](../architecture/platform-foundation-review.md) explains its adoption gate. Formatting approval does not accept a new business-result model.

## Tooling and review

Use file-scoped namespaces, nullable annotations, four-space C# indentation and the root .editorconfig. Keep compiler-enforced production XML documentation as described in [source documentation](platform-transport.md). Comments explain purpose, units, ownership and failure semantics; do not repeat assignments or use regions to hide large classes.

Run dotnet format src/platform/Cinis.Platform.slnx whitespace --no-restore --verify-no-changes and the [platform verification task](platform.md) before integration. The SDK whitespace formatter preserves the chosen signature layout but does not enforce one-parameter-per-line as a standalone rule. Review new declarations against this guide; do not claim .editorconfig or CI automatically detects every wrapping violation. An analyzer can be introduced later if drift justifies its maintenance/dependency cost.

Keep layout-only changes separate from behavioral changes. Existing tests verify behavior; do not add tests that merely restate whitespace. Rust, Python and PowerShell retain their language conventions from the [naming standard](naming-conventions.md).
