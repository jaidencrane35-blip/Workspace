/**
 * Purpose: Copy helpers for the Desktop Reality Stage / Programme I composition.
 * Owner: Frontend product shell (Product Contract V4 / Programme I IC2)
 * Inputs: optional profile name
 * Outputs: Desktop labels and composed Workspace workflow copy
 * Dependencies: None (pure)
 */

/** Product term for the running-desktop surface (view id remains `layouts`). */
export const WORKSPACE_DESKTOP_NAV_LABEL = "Desktop";

/** Product term for saved desktop layouts (DesktopArrangement). */
export const WORKSPACE_ARRANGEMENT_LABEL = "Arrangement";

export function layoutsStageTitle(workspaceName: string | null | undefined): string {
  const name = workspaceName?.trim() ?? "";
  return name ? name : "Desktop";
}

/**
 * Quiet composition line: Profile → Desktop → Arrangement.
 * Avoids architectural jargon (Stage, WorkspaceState, HWND).
 */
export function workspaceDesktopWorkflowLine(
  profileName: string | null | undefined,
): string {
  const profile = profileName?.trim();
  if (profile) {
    return `Profile · ${profile} · Desktop · Arrangements`;
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
