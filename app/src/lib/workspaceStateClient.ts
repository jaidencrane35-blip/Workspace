/**
 * Purpose: Single client path to refresh authoritative WorkspaceState.
 * Owner: Frontend product shell (Product Contract V8 / Product Foundation V14)
 * Inputs: observation consumer id
 * Outputs: WorkspaceState from ensure_observation_freshness + get_workspace_state;
 *   shared in-memory latest for all product consumers
 * Dependencies: ipc
 * Non-responsibilities: Window control, grouping computation, Assistant compose,
 *   inventing a second runtime (cache mirrors last WorkspaceState only)
 */

import { invokeIpc, isIpcRuntimeAvailable } from "./ipc";
import type { WorkspaceState } from "../types/domain";

export type WorkspaceObservationConsumerId =
  | "workspace_stage"
  | "workspace_applications"
  | "workspace_assistant"
  | "workspace_environment";

type WorkspaceStateListener = (state: WorkspaceState | null) => void;

let latestObservedWorkspaceState: WorkspaceState | null = null;
const listeners = new Set<WorkspaceStateListener>();

function publishObservedWorkspaceState(state: WorkspaceState | null): void {
  latestObservedWorkspaceState = state;
  for (const listener of listeners) {
    listener(state);
  }
}

/** Last WorkspaceState published by any consumer refresh (null until first success). */
export function getCachedObservedWorkspaceState(): WorkspaceState | null {
  return latestObservedWorkspaceState;
}

/**
 * Subscribe to shared WorkspaceState publications. Immediately receives current cache.
 * Returns unsubscribe.
 */
export function subscribeObservedWorkspaceState(
  listener: WorkspaceStateListener,
): () => void {
  listeners.add(listener);
  listener(latestObservedWorkspaceState);
  return () => {
    listeners.delete(listener);
  };
}

/**
 * Best-effort freshness ensure, then authoritative WorkspaceState read.
 * Publishes to all subscribers so Stage / Apps / Assistant / Operator share one hold.
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
  const state = await invokeIpc<WorkspaceState>("get_workspace_state");
  publishObservedWorkspaceState(state);
  return state;
}
