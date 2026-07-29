/**
 * Purpose: Assistant companion-rail chrome preference (open / collapsed).
 * Owner: Frontend product shell (Milestone C — companion rail)
 * Inputs: stored preference string / user toggle
 * Outputs: rail open state + persistence helpers
 * Dependencies: localStorage (UI preference only)
 * Non-responsibilities: Assistant reasoning, IPC, PermissionGateway, AI engines,
 *   DesktopArrangement, WindowController
 *
 * Problem solved: concept references show Assistant as a persistent side rail.
 * Why here: presentation preference — not domain ownership.
 */

/** localStorage key for companion-rail visibility (not a domain setting). */
export const ASSISTANT_RAIL_STORAGE_KEY = "workspace.ui.assistant_rail_open";

/** Default: rail visible on primary product views (concept alignment). */
export const DEFAULT_ASSISTANT_RAIL_OPEN = true;

export function parseAssistantRailOpen(
  value: string | null | undefined,
): boolean {
  if (value === "0" || value === "false") {
    return false;
  }
  if (value === "1" || value === "true") {
    return true;
  }
  return DEFAULT_ASSISTANT_RAIL_OPEN;
}

export function loadStoredAssistantRailOpen(): boolean {
  if (typeof localStorage === "undefined") {
    return DEFAULT_ASSISTANT_RAIL_OPEN;
  }
  try {
    return parseAssistantRailOpen(
      localStorage.getItem(ASSISTANT_RAIL_STORAGE_KEY),
    );
  } catch {
    return DEFAULT_ASSISTANT_RAIL_OPEN;
  }
}

export function storeAssistantRailOpen(open: boolean): void {
  if (typeof localStorage === "undefined") {
    return;
  }
  try {
    localStorage.setItem(ASSISTANT_RAIL_STORAGE_KEY, open ? "1" : "0");
  } catch {
    // Preference persistence is best-effort in restricted environments.
  }
}

export function assistantRailToggleLabel(open: boolean): string {
  return open ? "Hide Assistant companion" : "Show Assistant companion";
}
