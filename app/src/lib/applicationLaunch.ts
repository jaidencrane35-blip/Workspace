/**
 * Purpose: Shared governed application launch helper for product surfaces.
 * Owner: Frontend product shell (Milestone D — Workspace Stage)
 * Inputs: Registered ApplicationReference with executable path
 * Outputs: ApplicationLaunchResult from existing launch_application IPC
 * Dependencies: invokeIpc only
 * Non-responsibilities: Permission policy, WindowController, OS discovery, AI
 *
 * Why here: Applications panel and Layouts stage both launch the same way —
 * keep one call site so launch behaviour cannot drift.
 */

import { invokeIpc } from "./ipc";
import { canLaunchApplication } from "./applicationsUi";
import type {
  ApplicationLaunchResult,
  ApplicationReference,
} from "../types/domain";

export function launchBlockedReason(
  app: ApplicationReference,
): string | null {
  if (!canLaunchApplication(app)) {
    return "Add an executable path before launching this application.";
  }
  return null;
}

export async function launchRegisteredApplication(
  app: ApplicationReference,
): Promise<ApplicationLaunchResult> {
  const blocked = launchBlockedReason(app);
  if (blocked) {
    throw new Error(blocked);
  }
  return invokeIpc<ApplicationLaunchResult>("launch_application", {
    id: app.id,
  });
}

export function launchSuccessMessage(result: ApplicationLaunchResult): string {
  return result.simulated
    ? `Launch recorded for ${result.name} (simulated on this platform)`
    : `Launched ${result.name}`;
}
