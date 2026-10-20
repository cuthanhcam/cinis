using System.Globalization;
using System.Text.Json;
using Microsoft.Extensions.Logging.Abstractions;
using Microsoft.Extensions.Logging.Console;

namespace Cinis.Platform.Diagnostics;

/// <summary>Writes one UTC JSON record per line with an explicit, safe correlation-scope allowlist.</summary>
/// <param name="identity">Host identity attached even to startup/shutdown events outside HTTP scopes.</param>
/// <remarks>Raw exception objects and framework request-path scopes are excluded from this console sink.</remarks>
public sealed class CinisJsonConsoleFormatter(ServiceIdentity identity) : ConsoleFormatter("cinis-json")
{
    private static readonly HashSet<string> ScopeKeys =
        ["Service", "Environment", "Version", "ProcessId", "RunId", "RequestId", "TraceId", "SpanId", "ParentId"];

    /// <inheritdoc />
    public override void Write<TState>(
        in LogEntry<TState> logEntry,
        IExternalScopeProvider? scopeProvider,
        TextWriter textWriter)
    {
        var fields = new Dictionary<string, object?>();
        if (logEntry.State is IEnumerable<KeyValuePair<string, object?>> values)
            foreach (var pair in values)
                if (pair.Key != "{OriginalFormat}") fields[pair.Key] = pair.Value;
        var record = new
        {
            Service = "Cinis.Platform",
            Environment = identity.EnvironmentName,
            identity.Version,
            identity.ProcessId,
            identity.RunId,
            Timestamp = DateTimeOffset.UtcNow.ToString("O", CultureInfo.InvariantCulture),
            LogLevel = logEntry.LogLevel.ToString(),
            Category = logEntry.Category,
            EventId = logEntry.EventId.Id,
            EventName = logEntry.EventId.Name,
            Message = logEntry.Formatter(logEntry.State, logEntry.Exception),
            Fields = fields,
            Scope = CaptureScope(scopeProvider)
        };
        textWriter.WriteLine(JsonSerializer.Serialize(record));
    }

    internal static Dictionary<string, object?> CaptureScope(IExternalScopeProvider? provider)
    {
        var scope = new Dictionary<string, object?>();
        provider?.ForEachScope((item, result) =>
        {
            if (item is IEnumerable<KeyValuePair<string, object?>> values)
                foreach (var pair in values)
                    if (ScopeKeys.Contains(pair.Key)) result[pair.Key] = pair.Value;
        }, scope);
        return scope;
    }
}
