/**
 * Unified Experience translation boundary (Sprint 133–135).
 *
 * Public UI import surface for Experience translation.
 * Panels use DisplayReasonList; only this module and DisplayReasonList touch resolver internals.
 *
 * Traced APIs (`*Traced`) are developer diagnostics only — do not render on Work/Assistant.
 */

export type { AttentionReason, DecisionReason } from "../types/domain";

export {
  displayImportanceFromWeight,
  explanationCatalogVersion,
  formatResolverPathLabel,
  lookupCatalogEntry,
  lookupCatalogEntryWithPath,
  resolveAttentionReason,
  resolveAttentionReasons,
  resolveAttentionReasonTraced,
  resolveAttentionReasonsTraced,
  resolveDecisionReason,
  resolveDecisionReasons,
  resolveDecisionReasonTraced,
  resolveDecisionReasonsTraced,
  type DisplayImportance,
  type DisplayReason,
  type ExperienceResolverPath,
  type ExperienceResolverPathKind,
  type ExperienceTranslationTrace,
} from "./explanationResolver";
