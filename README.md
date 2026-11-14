# Sovereign Craft Suite — Web Showcase & In-Browser WASM Studio ⚡

![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Rust 1.90+](https://img.shields.io/badge/rust-1.90%2B-orange?logo=rust)
![MCP stdio](https://img.shields.io/badge/MCP-stdio-purple)
![ArtCraft harness](https://img.shields.io/badge/ArtCraft-harness-green)

**Interactive in-browser WebAssembly demo studio, client presentation portal, and economic calculator for the Sovereign Back-Office Suite.**

## Features

- **Live In-Browser WebAssembly Studio (`crates/craft-showcase-wasm`)**:
  - Full `egui / eframe 0.37` desktop workstation running directly on an HTML5 canvas at 60 FPS via WebGL2/WebGPU.
  - Interactive multi-app tabs:
    - 🧭 **HelmCraft**: Live executive KPI radar, cash runway, burn chart, and board briefing generator.
    - 🧾 **BillCraft**: Invoicing line items, Austrian ebInterface 6.1 XML export drawer, and SEPA Direct Debit pain.008 batch generator.
    - 💳 **SpendCraft**: Travel allowance calculator (§26 EStG Kilometergeld 42¢/km & Tagesdiäten €26.40/day) and SEPA pain.001 reimbursement generator.
    - 🎯 **HireCraft**: ATS recruiting funnel with blind quantitative scorecards and interactive onboarding cascade trigger.
    - 📋 **TaskCraft**: 4-column local-first Kanban sprint board.
- **Client Presentation Landing Page (`web/index.html`)**:
  - **Dynamic ROI Calculator**: Interactive seat slider calculating savings up to €314k/yr for 50 seats against Jira, Zendesk, Expensify, Stripe, and Greenhouse.
  - **Domino Cascade Visualizer**: Interactive flowchart explaining the zero-webhook, atomic event bus architecture.
  - **Austrian Statutory Compliance Matrix**: Detailed reference for ebInterface 6.1, §26 EStG, AZG 38.5h, AfA §7, and SEPA ISO 20022.
  - **The 2.5-Minute Cinematic Pitch**: Complete 4-act director's script and video presentation storyboard for executive demos.
  - **Executive Download Center**: One-click downloads for the A4 PDF brochure and digital EPUB edition.

## Quick Start

```powershell
# Run smoke tests
.\start.ps1 -Smoke

# Serve local showcase on port 11270 and open browser
.\start.ps1

# Recompile WebAssembly bundle from Rust source
.\start.ps1 -Build
```

## Central Launcher

Launch anytime from `mcp-central-docs`:
```cmd
D:\Dev\repos\mcp-central-docs\starts\craft-showcase-start.bat
```

## License

MIT OR Apache-2.0
