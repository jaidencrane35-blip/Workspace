/**
 * Unified Experience translation boundary (Sprint 133).
 *
 * All user-facing rationale copy must flow through these exports.
 * UI components import from here — not from cognition types directly.
 */

export {
  displayImportanceFromWeight,
  explanationCatalogVersion,
  lookupCatalogEntry,
  resolveAttentionReason,
  resolveAttentionReasons,
  resolveDecisionReason,
  resolveDecisionReasons,
  type DisplayImportance,
  type DisplayReason,
} from "./explanationResolver";
