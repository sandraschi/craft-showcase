#Requires -Version 5.1
<#
.SYNOPSIS
    Install and start the craft-showcase NSSM Windows service (requires Administrator).
#>
param(
    [string]$ServiceName = 'craft-showcase',
    [int]$Port = 11270
)

$ErrorActionPreference = 'Stop'
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Error "Run this script from an elevated (Administrator) PowerShell prompt."
    exit 1
}

$nssm = (Get-Command nssm -ErrorAction Stop).Source
$repoRoot = Split-Path -Parent $PSScriptRoot
$servePy = Join-Path $repoRoot 'scripts\serve.py'
$webDir = Join-Path $repoRoot 'web'
$logDir = Join-Path $repoRoot 'logs'
New-Item -ItemType Directory -Force -Path $logDir | Out-Null

$pythonExe = 'C:\Users\sandr\AppData\Roaming\uv\python\cpython-3.13.5-windows-x86_64-none\python.exe'
if (-not (Test-Path -LiteralPath $pythonExe)) {
    $pythonExe = (Get-Command python -ErrorAction Stop).Source
}

$existing = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if ($existing) {
    & sc.exe stop $ServiceName | Out-Null
    Start-Sleep -Seconds 1
    & $nssm remove $ServiceName confirm
}

& $nssm install $ServiceName $pythonExe "-u `"$servePy`" --port $Port --directory `"$webDir`""
& $nssm set $ServiceName AppDirectory $repoRoot
& $nssm set $ServiceName DisplayName "Craft Showcase Web Server ($Port)"
& $nssm set $ServiceName Start SERVICE_AUTO_START
& $nssm set $ServiceName AppStdout (Join-Path $logDir 'nssm-stdout.log')
& $nssm set $ServiceName AppStderr (Join-Path $logDir 'nssm-stderr.log')
& $nssm set $ServiceName AppRotateFiles 1
& $nssm set $ServiceName AppRotateBytes 1048576

& sc.exe start $ServiceName
Start-Sleep -Seconds 1
Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue | Select-Object LocalAddress, LocalPort, OwningProcess
