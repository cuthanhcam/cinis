# C# coding conventions

Status: Accepted repository style, 2026-10-20. Applies to production and test C# under src/platform.

## Declaration layout

Choose layout by readability and a target of 120 characters per C# line, including indentation. A short declaration with one to three parameters may stay on one line. Prefer multiline for four or more parameters, long or nested generic types, parameter attributes, or declarations that exceed the target. This is a team convention, not a C# language or universal .NET requirement.

When wrapping, place the opening parenthesis after the declaration name, start each parameter on its own line, indent one level (four spaces), and keep the closing parenthesis after the last parameter. Apply the same reasoning to positional records, primary constructors, ordinary constructors, methods, interface methods, delegates and local functions. Preserve parameter attributes, default values, nullability and XML comments. Keep readable existing multiline declarations; do not collapse or expand signatures solely to satisfy a parameter count.

```csharp
public sealed record CancelOrderIntent(
    ulong AccountId,
    ulong CommandId,
    ulong OrderId);

public ValueTask<bool> HasGrantAsync(AccountGrant grant, CancellationToken cancellationToken);

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

The tracked VS Code settings show a 120-column ruler for C# only. It is a visual aid, not a hard wrap or a change to other language formatters.

Run dotnet format src/platform/Cinis.Platform.slnx whitespace --no-restore --verify-no-changes and the [platform verification task](platform.md) before integration. max_line_length=120 in .editorconfig communicates the target to supporting editors/tools; the pinned SDK whitespace formatter does not automatically wrap C# parameter lists to that width or reject every long line. CI verifies supported SDK formatting, while review covers readability and wrapping. Long URLs, diagnostic templates and other literals may justify exceptions; do not change runtime strings to satisfy a width hint. An analyzer or dedicated formatter can be evaluated later if drift justifies its maintenance/dependency cost.

References: [Microsoft C# coding conventions](https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/coding-style/coding-conventions) and [supported C# formatting options](https://learn.microsoft.com/en-us/dotnet/fundamentals/code-analysis/style-rules/csharp-formatting-options). Neither defines the team's 120-character/four-parameter policy as a universal standard.

Keep layout-only changes separate from behavioral changes. Existing tests verify behavior; do not add tests that merely restate whitespace. Rust, Python and PowerShell retain their language conventions from the [naming standard](naming-conventions.md).
