/**
 * PP-P01B — Restore-limits copy.
 *
 * Canonical, user-language boundaries for Product Proof restore. Experience
 * presents these limits; Action behaviour is unchanged. Workspace must not
 * imply relaunch, cross-session restore, or invented continuation.
 */

export const RESTORE_LIMITS_HEADING = "What restore does — and does not do";

export const RESTORE_LIMITS_SUMMARY =
  "Resume places and focuses windows that are still open in this same Windows session. It does not relaunch closed apps, open files, or work after a reboot.";

/** What the product restores or shows when Resume succeeds. */
export const RESTORE_LIMITS_DOES = [
  "Your handoff note, shown exactly as you wrote it",
  "Position and focus for windows that are still open in this continuing desktop session",
] as const;

/** Explicit non-capabilities — must stay truthful to Saved Context Restore Identity. */
export const RESTORE_LIMITS_DOES_NOT = [
  "Relaunch applications you closed (nothing launches silently)",
  "Open files or links for you",
  "Restore windows after a reboot or a new Windows sign-in",
  "Guess or rewrite what you meant to do next",
] as const;
