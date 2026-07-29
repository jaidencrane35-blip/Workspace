/**
 * Purpose: Chrome-density work mode model (Flow ↔ Focus) for product UI only.
 * Owner: Frontend product shell (Milestone B — chrome density)
 * Inputs: stored preference string / user selection
 * Outputs: WorkMode value, labels, copy, persistence helpers
 * Dependencies: localStorage (UI preference only)
 * Non-responsibilities: OS window moves, DesktopArrangement apply, AI,
 *   WindowController, PermissionGateway, new layout engines
 *
 * Problem solved: user-controlled presentation density toward reference Flow/Focus.
 * Why here: presentation preference — not domain/arrangement ownership.
 * Why not elsewhere: must not live in DesktopArrangement or Assistant packages.
 */

export type WorkMode = "flow" | "focus";

/** localStorage key for chrome-density preference (not a domain setting). */
export const WORK_MODE_STORAGE_KEY = "workspace.ui.work_mode";

/** Default productive density. */
export const DEFAULT_WORK_MODE: WorkMode = "flow";

/**
 * In Focus chrome, how many applications receive primary emphasis.
 * Remaining apps stay available as supporting chips.
 */
export const FOCUS_PRIMARY_APP_COUNT = 1;

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
    return "Higher density — multiple applications visible for multitasking. Does not move OS windows yet.";
  }
  return "Lower density — primary application emphasised; others stay available. Does not move OS windows yet.";
}

export function workModeStageLede(mode: WorkMode): string {
  if (mode === "flow") {
    return "Flow keeps your workspace overview dense: applications, layouts, and arrangements stay easy to scan.";
  }
  return "Focus reduces chrome noise and emphasises one primary application. Supporting apps remain available — nothing is closed.";
}
