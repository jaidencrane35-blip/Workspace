/**
 * Purpose: Copy helpers for the Layouts application-stage overview.
 * Owner: Frontend product shell (Cycle 1)
 * Inputs: counts of apps / zones / arrangements awareness flags
 * Outputs: Stage labels and empty-state copy
 * Dependencies: None (pure)
 * Non-responsibilities: IPC, window control, Flow/Focus modes, Assistant
 */

export function layoutsStageEyebrow(): string {
  return "Layouts stage";
}

export function layoutsStageTitle(workspaceName: string): string {
  return workspaceName.trim() || "Workspace stage";
}

export function layoutsStageLede(): string {
  return "Applications belonging to this workspace appear on the stage. Use the arrangements rail to save and restore real desktop window layouts. Flow ↔ Focus density modes are not available yet.";
}

export function layoutsStageEmptyAppsCopy(): {
  title: string;
  body: string;
} {
  return {
    title: "No applications on this stage yet",
    body: "Add apps under Applications. They show here as workspace assets — not a substitute for live OS windows.",
  };
}

export function layoutsStageCanvasNote(zoneCount: number): string {
  if (zoneCount <= 0) {
    return "Companion canvas zones are empty. Add a zone below for board layout practice — separate from desktop arrangements.";
  }
  return zoneCount === 1
    ? "1 companion canvas zone below (board layout — not OS tiling)."
    : `${zoneCount} companion canvas zones below (board layout — not OS tiling).`;
}
