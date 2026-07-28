/**
 * Workspace explanation projection helpers.
 * React remains projection-only — evidence synthesis, never authority or command conversion.
 */
import type {
  WorkspaceExplanationHistoryEntry,
  WorkspaceExplanationSnapshot,
  WorkspaceExplanationSummary,
} from "../types/domain";

const TERMINAL_EXPLANATION_STATUSES = new Set(["superseded", "archived"]);

export function isWorkspaceExplanationHistoryNonActionable(
  entry: WorkspaceExplanationHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EXPLANATION_STATUSES.has(entry.status)
  );
}

export function isWorkspaceExplanationSnapshotNonCommandable(
  snapshot: WorkspaceExplanationSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isWorkspaceExplanationHistoryNonActionable) &&
    (snapshot.current == null ||
      (snapshot.current.authority_effect === "none" &&
        snapshot.current.actionable === false &&
        snapshot.current.terminal === false &&
        snapshot.current.sections.every(
          (s) => s.actionable === false && s.authority_effect === "none"
        ) &&
        snapshot.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        ) &&
        snapshot.current.conflicts.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        ) &&
        snapshot.current.confidence.actionable === false &&
        snapshot.current.confidence.authority_effect === "none"))
  );
}

export function workspaceExplanationHistoryCountIsAuthoritative(
  summary: WorkspaceExplanationSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
