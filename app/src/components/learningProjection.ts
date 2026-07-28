/**
 * Learning & Adaptation projection helpers.
 * React remains projection-only — observational meta-evidence, no mutation.
 */
import type {
  LearningHistoryEntry,
  LearningSnapshot,
  LearningSummary,
} from "../types/domain";

const TERMINAL_LEARNING_STATUSES = new Set(["superseded", "archived"]);

export function isLearningHistoryNonActionable(
  entry: LearningHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_LEARNING_STATUSES.has(entry.status)
  );
}

export function isLearningSnapshotNonCommandable(
  snapshot: LearningSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isLearningHistoryNonActionable) &&
    (snapshot.current == null ||
      (snapshot.current.authority_effect === "none" &&
        snapshot.current.meta.actionable === false &&
        snapshot.current.meta.terminal === false &&
        snapshot.current.adaptation_candidates.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        )))
  );
}

export function learningHistoryCountIsAuthoritative(
  summary: LearningSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
