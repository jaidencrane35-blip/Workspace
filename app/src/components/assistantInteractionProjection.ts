/**
 * Assistant interaction projection helpers.
 * Thin Batch 15 wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  AssistantInteractionHistoryEntry,
  WorkspaceAssistantInteractionProjection,
  WorkspaceAssistantInteractionSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isAssistantInteractionHistoryNonActionable(
  entry: AssistantInteractionHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isAssistantInteractionProjectionNonCommandable(
  projection: WorkspaceAssistantInteractionProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function assistantInteractionHistoryCountIsAuthoritative(
  summary: WorkspaceAssistantInteractionSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
