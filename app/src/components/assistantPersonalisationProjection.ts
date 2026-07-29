/**
 * Assistant personalisation projection helpers.
 * Thin Batch 16 wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  AssistantPersonalisationHistoryEntry,
  WorkspaceAssistantPersonalisationProjection,
  WorkspaceAssistantPersonalisationSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isAssistantPersonalisationHistoryNonActionable(
  entry: AssistantPersonalisationHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isAssistantPersonalisationProjectionNonCommandable(
  projection: WorkspaceAssistantPersonalisationProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function assistantPersonalisationHistoryCountIsAuthoritative(
  summary: WorkspaceAssistantPersonalisationSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
