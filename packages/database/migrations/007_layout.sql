-- Sprint 13: workspace spatial layout (presentation layer, separate from graph topology).

CREATE TABLE IF NOT EXISTS layouts (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL UNIQUE,
    viewport_origin_x REAL NOT NULL DEFAULT 0,
    viewport_origin_y REAL NOT NULL DEFAULT 0,
    viewport_width REAL NOT NULL DEFAULT 1920,
    viewport_height REAL NOT NULL DEFAULT 1080,
    viewport_zoom REAL NOT NULL DEFAULT 1,
    metadata TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS layout_nodes (
    layout_id TEXT NOT NULL,
    resource_kind TEXT NOT NULL,
    resource_id TEXT NOT NULL,
    position_x REAL NOT NULL DEFAULT 0,
    position_y REAL NOT NULL DEFAULT 0,
    size_width REAL NOT NULL DEFAULT 100,
    size_height REAL NOT NULL DEFAULT 100,
    z_index INTEGER NOT NULL DEFAULT 0,
    collapsed INTEGER NOT NULL DEFAULT 0,
    hidden INTEGER NOT NULL DEFAULT 0,
    locked INTEGER NOT NULL DEFAULT 0,
    metadata TEXT,
    PRIMARY KEY (layout_id, resource_kind, resource_id),
    FOREIGN KEY (layout_id) REFERENCES layouts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_layout_nodes_layout_id ON layout_nodes(layout_id);
