-- Durable execution claim and terminal completion state.
-- The canonical execution_request_id remains execution:{suggestion_id}.
CREATE TABLE IF NOT EXISTS execution_lifecycle (
    execution_request_id TEXT PRIMARY KEY NOT NULL,
    suggestion_id TEXT NOT NULL,
    intent_id TEXT,
    state TEXT NOT NULL CHECK (state IN ('in_progress', 'completed', 'cancelled')),
    claimed_at TEXT NOT NULL,
    completed_at TEXT,
    updated_at TEXT NOT NULL,
    CHECK (
        (state IN ('in_progress', 'cancelled') AND completed_at IS NULL)
        OR (state = 'completed' AND completed_at IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_execution_lifecycle_updated
    ON execution_lifecycle (updated_at DESC);

-- Preserve terminal outcomes created before this durable lifecycle existed.
INSERT OR IGNORE INTO execution_lifecycle (
    execution_request_id,
    suggestion_id,
    intent_id,
    state,
    claimed_at,
    completed_at,
    updated_at
)
SELECT
    json_extract(metadata, '$.execution_request_id'),
    json_extract(metadata, '$.suggestion_id'),
    json_extract(metadata, '$.intent_id'),
    'completed',
    timestamp,
    timestamp,
    timestamp
FROM audit_events
WHERE event_type = 'command.executed'
  AND command_name = 'ExecuteIntentRequest'
  AND success = 1
  AND metadata IS NOT NULL
  AND json_valid(metadata)
  AND json_extract(metadata, '$.execution_request') = 1
  AND json_extract(metadata, '$.execution_status') = 'executed'
  AND NULLIF(TRIM(json_extract(metadata, '$.execution_request_id')), '') IS NOT NULL
  AND NULLIF(TRIM(json_extract(metadata, '$.suggestion_id')), '') IS NOT NULL;

-- Cancellation is a durable outcome but remains retryable for execution.
INSERT OR IGNORE INTO execution_lifecycle (
    execution_request_id,
    suggestion_id,
    intent_id,
    state,
    claimed_at,
    completed_at,
    updated_at
)
SELECT
    json_extract(metadata, '$.execution_request_id'),
    SUBSTR(
        json_extract(metadata, '$.execution_request_id'),
        LENGTH('execution:') + 1
    ),
    NULL,
    'cancelled',
    timestamp,
    NULL,
    timestamp
FROM audit_events
WHERE event_type = 'command.executed'
  AND command_name = 'RequestExecutionCancellation'
  AND success = 1
  AND metadata IS NOT NULL
  AND json_valid(metadata)
  AND json_extract(metadata, '$.cancellation_status') IN ('requested', 'approved')
  AND NULLIF(TRIM(json_extract(metadata, '$.execution_request_id')), '') IS NOT NULL;
