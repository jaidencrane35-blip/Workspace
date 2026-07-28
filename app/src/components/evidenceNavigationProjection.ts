/**
 * Evidence navigation projection helpers.
 * React remains projection-only — navigation surface, never interpretation or authority.
 * navigation ≠ interpretation; path ≠ inference; traversal ≠ reasoning.
 */
import type {
  EvidenceNavigationHistoryEntry,
  WorkspaceEvidenceNavigationProjection,
  WorkspaceEvidenceNavigationSummary,
} from "../types/domain";

const TERMINAL_EVIDENCE_NAVIGATION_STATUSES = new Set(["superseded", "archived"]);

export function isEvidenceNavigationHistoryNonActionable(
  entry: EvidenceNavigationHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EVIDENCE_NAVIGATION_STATUSES.has(entry.status)
  );
}

export function isEvidenceNavigationProjectionNonCommandable(
  projection: WorkspaceEvidenceNavigationProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isEvidenceNavigationHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.session.executable === false &&
        projection.current.session.actionable === false &&
        projection.current.paths.every(
          (p) =>
            p.actionable === false &&
            p.authority_effect === "none" &&
            p.evidence_lineage.length > 0
        ) &&
        projection.current.summary.actionable === false &&
        projection.current.summary.authority_effect === "none" &&
        projection.current.lineage.actionable === false &&
        projection.current.lineage.authority_effect === "none" &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        )))
  );
}

export function evidenceNavigationHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceNavigationSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
