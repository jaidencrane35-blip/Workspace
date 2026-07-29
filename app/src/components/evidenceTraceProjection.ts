/**
 * Evidence trace projection helpers.
 * React remains projection-only — provenance surface, never inference or authority.
 * trace ≠ explanation; chain ≠ inference; segment ≠ invented hop.
 */
import type {
  EvidenceTraceHistoryEntry,
  WorkspaceEvidenceTraceProjection,
  WorkspaceEvidenceTraceSummary,
} from "../types/domain";

const TERMINAL_EVIDENCE_TRACE_STATUSES = new Set(["superseded", "archived"]);

export function isEvidenceTraceHistoryNonActionable(
  entry: EvidenceTraceHistoryEntry
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_EVIDENCE_TRACE_STATUSES.has(entry.status)
  );
}

export function isEvidenceTraceProjectionNonCommandable(
  projection: WorkspaceEvidenceTraceProjection
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isEvidenceTraceHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        projection.current.request.executable === false &&
        projection.current.request.actionable === false &&
        (projection.current.chain == null ||
          (projection.current.chain.actionable === false &&
            projection.current.chain.authority_effect === "none" &&
            projection.current.chain.segments.every(
              (s) =>
                s.actionable === false &&
                s.authority_effect === "none" &&
                s.evidence_ref.external_ref.length > 0
            ))) &&
        projection.current.diagnostics.actionable === false &&
        projection.current.diagnostics.authority_effect === "none" &&
        projection.current.lineage.actionable === false &&
        projection.current.lineage.authority_effect === "none" &&
        projection.current.gaps.every(
          (g) => g.actionable === false && g.authority_effect === "none"
        )))
  );
}

export function evidenceTraceHistoryCountIsAuthoritative(
  summary: WorkspaceEvidenceTraceSummary
): boolean {
  return summary.history_count >= summary.history.length;
}
