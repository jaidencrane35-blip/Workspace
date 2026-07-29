/**
 * Evidence dependency projection helpers.
 * React remains projection-only — dependency surface, never creation or authority.
 * dependency ≠ causation; relationship ≠ execution; observation ≠ scheduling.
 */
import type {
  EvidenceDependencyHistoryEntry,
  WorkspaceEvidenceDependencyProjection,
  WorkspaceEvidenceDependencySummary,
} from "../types/domain";

const TERMINAL_EVIDENCE_DEPENDENCY_STATUSES = new Set(["superseded", "archived"]);

export function isEvidenceDependencyHistoryNonActionable(
  entry: EvidenceDependencyHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EVIDENCE_DEPENDENCY_STATUSES.has(entry.status)
  );
}

export function isEvidenceDependencyProjectionNonCommandable(
  projection: WorkspaceEvidenceDependencyProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isEvidenceDependencyHistoryNonActionable) &&
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
        projection.current.nodes.every(
          (n) => n.actionable === false && n.authority_effect === "none"
        ) &&
        projection.current.relationships.every(
          (r) => r.actionable === false && r.authority_effect === "none"
        ) &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        )))
  );
}

export function evidenceDependencyHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceDependencySummary
): boolean {
  return summary.history_count >= summary.history.length;
}
