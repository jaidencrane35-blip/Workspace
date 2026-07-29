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
      title: "Choose a workspace first",
      body: "Applications belong to a workspace. Switch or create one under Workspaces, then add apps here as workspace assets.",
    };
  }
  return {
    title: "No applications in this workspace yet",
    body: "Add the apps you use here. They stay with this workspace and can be launched when an executable path is set. Pair them with Layouts to save and restore window arrangements.",
  };
}

export function applicationsLayoutsRelationCopy(): string {
  return "Layouts save and restore window arrangements for your desktop. Applications here are the registry of tools that belong to this workspace — launch them, then use Layouts to remember where they sit.";
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
    return "Identity only — add an executable path to launch";
  }
  return parts.join(" · ");
}

export function activeApplicationLabel(
  app: WorkspaceActiveApplication,
): string {
  const name = app.process_name?.trim() || `Process ${app.process_id}`;
  const windows =
    app.window_count === 1 ? "1 window open" : `${app.window_count} windows open`;
  return `${name} · ${windows}`;
}

export function canLaunchApplication(app: ApplicationReference): boolean {
  return Boolean(app.executable_path?.trim());
}

export function applicationStatusLabel(app: ApplicationReference): string {
  return canLaunchApplication(app) ? "Ready to launch" : "Registered";
}
