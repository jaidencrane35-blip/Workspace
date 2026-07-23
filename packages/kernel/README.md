# workspace-kernel

Platform Kernel boundary for Workspace — the future central runtime authority.

## Sprint 02

Owns:

- **Runtime state** (`WorkspaceState`) — version, status, initialization
- **Configuration** (`ConfigManager`) — SQLite-backed settings with defaults
- **Service registry** — type-keyed registration pattern for future domain services

## Architecture

```
React UI  →  Tauri commands  →  WorkspaceKernel  →  workspace-database
```

Application state lives in Rust. The frontend displays data only.

## Public API

- `WorkspaceKernel::initialize(path)` — open database, migrate, seed defaults
- `WorkspaceKernel::state()` — read runtime state
- `WorkspaceKernel::get_settings()` / `update_settings()` — configuration access

## Exclusions (Sprint 02)

No AI, automation, plugins, Windows APIs, permission gateway, or event bus yet.
