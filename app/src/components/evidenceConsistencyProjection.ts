/**
 * Evidence consistency projection helpers.
 * React remains projection-only — consistency surface, never resolution or authority.
 * consistency ≠ correctness; observation ≠ judgement; conflict ≠ resolution.
 */
import type {
  EvidenceConsistencyHistoryEntry,
  WorkspaceEvidenceConsistencyProjection,
  WorkspaceEvidenceConsistencySummary,
} from "../types/domain";

const TERMINAL_EVIDENCE_CONSISTENCY_STATUSES = new Set(["superseded", "archived"]);

export function isEvidenceConsistencyHistoryNonActionable(
  entry: EvidenceConsistencyHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EVIDENCE_CONSISTENCY_STATUSES.has(entry.status)
  );
}

export function isEvidenceConsistencyProjectionNonCommandable(
  projection: WorkspaceEvidenceConsistencyProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isEvidenceConsistencyHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.assessment.actionable === false &&
        projection.current.assessment.authority_effect === "none" &&
        projection.current.diagnostics.actionable === false &&
        projection.current.diagnostics.authority_effect === "none" &&
        projection.current.lineage.actionable === false &&
        projection.current.lineage.authority_effect === "none" &&
        projection.current.observations.every(
          (o) => o.actionable === false && o.authority_effect === "none"
        ) &&
        projection.current.conflicts.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        ) &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        )))
  );
}

export function evidenceConsistencyHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceConsistencySummary
): boolean {
  return summary.history_count >= summary.history.length;
}
