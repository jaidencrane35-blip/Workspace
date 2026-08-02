-- Restore identity for Product Proof PP-M1-02.
--
-- Nullable columns keep contexts saved under scope v1 loadable. Those rows are
-- never backfilled; Resume reports them as unsupported per item.
-- Either every identity field is present, or restore_identity_unavailable_reason
-- explains why the window cannot be restored. Partial identity is forbidden.

ALTER TABLE saved_context_windows ADD COLUMN identity_schema_version TEXT;
ALTER TABLE saved_context_windows ADD COLUMN desktop_session_id TEXT;
ALTER TABLE saved_context_windows ADD COLUMN captured_hwnd TEXT;
ALTER TABLE saved_context_windows ADD COLUMN title_fingerprint TEXT;
ALTER TABLE saved_context_windows ADD COLUMN restore_identity_unavailable_reason TEXT;
