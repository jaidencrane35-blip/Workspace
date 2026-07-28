/**
 * Decision support projection helpers.
 * React remains projection-only — evidence organisation, never decision authority.
 * support ≠ decision; trade-off ≠ recommendation; comparison ≠ ranking-as-authority.
 */
import type {
  DecisionSupportHistoryEntry,
  WorkspaceDecisionSupportProjection,
  WorkspaceDecisionSupportSummary,
} from "../types/domain";

const TERMINAL_DECISION_SUPPORT_STATUSES = new Set(["superseded", "archived"]);

export function isDecisionSupportHistoryNonActionable(
  entry: DecisionSupportHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_DECISION_SUPPORT_STATUSES.has(entry.status)
  );
}

export function isDecisionSupportProjectionNonCommandable(
  projection: WorkspaceDecisionSupportProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isDecisionSupportHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.contexts.every(
          (c) =>
            c.actionable === false &&
            c.authority_effect === "none" &&
            c.participating_evidence.length > 0
        ) &&
        projection.current.evidence_bundles.every(
          (b) => b.actionable === false && b.authority_effect === "none"
        ) &&
        projection.current.tradeoffs.every(
          (t) => t.actionable === false && t.authority_effect === "none"
        ) &&
        projection.current.dependencies.every(
          (d) =>
            d.actionable === false &&
            d.authority_effect === "none" &&
            d.evidence_refs.length > 0
        ) &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        ) &&
        projection.current.assessment.actionable === false &&
        projection.current.assessment.authority_effect === "none"))
  );
}

export function decisionSupportHistoryCountIsAuthoritative(
  summary: WorkspaceDecisionSupportSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
