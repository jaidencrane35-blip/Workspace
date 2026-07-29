-- Desktop Arrangement persistence (DAF-1c).
-- Named membership of observed OS window identities for a workspace.
-- Separate from canvas layouts (007_layout.sql). Never stores apply/geometry authority.

CREATE TABLE IF NOT EXISTS desktop_arrangements (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_desktop_arrangements_workspace
    ON desktop_arrangements (workspace_id, deleted);

CREATE TABLE IF NOT EXISTS desktop_arrangement_entries (
    id TEXT PRIMARY KEY NOT NULL,
    arrangement_id TEXT NOT NULL,
    stable_window_id TEXT,
    hwnd TEXT,
    process_id INTEGER,
    process_name TEXT,
    title_fingerprint TEXT,
    label TEXT NOT NULL DEFAULT '',
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (arrangement_id) REFERENCES desktop_arrangements(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_desktop_arrangement_entries_arrangement
    ON desktop_arrangement_entries (arrangement_id, sort_order);
