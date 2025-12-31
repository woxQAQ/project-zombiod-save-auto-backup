# PZ Backup Tool

A backup and restore tool for **Project Zomboid** save games. Built with Tauri + React + TypeScript.

## Features

- **Manual Backups**: Backup your Project Zomboid saves with timestamps
- **Safe Restore**: Pre-restore safety snapshots allow you to undo any restore operation
- **Garbage Collection**: Auto-delete old backups exceeding retention limit (default: 10)
- **Cross-Platform**: Support for Windows, macOS, and Linux
- **Dark Theme UI**: Minimal interface inspired by Project Zomboid's aesthetic

## Quick Start

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Development

```bash
# Install dependencies
pnpm install

# Development (with hot reload)
pnpm tauri dev

# Type checking
pnpm tsc --noEmit

# Lint frontend
pnpm lint

# Lint backend (Rust)
pnpm tauri:check

# Fix linting issues
pnpm lint:fix
```

## Project Structure

```
src/              # React frontend (TypeScript)
src-tauri/        # Rust backend (Tauri)
src-tauri/src/
  ├── main.rs     # Tauri entry point
  └── lib.rs      # Tauri commands (exposed to frontend)
docs/             # Product and UI documentation
public/           # Static assets
```

## Save Paths

Project Zomboid stores saves in:
- **Windows**: `C:\Users\<User>\Zomboid\Saves`
- **Mac/Linux**: `~/Zomboid/Saves`

## Tech Stack

- **Backend**: Rust (Tauri 2.x)
- **Frontend**: React 19 + TypeScript + Vite
- **Package Manager**: pnpm

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Documentation

- [CLAUDE.md](./CLAUDE.md) - Developer guidance and architecture
- [docs/prd.md](./docs/prd.md) - Product Requirements Document
- [docs/ui_design.md](./docs/ui_design.md) - UI/UX specifications
