/**
 * Insight coordination projection helpers.
 * React remains projection-only — understanding coordination, never authority.
 * coordination ≠ authority; prioritisation ≠ recommendation; intersection ≠ causation.
 */
import type {
  InsightCoordinationHistoryEntry,
  InsightCoordinationProjection,
  InsightCoordinationSummary,
} from "../types/domain";

const TERMINAL_COORDINATION_STATUSES = new Set(["superseded", "archived"]);

export function isInsightCoordinationHistoryNonActionable(
  entry: InsightCoordinationHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_COORDINATION_STATUSES.has(entry.status)
  );
}

export function isInsightCoordinationProjectionNonCommandable(
  projection: InsightCoordinationProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isInsightCoordinationHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.clusters.every(
          (c) =>
            c.actionable === false &&
            c.authority_effect === "none" &&
            c.evidence_references.length > 0
        ) &&
        projection.current.intersections.every(
          (i) =>
            i.actionable === false &&
            i.authority_effect === "none" &&
            i.source_refs.length > 0
        ) &&
        projection.current.attention_signals.every(
          (s) => s.actionable === false && s.authority_effect === "none"
        ) &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        ) &&
        projection.current.assessment.actionable === false &&
        projection.current.assessment.authority_effect === "none"))
  );
}

export function insightCoordinationHistoryCountIsAuthoritative(
  summary: InsightCoordinationSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
