/**
 * Historical reconstruction projection helpers.
 * React remains projection-only — explanation evidence, never authority or replay.
 */
import type {
  HistoricalReconstructionHistoryEntry,
  HistoricalReconstructionSnapshot,
  HistoricalReconstructionSummary,
} from "../types/domain";

const TERMINAL_HIST_STATUSES = new Set(["superseded", "archived"]);

export function isHistoricalReconstructionHistoryNonActionable(
  entry: HistoricalReconstructionHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_HIST_STATUSES.has(entry.status)
  );
}

export function isHistoricalReconstructionSnapshotNonCommandable(
  snapshot: HistoricalReconstructionSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isHistoricalReconstructionHistoryNonActionable) &&
    (snapshot.current == null ||
      (snapshot.current.authority_effect === "none" &&
        snapshot.current.actionable === false &&
        snapshot.current.terminal === false &&
        snapshot.current.timeline.every(
          (t) => t.actionable === false && t.authority_effect === "none"
        ) &&
        snapshot.current.comparisons.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        ) &&
        snapshot.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        )))
  );
}

export function historicalReconstructionHistoryCountIsAuthoritative(
  summary: HistoricalReconstructionSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
