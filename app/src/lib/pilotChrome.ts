/**
 * PP-P01D / PP-P01E — Pilot-safe chrome.
 *
 * Primary Experience navigation for Product Proof pilots. Engine and diagnostic
 * surfaces remain in the codebase but must not appear as default primary tabs.
 * Pilot measurement is a participant-facing evaluation surface, not an engine tab.
 */

export const PILOT_PRIMARY_VIEWS = ["save", "resume", "pilot", "help"] as const;

export type PilotPrimaryView = (typeof PILOT_PRIMARY_VIEWS)[number];

/** Labels shown in the primary tablist for pilot participants. */
export const PILOT_PRIMARY_TAB_LABELS = ["Save", "Resume", "Pilot", "Help"] as const;

/** Engine / diagnostic surfaces excluded from the default pilot chrome. */
export const PILOT_HIDDEN_ENGINE_TAB_LABELS = [
  "Canvas",
  "Work",
  "Assistant",
  "Diagnostic",
] as const;

export const PILOT_DEFAULT_VIEW: PilotPrimaryView = "save";
