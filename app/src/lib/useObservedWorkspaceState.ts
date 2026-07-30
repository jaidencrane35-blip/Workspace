/**
 * Purpose: React bind to shared WorkspaceState client cache.
 * Owner: Frontend product shell (Product Foundation V14)
 * Inputs: none (subscribes to workspaceStateClient)
 * Outputs: latest WorkspaceState + refresh helper
 * Dependencies: workspaceStateClient
 * Non-responsibilities: IPC directly, inventing desktop inference
 */

import { useCallback, useEffect, useState } from "react";
import type { WorkspaceState } from "../types/domain";
import {
  refreshObservedWorkspaceState,
  subscribeObservedWorkspaceState,
  type WorkspaceObservationConsumerId,
} from "./workspaceStateClient";

export function useObservedWorkspaceState(): {
  workspaceState: WorkspaceState | null;
  refreshWorkspaceState: (
    consumerId: WorkspaceObservationConsumerId,
  ) => Promise<WorkspaceState>;
} {
  const [workspaceState, setWorkspaceState] = useState<WorkspaceState | null>(
    null,
  );

  useEffect(() => subscribeObservedWorkspaceState(setWorkspaceState), []);

  const refreshWorkspaceState = useCallback(
    (consumerId: WorkspaceObservationConsumerId) =>
      refreshObservedWorkspaceState(consumerId),
    [],
  );

  return { workspaceState, refreshWorkspaceState };
}
