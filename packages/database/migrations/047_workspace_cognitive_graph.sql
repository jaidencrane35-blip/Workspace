-- Workspace Cognitive Graph (Programme II Batch 4).
-- Reference-only integration topology. Never owns domain objects or execution.

CREATE TABLE IF NOT EXISTS cognitive_graph_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('current', 'superseded', 'archived')),
    generated_at TEXT NOT NULL,
    superseded_at TEXT,
    node_count INTEGER NOT NULL DEFAULT 0,
    edge_count INTEGER NOT NULL DEFAULT 0,
    broken_node_count INTEGER NOT NULL DEFAULT 0,
    broken_edge_count INTEGER NOT NULL DEFAULT 0,
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none')
);

CREATE INDEX IF NOT EXISTS idx_cognitive_graph_snapshots_workspace
    ON cognitive_graph_snapshots (workspace_id, generated_at DESC);

CREATE INDEX IF NOT EXISTS idx_cognitive_graph_snapshots_status
    ON cognitive_graph_snapshots (workspace_id, status);

CREATE TABLE IF NOT EXISTS cognitive_graph_nodes (
    snapshot_id TEXT NOT NULL,
    external_ref TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    title TEXT NOT NULL,
    broken INTEGER NOT NULL DEFAULT 0 CHECK (broken IN (0, 1)),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    PRIMARY KEY (snapshot_id, external_ref),
    FOREIGN KEY (snapshot_id) REFERENCES cognitive_graph_snapshots(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_cognitive_graph_nodes_workspace
    ON cognitive_graph_nodes (workspace_id, kind);

CREATE TABLE IF NOT EXISTS cognitive_graph_edges (
    id TEXT PRIMARY KEY NOT NULL,
    snapshot_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    from_ref TEXT NOT NULL,
    to_ref TEXT NOT NULL,
    kind TEXT NOT NULL,
    explanation TEXT NOT NULL DEFAULT '',
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    evidence_refs_json TEXT NOT NULL DEFAULT '[]',
    broken INTEGER NOT NULL DEFAULT 0 CHECK (broken IN (0, 1)),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    CHECK (from_ref != to_ref),
    FOREIGN KEY (snapshot_id) REFERENCES cognitive_graph_snapshots(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_cognitive_graph_edges_snapshot
    ON cognitive_graph_edges (snapshot_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_cognitive_graph_edges_dedup
    ON cognitive_graph_edges (snapshot_id, from_ref, kind, to_ref);

-- Append-only terminal evidence. Snapshot cascade does not erase history rows.
CREATE TABLE IF NOT EXISTS cognitive_graph_history (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    snapshot_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('superseded', 'archived')),
    generated_at TEXT NOT NULL,
    superseded_at TEXT,
    node_count INTEGER NOT NULL DEFAULT 0,
    edge_count INTEGER NOT NULL DEFAULT 0,
    broken_node_count INTEGER NOT NULL DEFAULT 0,
    broken_edge_count INTEGER NOT NULL DEFAULT 0,
    terminal INTEGER NOT NULL DEFAULT 1 CHECK (terminal = 1),
    actionable INTEGER NOT NULL DEFAULT 0 CHECK (actionable = 0),
    authority_effect TEXT NOT NULL DEFAULT 'none' CHECK (authority_effect = 'none'),
    recorded_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cognitive_graph_history_workspace
    ON cognitive_graph_history (workspace_id, recorded_at DESC);
