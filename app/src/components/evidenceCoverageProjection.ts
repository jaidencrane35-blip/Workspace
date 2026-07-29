/**
 * Evidence coverage projection helpers.
 * Thin wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  EvidenceCoverageHistoryEntry,
  WorkspaceEvidenceCoverageProjection,
  WorkspaceEvidenceCoverageSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isEvidenceCoverageHistoryNonActionable(
  entry: EvidenceCoverageHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isEvidenceCoverageProjectionNonCommandable(
  projection: WorkspaceEvidenceCoverageProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function evidenceCoverageHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceCoverageSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
