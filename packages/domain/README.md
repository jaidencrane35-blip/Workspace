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
