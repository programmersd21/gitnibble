# Changelog

All notable changes to `gitnibble` will be documented in this file.

## [0.1.1] - 2026-07-26

### Fixed
- Mouse scroll sensitivity: cooldown gate prevents hyperscroll on a single tick.
- Enabled crossterm mouse capture so scroll events are handled directly instead
  of leaking through as raw escape sequences.

## [0.1.0] - 2026-07-22

### Added
- Instant offline `.gitignore` generator with ratatui TUI.
- Context-aware workspace scanner (depth-bounded).
- Pure-function non-destructive merge engine.
- XDG-compliant config layer (`~/.config/gitnibble/config.toml`).
- Curated truecolor themes (Catppuccin Mocha, Tokyo Night, Dracula, Nord, Obsidian, Solarized).
- Non-interactive CLI interface with diff/detect/add commands.
- Optional compile-time `--features fetch` flag for online fetching.

### Changed
- Refined TUI visual layout, row selection indicators, dynamic status bar constraints, and theme color palette definitions.

