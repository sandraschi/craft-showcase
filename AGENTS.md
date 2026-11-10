# Craft Showcase — AGENTS.md

Interactive WebAssembly demo studio, client presentation portal, and economic calculator for the Sovereign Back-Office Suite.

## Port

- Port **11270** (`web/` served via local web server)

## Crates

- `crates/craft-showcase-wasm`: Pure-Rust `eframe 0.37` WebAssembly application compiled via `trunk`.

## Assets

- `web/index.html`: High-production client landing page with embedded WASM studio, ROI calculator, and video storyboard.
- `web/wasm/`: Compiled WebAssembly distribution (`.wasm` + `.js` glue + canvas runner).
- `web/downloads/`: Printable A4 PDF brochure and digital EPUB edition.

## Smoke Test

```powershell
.\start.ps1 -Smoke
```
