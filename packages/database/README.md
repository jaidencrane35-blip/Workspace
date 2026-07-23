# workspace-database

SQLite persistence foundation for Workspace (DEC-010).

## Sprint 07

- `AuditRepository` — append and query recent audit records
- Migration `004_audit.sql` — `audit_events` table with timestamp/event_type indexes

## Sprint 06

- `Database::transaction()` — atomic multi-step operations (`BEGIN IMMEDIATE` / `COMMIT` / `ROLLBACK`)
- `Transaction` scope type for in-transaction repository helpers
- Typed domain IDs in repository mappings

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
