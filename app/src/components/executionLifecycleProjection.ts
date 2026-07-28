/**
 * Execution lifecycle projection helpers.
 * React remains projection-only — history never becomes dispatch/cancel input.
 */
import type { ExecutionLifecycleHistoryEntry } from "../types/domain";

const TERMINAL_EXECUTION_STATES = new Set([
  "completed",
  "failed",
  "cancelled",
]);

/**
 * Projection-only guard: history entries are never actionable.
 * Mirrors Rust `ExecutionLifecycleHistoryEntry::is_non_actionable`.
 */
export function isExecutionHistoryNonActionable(
  entry: ExecutionLifecycleHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EXECUTION_STATES.has(entry.state)
  );
}
