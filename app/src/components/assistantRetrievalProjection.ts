/**
 * Assistant retrieval projection helpers.
 * Thin Batch 13 wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  AssistantRetrievalHistoryEntry,
  WorkspaceAssistantRetrievalProjection,
  WorkspaceAssistantRetrievalSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isAssistantRetrievalHistoryNonActionable(
  entry: AssistantRetrievalHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isAssistantRetrievalProjectionNonCommandable(
  projection: WorkspaceAssistantRetrievalProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function assistantRetrievalHistoryCountIsAuthoritative(
  summary: WorkspaceAssistantRetrievalSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
