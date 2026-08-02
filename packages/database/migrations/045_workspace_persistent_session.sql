-- Persistent WorkspaceSessionStore — durable subset of WorkspaceRuntimeState.
-- Singleton row; full desktop projection is derived (never stored).

CREATE TABLE IF NOT EXISTS workspace_persistent_session (
    id TEXT PRIMARY KEY NOT NULL CHECK (id = 'singleton'),
    schema_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL,
    active_workspace_id TEXT,
    last_observation_pass_id TEXT,
    last_capture_timestamp TEXT,
    last_confidence_band TEXT,
    last_saved_context_id TEXT,
    last_active_monitor_index INTEGER,
    restore_history_json TEXT NOT NULL DEFAULT '[]',
    -- Integrity aid for corruption detection (fnv1a hex of restore_history_json + key fields).
    payload_checksum TEXT NOT NULL DEFAULT ''
);
