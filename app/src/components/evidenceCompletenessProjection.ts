/**
 * Evidence completeness projection helpers.
 * React remains projection-only — completeness surface, never repair or authority.
 * completeness ≠ truth; observation ≠ repair; partial ≠ fill request.
 */
import type {
  EvidenceCompletenessHistoryEntry,
  WorkspaceEvidenceCompletenessProjection,
  WorkspaceEvidenceCompletenessSummary,
} from "../types/domain";

const TERMINAL_EVIDENCE_COMPLETENESS_STATUSES = new Set(["superseded", "archived"]);

export function isEvidenceCompletenessHistoryNonActionable(
  entry: EvidenceCompletenessHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EVIDENCE_COMPLETENESS_STATUSES.has(entry.status)
  );
}

export function isEvidenceCompletenessProjectionNonCommandable(
  projection: WorkspaceEvidenceCompletenessProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isEvidenceCompletenessHistoryNonActionable) &&
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
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        )))
  );
}

export function evidenceCompletenessHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceCompletenessSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
