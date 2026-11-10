#Requires -Version 5.1
<#
.SYNOPSIS
  Sovereign Craft Suite Showcase launcher: serves interactive landing page & WASM studio on port 11270.
#>
param(
    [switch]$Smoke,
    [switch]$Build,
    [int]$Port = 11270
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

if ($Smoke -or ($args -contains "--smoke")) {
    $wasmHtml = Join-Path $RepoRoot "web\wasm\index.html"
    $landingHtml = Join-Path $RepoRoot "web\index.html"
    if ((Test-Path $wasmHtml) -and (Test-Path $landingHtml)) {
        Write-Host "smoke: ok (craft-showcase web & wasm)" -ForegroundColor Green
        exit 0
    } else {
        Write-Host "smoke: failed - missing web assets" -ForegroundColor Red
        exit 1
    }
}

if ($Build) {
    Set-Location (Join-Path $RepoRoot "crates\craft-showcase-wasm")
    & "$env:USERPROFILE\.cargo\bin\trunk.exe" build --release
    Copy-Item -Path "dist\*" -Destination (Join-Path $RepoRoot "web\wasm") -Recurse -Force
    Write-Host "Build complete: WASM copied to web\wasm" -ForegroundColor Green
    exit 0
}

# Start local preview web server
$webDir = Join-Path $RepoRoot "web"
Write-Host "Starting Sovereign Craft Suite Showcase at http://localhost:$Port ..." -ForegroundColor Cyan
Start-Process "http://localhost:$Port"
python -m http.server $Port --directory $webDir
