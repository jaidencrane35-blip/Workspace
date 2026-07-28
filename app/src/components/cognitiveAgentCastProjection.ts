/**
 * Cognitive Agent Cast projection helpers.
 * React remains projection-only — role perspectives, no mutation or authority.
 */
import type {
  CognitiveAgentCastHistoryEntry,
  CognitiveAgentCastSnapshot,
  CognitiveAgentCastSummary,
} from "../types/domain";

const TERMINAL_CAST_STATUSES = new Set(["superseded", "archived"]);

export function isCognitiveAgentCastHistoryNonActionable(
  entry: CognitiveAgentCastHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_CAST_STATUSES.has(entry.status)
  );
}

export function isCognitiveAgentCastSnapshotNonCommandable(
  snapshot: CognitiveAgentCastSnapshot
): boolean {
  return (
    snapshot.authority_effect === "none" &&
    snapshot.history.every(isCognitiveAgentCastHistoryNonActionable) &&
    (snapshot.current == null ||
      (snapshot.current.authority_effect === "none" &&
        snapshot.current.meta.actionable === false &&
        snapshot.current.meta.terminal === false &&
        snapshot.current.agents.every(
          (a) => a.actionable === false && a.authority_effect === "none"
        ) &&
        snapshot.current.perspectives.every(
          (p) => p.actionable === false && p.authority_effect === "none"
        ) &&
        snapshot.current.critiques.every(
          (c) => c.actionable === false && c.authority_effect === "none"
        ) &&
        snapshot.current.syntheses.every(
          (s) => s.actionable === false && s.authority_effect === "none"
        )))
  );
}

export function cognitiveAgentCastHistoryCountIsAuthoritative(
  summary: CognitiveAgentCastSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
