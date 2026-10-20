#Requires -Version 5.1
[CmdletBinding()]
param(
    [ValidatePattern('^\d{8}T\d{9}Z-[a-f0-9]{8}$')][string]$RunId,
    [switch]$Latest,
    [ValidateSet('Engine', 'Platform')][string]$Project = 'Platform',
    [ValidateSet('Build', 'Test', 'Run', 'Serve', 'RunMatching', 'Format', 'Lint', 'Docs', 'Workflows', 'Verify')][string]$Task = 'Serve',
    [ValidateRange(1, 10000)][int]$Tail = 50,
    [ValidateSet('All', 'stdout', 'stderr')][string]$Stream = 'All'
)
$ErrorActionPreference = 'Stop'
# Load this host's built-ins explicitly: inherited PS7 module paths can shadow WinPS 5.1 modules.
foreach ($moduleName in @('Microsoft.PowerShell.Utility', 'Microsoft.PowerShell.Management')) {
    Import-Module ([IO.Path]::Combine($PSHOME, 'Modules', $moduleName, ($moduleName + '.psd1'))) -ErrorAction Stop
}

$repositoryRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
$logRoot = Join-Path $repositoryRoot 'artifacts/logs'
if ($Latest -and $RunId) { throw 'Choose Latest or RunId, not both.' }
if ($Latest) { $directory = Join-Path $logRoot ('latest/' + $Project.ToLowerInvariant() + '/' + $Task.ToLowerInvariant()) }
elseif ($RunId) { $directory = Join-Path $logRoot $RunId }
else {
    $latest = Get-ChildItem -LiteralPath $logRoot -Filter run.json -File -Recurse |
        Sort-Object LastWriteTimeUtc -Descending | Select-Object -First 1
    if ($null -eq $latest) { throw 'No operations runs found.' }
    $directory = $latest.DirectoryName
}
Get-Content -LiteralPath (Join-Path $directory 'run.json')
$filter = if ($Stream -eq 'All') { '*.log' } else { '*.' + $Stream + '.log' }
Get-ChildItem -LiteralPath $directory -Filter $filter -File | Sort-Object Name | ForEach-Object {
    Write-Host "--- $($_.Name) ---"
    Get-Content -LiteralPath $_.FullName -Tail $Tail
}
