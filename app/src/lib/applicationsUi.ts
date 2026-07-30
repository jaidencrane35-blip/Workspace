/**
 * Purpose: Copy and presentation helpers for the product Applications surface.
 * Owner: Frontend product shell (Milestone A / A.1)
 * Inputs: Application registry rows + observed active applications
 * Outputs: Labels, identity lines, empty-state copy, layout relationship copy
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
      title: "Library needs a Profile",
      body: "Desktop already shows running apps. Add a Profile only to save a library.",
    };
  }
  return {
    title: "No library apps yet",
    body: "Optional — Desktop shows what is running. Add apps here only to launch later.",
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
    return "Add an executable path to launch";
  }
  return parts.join(" · ");
}

export function activeApplicationName(app: WorkspaceActiveApplication): string {
  return app.process_name?.trim() || `Process ${app.process_id}`;
}

export function activeApplicationWindowLine(
  app: WorkspaceActiveApplication,
): string {
  return app.window_count === 1
    ? "1 window"
    : `${app.window_count} windows`;
}

export function canLaunchApplication(app: ApplicationReference): boolean {
  return Boolean(app.executable_path?.trim());
}

export function applicationStatusLabel(app: ApplicationReference): string {
  return canLaunchApplication(app) ? "Ready to launch" : "Registered";
}
