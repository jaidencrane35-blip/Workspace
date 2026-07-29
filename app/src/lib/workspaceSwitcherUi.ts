/**
 * Purpose: Copy and presentation helpers for the product workspace switcher.
 * Owner: Frontend product shell (Milestone A)
 * Inputs: Workspace metadata + active id
 * Outputs: Labels and empty-state copy
 * Dependencies: None (pure helpers)
 * Non-responsibilities: IPC, permissions, arrangement restore, Assistant
 */

import type { Workspace } from "../types/domain";

export function workspaceSwitcherEmptyCopy(hasRuntime: boolean): {
  title: string;
  body: string;
} {
  if (!hasRuntime) {
    return {
      title: "Desktop runtime required",
      body: "Workspace switching needs the Workspace desktop app. The switcher UI still renders here.",
    };
  }
  return {
    title: "No workspaces yet",
    body: "Create a named workspace to organise applications and desktop arrangements.",
  };
}

export function workspaceRowLabel(
  workspace: Workspace,
  activeId: string | null,
): string {
  if (activeId && workspace.id === activeId) {
    return `${workspace.name} (current)`;
  }
  return workspace.name;
}

export function formatWorkspaceMeta(workspace: Workspace): string {
  const updated = workspace.updated_at?.trim();
  if (!updated) {
    return workspace.id;
  }
  return `Updated ${updated}`;
}
