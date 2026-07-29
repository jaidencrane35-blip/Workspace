/**
 * Evidence navigation projection helpers.
 * Thin wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  EvidenceNavigationHistoryEntry,
  WorkspaceEvidenceNavigationProjection,
  WorkspaceEvidenceNavigationSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isEvidenceNavigationHistoryNonActionable(
  entry: EvidenceNavigationHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isEvidenceNavigationProjectionNonCommandable(
  projection: WorkspaceEvidenceNavigationProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function evidenceNavigationHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceNavigationSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
