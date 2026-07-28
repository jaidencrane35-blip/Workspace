/**
 * Task Graph terminal evidence projection helpers.
 * React remains projection-only — no lifecycle ownership or mutation.
 */
import type { TaskHistoryEntry } from "../types/domain";

const TERMINAL_TASK_STATES = new Set(["completed", "cancelled"]);

/**
 * Projection-only guard: history entries are never actionable and never grant authority.
 * Mirrors Rust `TaskHistoryEntry::is_non_actionable`.
 */
export function isTaskHistoryNonActionable(entry: TaskHistoryEntry): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_TASK_STATES.has(entry.status)
  );
}
