# workspace-database — Connection Lifecycle

SQLite connection lifecycle for Workspace (DEC-010).

## Initialization sequence

1. **Open connection** — `Database::open(path)` creates parent directories and opens SQLite.
2. **Apply migrations** — `MigrationRunner` loads versioned `.sql` files from `migrations/`.
3. **Expose handle** — `DatabaseService` returns an owned connection for kernel use.

## Failure handling

| Failure | Behaviour |
|---------|-----------|
| Path / IO error | Initialization aborts; error logged; kernel enters `Error` lifecycle |
| Migration SQL error | Wrapped as `DatabaseError::Migration` with version context |
| Connection unavailable | Callers receive safe kernel/database errors (no raw SQLite strings over IPC) |

## Ownership

- The Platform Kernel owns the database service handle.
- React and Tauri commands never access SQLite directly.
- One connection per application instance (Sprint 03).

## Sprint 03 scope

No new product tables. Settings table only (`001_settings.sql`).
