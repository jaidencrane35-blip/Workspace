/**
 * Purpose: Copy helpers for Desktop product language (Programme I IC2–IC5).
 * Owner: Frontend product shell
 * Inputs: optional profile name
 * Outputs: Desktop labels and composed Workspace workflow copy
 * Dependencies: None (pure)
 *
 * Product State: Profile · Desktop · Arrangement · Restore
 * Interaction State (never persist): Editing · Preview · Selection · Guidance dismissed
 */

/** Product term for the running-desktop surface (view id remains `layouts`). */
export const WORKSPACE_DESKTOP_NAV_LABEL = "Desktop";

/** Product term for saved desktop layouts (DesktopArrangement). */
export const WORKSPACE_ARRANGEMENT_LABEL = "Arrangement";

/** Consistent product verbs across Desktop and Arrangements. */
export const WORKSPACE_SAVE_VERB = "Save";
export const WORKSPACE_UPDATE_VERB = "Update";
export const WORKSPACE_RESTORE_VERB = "Restore";
export const WORKSPACE_EDIT_LAYOUT_VERB = "Edit layout";

export function layoutsStageTitle(workspaceName: string | null | undefined): string {
  const name = workspaceName?.trim() ?? "";
  return name ? name : "Desktop";
}

/**
 * Quiet composition line: Profile → Desktop → Arrangement → Restore.
 * Avoids architectural jargon (Stage, WorkspaceState, HWND).
 */
export function workspaceDesktopWorkflowLine(
  profileName: string | null | undefined,
): string {
  const profile = profileName?.trim();
  if (profile) {
    return `Profile · ${profile} · Desktop · Arrangement · Restore`;
  }
  return "Desktop · choose a Profile to save Arrangements";
}

export function layoutsStageEmptyAppsCopy(): {
  title: string;
  body: string;
} {
  return {
    title: "No library apps",
    body: "",
  };
}

export function layoutsStageRegistryHeading(): string {
  return "Library";
}
