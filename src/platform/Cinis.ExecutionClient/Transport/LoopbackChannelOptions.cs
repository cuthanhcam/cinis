using System.Net;

namespace Cinis.ExecutionClient.Transport;

/// <summary>Immutable endpoint, allocation and time budgets for the experimental channel.</summary>
/// <remarks>Only numeric loopback addresses are accepted. Loopback restriction is not service authentication.</remarks>
public sealed class LoopbackChannelOptions
{
    private readonly string _address;

    /// <summary>Creates a validated channel configuration.</summary>
    /// <param name="address">Numeric IPv4 or IPv6 loopback address; DNS names are not resolved.</param>
    /// <param name="port">Listening peer port, from 1 through 65535.</param>
    /// <param name="maxPayloadBytes">Maximum request/reply body, from 1 byte through 1 MiB.</param>
    /// <param name="connectTimeout">Positive connection budget, representable in signed millisecond timers.</param>
    /// <param name="exchangeTimeout">Positive combined write/read budget, independent of connection time.</param>
    /// <exception cref="ArgumentException">The address is absent or not loopback.</exception>
    /// <exception cref="ArgumentOutOfRangeException">A port, size or time budget is invalid.</exception>
    public LoopbackChannelOptions(
        IPAddress address,
        int port,
        int maxPayloadBytes,
        TimeSpan connectTimeout,
        TimeSpan exchangeTimeout)
    {
        ArgumentNullException.ThrowIfNull(address);
        // Snapshot before validation so another owner cannot change a mutable address between checking and storing it.
        var snapshot = IPAddress.Parse(address.ToString());
        if (!IPAddress.IsLoopback(snapshot)) throw new ArgumentException("Only loopback peers are supported.", nameof(address));
        if (port is < 1 or > 65535) throw new ArgumentOutOfRangeException(nameof(port));
        if (maxPayloadBytes is < 1 or > 1_048_576) throw new ArgumentOutOfRangeException(nameof(maxPayloadBytes));
        ValidateTimeout(connectTimeout, nameof(connectTimeout));
        ValidateTimeout(exchangeTimeout, nameof(exchangeTimeout));
        // IPAddress can carry mutable scope information. Store its value and return copies.
        _address = snapshot.ToString();
        Port = port;
        MaxPayloadBytes = maxPayloadBytes;
        ConnectTimeout = connectTimeout;
        ExchangeTimeout = exchangeTimeout;
    }

    /// <summary>Gets a copy of the configured numeric loopback address.</summary>
    public IPAddress Address => IPAddress.Parse(_address);
    /// <summary>Gets the peer's listening TCP port.</summary>
    public int Port { get; }
    /// <summary>Gets the body allocation bound in bytes; the four-byte header is separate.</summary>
    public int MaxPayloadBytes { get; }
    /// <summary>Gets the connection-attempt budget.</summary>
    public TimeSpan ConnectTimeout { get; }
    /// <summary>Gets the combined request-write and response-read budget.</summary>
    public TimeSpan ExchangeTimeout { get; }

    private static void ValidateTimeout(
        TimeSpan timeout,
        string parameter)
    {
        if (timeout.TotalMilliseconds is < 1 or > int.MaxValue)
            throw new ArgumentOutOfRangeException(parameter, "Use a finite budget of at least one millisecond.");
    }
}
