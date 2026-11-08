using System.Diagnostics;
using Cinis.ExecutionClient.Protocol;

namespace Cinis.ProtocolProbe;

// Owns only the exact child it starts. An uncertain/partial pipe exchange is fatal to this research session;
// cleanup never kills processes by name and no timeout is converted to a known business rejection.
internal sealed class OwnedPeer : IAsyncDisposable
{
    internal static readonly BoundedFrameCodec Codec = new(128);
    private readonly Process _process;
    private readonly Task<string> _stderr;
    private readonly CancellationTokenSource _deadline;
    private Task<byte[]>? _pendingRead;

    internal OwnedPeer(string executable, bool dropFirst = false, TimeSpan? budget = null)
    {
        _deadline = new CancellationTokenSource(budget ?? TimeSpan.FromSeconds(30));
        var start = new ProcessStartInfo(executable)
        {
            UseShellExecute = false,
            RedirectStandardInput = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true
        };
        start.ArgumentList.Add("--stdio-research");
        if (dropFirst) start.ArgumentList.Add("--drop-first-reply");
        try { _process = Process.Start(start) ?? throw new InvalidOperationException("Cannot start research peer."); }
        catch { _deadline.Dispose(); throw; }
        // The owned peer emits only a bounded static error message, never arbitrary peer input.
        _stderr = _process.StandardError.ReadToEndAsync();
    }

    internal async Task SendAsync(byte[] body, bool fragmented = false)
    {
        if (fragmented)
        {
            using var encoded = new MemoryStream();
            await Codec.WriteAsync(encoded, body, _deadline.Token);
            foreach (var value in encoded.ToArray())
                await _process.StandardInput.BaseStream.WriteAsync(new byte[] { value }, _deadline.Token);
        }
        else await Codec.WriteAsync(_process.StandardInput.BaseStream, body, _deadline.Token);
        await _process.StandardInput.BaseStream.FlushAsync(_deadline.Token);
    }

    internal async Task<byte[]> ReadAsync()
    {
        // Some redirected pipe implementations cannot interrupt an already pending native read. Bound the await
        // independently, then terminate/close the session during disposal; never reuse a timed-out pipe.
        _pendingRead = Codec.ReadAsync(_process.StandardOutput.BaseStream, _deadline.Token);
        return await _pendingRead.WaitAsync(_deadline.Token);
    }

    internal async Task<FundingReply> ExchangeAsync(FundingRequest request)
    {
        await SendAsync(request.Encode());
        return FundingReply.Decode(await ReadAsync(), request);
    }

    internal async Task InvalidFrameAsync(byte[] frame)
    {
        await _process.StandardInput.BaseStream.WriteAsync(frame, _deadline.Token);
        await FinishAsync(2);
        var count = await _process.StandardOutput.BaseStream.ReadAsync(new byte[1], _deadline.Token);
        if (count != 0) throw new InvalidDataException("Invalid framing unexpectedly produced a response.");
    }

    internal async Task FinishAsync(int expectedExit = 0)
    {
        _process.StandardInput.Close();
        await _process.WaitForExitAsync(_deadline.Token);
        var diagnostic = await _stderr;
        if (_process.ExitCode != expectedExit || (expectedExit == 0 && diagnostic.Length != 0))
            throw new InvalidOperationException("Unexpected research peer exit/diagnostics.");
    }

    public async ValueTask DisposeAsync()
    {
        try
        {
            if (!_process.HasExited)
            {
                _process.StandardInput.Close();
                try { await _process.WaitForExitAsync().WaitAsync(TimeSpan.FromSeconds(3)); }
                catch (TimeoutException)
                {
                    _process.Kill(entireProcessTree: true);
                    await _process.WaitForExitAsync();
                }
            }
            await _stderr;
            if (_pendingRead is not null)
            {
                try { await _pendingRead.WaitAsync(TimeSpan.FromSeconds(3)); }
                catch (OperationCanceledException) { }
                catch (IOException) { }
            }
        }
        finally { _process.Dispose(); _deadline.Dispose(); }
    }
}
