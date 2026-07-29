/**
 * Evidence coverage projection helpers.
 * React remains projection-only — completeness surface, never truth or authority.
 * coverage ≠ correctness; completeness ≠ confidence; gap ≠ recommendation.
 */
import type {
  EvidenceCoverageHistoryEntry,
  WorkspaceEvidenceCoverageProjection,
  WorkspaceEvidenceCoverageSummary,
} from "../types/domain";

const TERMINAL_EVIDENCE_COVERAGE_STATUSES = new Set(["superseded", "archived"]);

export function isEvidenceCoverageHistoryNonActionable(
  entry: EvidenceCoverageHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EVIDENCE_COVERAGE_STATUSES.has(entry.status)
  );
}

export function isEvidenceCoverageProjectionNonCommandable(
  projection: WorkspaceEvidenceCoverageProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isEvidenceCoverageHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.assessment.actionable === false &&
        projection.current.assessment.authority_effect === "none" &&
        projection.current.metrics.actionable === false &&
        projection.current.metrics.authority_effect === "none" &&
        projection.current.diagnostics.actionable === false &&
        projection.current.diagnostics.authority_effect === "none" &&
        projection.current.lineage.actionable === false &&
        projection.current.lineage.authority_effect === "none" &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        )))
  );
}

export function evidenceCoverageHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceCoverageSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
