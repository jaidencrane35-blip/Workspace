-- Sprint 12: passive workspace graph (nodes + edges) and audit resource attribution.

CREATE TABLE IF NOT EXISTS graph_nodes (
    kind TEXT NOT NULL,
    id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (kind, id)
);

CREATE TABLE IF NOT EXISTS graph_edges (
    id TEXT PRIMARY KEY NOT NULL,
    source_kind TEXT NOT NULL,
    source_id TEXT NOT NULL,
    relationship TEXT NOT NULL,
    target_kind TEXT NOT NULL,
    target_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (source_kind, source_id) REFERENCES graph_nodes(kind, id) ON DELETE CASCADE,
    FOREIGN KEY (target_kind, target_id) REFERENCES graph_nodes(kind, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_graph_edges_source ON graph_edges(source_kind, source_id);
CREATE INDEX IF NOT EXISTS idx_graph_edges_target ON graph_edges(target_kind, target_id);

ALTER TABLE audit_events ADD COLUMN resource_ref TEXT;
