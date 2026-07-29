/**
 * Assistant surface projection helpers.
 * Thin Batch 11 wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  AssistantSurfaceHistoryEntry,
  WorkspaceAssistantSurfaceProjection,
  WorkspaceAssistantSurfaceSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isAssistantSurfaceHistoryNonActionable(
  entry: AssistantSurfaceHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isAssistantSurfaceProjectionNonCommandable(
  projection: WorkspaceAssistantSurfaceProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function assistantSurfaceHistoryCountIsAuthoritative(
  summary: WorkspaceAssistantSurfaceSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
