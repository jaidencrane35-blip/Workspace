/**
 * Evidence consistency projection helpers.
 * Thin wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  EvidenceConsistencyHistoryEntry,
  WorkspaceEvidenceConsistencyProjection,
  WorkspaceEvidenceConsistencySummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isEvidenceConsistencyHistoryNonActionable(
  entry: EvidenceConsistencyHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isEvidenceConsistencyProjectionNonCommandable(
  projection: WorkspaceEvidenceConsistencyProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function evidenceConsistencyHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceConsistencySummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
