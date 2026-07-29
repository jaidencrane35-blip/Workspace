/**
 * Evidence freshness projection helpers.
 * React remains projection-only — freshness surface, never refresh or authority.
 * freshness ≠ validity; observation ≠ regeneration; stale ≠ refresh request.
 */
import type {
  EvidenceFreshnessHistoryEntry,
  WorkspaceEvidenceFreshnessProjection,
  WorkspaceEvidenceFreshnessSummary,
} from "../types/domain";

const TERMINAL_EVIDENCE_FRESHNESS_STATUSES = new Set(["superseded", "archived"]);

export function isEvidenceFreshnessHistoryNonActionable(
  entry: EvidenceFreshnessHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EVIDENCE_FRESHNESS_STATUSES.has(entry.status)
  );
}

export function isEvidenceFreshnessProjectionNonCommandable(
  projection: WorkspaceEvidenceFreshnessProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isEvidenceFreshnessHistoryNonActionable) &&
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

export function evidenceFreshnessHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceFreshnessSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
