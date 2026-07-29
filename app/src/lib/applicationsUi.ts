/**
 * Purpose: Copy and presentation helpers for the product Applications surface.
 * Owner: Frontend product shell (Milestone A)
 * Inputs: Application registry rows + observed active applications
 * Outputs: Labels, identity lines, empty-state copy
 * Dependencies: None (pure helpers)
 * Non-responsibilities: Launch policy, permissions, window control, Assistant
 */

import type {
  ApplicationReference,
  WorkspaceActiveApplication,
} from "../types/domain";

export function applicationsEmptyCopy(hasWorkspace: boolean): {
  title: string;
  body: string;
} {
  if (!hasWorkspace) {
    return {
      title: "Select a workspace first",
      body: "Applications belong to a workspace. Create or switch to one, then register apps here.",
    };
  }
  return {
    title: "No applications registered",
    body: "Register an application with a name and optional executable path, then launch through the governed path.",
  };
}

export function applicationIdentityLine(app: ApplicationReference): string {
  const parts: string[] = [];
  if (app.identifier?.trim()) {
    parts.push(app.identifier.trim());
  }
  if (app.executable_path?.trim()) {
    parts.push(app.executable_path.trim());
  }
  if (parts.length === 0) {
    return `id ${app.id}`;
  }
  return parts.join(" · ");
}

export function activeApplicationLabel(
  app: WorkspaceActiveApplication,
): string {
  const name = app.process_name?.trim() || `PID ${app.process_id}`;
  const windows =
    app.window_count === 1 ? "1 window" : `${app.window_count} windows`;
  return `${name} · ${windows}`;
}

export function canLaunchApplication(app: ApplicationReference): boolean {
  return Boolean(app.executable_path?.trim());
}
