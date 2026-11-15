#Requires -Version 5.1
<#
.SYNOPSIS
  Scans the local *craft* fleet and regenerates web/data/diy-apps.json and web/data/all-apps.json.
#>
param(
    [string]$FleetRoot
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
if (-not $FleetRoot) {
    $FleetRoot = Split-Path -Parent $RepoRoot
}

$BackofficeMeta = @{
    'billcraft'     = @{ name = 'BillCraft';     icon = '🧾'; badge = 'ebInterface 6.1'; replaces = 'Stripe Billing & Chargebee';        desc = 'Austrian-compliant invoicing, recurring subscriptions, PDF/A-1b archiving, ebInterface 6.1 XML export, and SEPA Direct Debit (pain.008) batch generation.' }
    'spendcraft'    = @{ name = 'SpendCraft';    icon = '💳'; badge = '§26 EStG';        replaces = 'Expensify, Pleo & Circula';          desc = 'Travel & expense engine calculating Austrian §26 EStG Kilometergeld (€0.42/km), domestic & international Tagesdiäten, and SEPA pain.001 payouts.' }
    'hirecraft'     = @{ name = 'HireCraft';     icon = '🎯'; badge = 'Blind ATS';       replaces = 'Greenhouse, Lever & Ashby';          desc = 'Applicant tracking system with anonymized blind candidate scorecards, weighted rubric scoring, and one-click multi-app onboarding cascades.' }
    'helmcraft'     = @{ name = 'HelmCraft';     icon = '🧭'; badge = 'CFO Cockpit';     replaces = 'Geckoboard & Baremetrics';           desc = 'Executive telemetry radar aggregating cross-app SQLite ledgers, live cash runway projections, burn rate charts, and printable board briefings.' }
    'taskcraft'     = @{ name = 'TaskCraft';     icon = '📋'; badge = 'Kanban + Git';    replaces = 'Jira Enterprise & Linear';           desc = 'Local-first agile sprint board and issue tracker featuring automated Git commit closing (`Closes #ID`) and cross-app escalation intake.' }
    'deskcraft'     = @{ name = 'DeskCraft';     icon = '🎧'; badge = 'Maildir SLA';     replaces = 'Zendesk Suite Enterprise';           desc = 'Customer support desk with offline .eml Maildir archiving, SQLite FTS5 instant search, SLA breach timers, and negative sentiment alerts.' }
    'contractcraft' = @{ name = 'ContractCraft'; icon = '📜'; badge = 'Ed25519 Sealing'; replaces = 'Ironclad & DocuSign CLM';            desc = 'Contract lifecycle pipeline with cryptographic Ed25519 tamper-evident seals, automated clause risk scanning, and Austrian Kündigungsfrist notice calculators.' }
    'gaincraft'     = @{ name = 'GainCraft';     icon = '📊'; badge = 'Customer 360';    replaces = 'Gainsight CS & ChurnZero';           desc = 'Customer success cockpit evaluating account health bands (Green / Yellow / Red), tracking NPS sentiment decay, and managing triage Call-to-Actions.' }
    'teamcraft'     = @{ name = 'TeamCraft';     icon = '⏱️'; badge = 'AZG 38.5h';       replaces = 'Personio & Clockodo';                desc = 'Workforce attendance workstation enforcing Austrian Arbeitszeitgesetz (AZG): 11h daily rest periods, max 10h/day, 50h weekly caps, and leave entitlements.' }
    'assetcraft'    = @{ name = 'AssetCraft';    icon = '💻'; badge = '§7 EStG AfA';     replaces = 'Snipe-IT & Oomnitza';                desc = 'IT asset registry calculating Austrian linear depreciation (AfA §7 EStG), Halbjahres-AfA, GWG write-offs, and printable QR asset inventory tags.' }
    'formcraft'     = @{ name = 'FormCraft';     icon = '📝'; badge = 'NPS Detractors';  replaces = 'Typeform & Delighted';               desc = 'Local-first survey intake engine with automated Net Promoter Score (NPS) rollups and real-time detractor event routing directly into Customer Success.' }
    'wikicraft'     = @{ name = 'WikiCraft';     icon = '📚'; badge = 'FTS5 & ADRs';     replaces = 'Notion Plus & Confluence';           desc = 'Markdown documentation hub with SQLite FTS5 full-text snippet queries, wiki-link backlink extraction, and formal Architecture Decision Record state machines.' }
    'buchcraft'     = @{ name = 'BuchCraft';     icon = '📒'; badge = 'EKR & U30';       replaces = 'BMD NTCS, RZL, DATEV & sevDesk';     desc = 'Austrian double-entry bookkeeping with EKR chart, integer-cent balanced entries, immutable SQLite triggers with linked Storno, VAT keys (U20/U10/V20/IGE20), Saldenliste, and U30 Kennzahlen pre-fill.' }
}

$ExcludeRepos = @(
    'craft-showcase', 'craft-biz', 'craft-ai', 'craft-sandbox-vbox',
    'craft-anything-demo', 'craft-demo-20261112', 'craft-site-demo',
    'codercraft test', 'gamecraft-cli-wt', 'on-metacraft'
)

$DiyMeta = @{
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

function Get-ReadmeDetails {
    param([string]$Dir, [string]$FallbackName)
    $readme = Join-Path $Dir "README.md"
    $title = $FallbackName
    $desc = ""
    $highlights = @()

    if (-not (Test-Path $readme)) {
        return @{ name = $title; desc = "Local-first pure-Rust desktop and CLI app."; highlights = @() }
    }

    $lines = Get-Content $readme -TotalCount 110 -Encoding UTF8
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
            if ($cleaned) { $title = $cleaned }
            continue
        }

        if ($line -match '^(##|!\[|<|Tell Cursor|Installs the Windows app|powershell|\[!\[|\|---)') { continue }

        # Collect bullet points or table feature rows as highlights
        if ($line -match '^[-*]\s+(.+)$' -and $highlights.Count -lt 6) {
            $b = ($Matches[1] -replace '\[([^\]]+)\]\([^\)]+\)', '$1' -replace '[\*`]', '' -replace '\b[Ss]overeign\b', 'local-first').Trim()
            if ($b.Length -ge 15 -and $b -notmatch '^How to use|^What it is|^Every MCP|^Changes:') {
                if ($b.Length -gt 180) { $b = $b.Substring(0, 177).TrimEnd() + "..." }
                $highlights += $b
            }
            continue
        }

        $plain = $line -replace '\[([^\]]+)\]\([^\)]+\)', '$1' -replace '[\*`>]', '' -replace '\b[Ss]overeign\b', 'local-first'
        $plain = $plain.Trim()
        if (-not $desc -and $plain.Length -ge 20 -and $plain -notmatch '^\|') {
            $desc = $plain
        }
    }

    if (-not $desc) { $desc = "Local-first pure-Rust desktop and CLI app." }
    if ($desc.Length -gt 200) { $desc = $desc.Substring(0, 197).TrimEnd() + "..." }
    return @{ name = $title; desc = $desc; highlights = $highlights }
}

$diyApps = @()
$allApps = @()
$dirs = Get-ChildItem -Directory (Join-Path $FleetRoot "*craft*") | Sort-Object Name

foreach ($d in $dirs) {
    $key = $d.Name
    if ($ExcludeRepos -contains $key) { continue }

    $cargo = Join-Path $d.FullName "Cargo.toml"
    $isBackoffice = ($BackofficeMeta.ContainsKey($key)) -or ((Test-Path $cargo) -and (Select-String -Path $cargo -Pattern 'craft-biz' -Quiet))

    $details = Get-ReadmeDetails -Dir $d.FullName -FallbackName $key
    $ver = Get-RepoVersion -Dir $d.FullName
    $gh = Get-GitHubUrl -Dir $d.FullName -Key $key
    $hasInstaller = Test-Path (Join-Path $d.FullName "scripts\install-craft.ps1")

    if ($isBackoffice) {
        $bm = if ($BackofficeMeta.ContainsKey($key)) { $BackofficeMeta[$key] } else { @{ name = $details.name; icon = '🏢'; badge = 'Back-Office'; replaces = 'Cloud SaaS'; desc = $details.desc } }
        $allApps += [ordered]@{
            id         = $key
            name       = $bm.name
            repo       = $key
            suite      = 'backoffice'
            cat        = 'backoffice'
            icon       = $bm.icon
            badge      = $bm.badge
            replaces   = $bm.replaces
            version    = $ver
            github     = $gh
            installer  = $hasInstaller
            desc       = $bm.desc
            highlights = $details.highlights
        }
        continue
    }

    $m = if ($DiyMeta.ContainsKey($key)) { $DiyMeta[$key] } else { @{ cat = 'infra'; icon = '🛠️'; badge = if ($ver) { $ver } else { 'DIY Craft' }; desc = $null } }
    $finalDesc = if ($m.desc) { $m.desc } else { $details.desc }

    $diyEntry = [ordered]@{
        id      = $key
        name    = $details.name
        repo    = $key
        cat     = $m.cat
        icon    = $m.icon
        badge   = $m.badge
        version = $ver
        github  = $gh
        desc    = $finalDesc
    }
    $diyApps += $diyEntry

    $allApps += [ordered]@{
        id         = $key
        name       = $details.name
        repo       = $key
        suite      = 'diy'
        cat        = $m.cat
        icon       = $m.icon
        badge      = $m.badge
        replaces   = $null
        version    = $ver
        github     = $gh
        installer  = $hasInstaller
        desc       = $finalDesc
        highlights = $details.highlights
    }
}

$outDir = Join-Path $RepoRoot "web\data"
if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Path $outDir -Force | Out-Null
}

$stamp = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssK")
$diyPayload = [ordered]@{ generated_at = $stamp; count = $diyApps.Count; apps = $diyApps }
$allPayload = [ordered]@{ generated_at = $stamp; count = $allApps.Count; apps = $allApps }

[System.IO.File]::WriteAllText((Join-Path $outDir "diy-apps.json"), ($diyPayload | ConvertTo-Json -Depth 5), [System.Text.UTF8Encoding]::new($false))
[System.IO.File]::WriteAllText((Join-Path $outDir "all-apps.json"), ($allPayload | ConvertTo-Json -Depth 5), [System.Text.UTF8Encoding]::new($false))
Write-Host "sync: wrote $($diyApps.Count) DIY apps and $($allApps.Count) total apps to $outDir" -ForegroundColor Green
