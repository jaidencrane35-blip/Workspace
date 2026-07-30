/**
 * Purpose: Copy and presentation helpers for named desktop profiles.
 * Owner: Frontend product shell (IM-2 — Desktop Reality First)
 * Inputs: Workspace metadata + active id
 * Outputs: Labels and empty-state copy
 * Dependencies: None (pure helpers)
 * Non-responsibilities: IPC, permissions, arrangement restore, Assistant,
 *   inventing observed desktop data
 */

import type { Workspace } from "../types/domain";

export function workspaceSwitcherEmptyCopy(hasRuntime: boolean): {
  title: string;
  body: string;
} {
  if (!hasRuntime) {
    return {
      title: "Desktop runtime required",
      body: "Profile switching needs the Workspace desktop app. Stage still shows your desktop without a profile.",
    };
  }
  return {
    title: "No named profiles yet",
    body: "Optional. Open Stage to see your desktop — add a profile only to save arrangements or a library.",
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
