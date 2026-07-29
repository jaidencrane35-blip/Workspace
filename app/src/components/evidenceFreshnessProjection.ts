/**
 * Evidence freshness projection helpers.
 * Thin wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  EvidenceFreshnessHistoryEntry,
  WorkspaceEvidenceFreshnessProjection,
  WorkspaceEvidenceFreshnessSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isEvidenceFreshnessHistoryNonActionable(
  entry: EvidenceFreshnessHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isEvidenceFreshnessProjectionNonCommandable(
  projection: WorkspaceEvidenceFreshnessProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function evidenceFreshnessHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceFreshnessSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
