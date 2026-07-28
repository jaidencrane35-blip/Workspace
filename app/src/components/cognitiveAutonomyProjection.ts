/**
 * Cognitive Autonomy projection helpers.
 * React remains projection-only — suggestions only; never commands or authority.
 */
import type {
  CognitiveAutonomyHistoryEntry,
  CognitiveAutonomySnapshot,
  CognitiveAutonomySummary,
} from "../types/domain";

const TERMINAL_AUTONOMY_STATUSES = new Set(["superseded", "archived"]);

export function isCognitiveAutonomyHistoryNonActionable(
  entry: CognitiveAutonomyHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_AUTONOMY_STATUSES.has(entry.status)
  );
}

export function isCognitiveAutonomySnapshotNonCommandable(
  snapshot: CognitiveAutonomySnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isCognitiveAutonomyHistoryNonActionable) &&
    (snapshot.current == null ||
      (snapshot.current.authority_effect === "none" &&
        snapshot.current.meta.actionable === false &&
        snapshot.current.meta.terminal === false &&
        snapshot.current.opportunities.every(
          (o) =>
            o.actionable === false &&
            o.required_approval === true &&
            o.authority_effect === "none"
        ) &&
        snapshot.current.proposals.every(
          (p) =>
            p.actionable === false &&
            p.executable_payload == null &&
            p.authority_effect === "none"
        ) &&
        snapshot.current.recommendations.every(
          (r) =>
            r.actionable === false &&
            r.requires_approval === true &&
            r.authority_effect === "none"
        ) &&
        snapshot.current.risk_assessments.every(
          (s) => s.actionable === false && s.authority_effect === "none"
        )))
  );
}

export function cognitiveAutonomyHistoryCountIsAuthoritative(
  summary: CognitiveAutonomySummary
): boolean {
  return summary.history_count >= summary.history.length;
}
