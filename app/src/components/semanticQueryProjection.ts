/**
 * Semantic query projection helpers.
 * React remains projection-only — retrieval surface, never synthesis or authority.
 * retrieval ≠ synthesis; match ≠ recommendation; lineage ≠ inferred provenance.
 */
import type {
  SemanticQueryHistoryEntry,
  WorkspaceSemanticQueryProjection,
  WorkspaceSemanticQuerySummary,
} from "../types/domain";

const TERMINAL_SEMANTIC_QUERY_STATUSES = new Set(["superseded", "archived"]);

export function isSemanticQueryHistoryNonActionable(
  entry: SemanticQueryHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_SEMANTIC_QUERY_STATUSES.has(entry.status)
  );
}

export function isSemanticQueryProjectionNonCommandable(
  projection: WorkspaceSemanticQueryProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isSemanticQueryHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.query.executable === false &&
        projection.current.query.actionable === false &&
        projection.current.result.actionable === false &&
        projection.current.result.authority_effect === "none" &&
        projection.current.result.matches.every(
          (m) =>
            m.actionable === false &&
            m.authority_effect === "none" &&
            m.evidence_ref.external_ref.length > 0
        ) &&
        projection.current.lineage.actionable === false &&
        projection.current.lineage.authority_effect === "none" &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        ) &&
        projection.current.diagnostics.actionable === false &&
        projection.current.diagnostics.authority_effect === "none"))
  );
}

export function semanticQueryHistoryCountIsAuthoritative(
  summary: WorkspaceSemanticQuerySummary
): boolean {
  return summary.history_count >= summary.history.length;
}
