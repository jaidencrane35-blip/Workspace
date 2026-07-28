/**
 * Planning Engine projection helpers.
 * React remains projection-only — no execution, approval, or lifecycle mutation.
 */
import type { PlanningHistoryEntry, PlanningSnapshot, PlanningSummary } from "../types/domain";

const TERMINAL_PLAN_STATUSES = new Set(["superseded", "abandoned"]);

/**
 * Projection-only guard: planning history is never actionable and never grants authority.
 * Mirrors Rust `PlanningHistoryEntry::is_non_actionable`.
 */
export function isPlanningHistoryNonActionable(
  entry: PlanningHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_PLAN_STATUSES.has(entry.status)
  );
}

/** Snapshot must remain non-commandable (no execution controls). */
export function isPlanningSnapshotNonCommandable(
  snapshot: PlanningSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isPlanningHistoryNonActionable) &&
    (snapshot.current == null ||
      snapshot.current.authority_effect === "none")
  );
}

/** history_count is authoritative even when the returned history window is truncated. */
export function planningHistoryCountIsAuthoritative(
  summary: PlanningSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
