/**
 * Unified Experience translation boundary (Sprint 133–134).
 *
 * Public UI import surface for Experience translation.
 * Panels use DisplayReasonList; only this module and DisplayReasonList touch resolver internals.
 */

export type { AttentionReason, DecisionReason } from "../types/domain";

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
