/**
 * Decision Engine terminal artifact projection helpers.
 * React remains projection-only — no lifecycle ownership or execution.
 */
import type { DecisionArtifactHistoryEntry } from "../types/domain";

const TERMINAL_DE_STATES = new Set(["selected", "dismissed", "expired"]);

/**
 * Projection-only guard: history entries are never actionable and never grant authority.
 * Mirrors Rust `DecisionArtifactHistoryEntry::is_non_actionable`.
 */
export function isDecisionArtifactHistoryNonActionable(
  entry: DecisionArtifactHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_DE_STATES.has(entry.decision_state)
  );
}
