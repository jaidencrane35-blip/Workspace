/**
 * Purpose: React hook for Assistant companion-rail open preference.
 * Owner: Frontend product shell (Milestone C)
 * Inputs: optional status callback
 * Outputs: open state + change handler
 * Dependencies: assistantRail.ts
 * Non-responsibilities: Assistant reasoning, IPC, AI engines
 */

import { useCallback, useState } from "react";
import {
  loadStoredAssistantRailOpen,
  storeAssistantRailOpen,
} from "./assistantRail";

export function useAssistantRail(onStatus?: (message: string) => void): {
  assistantRailOpen: boolean;
  onAssistantRailOpenChange: (open: boolean) => void;
} {
  const [assistantRailOpen, setAssistantRailOpen] = useState(() =>
    loadStoredAssistantRailOpen(),
  );

  const onAssistantRailOpenChange = useCallback(
    (open: boolean) => {
      setAssistantRailOpen(open);
      storeAssistantRailOpen(open);
      onStatus?.(
        open
          ? "Assistant companion shown"
          : "Assistant companion hidden — product stage stays primary",
      );
    },
    [onStatus],
  );

  return { assistantRailOpen, onAssistantRailOpenChange };
}
