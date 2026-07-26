-- Durable user-owned Workspace Environment Profiles (Phase 6).
-- Describes preferred setups. Never automation, restoration, or authority.
-- References existing entities; does not duplicate Task/Memory/Permission ownership.

CREATE TABLE IF NOT EXISTS workspace_profiles (
    id TEXT PRIMARY KEY NOT NULL,
    workspace_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_workspace_profiles_workspace
    ON workspace_profiles (workspace_id, deleted);

CREATE TABLE IF NOT EXISTS workspace_profile_members (
    id TEXT PRIMARY KEY NOT NULL,
    profile_id TEXT NOT NULL,
    member_type TEXT NOT NULL,
    reference_id TEXT NOT NULL,
    relationship TEXT NOT NULL,
    evidence TEXT NOT NULL,
    label TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES workspace_profiles(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_workspace_profile_members_profile
    ON workspace_profile_members (profile_id);
