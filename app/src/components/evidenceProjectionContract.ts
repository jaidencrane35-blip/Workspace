/**
 * Shared Programme IV evidence projection contract (Batch 10 scaffold).
 * React remains projection-only — observational surfaces never repair, trust, or execute.
 *
 * Engine-specific files may re-export thin wrappers; prefer importing from here.
 */
export type EvidenceHistoryLike = {
  terminal: boolean;
  actionable: boolean;
  authority_effect: string;
  status: string;
};

export type EvidenceAuthorityLike = {
  actionable: boolean;
  authority_effect: string;
};

export type EvidenceProjectionLike = {
  authority_effect: string;
  history: EvidenceHistoryLike[];
  current: null | {
    authority_effect: string;
    actionable: boolean;
    terminal: boolean;
    assessment?: EvidenceAuthorityLike;
    diagnostics?: EvidenceAuthorityLike;
    lineage?: EvidenceAuthorityLike;
    observations?: EvidenceAuthorityLike[];
    gaps?: EvidenceAuthorityLike[];
    nodes?: EvidenceAuthorityLike[];
    relationships?: EvidenceAuthorityLike[];
  };
};

export type EvidenceSummaryLike = {
  history: unknown[];
  history_count: number;
};

const TERMINAL_STATUSES = new Set(["superseded", "archived"]);

export function isEvidenceHistoryNonActionable(
  entry: EvidenceHistoryLike
): boolean {
  return (
    entry.terminal === true &&
    entry.actionable === false &&
    entry.authority_effect === "none" &&
    TERMINAL_STATUSES.has(entry.status)
  );
}

function authorityIsNone(item: EvidenceAuthorityLike): boolean {
  return item.actionable === false && item.authority_effect === "none";
}

function authorityIsAbsentOrNone(item?: EvidenceAuthorityLike): boolean {
  return item == null || authorityIsNone(item);
}

export function isEvidenceProjectionNonCommandable(
  projection: EvidenceProjectionLike
): boolean {
  return (
    projection.authority_effect === "none" &&
    projection.history.every(isEvidenceHistoryNonActionable) &&
    (projection.current == null ||
      (projection.current.authority_effect === "none" &&
        projection.current.actionable === false &&
        projection.current.terminal === false &&
        authorityIsAbsentOrNone(projection.current.assessment) &&
        authorityIsAbsentOrNone(projection.current.diagnostics) &&
        authorityIsAbsentOrNone(projection.current.lineage) &&
        (projection.current.observations ?? []).every(authorityIsNone) &&
        (projection.current.gaps ?? []).every(authorityIsNone) &&
        (projection.current.nodes ?? []).every(authorityIsNone) &&
        (projection.current.relationships ?? []).every(authorityIsNone)))
  );
}

export function evidenceHistoryCountIsAuthoritative(
  summary: EvidenceSummaryLike
): boolean {
  return summary.history_count >= summary.history.length;
}
