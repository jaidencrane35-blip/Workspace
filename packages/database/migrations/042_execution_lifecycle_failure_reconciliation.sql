-- Forward-only expansion of immutable migration 041.
ALTER TABLE execution_lifecycle RENAME TO execution_lifecycle_041;

CREATE TABLE execution_lifecycle (
    execution_request_id TEXT PRIMARY KEY NOT NULL,
    suggestion_id TEXT NOT NULL,
    intent_id TEXT,
    state TEXT NOT NULL CHECK (state IN ('in_progress', 'completed', 'failed', 'cancelled')),
    retry_allowed INTEGER NOT NULL DEFAULT 0 CHECK (retry_allowed IN (0, 1)),
    failure_reason TEXT,
    claimed_at TEXT NOT NULL,
    completed_at TEXT,
    updated_at TEXT NOT NULL,
    CHECK (
        (state = 'completed' AND completed_at IS NOT NULL AND retry_allowed = 0)
        OR (state = 'failed' AND completed_at IS NULL AND failure_reason IS NOT NULL)
        OR (state = 'cancelled' AND completed_at IS NULL AND retry_allowed = 1)
        OR (state = 'in_progress' AND completed_at IS NULL AND retry_allowed = 0)
    )
);

INSERT INTO execution_lifecycle (
    execution_request_id, suggestion_id, intent_id, state, retry_allowed,
    failure_reason, claimed_at, completed_at, updated_at
)
SELECT
    execution_request_id,
    suggestion_id,
    intent_id,
    state,
    CASE
        WHEN state = 'cancelled' THEN 1
        ELSE 0
    END,
    CASE
        WHEN state = 'failed' THEN 'historical failure requires reconciliation'
        ELSE NULL
    END,
    claimed_at,
    completed_at,
    updated_at
FROM execution_lifecycle_041;

DROP TABLE execution_lifecycle_041;

CREATE INDEX idx_execution_lifecycle_updated
    ON execution_lifecycle (updated_at DESC);
CREATE INDEX idx_execution_lifecycle_stale
    ON execution_lifecycle (state, claimed_at)
    WHERE state = 'in_progress';

INSERT OR IGNORE INTO execution_lifecycle (
    execution_request_id, suggestion_id, intent_id, state, retry_allowed,
    failure_reason, claimed_at, completed_at, updated_at
)
SELECT
    json_extract(metadata, '$.execution_request_id'),
    SUBSTR(json_extract(metadata, '$.execution_request_id'), LENGTH('execution:') + 1),
    NULL, 'cancelled', 1, NULL, timestamp, NULL, timestamp
FROM audit_events
WHERE event_type = 'command.executed'
  AND command_name = 'RequestExecutionCancellation'
  AND success = 1
  AND metadata IS NOT NULL
  AND json_valid(metadata)
  AND json_extract(metadata, '$.cancellation_status') IN ('requested', 'approved')
  AND NULLIF(TRIM(json_extract(metadata, '$.execution_request_id')), '') IS NOT NULL;

INSERT OR IGNORE INTO execution_lifecycle (
    execution_request_id, suggestion_id, intent_id, state, retry_allowed,
    failure_reason, claimed_at, completed_at, updated_at
)
SELECT
    json_extract(metadata, '$.execution_request_id'),
    json_extract(metadata, '$.suggestion_id'),
    json_extract(metadata, '$.intent_id'),
    'failed', 1, 'historical dispatch failure', timestamp, NULL, timestamp
FROM audit_events
WHERE event_type = 'command.failed'
  AND command_name = 'ExecuteIntentRequest'
  AND metadata IS NOT NULL
  AND json_valid(metadata)
  AND json_extract(metadata, '$.execution_request') = 1
  AND NULLIF(TRIM(json_extract(metadata, '$.execution_request_id')), '') IS NOT NULL
  AND NULLIF(TRIM(json_extract(metadata, '$.suggestion_id')), '') IS NOT NULL;
