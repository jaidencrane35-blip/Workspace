/**
 * Purpose: Copy helpers for the Desktop Reality Stage (Layouts).
 * Owner: Frontend product shell (Milestone R — Desktop Reality Stage)
 * Inputs: workspace name (optional profile), zone counts
 * Outputs: Stage labels and secondary copy
 * Dependencies: None (pure)
 * Non-responsibilities: IPC, window control, Flow/Focus geometry apply, Assistant,
 *   inventing observed windows
 */

export function layoutsStageEyebrow(): string {
  return "Desktop reality";
}

export function layoutsStageTitle(workspaceName: string | null | undefined): string {
  const name = workspaceName?.trim() ?? "";
  return name ? `${name} — desktop stage` : "Your desktop";
}

/** Home / marketing lede — reality first. */
export function layoutsStageLede(): string {
  return "Workspace represents your existing computing environment. Observed windows appear on the stage; arrangements remember real layouts. The companion canvas below is optional board practice — not your OS desktop.";
}

/** @deprecated Prefer stageDesktopEmptyCopy — kept for any remaining registry-only empty paths. */
export function layoutsStageEmptyAppsCopy(): {
  title: string;
  body: string;
} {
  return {
    title: "No library apps",
    body: "",
  };
}

export function layoutsStageCanvasNote(zoneCount: number): string {
  if (zoneCount <= 0) {
    return "Optional: companion canvas below for zone board practice (not OS window tiling).";
  }
  return zoneCount === 1
    ? "Optional companion canvas: 1 zone below (board practice — not OS tiling)."
    : `Optional companion canvas: ${zoneCount} zones below (board practice — not OS tiling).`;
}

export function layoutsStageFocusNote(): string {
  return "Focus emphasises the focused observed window. Supporting windows stay on stage. Companion canvas is hidden — switch to Flow to edit zones. Arrangements stay in the rail.";
}

export function layoutsStageFlowHint(): string {
  return "Save and restore real desktop window layouts from the arrangements area when a profile is selected.";
}

export function layoutsStageFocusHint(): string {
  return "Supporting windows stay available — Focus does not quit applications.";
}

export function layoutsStageRegistryHeading(): string {
  return "Library";
}

export function layoutsStageRegistryNote(): string {
  return "";
}
