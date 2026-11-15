#Requires -Version 5.1
<#
.SYNOPSIS
  Scans the local *craft* fleet and regenerates web/data/diy-apps.json for the DIY Craft Fleet Showcase.
#>
param(
    [string]$FleetRoot
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
if (-not $FleetRoot) {
    $FleetRoot = Split-Path -Parent $RepoRoot
}

$BackofficeRepos = @(
    'assetcraft', 'billcraft', 'buchcraft', 'contractcraft', 'deskcraft',
    'formcraft', 'gaincraft', 'helmcraft', 'hirecraft', 'spendcraft',
    'taskcraft', 'teamcraft', 'wikicraft'
)

$ExcludeRepos = @(
    'craft-showcase', 'craft-biz', 'craft-ai', 'craft-sandbox-vbox',
    'craft-anything-demo', 'craft-demo-20261112', 'craft-site-demo',
    'codercraft test', 'gamecraft-cli-wt', 'on-metacraft'
)

# Curated category, icon, badge, and plain-English description per repo; new repos fall back to README.md.
$Meta = @{
    'aicraft'         = @{ cat = 'ai';       icon = '🧠'; badge = 'AI Ops';         desc = 'Local AI model router and token-cost dashboard for managing local LLMs and API usage in one place.' }
    'codercraft'      = @{ cat = 'ai';       icon = '💻'; badge = 'Port 14128';     desc = 'Fast, native Rust coding IDE with built-in AI agent tools — ~35 MB RAM with zero Electron overhead.' }
    'rsicraft'        = @{ cat = 'ai';       icon = '🔁'; badge = 'Local RSI';      desc = 'Self-improving coding loop where a local LLM writes, tests, and refines small programs on your own GPU.' }
    'lenscraft'       = @{ cat = 'ai';       icon = '📷'; badge = 'Vision L1';      desc = 'Point a camera at a painting, sculpture, or building and get ranked identifications with evidence from a local vision model.' }
    'speechcraft'     = @{ cat = 'ai';       icon = '🎙️'; badge = 'Port 14132';     desc = 'Local voice studio with wake-word detection, offline speech recognition, and audio mastering.' }

    'celcraft'        = @{ cat = 'creative'; icon = '🎨'; badge = '2D Animation';   desc = 'Classic hand-drawn 2D animation and ink-and-paint studio in pure Rust — a modern OpenToonz replacement.' }
    'musiccraft'      = @{ cat = 'creative'; icon = '🎼'; badge = 'DAW + Stems';    desc = 'Music production workstation with multitrack arrangement, virtual vocal synthesis, 4-band stem separation, and offline mix export.' }
    'djcraft'         = @{ cat = 'creative'; icon = '🎧'; badge = '4-Deck Mix';     desc = '4-deck DJ mixing studio with real-time stem isolation and harmonic transitions, running 100% locally.' }
    'soundcraft'      = @{ cat = 'creative'; icon = '🔊'; badge = 'Audio DAW';      desc = 'Multitrack digital audio workstation in pure Rust with mixer, plugins, MIDI, and scriptable CLI.' }
    'gamecraft'       = @{ cat = 'creative'; icon = '🎮'; badge = 'Godot 4';        desc = 'Describe a game in one sentence and generate a playable, editable Godot 4 project on your machine.' }
    'splatcraft'      = @{ cat = 'creative'; icon = '✨'; badge = '3D Splats';      desc = 'Pure-Rust viewer and toolkit for 3D Gaussian splats.' }
    'studiocraft'     = @{ cat = 'creative'; icon = '🖼️'; badge = 'Canvas Host';    desc = 'Shared project format (.studiocraft) and universal canvas host connecting the creative Craft apps.' }
    'smallvideocraft' = @{ cat = 'creative'; icon = '🎬'; badge = 'Alpha';          desc = 'Planned pure-Rust video generator companion to the ittybittyvideos script-to-video pipeline.' }

    'writercraft'     = @{ cat = 'writing';  icon = '✍️'; badge = '6 Workspaces';   desc = 'Offline writing studio for novels, screenplays (Fountain), and academic papers, with a narrative corkboard and Calibre sync.' }
    'bookcraft'       = @{ cat = 'writing';  icon = '📖'; badge = 'Calibre';        desc = 'Fast Calibre e-book library manager in pure Rust: search, inspect, edit metadata, convert, and import books.' }
    'lookcraft'       = @{ cat = 'writing';  icon = '✉️'; badge = 'Mail Core';      desc = 'Local-first email engine with offline EML parsing, Maildir storage, full-text search, and IMAP/SMTP support.' }
    'travelcraft'     = @{ cat = 'writing';  icon = '🧭'; badge = 'Baedeker 2030';  desc = 'Offline travel guide and map studio (Austria & Japan) inspired by classic Baedeker handbooks.' }
    'sportscraft'     = @{ cat = 'writing';  icon = '⚽'; badge = 'Spectator + PT'; desc = 'Local sports companion for European football tables, fixtures, match predictions, and personal training logs.' }

    'modcraft'        = @{ cat = 'hardware'; icon = '🧊'; badge = '3D CSG';         desc = 'Parametric 3D CAD and solid modeling tool with automated Bill of Materials (BOM) generation.' }
    'makercraft'      = @{ cat = 'hardware'; icon = '🔧'; badge = 'Fab Bench';      desc = 'Workshop control bench for 3D printers, CNC routers, and hardware protocol debugging.' }
    'patterncraft'    = @{ cat = 'hardware'; icon = '📐'; badge = 'Pattern IR';     desc = 'Converts vector artwork into machine-ready paths for pen plotters and CNC routers.' }
    'robotcraft'      = @{ cat = 'hardware'; icon = '🤖'; badge = 'Kinematics';     desc = 'Robotics workstation for hardware telemetry, kinematics solvers, URDF models, Gazebo bridges, and live tele-op.' }
    'giscraft'        = @{ cat = 'hardware'; icon = '🗺️'; badge = 'QGIS Shell';     desc = 'Desktop GIS mapping tool over OpenStreetMap with a familiar QGIS-style layout and a modern Rust core.' }
    'homecraft'       = @{ cat = 'hardware'; icon = '🏠'; badge = 'Smart Home';     desc = 'Local smart-home controller and lightweight Home Assistant alternative in pure Rust.' }

    'moneycraft'      = @{ cat = 'infra';    icon = '📈'; badge = 'Quant Terminal'; desc = 'Offline financial research and time-series charting terminal for personal quantitative analysis.' }
    'databasecraft'   = @{ cat = 'infra';    icon = '🗄️'; badge = 'Port 14113';     desc = 'Pure-Rust relational database and schema studio for managing structured app data locally.' }
    'appcraft'        = @{ cat = 'infra';    icon = '🏭'; badge = 'App Factory';    desc = 'Code generator that scaffolds new local-first Rust desktop and web apps from structured domain blueprints.' }
    'craft-deck'      = @{ cat = 'infra';    icon = '🚀'; badge = 'Launcher';       desc = 'Desktop launcher to install, update, and open all Craft apps from one place.' }
    'craftcraft'      = @{ cat = 'infra';    icon = '🦉'; badge = 'Catalog';        desc = 'Registry and tracker monitoring all local Craft apps alongside upstream ArtCraft releases.' }
    'devcraft'        = @{ cat = 'infra';    icon = '🛠️'; badge = 'Fleet Cockpit';  desc = 'Local developer cockpit for managing repos, builds, and AI coding sessions across IDEs.' }
    'wincraft'        = @{ cat = 'infra';    icon = '🪟'; badge = 'Win SysAdmin';   desc = 'Windows system utility for memory cleanup, file-lock inspection, fast file ops, and service management.' }
    'vmcraft'         = @{ cat = 'infra';    icon = '📦'; badge = 'Multi-VM';       desc = 'Local VM manager for VirtualBox, Hyper-V, and Firecracker with isolated sandbox testing.' }
    'democraft-mcp'   = @{ cat = 'infra';    icon = '🧪'; badge = 'Runner';         desc = 'Automated demo runner and screenshot harness for testing ArtCraft applications.' }
}

function Get-GitHubUrl {
    param([string]$Dir, [string]$Key)
    if (Test-Path (Join-Path $Dir ".git")) {
        $prev = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        $remotes = git -C $Dir remote 2>$null
        if ($remotes -contains "origin") {
            $raw = git -C $Dir remote get-url origin 2>$null
            $ErrorActionPreference = $prev
            if ($raw) {
                $url = $raw.Trim() -replace '^git@github\.com:', 'https://github.com/' -replace '\.git$', ''
                if ($url -match '^https?://') { return $url }
            }
        } else {
            $ErrorActionPreference = $prev
        }
    }
    return "https://github.com/sandraschi/$Key"
}

function Get-RepoVersion {
    param([string]$Dir)
    $cargo = Join-Path $Dir "Cargo.toml"
    if (Test-Path $cargo) {
        $match = Select-String -Path $cargo -Pattern '^\s*version\s*=\s*"([^"]+)"' | Select-Object -First 1
        if ($match) { return "v$($match.Matches.Groups[1].Value)" }
    }
    return ""
}

function Get-ReadmeSummary {
    param([string]$Dir, [string]$FallbackName)
    $readme = Join-Path $Dir "README.md"
    $title = $FallbackName
    $desc = ""

    if (-not (Test-Path $readme)) {
        return @{ name = $title; desc = "Local-first pure-Rust desktop and CLI app." }
    }

    $lines = Get-Content $readme -TotalCount 50 -Encoding UTF8
    $inCode = $false
    foreach ($raw in $lines) {
        $line = $raw.Trim()
        if ($line -match '^```') { $inCode = -not $inCode; continue }
        if ($inCode -or -not $line) { continue }

        if ($line -match '^#\s+(.+)$' -and $title -eq $FallbackName) {
            $rawTitle = $Matches[1]
            if ($rawTitle -match '^(.+?)\s*(?::|\u2014|\s-\s)') {
                $rawTitle = $Matches[1]
            }
            $cleaned = ($rawTitle -replace '[^\x20-\x7E]', '').Trim()
            if ($cleaned) {
                $title = $cleaned
            }
            continue
        }

        if ($line -match '^(##|!\[|<|Tell Cursor|Installs the Windows app|powershell|\[!\[)') { continue }

        $plain = $line -replace '\[([^\]]+)\]\([^\)]+\)', '$1'
        $plain = $plain -replace '[\*`>]', ''
        $plain = $plain -replace '\b[Ss]overeign\b', 'local-first'
        $plain = $plain.Trim()
        if ($plain.Length -ge 20) {
            $desc = $plain
            break
        }
    }

    if (-not $desc) { $desc = "Local-first pure-Rust desktop and CLI app." }
    if ($desc.Length -gt 200) { $desc = $desc.Substring(0, 197).TrimEnd() + "..." }
    return @{ name = $title; desc = $desc }
}

$apps = @()
$dirs = Get-ChildItem -Directory (Join-Path $FleetRoot "*craft*") | Sort-Object Name

foreach ($d in $dirs) {
    $key = $d.Name
    if ($BackofficeRepos -contains $key -or $ExcludeRepos -contains $key) { continue }

    $cargo = Join-Path $d.FullName "Cargo.toml"
    if ((Test-Path $cargo) -and (Select-String -Path $cargo -Pattern 'craft-biz' -Quiet)) {
        continue
    }

    $summary = Get-ReadmeSummary -Dir $d.FullName -FallbackName $key
    $ver = Get-RepoVersion -Dir $d.FullName
    $gh = Get-GitHubUrl -Dir $d.FullName -Key $key
    $m = if ($Meta.ContainsKey($key)) { $Meta[$key] } else { @{ cat = 'infra'; icon = '🛠️'; badge = if ($ver) { $ver } else { 'DIY Craft' }; desc = $null } }
    $finalDesc = if ($m.desc) { $m.desc } else { $summary.desc }

    $apps += [ordered]@{
        name    = $summary.name
        repo    = $key
        cat     = $m.cat
        icon    = $m.icon
        badge   = $m.badge
        version = $ver
        github  = $gh
        desc    = $finalDesc
    }
}

$outDir = Join-Path $RepoRoot "web\data"
if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Path $outDir -Force | Out-Null
}
$outFile = Join-Path $outDir "diy-apps.json"

$payload = [ordered]@{
    generated_at = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssK")
    count        = $apps.Count
    apps         = $apps
}

$json = $payload | ConvertTo-Json -Depth 4
[System.IO.File]::WriteAllText($outFile, $json, [System.Text.UTF8Encoding]::new($false))
Write-Host "sync: wrote $($apps.Count) DIY craft apps to $outFile" -ForegroundColor Green
