/**
 * Policy governance projection helpers.
 * React remains projection-only — evaluation evidence, never authority.
 */
import type {
  PolicyGovernanceHistoryEntry,
  PolicyGovernanceSnapshot,
  PolicyGovernanceSummary,
} from "../types/domain";

const TERMINAL_GOV_STATUSES = new Set(["superseded", "archived"]);

export function isPolicyGovernanceHistoryNonActionable(
  entry: PolicyGovernanceHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_GOV_STATUSES.has(entry.status)
  );
}

export function isPolicyGovernanceSnapshotNonCommandable(
  snapshot: PolicyGovernanceSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isPolicyGovernanceHistoryNonActionable) &&
    (snapshot.current == null ||
      (snapshot.current.authority_effect === "none" &&
        snapshot.current.meta.actionable === false &&
        snapshot.current.meta.terminal === false &&
        snapshot.current.evaluations.every(
          (e) => e.actionable === false && e.authority_effect === "none"
        ) &&
        (snapshot.current.recommendation == null ||
          (snapshot.current.recommendation.actionable === false &&
            snapshot.current.recommendation.authority_effect === "none"))))
  );
}

export function policyGovernanceHistoryCountIsAuthoritative(
  summary: PolicyGovernanceSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
