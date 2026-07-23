-- Explicit user preferences for governed personalization (DEC-014 adjacent).
-- Informational only — never authority. No automatic inference in this batch.
CREATE TABLE IF NOT EXISTS user_preferences (
    id TEXT PRIMARY KEY NOT NULL,
    category TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    label TEXT,
    source TEXT NOT NULL,
    confidence INTEGER NOT NULL DEFAULT 4,
    scope_workspace_id TEXT,
    editable INTEGER NOT NULL DEFAULT 1,
    attributes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_user_preferences_category_deleted
    ON user_preferences (category, deleted);

CREATE INDEX IF NOT EXISTS idx_user_preferences_workspace
    ON user_preferences (scope_workspace_id);
