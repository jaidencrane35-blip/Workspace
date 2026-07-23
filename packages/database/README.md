# workspace-database

SQLite persistence foundation for Workspace (DEC-010).

## Sprint 05

- `WorkspaceRepository`, `ZoneRepository` — repository pattern for domain entities
- Migration `002_workspace.sql` — workspaces, zones, applications, widgets tables

## Sprint 02

- `DatabaseService` — open, migrate, lifecycle management
- `SettingsRepository` — key/value configuration storage
- Migration `001_settings.sql` — settings table only

See [LIFECYCLE.md](LIFECYCLE.md) for connection lifecycle and failure handling (Sprint 03).

## Sprint 01

- Connection handling
- Migration framework
- Encryption provider trait placeholder (DEC-015)

## Not implemented

- User, AI, automation, or permission tables
- Full encryption (Tier 1/2)
