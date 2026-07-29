/**
 * Evidence completeness projection helpers.
 * Thin wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  EvidenceCompletenessHistoryEntry,
  WorkspaceEvidenceCompletenessProjection,
  WorkspaceEvidenceCompletenessSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isEvidenceCompletenessHistoryNonActionable(
  entry: EvidenceCompletenessHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isEvidenceCompletenessProjectionNonCommandable(
  projection: WorkspaceEvidenceCompletenessProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function evidenceCompletenessHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceCompletenessSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
