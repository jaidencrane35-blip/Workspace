-- PP-P01E — Consented local pilot measurement (evaluation data, not product Memory).
-- Records exist only after explicit consent to pilot-measurement-scope-v1.
-- Nothing is uploaded; no ambient observation.

CREATE TABLE IF NOT EXISTS pilot_consent (
    id TEXT PRIMARY KEY NOT NULL,
    scope_id TEXT NOT NULL,
    consented_at TEXT NOT NULL,
    withdrawn_at TEXT
);

CREATE TABLE IF NOT EXISTS pilot_records (
    id TEXT PRIMARY KEY NOT NULL,
    kind TEXT NOT NULL,
    recorded_at TEXT NOT NULL,
    local_day TEXT NOT NULL,
    payload_json TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_pilot_records_kind_day
    ON pilot_records (kind, local_day);
