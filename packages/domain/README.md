# workspace-domain

Shared Workspace domain models — pure data, no persistence or UI.

## Sprint 05 entities

| Entity | Purpose |
|--------|---------|
| `Workspace` | User workspace container |
| `Zone` | Zone within a workspace (position placeholder) |
| `ApplicationReference` | Linked application metadata |
| `WidgetReference` | Linked widget metadata |

## Boundaries

- **In scope:** entity definitions, domain validation
- **Out of scope:** SQLite, React, kernel commands, rendering

Persistence lives in `workspace-database`. Operations live in `workspace-kernel` services.
