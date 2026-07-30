/**
 * Purpose: Chrome-density work mode model (Flow ↔ Focus) for product UI only.
 * Owner: Frontend product shell (Milestone B — chrome density)
 * Inputs: stored preference string / user selection
 * Outputs: WorkMode value, labels, copy, persistence helpers, Focus partition
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
    return "Same desktop, denser spatial view.";
  }
  return "Same desktop, focused window emphasised.";
}

/** Focus chrome partition: one primary app + remaining supporting apps. */
export interface FocusApplicationPartition<T extends { id: string }> {
  primary: T | null;
  supporting: T[];
}

/**
 * Split a registry list into Focus primary vs supporting.
 * Preferred id wins when still present; otherwise first N apps (N = FOCUS_PRIMARY_APP_COUNT).
 */
export function partitionFocusApplications<T extends { id: string }>(
  applications: readonly T[],
  preferredPrimaryId: string | null | undefined,
): FocusApplicationPartition<T> {
  if (applications.length === 0) {
    return { primary: null, supporting: [] };
  }

  const preferred =
    preferredPrimaryId != null && preferredPrimaryId !== ""
      ? applications.find((app) => app.id === preferredPrimaryId)
      : undefined;
  const primary =
    preferred ?? applications.slice(0, FOCUS_PRIMARY_APP_COUNT)[0] ?? null;
  const supporting = applications.filter((app) => app.id !== primary?.id);
  return { primary, supporting };
}
