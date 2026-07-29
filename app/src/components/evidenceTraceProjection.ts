/**
 * Evidence trace projection helpers.
 * Thin wrappers over shared Programme IV evidenceProjectionContract.
 */
import type {
  EvidenceTraceHistoryEntry,
  WorkspaceEvidenceTraceProjection,
  WorkspaceEvidenceTraceSummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isEvidenceTraceHistoryNonActionable(
  entry: EvidenceTraceHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isEvidenceTraceProjectionNonCommandable(
  projection: WorkspaceEvidenceTraceProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function evidenceTraceHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceTraceSummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
