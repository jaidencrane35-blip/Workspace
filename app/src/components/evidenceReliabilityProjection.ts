/**
 * Evidence reliability projection helpers.
 * React remains projection-only — reliability surface, never repair or authority.
 * reliability ≠ truth; observation ≠ repair; partial ≠ fill request.
 */
import type {
  EvidenceReliabilityHistoryEntry,
  WorkspaceEvidenceReliabilityProjection,
  WorkspaceEvidenceReliabilitySummary,
} from "../types/domain";

const TERMINAL_EVIDENCE_RELIABILITY_STATUSES = new Set(["superseded", "archived"]);

export function isEvidenceReliabilityHistoryNonActionable(
  entry: EvidenceReliabilityHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EVIDENCE_RELIABILITY_STATUSES.has(entry.status)
  );
}

export function isEvidenceReliabilityProjectionNonCommandable(
  projection: WorkspaceEvidenceReliabilityProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isEvidenceReliabilityHistoryNonActionable) &&
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

export function evidenceReliabilityHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceReliabilitySummary
): boolean {
  return summary.history_count >= summary.history.length;
}
