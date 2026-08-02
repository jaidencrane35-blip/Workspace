/**
 * PP-P01D — Pilot-safe chrome.
 *
 * Primary Experience navigation for Product Proof pilots. Engine and diagnostic
 * surfaces remain in the codebase but must not appear as default primary tabs.
 */

export const PILOT_PRIMARY_VIEWS = ["save", "resume", "help"] as const;

export type PilotPrimaryView = (typeof PILOT_PRIMARY_VIEWS)[number];

/** Labels shown in the primary tablist for pilot participants. */
export const PILOT_PRIMARY_TAB_LABELS = ["Save", "Resume", "Help"] as const;

/** Engine / diagnostic surfaces excluded from the default pilot chrome. */
export const PILOT_HIDDEN_ENGINE_TAB_LABELS = [
  "Canvas",
  "Work",
  "Assistant",
  "Diagnostic",
] as const;

export const PILOT_DEFAULT_VIEW: PilotPrimaryView = "save";
