/**
 * Contextual workspace understanding projection helpers.
 * React remains projection-only — situational framing, never authority or command conversion.
 */
import type {
  ContextualUnderstandingHistoryEntry,
  ContextualUnderstandingProjection,
  ContextualUnderstandingSummary,
} from "../types/domain";

const TERMINAL_CONTEXTUAL_STATUSES = new Set(["superseded", "archived"]);

export function isContextualUnderstandingHistoryNonActionable(
  entry: ContextualUnderstandingHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_CONTEXTUAL_STATUSES.has(entry.status)
  );
}

export function isContextualUnderstandingProjectionNonCommandable(
  projection: ContextualUnderstandingProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isContextualUnderstandingHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.themes.every(
          (t) => t.actionable === false && t.authority_effect === "none"
        ) &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        ) &&
        projection.current.confidence.actionable === false &&
        projection.current.confidence.authority_effect === "none"))
  );
}

export function contextualUnderstandingHistoryCountIsAuthoritative(
  summary: ContextualUnderstandingSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
