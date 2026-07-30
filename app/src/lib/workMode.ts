/**
 * Purpose: Chrome-density work mode model (Flow ↔ Focus) for product UI.
 * Owner: Frontend product shell (Product Contract V5)
 * Inputs: stored preference string / user selection
 * Outputs: WorkMode value, labels, copy, persistence helpers
 * Dependencies: localStorage (UI preference only)
 * Non-responsibilities: OS window moves, DesktopArrangement apply, AI,
 *   WindowController, PermissionGateway, new layout engines
 *
 * Focus organisation of desktop objects lives in stageDesktopUi
 * (`organiseStageForWorkMode`) — not registry chip partitions.
 */

export type WorkMode = "flow" | "focus";

/** localStorage key for chrome-density preference (not a domain setting). */
export const WORK_MODE_STORAGE_KEY = "workspace.ui.work_mode";

/** Default productive density. */
export const DEFAULT_WORK_MODE: WorkMode = "flow";

export function isWorkMode(value: unknown): value is WorkMode {
  return value === "flow" || value === "focus";
}

export function parseWorkMode(value: string | null | undefined): WorkMode {
  if (isWorkMode(value)) {
    return value;
  }
  return DEFAULT_WORK_MODE;
}

export function loadStoredWorkMode(): WorkMode {
  if (typeof localStorage === "undefined") {
    return DEFAULT_WORK_MODE;
  }
  try {
    return parseWorkMode(localStorage.getItem(WORK_MODE_STORAGE_KEY));
  } catch {
    return DEFAULT_WORK_MODE;
  }
}

export function storeWorkMode(mode: WorkMode): void {
  if (typeof localStorage === "undefined") {
    return;
  }
  try {
    localStorage.setItem(WORK_MODE_STORAGE_KEY, mode);
  } catch {
    // Preference persistence is best-effort in restricted environments.
  }
}

export function workModeLabel(mode: WorkMode): string {
  return mode === "flow" ? "Flow" : "Focus";
}

export function workModeDescription(mode: WorkMode): string {
  if (mode === "flow") {
    return "Same desktop, all windows on the map.";
  }
  return "Same desktop, one process on the map; others docked.";
}
