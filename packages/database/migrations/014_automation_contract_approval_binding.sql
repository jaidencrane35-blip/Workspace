-- Batch 6.5: bind contract approval to an exact definition fingerprint.
-- Approval attribution fields — never execution authority.

ALTER TABLE automation_contracts ADD COLUMN approved_by_actor TEXT;
ALTER TABLE automation_contracts ADD COLUMN approved_at TEXT;
ALTER TABLE automation_contracts ADD COLUMN approved_definition_fingerprint TEXT;
