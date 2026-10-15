#Requires -Version 5.1
[CmdletBinding()]
param(
    [ValidateSet('Engine')][string]$Project = 'Engine',
    [ValidateSet('Build', 'Test', 'Run', 'Format', 'Lint', 'Docs', 'Verify')][string]$Task = 'Verify',
    [ValidateRange(1, 86400)][int]$TimeoutSeconds = 600
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repositoryRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
Import-Module (Join-Path $PSScriptRoot 'Cinis.Operations.psm1') -Force
$projects = Import-PowerShellDataFile (Join-Path $repositoryRoot 'ops/config/projects.psd1')
$definition = $projects[$Project]
$runId = [DateTime]::UtcNow.ToString('yyyyMMddTHHmmssfffZ') + '-' + [Guid]::NewGuid().ToString('N').Substring(0, 8)
$directory = Join-Path $repositoryRoot ('artifacts/logs/' + $runId)
[void](New-Item -ItemType Directory -Path $directory -Force)
$revision = $null
if ((Test-Path -LiteralPath (Join-Path $repositoryRoot '.git')) -and (Get-Command git -CommandType Application -ErrorAction SilentlyContinue)) {
    $revision = & git -C $repositoryRoot rev-parse HEAD
    if ($LASTEXITCODE -ne 0) { $revision = $null }
}
$summary = [ordered]@{
    schemaVersion = 1; revision = $revision; powerShellVersion = $PSVersionTable.PSVersion.ToString()
    timeoutSecondsPerStep = $TimeoutSeconds; runId = $runId; project = $Project; task = $Task; kind = $definition.Kind
    startedUtc = [DateTime]::UtcNow.ToString('o'); finishedUtc = $null
    status = 'Running'; exitCode = $null; steps = @()
}
$summaryFile = Join-Path $directory 'run.json'
$summary | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $summaryFile -Encoding UTF8
$exitCode = 1
try {
    $index = 0
    foreach ($step in $definition.Tasks[$Task]) {
        $index++
        $toolName = [IO.Path]::GetFileNameWithoutExtension($step.Tool) -replace '[^a-zA-Z0-9-]', '-'
        $name = '{0:d2}-{1}' -f $index, $toolName
        $result = Invoke-CinisLoggedProcess -Tool $step.Tool -Arguments $step.Arguments -WorkingDirectory $repositoryRoot -LogDirectory $directory -StepName $name -TimeoutSeconds $TimeoutSeconds
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
    $summary | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $summaryFile -Encoding UTF8
    Write-Host "Run $runId : $($summary.status). Summary: $summaryFile"
}
exit $exitCode
