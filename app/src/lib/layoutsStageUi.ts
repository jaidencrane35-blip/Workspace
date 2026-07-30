/**
 * Purpose: Copy helpers for the Layouts application-stage overview.
 * Owner: Frontend product shell (Milestone D — Workspace Stage)
 * Inputs: workspace name, zone counts
 * Outputs: Stage labels and empty-state copy (apps-first product identity)
 * Dependencies: None (pure)
 * Non-responsibilities: IPC, window control, Flow/Focus geometry apply, Assistant
 */

export function layoutsStageEyebrow(): string {
  return "Workspace stage";
}

export function layoutsStageTitle(workspaceName: string): string {
  const name = workspaceName.trim();
  return name ? `${name} — applications` : "Applications on stage";
}

/** Home / marketing lede — apps organise the workspace. */
export function layoutsStageLede(): string {
  return "This is where you organise and work with the applications that belong to this workspace. Desktop arrangements remember window layouts; the companion canvas below is optional board practice — not your OS desktop.";
}

export function layoutsStageEmptyAppsCopy(): {
  title: string;
  body: string;
} {
  return {
    title: "Add the apps you work with",
    body: "Applications are the centre of this workspace. Register them under Applications, then return here to launch and organise. Live OS windows are separate — arrangements save and restore those later.",
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
  return "Focus emphasises one application on stage. Supporting apps stay available. Companion canvas is hidden to reduce noise — switch to Flow to edit zones. Desktop arrangements stay in the rail.";
}

export function layoutsStageFlowHint(): string {
  return "Save and restore real desktop window layouts from the arrangements area below the stage.";
}

export function layoutsStageFocusHint(): string {
  return "Supporting apps stay available — Focus does not quit them.";
}
