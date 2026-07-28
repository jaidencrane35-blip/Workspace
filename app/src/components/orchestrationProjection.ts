/**
 * Cognitive Orchestration projection helpers.
 * React remains projection-only — coordination evidence, no mutation.
 */
import type {
  OrchestrationHistoryEntry,
  WorkspaceOrchestrationSnapshot,
  WorkspaceOrchestrationSummary,
} from "../types/domain";

const TERMINAL_ORCHESTRATION_STATUSES = new Set(["superseded", "archived"]);

export function isOrchestrationHistoryNonActionable(
  entry: OrchestrationHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_ORCHESTRATION_STATUSES.has(entry.status)
  );
}

export function isOrchestrationSnapshotNonCommandable(
  snapshot: WorkspaceOrchestrationSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isOrchestrationHistoryNonActionable) &&
    (snapshot.current == null ||
      (snapshot.current.authority_effect === "none" &&
        snapshot.current.meta.actionable === false &&
        snapshot.current.meta.terminal === false))
  );
}

export function orchestrationHistoryCountIsAuthoritative(
  summary: WorkspaceOrchestrationSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
