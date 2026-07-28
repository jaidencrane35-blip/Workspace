/**
 * Knowledge integration projection helpers.
 * React remains projection-only — retrieval/composition evidence, never authority.
 * integration ≠ authority; retrieval ≠ truth; relevance ≠ correctness; confidence ≠ permission.
 */
import type {
  KnowledgeIntegrationHistoryEntry,
  KnowledgeIntegrationProjection,
  KnowledgeIntegrationSummary,
} from "../types/domain";

const TERMINAL_INTEGRATION_STATUSES = new Set(["superseded", "archived"]);

export function isKnowledgeIntegrationHistoryNonActionable(
  entry: KnowledgeIntegrationHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_INTEGRATION_STATUSES.has(entry.status)
  );
}

export function isKnowledgeIntegrationProjectionNonCommandable(
  projection: KnowledgeIntegrationProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isKnowledgeIntegrationHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.links.every(
          (l) =>
            l.actionable === false &&
            l.authority_effect === "none" &&
            l.evidence_refs.length > 0
        ) &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        ) &&
        projection.current.confidence.actionable === false &&
        projection.current.confidence.authority_effect === "none"))
  );
}

export function knowledgeIntegrationHistoryCountIsAuthoritative(
  summary: KnowledgeIntegrationSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
