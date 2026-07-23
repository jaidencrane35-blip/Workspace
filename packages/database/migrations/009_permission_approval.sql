-- Sprint 43: permission approval requests + allow-once capability grants.

CREATE TABLE IF NOT EXISTS permission_approval_requests (
    id TEXT PRIMARY KEY NOT NULL,
    created_at TEXT NOT NULL,
    status TEXT NOT NULL,
    requesting_actor_type TEXT NOT NULL,
    requesting_actor_id TEXT NOT NULL,
    command_name TEXT NOT NULL,
    capability TEXT NOT NULL,
    subject TEXT NOT NULL,
    intent_type TEXT NOT NULL,
    reason TEXT NOT NULL,
    decided_at TEXT,
    decided_by_actor_id TEXT
);

CREATE INDEX IF NOT EXISTS idx_permission_approvals_status
    ON permission_approval_requests(status);
CREATE INDEX IF NOT EXISTS idx_permission_approvals_actor
    ON permission_approval_requests(requesting_actor_id, status);

CREATE TABLE IF NOT EXISTS capability_grants (
    id TEXT PRIMARY KEY NOT NULL,
    approval_request_id TEXT NOT NULL,
    grantee_actor_id TEXT NOT NULL,
    capability TEXT NOT NULL,
    command_name TEXT,
    grant_kind TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    consumed_at TEXT,
    FOREIGN KEY (approval_request_id) REFERENCES permission_approval_requests(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_capability_grants_active
    ON capability_grants(grantee_actor_id, status);
