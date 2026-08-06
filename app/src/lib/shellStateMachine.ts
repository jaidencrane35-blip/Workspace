/**
 * Shell state machine — runtime modes with Zero-Trap exits.
 * Every state must expose at least one obvious recovery path.
 */

import type { ShellMode } from "./shellRuntime";

/** Allowed directed transitions (Zero-Trap graph). */
export const SHELL_TRANSITIONS: Record<ShellMode, readonly ShellMode[]> = {
  0: [1, 2], // Floating → Compact | Expand
  1: [0, 2, 3], // Compact → Floating | Expand | Specialized
  2: [0, 1, 3], // Expanded → Floating | Compact | Specialized
  3: [0, 1, 2], // Specialized → Floating | Compact | Expanded
};

export interface ShellExitAction {
  id: string;
  label: string;
  to: ShellMode | "hide" | "exit";
}

/** Obvious exits available from each state (UI + recovery). */
export const SHELL_EXITS: Record<ShellMode, readonly ShellExitAction[]> = {
  0: [
    { id: "open", label: "Open Workspace", to: 1 },
    { id: "expand", label: "Expand Workspace", to: 2 },
    { id: "hide", label: "Hide", to: "hide" },
    { id: "exit", label: "Exit Workspace", to: "exit" },
  ],
  1: [
    { id: "collapse", label: "Collapse", to: 0 },
    { id: "expand", label: "Expand", to: 2 },
    { id: "exit", label: "Exit Workspace", to: "exit" },
  ],
  2: [
    { id: "compact", label: "Compact", to: 1 },
    { id: "collapse", label: "Collapse", to: 0 },
    { id: "exit", label: "Exit Workspace", to: "exit" },
  ],
  3: [
    { id: "compact", label: "Compact", to: 1 },
    { id: "expand", label: "Back to Expand", to: 2 },
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

/** Assert graph: every mode reachable from 1; every mode has ≥1 exit. */
export function verifyZeroTrapGraph(): {
  ok: boolean;
  issues: string[];
} {
  const issues: string[] = [];
  for (const mode of [0, 1, 2, 3] as ShellMode[]) {
    if (SHELL_EXITS[mode].length < 1) {
      issues.push(`Mode ${mode} has no exits`);
    }
    if (SHELL_TRANSITIONS[mode].length < 1) {
      issues.push(`Mode ${mode} has no transitions`);
    }
  }
  // Reachability from Mode 1 (default launch)
  const seen = new Set<ShellMode>([1]);
  const queue: ShellMode[] = [1];
  while (queue.length) {
    const cur = queue.shift()!;
    for (const next of SHELL_TRANSITIONS[cur]) {
      if (!seen.has(next)) {
        seen.add(next);
        queue.push(next);
      }
    }
  }
  for (const mode of [0, 1, 2, 3] as ShellMode[]) {
    if (!seen.has(mode)) {
      issues.push(`Mode ${mode} unreachable from Compact (1)`);
    }
  }
  // Floating can return to Compact (recovery)
  if (!SHELL_TRANSITIONS[0].includes(1)) {
    issues.push("Floating cannot open Compact");
  }
  return { ok: issues.length === 0, issues };
}
