-- Governed AI memory entries (DEC-014). Informational only — never authority.
CREATE TABLE IF NOT EXISTS ai_memory_entries (
    id TEXT PRIMARY KEY NOT NULL,
    memory_type TEXT NOT NULL,
    key TEXT NOT NULL,
    summary TEXT NOT NULL,
    source TEXT NOT NULL,
    workspace_id TEXT,
    lifecycle TEXT NOT NULL,
    confidence_level INTEGER NOT NULL DEFAULT 2,
    occurrence_count INTEGER NOT NULL DEFAULT 1,
    user_visible INTEGER NOT NULL DEFAULT 1,
    attributes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ai_memory_type_lifecycle
    ON ai_memory_entries (memory_type, lifecycle);

CREATE INDEX IF NOT EXISTS idx_ai_memory_workspace
    ON ai_memory_entries (workspace_id);
