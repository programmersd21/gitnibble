<div align="center">

### gitnibble

![Demo](https://raw.githubusercontent.com/programmersd21/gitnibble/main/assets/demo.gif)

[![CI](https://img.shields.io/github/actions/workflow/status/programmersd21/gitnibble/ci.yml?style=for-the-badge&logo=githubactions&logoColor=white&color=2EA043&labelColor=000000)](https://github.com/programmersd21/gitnibble/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/gitnibble-rs?style=for-the-badge&logo=rust&color=FE7D37&labelColor=000000)](https://crates.io/crates/gitnibble-rs)
[![Downloads](https://img.shields.io/crates/d/gitnibble-rs?style=for-the-badge&logo=rust&color=58A6FF&labelColor=000000)](https://crates.io/crates/gitnibble-rs)
[![License](https://img.shields.io/badge/license-MIT-8250DF?style=for-the-badge&labelColor=000000)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-win%20%7C%20linux%20%7C%20macos-00ADB5?style=for-the-badge&labelColor=000000)](https://github.com/programmersd21/gitnibble)
[![Built with](https://img.shields.io/badge/built%20with-ratatui-FF2D55?style=for-the-badge&logo=rust&labelColor=000000)](https://ratatui.rs)

</div>

> Instant, offline-first, context-aware `.gitignore` generator and manager in Rust with a ratatui-based terminal UI.

**Zero network calls by default.** Embedded templates for ~60+ languages, frameworks, and tools.

## Features

- **Instant cold-start**: Sub-20ms startup time with zero async runtime overhead.
- **Safe by Default**: Additive-only merges. Never deletes or alters existing custom ignore rules.
- **Context-Aware Workspace Scanner**: Scans workspace root to detect languages, frameworks, IDEs, and OS settings.
- **Ratatui TUI**: Three-pane interface with 12 curated truecolor themes, fuzzy search, and real-time diff preview.
- **Scriptable CLI**: Non-interactive commands with exit codes designed for CI/CD pipelines and git hooks.
- **Zero Network Overhead**: Embedded offline templates with optional opt-in `--features fetch` for remote retrieval.

## Themes

catppuccin_mocha, tokyo_night, dracula, nord, obsidian, solarized, gruvbox, rose_pine, everforest, kanagawa, monokai_pro, one_dark

Press `t` in the TUI to open the theme selector.

## Installation

```bash
cargo install gitnibble-rs
```

Or build from source:

```bash
git clone https://github.com/programmersd21/gitnibble.git
cd gitnibble
cargo build --release
```

## Keyboard Shortcuts (TUI)

| Key | Action |
| --- | --- |
| `j` / `Down` | Move selection down |
| `k` / `Up` | Move selection up |
| `Space` | Toggle template selection |
| `a` | Select all detected templates |
| `c` | Clear selection |
| `/` | Fuzzy search templates |
| `Tab` | Switch active pane (Stack / Templates / Diff) |
| `r` | Rescan workspace |
| `t` | Open theme selector |
| `Enter` | Apply selected templates to `.gitignore` |
| `y` | Copy diff preview to clipboard |
| `?` | Toggle help modal |
| `q` / `Esc` | Quit |

## CLI Usage

```bash
# Detect project stack without launching TUI
gitnibble detect

# Preview changes for specified templates
gitnibble diff Rust Node macOS

# Add templates non-interactively
gitnibble add Rust Node --yes

# Perform dry-run without writing to disk
gitnibble add Python --dry-run

# Launch interactive TUI
gitnibble
```

### Exit Codes (CLI Diff Mode)

- `0`: Success / no pending changes to write
- `1`: Pending changes exist (diff detected)
- `2`: Execution error / template not found

## Why not `gitignore.io`?

- **Offline & Instant**: No HTTP requests, latency, or remote server reliance. Works on air-gapped systems.
- **Non-Destructive Merge Engine**: Safely appends rules without destroying custom notes or manual rules.
- **CLI & Scripting Composability**: Integrates cleanly into `pre-commit` hooks with meaningful exit codes.

## License

MIT
