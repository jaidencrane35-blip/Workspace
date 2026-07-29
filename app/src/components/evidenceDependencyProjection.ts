/**
 * Evidence dependency projection helpers.
 * Thin wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  EvidenceDependencyHistoryEntry,
  WorkspaceEvidenceDependencyProjection,
  WorkspaceEvidenceDependencySummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isEvidenceDependencyHistoryNonActionable(
  entry: EvidenceDependencyHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isEvidenceDependencyProjectionNonCommandable(
  projection: WorkspaceEvidenceDependencyProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function evidenceDependencyHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceDependencySummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
