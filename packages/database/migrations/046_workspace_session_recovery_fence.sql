-- Session recovery fences (schema v2 durable fields).
-- pending_operation_json: in-flight mutation marker for crash detection.
-- last_recovery_json: acknowledged interruption (never silently discarded).

ALTER TABLE workspace_persistent_session ADD COLUMN pending_operation_json TEXT;
ALTER TABLE workspace_persistent_session ADD COLUMN last_recovery_json TEXT;
