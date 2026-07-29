/**
 * Evidence reliability projection helpers.
 * Thin Batch 9 wrappers over shared Programme IV evidenceProjectionContract (Batch 10).
 */
import type {
  EvidenceReliabilityHistoryEntry,
  WorkspaceEvidenceReliabilityProjection,
  WorkspaceEvidenceReliabilitySummary,
} from "../types/domain";
import {
  evidenceHistoryCountIsAuthoritative,
  isEvidenceHistoryNonActionable,
  isEvidenceProjectionNonCommandable,
} from "./evidenceProjectionContract";

export function isEvidenceReliabilityHistoryNonActionable(
  entry: EvidenceReliabilityHistoryEntry
): boolean {
  return isEvidenceHistoryNonActionable(entry);
}

export function isEvidenceReliabilityProjectionNonCommandable(
  projection: WorkspaceEvidenceReliabilityProjection
): boolean {
  return isEvidenceProjectionNonCommandable(projection);
}

export function evidenceReliabilityHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceReliabilitySummary
): boolean {
  return evidenceHistoryCountIsAuthoritative(summary);
}
