/**
 * Assistant context projection helpers.
 * Thin Batch 12 wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  AssistantContextHistoryEntry,
  WorkspaceAssistantContextProjection,
  WorkspaceAssistantContextSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isAssistantContextHistoryNonActionable(
  entry: AssistantContextHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isAssistantContextProjectionNonCommandable(
  projection: WorkspaceAssistantContextProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function assistantContextHistoryCountIsAuthoritative(
  summary: WorkspaceAssistantContextSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
