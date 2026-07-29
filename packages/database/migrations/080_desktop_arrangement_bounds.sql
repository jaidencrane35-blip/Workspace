-- DAF-1d: optional restore geometry on arrangement membership entries.
-- Capture may store observed bounds; restore uses them via WindowController.
-- Does not grant authority; apply remains kernel + PermissionGateway owned.

ALTER TABLE desktop_arrangement_entries ADD COLUMN x INTEGER;
ALTER TABLE desktop_arrangement_entries ADD COLUMN y INTEGER;
ALTER TABLE desktop_arrangement_entries ADD COLUMN width INTEGER;
ALTER TABLE desktop_arrangement_entries ADD COLUMN height INTEGER;
