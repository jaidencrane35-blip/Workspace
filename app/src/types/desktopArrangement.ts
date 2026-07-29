export type DesktopArrangementStatus = "draft" | "active" | "archived";

export type ObservedWindowAvailability =
  | "available"
  | "identity_known_window_missing"
  | "unavailable";

export type DesktopArrangementApplyStatus =
  | "applied"
  | "failed"
  | "skipped";

export interface DesktopArrangementEntry {
  id: string;
  arrangement_id: string;
  stable_window_id: string | null;
  hwnd: string | null;
  process_id: number | null;
  process_name: string | null;
  title_fingerprint: string | null;
  label: string;
  sort_order: number;
  x: number | null;
  y: number | null;
  width: number | null;
  height: number | null;
  authority_effect: string;
}

export interface DesktopArrangement {
  id: string;
  workspace_id: string;
  name: string;
  description: string;
  status: DesktopArrangementStatus;
  entries: DesktopArrangementEntry[];
  created_at: string;
  updated_at: string;
  authority_effect: string;
}

export interface DesktopArrangementEntryDiagnostic {
  entry_id: string;
  label: string;
  stable_window_id: string | null;
  hwnd: string | null;
  availability: ObservedWindowAvailability;
  detail: string;
  restore_gap: boolean;
  gap_reason: string | null;
}

export interface DesktopArrangementApplyOutcome {
  entry_id: string;
  label: string;
  hwnd: string;
  status: DesktopArrangementApplyStatus;
  detail: string;
  simulated: boolean;
}

export interface DesktopArrangementRestoreResult {
  arrangement_id: string;
  diagnostics: DesktopArrangementEntryDiagnostic[];
  outcomes: DesktopArrangementApplyOutcome[];
  applied_count: number;
  gap_count: number;
  failed_count: number;
}
