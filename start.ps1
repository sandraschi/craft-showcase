#Requires -Version 5.1
<#
.SYNOPSIS
  Local Craft Suite Showcase launcher: serves interactive landing page & WASM studio on port 11270.
#>
param(
    [switch]$Smoke,
    [switch]$Build,
    [switch]$Sync,
    [int]$Port = 11270
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

if ($Sync) {
    & (Join-Path $RepoRoot "scripts\sync-diy-catalog.ps1")
    exit 0
}

if ($Smoke -or ($args -contains "--smoke")) {
    $wasmHtml = Join-Path $RepoRoot "web\wasm\index.html"
    $landingHtml = Join-Path $RepoRoot "web\index.html"
    $diyHtml = Join-Path $RepoRoot "web\diy.html"
    $localHtml = Join-Path $RepoRoot "web\local.html"
    $appHtml = Join-Path $RepoRoot "web\app.html"
    $diyData = Join-Path $RepoRoot "web\data\diy-apps.json"
    $allData = Join-Path $RepoRoot "web\data\all-apps.json"
    if ((Test-Path $wasmHtml) -and (Test-Path $landingHtml) -and (Test-Path $diyHtml) -and (Test-Path $localHtml) -and (Test-Path $appHtml) -and (Test-Path $diyData) -and (Test-Path $allData)) {
        Write-Host "smoke: ok (craft-showcase web, diy & wasm)" -ForegroundColor Green
        exit 0
    } else {
        Write-Host "smoke: failed - missing web assets" -ForegroundColor Red
        exit 1
    }
}

if ($Build) {
    Set-Location (Join-Path $RepoRoot "crates\craft-showcase-wasm")
    & "$env:USERPROFILE\.cargo\bin\trunk.exe" build --release --public-url ./
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    Copy-Item -Path "dist\*" -Destination (Join-Path $RepoRoot "web\wasm") -Recurse -Force
    Write-Host "Build complete: WASM copied to web\wasm" -ForegroundColor Green
    exit 0
}

# Start local preview web server
$webDir = Join-Path $RepoRoot "web"
Write-Host "Starting Local Craft Suite Showcase at http://localhost:$Port ..." -ForegroundColor Cyan
Start-Process "http://localhost:$Port"

$pyLauncher = if (Get-Command py -ErrorAction SilentlyContinue) { "py" }
              elseif (Get-Command python -ErrorAction SilentlyContinue) { "python" }
              elseif (Get-Command python3 -ErrorAction SilentlyContinue) { "python3" }
              else { $null }

if ($pyLauncher) {
    & $pyLauncher -m http.server $Port --directory $webDir
} elseif (Get-Command uv -ErrorAction SilentlyContinue) {
    uv run python -m http.server $Port --directory $webDir
} else {
    Write-Host "ERROR: Neither py, python, python3, nor uv found." -ForegroundColor Red
    exit 1
}
