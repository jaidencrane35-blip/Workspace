-- Singleton last-capture-failure record for observation diagnostics (Sprint 107B).
-- Read-only visibility. Cleared on successful capture. Never grants authority.

CREATE TABLE IF NOT EXISTS observation_capture_failures (
    id INTEGER PRIMARY KEY NOT NULL CHECK (id = 1),
    failed_at TEXT NOT NULL,
    error_class TEXT NOT NULL,
    message TEXT NOT NULL,
    source TEXT
);
