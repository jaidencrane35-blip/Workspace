-- Sprint 02: application configuration storage (metadata only).
-- No user profiles, AI memory, automation, or permission tables.

CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
