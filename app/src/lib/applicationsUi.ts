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
      title: "Optional library needs a named profile",
      body: "Observed desktop windows appear on the Stage without registration. Create or select a profile under Workspaces only if you want a saved library or arrangements tied to a name.",
    };
  }
  return {
    title: "No library apps in this profile yet",
    body: "Registration is optional. Add apps here to launch them later. The Stage shows your observed desktop separately — arrangements remember real window layouts.",
  };
}

export function applicationsLayoutsRelationCopy(): string {
  return "Running apps are what matter. The optional library is only for launch shortcuts tied to a named profile.";
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
