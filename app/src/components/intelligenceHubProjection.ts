/**
 * Intelligence hub projection helpers.
 * React remains projection-only — aggregation surface, never upstream authority.
 * aggregation ≠ reinterpretation; hub package ≠ new SoT; conflict record ≠ resolution.
 */
import type {
  IntelligenceHubHistoryEntry,
  WorkspaceIntelligenceHubProjection,
  WorkspaceIntelligenceHubSummary,
} from "../types/domain";

const TERMINAL_INTELLIGENCE_HUB_STATUSES = new Set(["superseded", "archived"]);

export function isIntelligenceHubHistoryNonActionable(
  entry: IntelligenceHubHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_INTELLIGENCE_HUB_STATUSES.has(entry.status)
  );
}

export function isIntelligenceHubProjectionNonCommandable(
  projection: WorkspaceIntelligenceHubProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isIntelligenceHubHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.packages.every(
          (p) =>
            p.actionable === false &&
            p.authority_effect === "none" &&
            (p.availability !== "available" || p.evidence_refs.length > 0)
        ) &&
        projection.current.summary.actionable === false &&
        projection.current.summary.authority_effect === "none" &&
        projection.current.lineage.actionable === false &&
        projection.current.lineage.authority_effect === "none" &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        ) &&
        projection.current.conflicts.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        ) &&
        projection.current.assessment.actionable === false &&
        projection.current.assessment.authority_effect === "none"))
  );
}

export function intelligenceHubHistoryCountIsAuthoritative(
  summary: WorkspaceIntelligenceHubSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
