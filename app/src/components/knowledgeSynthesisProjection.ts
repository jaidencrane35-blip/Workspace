/**
 * Knowledge synthesis projection helpers.
 * React remains projection-only — derived concepts, never Memory/SoT/authority.
 */
import type {
  KnowledgeSynthesisHistoryEntry,
  KnowledgeSynthesisProjection,
  KnowledgeSynthesisSummary,
} from "../types/domain";

const TERMINAL_KNOWLEDGE_STATUSES = new Set(["superseded", "archived"]);

export function isKnowledgeSynthesisHistoryNonActionable(
  entry: KnowledgeSynthesisHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_KNOWLEDGE_STATUSES.has(entry.status)
  );
}

export function isKnowledgeSynthesisProjectionNonCommandable(
  projection: KnowledgeSynthesisProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isKnowledgeSynthesisHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.concepts.every(
          (c) =>
            c.actionable === false &&
            c.authority_effect === "none" &&
            c.evidence_refs.length > 0
        ) &&
        projection.current.clusters.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        ) &&
        projection.current.relationships.every(
          (r) => r.actionable === false && r.authority_effect === "none"
        ) &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        ) &&
        projection.current.confidence.actionable === false &&
        projection.current.confidence.authority_effect === "none"))
  );
}

export function knowledgeSynthesisHistoryCountIsAuthoritative(
  summary: KnowledgeSynthesisSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
