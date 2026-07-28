/**
 * Cognitive Graph projection helpers.
 * React remains projection-only — observational topology, no mutation.
 */
import type {
  CognitiveGraphHistoryEntry,
  CognitiveGraphSnapshot,
  CognitiveGraphSummary,
} from "../types/domain";

const TERMINAL_GRAPH_STATUSES = new Set(["superseded", "archived"]);

export function isCognitiveGraphHistoryNonActionable(
  entry: CognitiveGraphHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_GRAPH_STATUSES.has(entry.status)
  );
}

export function isCognitiveGraphSnapshotNonCommandable(
  snapshot: CognitiveGraphSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isCognitiveGraphHistoryNonActionable) &&
    (snapshot.current == null ||
      snapshot.current.authority_effect === "none")
  );
}

export function cognitiveGraphHistoryCountIsAuthoritative(
  summary: CognitiveGraphSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
