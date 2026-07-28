/**
 * Cross-workspace intelligence projection helpers.
 * React remains projection-only — aggregate understanding, never authority.
 * aggregation ≠ authority; statistics ≠ recommendations.
 */
import type {
  CrossWorkspaceIntelligenceHistoryEntry,
  CrossWorkspaceIntelligenceProjection,
  CrossWorkspaceIntelligenceSummary,
} from "../types/domain";

const TERMINAL_STATUSES = new Set(["superseded", "archived"]);

export function isCrossWorkspaceIntelligenceHistoryNonActionable(
  entry: CrossWorkspaceIntelligenceHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_STATUSES.has(entry.status)
  );
}

export function isCrossWorkspaceIntelligenceProjectionNonCommandable(
  projection: CrossWorkspaceIntelligenceProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isCrossWorkspaceIntelligenceHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.patterns.every(
          (p) =>
            p.actionable === false &&
            p.authority_effect === "none" &&
            p.evidence_references.length > 0
        ) &&
        projection.current.themes.every(
          (t) =>
            t.actionable === false &&
            t.authority_effect === "none" &&
            t.supporting_evidence.length > 0
        ) &&
        projection.current.risk_signals.every(
          (r) => r.actionable === false && r.authority_effect === "none"
        ) &&
        projection.current.constraint_patterns.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        ) &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        ) &&
        projection.current.assessment.actionable === false &&
        projection.current.assessment.authority_effect === "none"))
  );
}

export function crossWorkspaceIntelligenceHistoryCountIsAuthoritative(
  summary: CrossWorkspaceIntelligenceSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
