-- Sprint 09: intent and capability attribution on audit records.

ALTER TABLE audit_events ADD COLUMN intent_type TEXT;
ALTER TABLE audit_events ADD COLUMN capability TEXT;

CREATE INDEX IF NOT EXISTS idx_audit_events_intent_type ON audit_events(intent_type);
