#Requires -Version 5.1
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function ConvertTo-CinisArgument {
    param([AllowEmptyString()][string]$Value)
    # Windows CommandLineToArgvW-compatible quoting, including trailing backslashes.
    $escaped = [regex]::Replace($Value, '(\\*)"', '$1$1\"')
    $escaped = [regex]::Replace($escaped, '(\\+)$', '$1$1')
    return '"' + $escaped + '"'
}

function Stop-CinisOwnedProcess {
    param([System.Diagnostics.Process]$Process)
    $Process.Refresh()
    if (-not $Process.HasExited) {
        # Only the process object started by this invocation and its descendants.
        $killer = Join-Path $env:SystemRoot 'System32/taskkill.exe'
        & $killer /PID $Process.Id /T /F 2>&1 | Out-Null
        if (-not $Process.WaitForExit(5000)) { throw 'Unable to stop owned process tree within five seconds.' }
    }
}

function Invoke-CinisLoggedProcess {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)][string]$Tool,
        [Parameter(Mandatory)][AllowEmptyString()][string[]]$Arguments,
        [Parameter(Mandatory)][string]$WorkingDirectory,
        [Parameter(Mandatory)][string]$LogDirectory,
        [Parameter(Mandatory)][string]$StepName,
        [ValidateRange(1, 86400)][int]$TimeoutSeconds = 600
    )
    if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT) {
        throw 'This operations runner currently supports Windows PowerShell 5.1+ on Windows.'
    }
    if ($StepName -notmatch '^[a-zA-Z0-9-]+$') { throw 'Invalid step name.' }
    $stdout = Join-Path $LogDirectory ($StepName + '.stdout.log')
    $stderr = Join-Path $LogDirectory ($StepName + '.stderr.log')
    $stepFile = Join-Path $LogDirectory ($StepName + '.json')
    $record = [ordered]@{
        step = $StepName; tool = $Tool; status = 'Starting'; processId = $null
        startedUtc = [DateTime]::UtcNow.ToString('o'); finishedUtc = $null
        exitCode = $null; failureType = $null; stdout = [IO.Path]::GetFileName($stdout); stderr = [IO.Path]::GetFileName($stderr)
    }
    $process = $null
    try {
        $application = Get-Command $Tool -CommandType Application -ErrorAction Stop | Select-Object -First 1
        $parameters = @{
            FilePath = $application.Source
            ArgumentList = (($Arguments | ForEach-Object { ConvertTo-CinisArgument $_ }) -join ' ')
            WorkingDirectory = $WorkingDirectory
            RedirectStandardOutput = $stdout
            RedirectStandardError = $stderr
            WindowStyle = 'Hidden'
            PassThru = $true
        }
        $process = Start-Process @parameters
        # Cache the handle before a fast child exits (Windows PowerShell 5.1).
        $null = $process.Handle
        $record.processId = $process.Id
        $record.status = 'Running'
        $record | ConvertTo-Json | Set-Content -LiteralPath $stepFile -Encoding UTF8
        Write-Host "Started $StepName (PID $($process.Id)); logs: $LogDirectory"
        $timer = [Diagnostics.Stopwatch]::StartNew()
        while (-not $process.WaitForExit(250)) {
            if ($timer.Elapsed.TotalSeconds -ge $TimeoutSeconds) {
                $record.status = 'TimedOut'
                Stop-CinisOwnedProcess $process
                $record.exitCode = 124
                break
            }
        }
        $process.WaitForExit()
        if ($record.status -ne 'TimedOut') {
            if ($null -eq $process.ExitCode) { throw 'Process exit code unavailable.' }
            $record.exitCode = $process.ExitCode
            $record.status = if ($process.ExitCode -eq 0) { 'Succeeded' } else { 'Failed' }
        }
    }
    catch {
        $record.status = 'Failed'
        $record.failureType = $_.Exception.GetType().FullName
        $record.exitCode = 1
        # No environment or command arguments are serialized into metadata.
        Write-Host "Unable to complete $StepName. Check the executable/toolchain and log paths."
    }
    finally {
        if ($null -ne $process) {
            $process.Refresh()
            if (-not $process.HasExited) {
                $record.status = 'Interrupted'
                $record.exitCode = 130
                Stop-CinisOwnedProcess $process
            }
            $process.Dispose()
        }
        $record.finishedUtc = [DateTime]::UtcNow.ToString('o')
        $record | ConvertTo-Json | Set-Content -LiteralPath $stepFile -Encoding UTF8
    }
    foreach ($file in @($stdout, $stderr)) {
        if (Test-Path -LiteralPath $file) {
            Write-Host "--- $([IO.Path]::GetFileName($file)) (last 30 lines) ---"
            Get-Content -LiteralPath $file -Tail 30 | ForEach-Object { Write-Host $_ }
        }
    }
    return [pscustomobject]$record
}
Export-ModuleMember -Function Invoke-CinisLoggedProcess
