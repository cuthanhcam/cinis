namespace Cinis.Platform.Diagnostics;

/// <summary>Immutable diagnostic identity shared by all requests in one host process.</summary>
/// <param name="EnvironmentName">Hosting environment, not a dump of environment variables.</param>
/// <param name="Version">Build identity reported by the host assembly.</param>
/// <param name="ProcessId">Local process identifier; meaningful together with the run identity.</param>
/// <param name="RunId">Validated operations run ID, or a generated ID for direct launches.</param>
public sealed record ServiceIdentity(
    string EnvironmentName,
    string Version,
    int ProcessId,
    string RunId);
