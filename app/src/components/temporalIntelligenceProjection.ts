/**
 * Temporal intelligence projection helpers.
 * React remains projection-only — understanding evidence, never authority / forecast / correction.
 */
import type {
  TemporalIntelligenceHistoryEntry,
  TemporalIntelligenceSnapshot,
  TemporalIntelligenceSummary,
} from "../types/domain";

const TERMINAL_TEMPORAL_STATUSES = new Set(["superseded", "archived"]);

export function isTemporalIntelligenceHistoryNonActionable(
  entry: TemporalIntelligenceHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_TEMPORAL_STATUSES.has(entry.status)
  );
}

export function isTemporalIntelligenceSnapshotNonCommandable(
  snapshot: TemporalIntelligenceSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isTemporalIntelligenceHistoryNonActionable) &&
    (snapshot.current == null ||
      (snapshot.current.authority_effect === "none" &&
        snapshot.current.actionable === false &&
        snapshot.current.terminal === false &&
        snapshot.current.chain_summary.actionable === false &&
        snapshot.current.chain_summary.authority_effect === "none" &&
        snapshot.current.conflict_explanations.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        ) &&
        snapshot.current.evidence_quality.actionable === false &&
        snapshot.current.evidence_quality.authority_effect === "none" &&
        snapshot.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        )))
  );
}

export function temporalIntelligenceHistoryCountIsAuthoritative(
  summary: TemporalIntelligenceSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
