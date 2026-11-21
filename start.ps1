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
    if (-not ((Test-Path $wasmHtml) -and (Test-Path $landingHtml) -and (Test-Path $diyHtml) -and (Test-Path $localHtml) -and (Test-Path $appHtml) -and (Test-Path $diyData) -and (Test-Path $allData))) {
        Write-Host "smoke: failed - missing web assets" -ForegroundColor Red
        exit 1
    }
    $wasmDir = Split-Path $wasmHtml
    $wasmHtmlText = [System.IO.File]::ReadAllText($wasmHtml)
    if ($wasmHtmlText.Contains("'/craft-showcase-wasm-") -or $wasmHtmlText.Contains('"/craft-showcase-wasm-')) {
        Write-Host "smoke: failed - wasm bundles are requested from the site root" -ForegroundColor Red
        exit 1
    }
    $assetPattern = '[''"](?:\./|/wasm/)(craft-showcase-wasm-[^''"]+\.(?:js|wasm))[''"]'
    $assetMatches = [regex]::Matches($wasmHtmlText, $assetPattern)
    if ($assetMatches.Count -lt 2) {
        Write-Host "smoke: failed - wasm index.html does not reference the js and wasm bundles" -ForegroundColor Red
        exit 1
    }
    $seen = @{}
    foreach ($assetMatch in $assetMatches) {
        $assetName = $assetMatch.Groups[1].Value
        if ($seen.ContainsKey($assetName)) { continue }
        $seen[$assetName] = $true
        $assetPath = Join-Path $wasmDir $assetName
        if (-not (Test-Path -LiteralPath $assetPath)) {
            Write-Host "smoke: failed - missing $assetPath" -ForegroundColor Red
            exit 1
        }
    }
    Write-Host "smoke: ok (craft-showcase web, diy & wasm)" -ForegroundColor Green
    exit 0
}

if ($Build) {
    Set-Location (Join-Path $RepoRoot "crates\craft-showcase-wasm")
    & "$env:USERPROFILE\.cargo\bin\trunk.exe" build --release --public-url ./
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    $wasmOut = Join-Path $RepoRoot "web\wasm"
    Copy-Item -Path "dist\*" -Destination $wasmOut -Recurse -Force
    $wasmHtmlOut = Join-Path $wasmOut "index.html"
    $utf8 = New-Object System.Text.UTF8Encoding $false
    $builtHtml = [System.IO.File]::ReadAllText($wasmHtmlOut)
    $builtHtml = $builtHtml.Replace("'/craft-showcase-wasm-", "'./craft-showcase-wasm-")
    $builtHtml = $builtHtml.Replace('"/craft-showcase-wasm-', '"./craft-showcase-wasm-')
    $builtHtml = $builtHtml.Replace("'././craft-showcase-wasm-", "'./craft-showcase-wasm-")
    $builtHtml = $builtHtml.Replace('"././craft-showcase-wasm-', '"./craft-showcase-wasm-')
    [System.IO.File]::WriteAllText($wasmHtmlOut, $builtHtml, $utf8)
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

$serveScript = Join-Path $RepoRoot "scripts\serve.py"
if ($pyLauncher) {
    & $pyLauncher $serveScript --port $Port --directory $webDir
} elseif (Get-Command uv -ErrorAction SilentlyContinue) {
    uv run python $serveScript --port $Port --directory $webDir
} else {
    Write-Host "ERROR: Neither py, python, python3, nor uv found." -ForegroundColor Red
    exit 1
}
