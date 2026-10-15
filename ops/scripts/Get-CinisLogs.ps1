#Requires -Version 5.1
[CmdletBinding()]
param(
    [ValidatePattern('^\d{8}T\d{9}Z-[a-f0-9]{8}$')][string]$RunId,
    [ValidateRange(1, 10000)][int]$Tail = 50,
    [ValidateSet('All', 'stdout', 'stderr')][string]$Stream = 'All'
)
$ErrorActionPreference = 'Stop'
$repositoryRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
$logRoot = Join-Path $repositoryRoot 'artifacts/logs'
if ($RunId) { $directory = Join-Path $logRoot $RunId }
else {
    $latest = Get-ChildItem -LiteralPath $logRoot -Directory | Sort-Object Name -Descending | Select-Object -First 1
    if ($null -eq $latest) { throw 'No operations runs found.' }
    $directory = $latest.FullName
}
Get-Content -LiteralPath (Join-Path $directory 'run.json')
$filter = if ($Stream -eq 'All') { '*.log' } else { '*.' + $Stream + '.log' }
Get-ChildItem -LiteralPath $directory -Filter $filter -File | Sort-Object Name | ForEach-Object {
    Write-Host "--- $($_.Name) ---"
    Get-Content -LiteralPath $_.FullName -Tail $Tail
}
