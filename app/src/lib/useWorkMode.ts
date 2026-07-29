/**
 * Purpose: React hook for Flow/Focus chrome-density preference.
 * Owner: Frontend product shell
 * Inputs: none (reads/writes localStorage via workMode helpers)
 * Outputs: mode + change handler with status message callback
 * Dependencies: workMode.ts
 * Non-responsibilities: OS windows, DesktopArrangement apply, AI
 */

import { useCallback, useState } from "react";
import {
  loadStoredWorkMode,
  storeWorkMode,
  type WorkMode,
} from "./workMode";

export function useWorkMode(onStatus?: (message: string) => void): {
  workMode: WorkMode;
  onWorkModeChange: (mode: WorkMode) => void;
} {
  const [workMode, setWorkMode] = useState<WorkMode>(() => loadStoredWorkMode());

  const onWorkModeChange = useCallback(
    (mode: WorkMode) => {
      setWorkMode(mode);
      storeWorkMode(mode);
      onStatus?.(
        mode === "flow"
          ? "Flow presentation — denser workspace overview"
          : "Focus presentation — quieter chrome (OS windows unchanged)",
      );
    },
    [onStatus],
  );

  return { workMode, onWorkModeChange };
}
