#Requires -Version 5.1
[CmdletBinding()]
param(
    [ValidateSet('Engine', 'Platform')][string]$Project = 'Engine',
    [ValidateSet('Build', 'Test', 'Run', 'Serve', 'RunMatching', 'Format', 'Lint', 'Docs', 'Workflows', 'Verify')][string]$Task = 'Verify',
    [ValidateRange(1, 86400)][int]$TimeoutSeconds = 600,
    [ValidateSet('Auto', 'Archive', 'Latest')][string]$LogMode = 'Auto'
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
# Load this host's built-ins explicitly: inherited PS7 module paths can shadow WinPS 5.1 modules.
foreach ($moduleName in @('Microsoft.PowerShell.Utility', 'Microsoft.PowerShell.Management')) {
    Import-Module ([IO.Path]::Combine($PSHOME, 'Modules', $moduleName, ($moduleName + '.psd1'))) -ErrorAction Stop
}

$repositoryRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
Import-Module (Join-Path $PSScriptRoot 'Cinis.Operations.psm1') -Force
$projects = Import-PowerShellDataFile (Join-Path $repositoryRoot 'ops/config/projects.psd1')
$definition = $projects[$Project]
$runId = [DateTime]::UtcNow.ToString('yyyyMMddTHHmmssfffZ') + '-' + [Guid]::NewGuid().ToString('N').Substring(0, 8)
$effectiveMode = if ($LogMode -eq 'Auto') { if ($Task -eq 'Serve') { 'Latest' } else { 'Archive' } } else { $LogMode }
$logRoot = Join-Path $repositoryRoot 'artifacts/logs'
$directory = if ($effectiveMode -eq 'Latest') {
    Join-Path $logRoot ('latest/' + $Project.ToLowerInvariant() + '/' + $Task.ToLowerInvariant())
} else { Join-Path $logRoot $runId }
# Keep the lock outside the output directory. A second writer must fail BEFORE truncating any previous logs.
$lock = $null
if ($effectiveMode -eq 'Latest') {
    $lockRoot = Join-Path $logRoot '.locks'
    [void](New-Item -ItemType Directory -Path $lockRoot -Force)
    try {
        $lock = [IO.File]::Open((Join-Path $lockRoot ($Project + '-' + $Task + '.lock')),
            [IO.FileMode]::OpenOrCreate, [IO.FileAccess]::ReadWrite, [IO.FileShare]::None)
    }
    catch { Write-Error 'Another runner owns this latest project/task log directory.'; exit 1 }
}
try {
    [void](New-Item -ItemType Directory -Path $directory -Force)
    if ($effectiveMode -eq 'Latest') {
        # Remove only known runner outputs, never recursively delete a directory or arbitrary caller path.
        Get-ChildItem -LiteralPath $directory -File | Where-Object {
            $_.Name -eq 'run.json' -or $_.Name -match '^\d{2}-[a-zA-Z0-9-]+\.(json|stdout\.log|stderr\.log)$'
        } | ForEach-Object { Remove-Item -LiteralPath $_.FullName -Force }
    }
}
catch { if ($null -ne $lock) { $lock.Dispose() }; throw }
$revision = $null
if ((Test-Path -LiteralPath (Join-Path $repositoryRoot '.git')) -and (Get-Command git -CommandType Application -ErrorAction SilentlyContinue)) {
    $revision = & git -C $repositoryRoot rev-parse HEAD
    if ($LASTEXITCODE -ne 0) { $revision = $null }
}
$summary = [ordered]@{
    schemaVersion = 2; revision = $revision; powerShellVersion = $PSVersionTable.PSVersion.ToString(); logMode = $effectiveMode
    timeoutSecondsPerStep = $TimeoutSeconds; runId = $runId; project = $Project; task = $Task; kind = $definition.Kind
    startedUtc = [DateTime]::UtcNow.ToString('o'); finishedUtc = $null
    status = 'Running'; exitCode = $null; steps = @()
}
$summaryFile = Join-Path $directory 'run.json'
$summary | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $summaryFile -Encoding UTF8
$exitCode = 1
try {
    if (-not $definition.Tasks.ContainsKey($Task)) { throw "Task is not supported by this project." }
    $index = 0
    foreach ($step in $definition.Tasks[$Task]) {
        $index++
        $toolName = [IO.Path]::GetFileNameWithoutExtension($step.Tool) -replace '[^a-zA-Z0-9-]', '-'
        $name = '{0:d2}-{1}' -f $index, $toolName
        $result = Invoke-CinisLoggedProcess -Tool $step.Tool -Arguments $step.Arguments -WorkingDirectory $repositoryRoot -LogDirectory $directory -StepName $name -TimeoutSeconds $TimeoutSeconds -RunId $runId
        $summary.steps += $result
        $summary | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $summaryFile -Encoding UTF8
        $exitCode = [int]$result.exitCode
        if ($exitCode -ne 0) { break }
    }
    $summary.status = if ($exitCode -eq 0) { 'Succeeded' } else { 'Failed' }
}
catch {
    $summary.status = 'Failed'
    $exitCode = 1
    Write-Host 'Runner failed. Inspect per-step metadata and tool availability.'
}
finally {
    if ($summary.status -eq 'Running') { $summary.status = 'Interrupted'; $exitCode = 130 }
    $summary.exitCode = $exitCode
    $summary.finishedUtc = [DateTime]::UtcNow.ToString('o')
    try {
        $summary | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $summaryFile -Encoding UTF8
        Write-Host "Run $runId : $($summary.status). Summary: $summaryFile"
    }
    finally { if ($null -ne $lock) { $lock.Dispose() } }
}
exit $exitCode
