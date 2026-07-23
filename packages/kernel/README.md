# workspace-kernel

Platform Kernel boundary for Workspace — the central runtime authority.

## Sprint 06

- **Command pipeline** — uniform `MutationCommand` / `QueryCommand` dispatch via `CommandPipeline`
- **Permission gate** — `PermissionGate` trait with `AllowAllPermissionGate` (future: approval, AI, automation)
- **Security module** — `packages/kernel/src/security/`

## Sprint 05

- **WorkspaceService** — domain operations for workspace entities
- **CreateWorkspace / GetWorkspace** commands

## Sprint 04

- **Event bus** — synchronous, thread-safe internal `DomainEvent` publishing (Sprint 06: safe dispatch)
- **Command layer** — `InitializeWorkspace`, `UpdateSettings`, shutdown via `CommandHandler`

## Sprint 03

- Lifecycle management, health reporting, service registry, IPC envelope

## Architecture

```
React UI  →  Tauri IPC  →  CommandHandler  →  CommandPipeline  →  PermissionGate
                                                    ↓
                                              Services  →  workspace-database
                                                    ↓
                                                EventBus
```

- **IPC** — external communication (React ↔ Rust)
- **Command pipeline** — single path for mutations (post-bootstrap)
- **Permission gate** — authorization seam (Observe → Learn → Suggest → **Permission** → Automate)
- **Event bus** — internal communication (kernel ↔ future modules)
- **ServiceRegistry** — runtime health tracking only; not for domain resources

## Registry naming

**Registry** is reserved for `ServiceRegistry` (runtime service health). Future domain resources use `*Service` types (e.g. `ZoneService`, `ApplicationService`), not `ResourceRegistry`.

## Domain events

- `system.workspace.started`
- `system.workspace.ready`
- `system.workspace.shutdown`
- `system.settings.changed`
- `workspace.entity.created`
- `workspace.entity.updated`

## Exclusions

No AI, automation, plugins, Windows APIs, or async workers yet.
