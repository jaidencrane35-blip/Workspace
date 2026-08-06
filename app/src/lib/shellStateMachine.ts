/**
 * Two-form shell state machine — Zero-Trap between Operator and Conversation.
 */

import type { ShellMode } from "./shellRuntime";

/** Allowed directed transitions. */
export const SHELL_TRANSITIONS: Record<ShellMode, readonly ShellMode[]> = {
  0: [1], // Desktop Operator → Conversation
  1: [0], // Conversation → Desktop Operator
};

export interface ShellExitAction {
  id: string;
  label: string;
  to: ShellMode | "exit";
}

/** Obvious exits — Collapse/Close ≠ Exit. */
export const SHELL_EXITS: Record<ShellMode, readonly ShellExitAction[]> = {
  0: [
    { id: "open", label: "Open Conversation", to: 1 },
    { id: "exit", label: "Exit Workspace", to: "exit" },
  ],
  1: [
    { id: "collapse", label: "Collapse", to: 0 },
    { id: "exit", label: "Exit Workspace", to: "exit" },
  ],
};

export function canTransition(from: ShellMode, to: ShellMode): boolean {
  if (from === to) {
    return true;
  }
  return SHELL_TRANSITIONS[from].includes(to);
}

/** Assert: both forms reachable; every form has exits; Operator ↔ Conversation. */
export function verifyZeroTrapGraph(): {
  ok: boolean;
  issues: string[];
} {
  const issues: string[] = [];
  for (const mode of [0, 1] as ShellMode[]) {
    if (SHELL_EXITS[mode].length < 1) {
      issues.push(`Form ${mode} has no exits`);
    }
    if (SHELL_TRANSITIONS[mode].length < 1) {
      issues.push(`Form ${mode} has no transitions`);
    }
  }
  if (!SHELL_TRANSITIONS[0].includes(1)) {
    issues.push("Desktop Operator cannot open Conversation");
  }
  if (!SHELL_TRANSITIONS[1].includes(0)) {
    issues.push("Conversation cannot return to Desktop Operator");
  }
  // Reachability from idle Operator
  const seen = new Set<ShellMode>([0]);
  const queue: ShellMode[] = [0];
  while (queue.length) {
    const cur = queue.shift()!;
    for (const next of SHELL_TRANSITIONS[cur]) {
      if (!seen.has(next)) {
        seen.add(next);
        queue.push(next);
      }
    }
  }
  for (const mode of [0, 1] as ShellMode[]) {
    if (!seen.has(mode)) {
      issues.push(`Form ${mode} unreachable from Desktop Operator`);
    }
  }
  return { ok: issues.length === 0, issues };
}
