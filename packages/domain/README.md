# workspace-domain

Shared Workspace domain models — pure data, no persistence or UI.

## Sprint 06 typed identifiers

| Type | Entity |
|------|--------|
| `WorkspaceId` | `Workspace` |
| `ZoneId` | `Zone` |
| `ApplicationId` | `ApplicationReference` |
| `WidgetId` | `WidgetReference` |

IDs serialize as JSON strings (`#[serde(transparent)]`) for IPC compatibility.

## Sprint 09 intent & capability

| Type | Purpose |
|------|---------|
| `Intent` / `IntentContext` | Why an action is performed |
| `IntentId` / `IntentType` | Typed intent classification |
| `Capability` / `CapabilityId` | Authority identifier (not enforced yet) |
| `CapabilitySet` | Collection of granted capability ids |

Active intents: `UserRequest`, `SystemStartup`, `SystemShutdown`.

## Sprint 08 execution identity

| Type | Purpose |
|------|---------|
| `Actor` | Immutable execution identity |
| `ActorContext` | Passed through every command |
| `ActorId` | Typed actor identifier |
| `ActorType` | LocalUser, System (+ future placeholders) |

Only `LocalUser` and `System` are active in Sprint 08. This is identity, not authentication.

## Sprint 07 audit entities

| Type | Purpose |
|------|---------|
| `AuditEvent` | Durable activity record |
| `ActorType` | Who initiated an action (`system`, `user`, `service`) |
| `AuditEventId` | Typed audit record identifier |

## Sprint 05 entities

| Entity | Purpose |
|--------|---------|
| `Workspace` | User workspace container |
| `Zone` | Zone within a workspace (position placeholder) |
| `ApplicationReference` | Linked application metadata |
| `WidgetReference` | Linked widget metadata |

## Boundaries

- **In scope:** entity definitions, typed IDs, domain validation
- **Out of scope:** SQLite, React, kernel commands, rendering

Persistence lives in `workspace-database`. Operations live in `workspace-kernel` services.
