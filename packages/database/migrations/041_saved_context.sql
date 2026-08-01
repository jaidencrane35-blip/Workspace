-- User-authored bounded workspace contexts (Product Proof PP-M1-01).
--
-- A saved context is the user's own artefact: it exists only because the user
-- named it and confirmed a capture scope. It therefore keeps its own copy of
-- the windows and monitors it was built from rather than referencing an
-- observation pass, because observation passes are a rolling perception buffer
-- and are purged by retention. `observation_pass_id` records provenance only.
--
-- `approved_scope` is the capture scope the user actually confirmed. It is
-- stored so a later read can tell whether the content matches what was agreed.

CREATE TABLE IF NOT EXISTS saved_contexts (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL,
    approved_scope TEXT NOT NULL,
    observation_pass_id TEXT NOT NULL,
    captured_at TEXT NOT NULL,
    window_count INTEGER NOT NULL DEFAULT 0,
    monitor_count INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_saved_contexts_workspace
    ON saved_contexts (workspace_id, created_at DESC);

CREATE TABLE IF NOT EXISTS saved_context_windows (
    id TEXT PRIMARY KEY NOT NULL,
    saved_context_id TEXT NOT NULL,
    title TEXT NOT NULL DEFAULT '',
    process_id INTEGER NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    monitor_index INTEGER,
    minimized INTEGER NOT NULL DEFAULT 0,
    focused INTEGER NOT NULL DEFAULT 0,
    z_order INTEGER,
    FOREIGN KEY (saved_context_id) REFERENCES saved_contexts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_saved_context_windows_context
    ON saved_context_windows (saved_context_id);

CREATE TABLE IF NOT EXISTS saved_context_monitors (
    id TEXT PRIMARY KEY NOT NULL,
    saved_context_id TEXT NOT NULL,
    monitor_index INTEGER NOT NULL,
    name TEXT NOT NULL DEFAULT '',
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    is_primary INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (saved_context_id) REFERENCES saved_contexts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_saved_context_monitors_context
    ON saved_context_monitors (saved_context_id);
