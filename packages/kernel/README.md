# workspace-kernel

Platform Kernel boundary for Workspace — the central runtime authority.

## Sprint 04

- **Event bus** — synchronous, thread-safe internal `DomainEvent` publishing
- **Command layer** — `InitializeWorkspace`, `UpdateSettings`, shutdown via `CommandHandler`
- **Observability** — command and event logging

## Sprint 03

- Lifecycle management, health reporting, service registry, IPC envelope

## Architecture

```
React UI  →  Tauri IPC  →  CommandHandler  →  Services  →  workspace-database
                                ↓
                            EventBus
```

- **IPC** — external communication (React ↔ Rust)
- **Event bus** — internal communication (kernel ↔ future modules)
- **Commands** — state-changing operations only

## Domain events

- `system.workspace.started`
- `system.workspace.ready`
- `system.workspace.shutdown`
- `system.settings.changed`

## Exclusions

No AI, automation, plugins, Windows APIs, or async workers yet.
