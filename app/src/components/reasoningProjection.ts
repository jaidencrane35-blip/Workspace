/**
 * Reasoning Memory projection helpers.
 * React remains projection-only — no execution, approval, or lifecycle mutation.
 */
import type {
  ReasoningHistoryEntry,
  ReasoningSnapshot,
  ReasoningSummary,
} from "../types/domain";

const TERMINAL_REASONING_STATUSES = new Set(["superseded", "archived"]);

/**
 * Projection-only guard: reasoning history is never actionable.
 * Mirrors Rust `ReasoningHistoryEntry::is_non_actionable`.
 */
export function isReasoningHistoryNonActionable(
  entry: ReasoningHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_REASONING_STATUSES.has(entry.status)
  );
}

/** Snapshot must remain non-commandable (no execution controls). */
export function isReasoningSnapshotNonCommandable(
  snapshot: ReasoningSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isReasoningHistoryNonActionable) &&
    (snapshot.current == null ||
      snapshot.current.authority_effect === "none")
  );
}

/** history_count is authoritative even when the returned history window is truncated. */
export function reasoningHistoryCountIsAuthoritative(
  summary: ReasoningSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
