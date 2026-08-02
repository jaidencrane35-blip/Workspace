/**
 * Product Proof pilot chrome — Experience presentation.
 *
 * Primary navigation is a spatial dock (Home / Save / Continue / Check-in /
 * Guide). Engine and diagnostic surfaces remain in the codebase but must not
 * appear as default primary chrome. Check-in is the consented pilot measurement
 * surface (PP-P01E), not an engine tab.
 */

export const PILOT_PRIMARY_VIEWS = [
  "home",
  "save",
  "resume",
  "pilot",
  "help",
] as const;

export type PilotPrimaryView = (typeof PILOT_PRIMARY_VIEWS)[number];

/** Labels shown in the primary tablist for pilot participants. */
export const PILOT_PRIMARY_TAB_LABELS = [
  "Home",
  "Save",
  "Continue",
  "Check-in",
  "Guide",
] as const;

/** Engine / diagnostic surfaces excluded from the default pilot chrome. */
export const PILOT_HIDDEN_ENGINE_TAB_LABELS = [
  "Canvas",
  "Work",
  "Assistant",
  "Diagnostic",
] as const;

export const PILOT_DEFAULT_VIEW: PilotPrimaryView = "home";

/** Map each view id to its chrome label. */
export const PILOT_VIEW_LABELS: Record<PilotPrimaryView, string> = {
  home: "Home",
  save: "Save",
  resume: "Continue",
  pilot: "Check-in",
  help: "Guide",
};
