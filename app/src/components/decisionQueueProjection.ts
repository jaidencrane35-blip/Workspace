/**
 * Decision Queue terminal overlay projection helpers.
 * React remains projection-only — no lifecycle ownership or execution.
 */
import type { DecisionOverlayHistoryEntry } from "../types/domain";

const TERMINAL_OVERLAY_STATES = new Set(["dismissed", "expired"]);

/**
 * Projection-only guard: history entries are never actionable and never grant authority.
 * Mirrors Rust `DecisionOverlayHistoryEntry::is_non_actionable`.
 */
export function isDecisionOverlayHistoryNonActionable(
  entry: DecisionOverlayHistoryEntry
): boolean {
  return (
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_OVERLAY_STATES.has(entry.decision_state)
  );
}

/**
 * Distinguishes expired vs orphaned retention without inventing lifecycle truth.
 * Orphaned dismissed remains `dismissed` with `orphaned: true`.
 */
export function decisionOverlayHistoryRetentionLabel(
  entry: DecisionOverlayHistoryEntry
): string {
  if (entry.orphaned && entry.decision_state === "expired") {
    return "orphaned_expired";
  }
  if (entry.orphaned && entry.decision_state === "dismissed") {
    return "orphaned_dismissed";
  }
  return entry.decision_state;
}
