# workspace-kernel

Platform Kernel boundary for Workspace — the central runtime authority.

## Governance hardening (DEC-016, DEC-017)

- **Resource-addressed permissions (DEC-016)** — `PermissionSubject` is now `System | Resource(ResourceKind)`; commands address resources uniformly
- **Read governance (DEC-017)** — `QueryCommand` declares a `*.read` capability and a `GovernanceClass`; `execute_query` governs + audits non-human reads and sensitive-kind reads, while local-human non-sensitive reads bypass governance. Enforcement remains allow-all.

## Sprint 09

- **Intent model** — `Intent`, `IntentContext`, `IntentType` in domain
- **Capability model** — `Capability`, `CapabilitySet` (identifiers only)
- **Policy layer** — `PermissionPolicy`, `PolicyEvaluator`, `AlwaysAllowPolicy`
- **Pipeline** — actor + intent + capability through permission and audit

## Sprint 08

- **Actor model** — `Actor`, `ActorContext`, `ActorId`, `ActorType` in domain
- **Command context** — every command receives `ActorContext`
- **Permission + audit** — actor propagated through pipeline (no auth changes)

## Sprint 07

- **Audit trail** — durable `audit_events` persistence via `AuditService`
- **Command auditing** — `CommandPipeline` records success/failure (no payloads)
- **Event auditing** — `AuditEventSubscriber` on `EventBus`
- **Query IPC** — `get_audit_history` (read-only)

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
React UI  →  Tauri IPC  →  CommandHandler  →  CommandPipeline  →  PermissionPolicy
                                                    ↓                      ↓
                                              PermissionGate         (AlwaysAllow)
                                                    ↓
                                              Services  →  workspace-database
                                                    ↓
                                                EventBus  →  Audit (intent + capability)
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
