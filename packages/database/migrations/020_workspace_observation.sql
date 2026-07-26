-- Durable system-scoped desktop observation passes (Phase 6 / Sprint 103).
-- Read-only perception boundary. Never window control, automation, or authority.
-- Environment Model aggregates from observation; this layer owns raw desktop truth.

CREATE TABLE IF NOT EXISTS observation_passes (
    id TEXT PRIMARY KEY NOT NULL,
    captured_at TEXT NOT NULL,
    schema_version INTEGER NOT NULL DEFAULT 1,
    source TEXT NOT NULL,
    foreground_hwnd TEXT,
    window_count INTEGER NOT NULL DEFAULT 0,
    monitor_count INTEGER NOT NULL DEFAULT 0,
    duration_ms INTEGER,
    metadata_json TEXT NOT NULL DEFAULT '{}'
);

CREATE INDEX IF NOT EXISTS idx_observation_passes_captured
    ON observation_passes (captured_at DESC);

CREATE TABLE IF NOT EXISTS observation_monitors (
    id TEXT PRIMARY KEY NOT NULL,
    pass_id TEXT NOT NULL,
    monitor_index INTEGER NOT NULL,
    name TEXT NOT NULL DEFAULT '',
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    work_x INTEGER NOT NULL,
    work_y INTEGER NOT NULL,
    work_w INTEGER NOT NULL,
    work_h INTEGER NOT NULL,
    is_primary INTEGER NOT NULL DEFAULT 0,
    dpi_scale REAL,
    FOREIGN KEY (pass_id) REFERENCES observation_passes(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_observation_monitors_pass
    ON observation_monitors (pass_id);

CREATE TABLE IF NOT EXISTS observation_windows (
    id TEXT PRIMARY KEY NOT NULL,
    pass_id TEXT NOT NULL,
    hwnd TEXT NOT NULL,
    stable_window_id TEXT,
    title TEXT NOT NULL DEFAULT '',
    process_id INTEGER NOT NULL,
    process_name TEXT,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    monitor_id TEXT,
    visible INTEGER NOT NULL DEFAULT 1,
    minimized INTEGER NOT NULL DEFAULT 0,
    focused INTEGER NOT NULL DEFAULT 0,
    z_order INTEGER,
    FOREIGN KEY (pass_id) REFERENCES observation_passes(id) ON DELETE CASCADE,
    FOREIGN KEY (monitor_id) REFERENCES observation_monitors(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_observation_windows_pass
    ON observation_windows (pass_id);

CREATE TABLE IF NOT EXISTS observation_window_identities (
    id TEXT PRIMARY KEY NOT NULL,
    process_id INTEGER NOT NULL,
    title_fingerprint TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL,
    last_hwnd TEXT NOT NULL,
    confidence TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_observation_window_identities_process
    ON observation_window_identities (process_id, title_fingerprint);
