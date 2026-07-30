/**
 * Purpose: Single client path to refresh authoritative WorkspaceState.
 * Owner: Frontend product shell (Product Contract V8)
 * Inputs: observation consumer id
 * Outputs: WorkspaceState from ensure_observation_freshness + get_workspace_state
 * Dependencies: ipc
 * Non-responsibilities: Window control, grouping computation, Assistant compose
 */

import { invokeIpc, isIpcRuntimeAvailable } from "./ipc";
import type { WorkspaceState } from "../types/domain";

export type WorkspaceObservationConsumerId =
  | "workspace_stage"
  | "workspace_applications"
  | "workspace_assistant"
  | "workspace_environment";

/**
 * Best-effort freshness ensure, then authoritative WorkspaceState read.
 * Callers must not invent parallel observation paths.
 */
export async function refreshObservedWorkspaceState(
  consumerId: WorkspaceObservationConsumerId,
): Promise<WorkspaceState> {
  if (!isIpcRuntimeAvailable()) {
    throw new Error("Workspace desktop runtime is unavailable.");
  }
  try {
    await invokeIpc("ensure_observation_freshness", { consumerId });
  } catch {
    // Freshness is best-effort; still read projected state.
  }
  return invokeIpc<WorkspaceState>("get_workspace_state");
}
