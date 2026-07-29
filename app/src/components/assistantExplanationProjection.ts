/**
 * Assistant explanation projection helpers.
 * Thin Batch 14 wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  AssistantExplanationHistoryEntry,
  WorkspaceAssistantExplanationProjection,
  WorkspaceAssistantExplanationSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isAssistantExplanationHistoryNonActionable(
  entry: AssistantExplanationHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isAssistantExplanationProjectionNonCommandable(
  projection: WorkspaceAssistantExplanationProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function assistantExplanationHistoryCountIsAuthoritative(
  summary: WorkspaceAssistantExplanationSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
