/**
 * Unified Workspace State Envelope projection helpers.
 * React remains projection-only — composition evidence, no mutation or authority.
 */
import type {
  WorkspaceStateHistoryEntry,
  WorkspaceStateSnapshot,
  WorkspaceStateSummary,
} from "../types/domain";

const TERMINAL_ENVELOPE_STATUSES = new Set(["superseded", "archived"]);

export function isWorkspaceStateHistoryNonActionable(
  entry: WorkspaceStateHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_ENVELOPE_STATUSES.has(entry.status)
  );
}

export function isWorkspaceStateSnapshotNonCommandable(
  snapshot: WorkspaceStateSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isWorkspaceStateHistoryNonActionable) &&
    (snapshot.current == null ||
      (snapshot.current.authority_effect === "none" &&
        snapshot.current.actionable === false &&
        snapshot.current.terminal === false &&
        snapshot.current.sources.every(
          (s) => s.actionable === false && s.authority_effect === "none"
        ) &&
        snapshot.current.contradictions.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        )))
  );
}

export function workspaceStateHistoryCountIsAuthoritative(
  summary: WorkspaceStateSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
