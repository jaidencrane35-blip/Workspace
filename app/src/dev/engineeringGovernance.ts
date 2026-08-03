/**
 * Engineering Governance — certifies implementation changes against architecture authority.
 * Governs engineering process only. Not user behaviour, Experience runtime, or Runtime Core.
 * Developer-authorized. No automatic advancement. No AI autonomy.
 */

import { fnv1a } from "./devHash";
import {
  evaluateValidationContract,
  type ExperienceChangeProposal,
} from "./experienceGovernance";
import {
  browserStore,
  memoryStore,
  type ExperienceStoreAdapter,
} from "./experienceStore";
import {
  clearJsonKey,
  loadJsonBundle,
  saveJsonBundle,
} from "./governanceStore";

export const ENGINEERING_STORAGE_KEY = "ws.dev.experience.engineering.v1";
export const MAX_ENGINEERING_RECORDS = 50;
export const MAX_ENGINEERING_HISTORY = 2000;

/** Version 2 architecture authority documents (40–61). */
export const ARCHITECTURE_AUTHORITY_DOCS = [
  "40_Experience_Refoundation.md",
  "41_Perceptual_Convergence.md",
  "42_Experience_Validation.md",
  "43_Experience_Evidence_Model.md",
  "44_Experience_Improvement_Model.md",
  "45_Experience_Change_Governance.md",
  "46_Engineering_Governance.md",
  "47_Architectural_Integrity.md",
  "48_Adaptive_Workspace.md",
  "49_Adaptation_Experiments.md",
  "50_Longitudinal_Adaptation_Validation.md",
  "51_Adaptation_Operations.md",
  "52_First_Production_Adaptation.md",
  "53_Adaptation_Composition.md",
  "54_Adaptation_Certification.md",
  "55_Adaptation_Packs.md",
  "56_Governance_Consolidation.md",
  "57_Real_World_Adaptation_Validation.md",
  "58_Workspace_Memory_Evolution.md",
  "59_Workspace_Presence.md",
  "60_Workspace_Anticipation.md",
  "61_Workspace_Calibration.md",
] as const;

export type ArchitectureAuthorityDoc =
  (typeof ARCHITECTURE_AUTHORITY_DOCS)[number];

export type EngineeringLifecycle =
  | "draft"
  | "implemented"
  | "validated"
  | "architecturally_accepted"
  | "released";

export type ReleaseImpact =
  | "none"
  | "dev_tooling"
  | "documentation"
  | "tests"
  | "architecture"
  | "validation_tooling";

export type ConsistencyStatus = "incomplete" | "complete";

export type ConsistencyError =
  | "missing_proposal"
  | "missing_evidence"
  | "missing_validation"
  | "missing_architecture_authority"
  | "missing_commit"
  | "missing_module"
  | "missing_test"
  | "proposal_validation_incomplete"
  | "orphan_opportunity"
  | "orphan_replay";

export type TraceabilityGap =
  | "missing_commit"
  | "missing_proposal"
  | "missing_opportunity"
  | "missing_evidence"
  | "missing_replay"
  | "incomplete_proposal_validation";

export type EngineeringReason =
  | "created_from_proposals"
  | "mark_implemented"
  | "mark_validated"
  | "architecturally_accept"
  | "release";

export type EngineeringTransitionError =
  | "not_found"
  | "invalid_transition"
  | "consistency_incomplete"
  | "traceability_incomplete"
  | "released";

export interface EngineeringValidationEvidence {
  proposalIds: string[];
  evidenceSnapshotIds: string[];
  opportunityIds: string[];
  replaySessionIds: string[];
  proposalValidationComplete: boolean;
  architectureAuthorityIds: ArchitectureAuthorityDoc[];
}

export interface EngineeringChangeRecord {
  schemaVersion: 1;
  changeId: string;
  proposalIds: string[];
  commits: string[];
  architectureDocuments: ArchitectureAuthorityDoc[];
  affectedModules: string[];
  affectedTests: string[];
  validationEvidence: EngineeringValidationEvidence;
  releaseImpact: ReleaseImpact;
  state: EngineeringLifecycle;
  consistencyStatus: ConsistencyStatus;
}

export interface EngineeringHistoryEntry {
  schemaVersion: 1;
  entryId: string;
  changeId: string;
  seq: number;
  t: number;
  previousState: EngineeringLifecycle | null;
  newState: EngineeringLifecycle;
  reason: EngineeringReason;
  authorityReference: ArchitectureAuthorityDoc;
}

export interface ConsistencyResult {
  status: ConsistencyStatus;
  errors: ConsistencyError[];
}

export interface ReleaseTraceability {
  changeId: string;
  commits: string[];
  proposalIds: string[];
  opportunityIds: string[];
  evidenceSnapshotIds: string[];
  replaySessionIds: string[];
  /** Interaction trace session ids (same store as replay references). */
  interactionSessionIds: string[];
  complete: boolean;
  missing: TraceabilityGap[];
}

export interface EngineeringConsistencyContext {
  proposals: ExperienceChangeProposal[];
  /** Known evidence snapshot ids from the evidence store. */
  evidenceIds: readonly string[];
}

interface EngineeringBundle {
  schemaVersion: 1;
  records: EngineeringChangeRecord[];
  history: EngineeringHistoryEntry[];
}

const ALLOWED_TRANSITIONS: Record<
  EngineeringLifecycle,
  readonly EngineeringLifecycle[]
> = {
  draft: ["implemented"],
  implemented: ["validated"],
  validated: ["architecturally_accepted"],
  architecturally_accepted: ["released"],
  released: [],
};

const REASON_FOR_TRANSITION: Partial<
  Record<`${EngineeringLifecycle}->${EngineeringLifecycle}`, EngineeringReason>
> = {
  "draft->implemented": "mark_implemented",
  "implemented->validated": "mark_validated",
  "validated->architecturally_accepted": "architecturally_accept",
  "architecturally_accepted->released": "release",
};

const COMMIT_RE = /^[a-f0-9]{7,40}$/i;
const PATH_RE = /^[a-z0-9_./@-]{1,160}$/i;

function emptyBundle(): EngineeringBundle {
  return { schemaVersion: 1, records: [], history: [] };
}

export function isArchitectureAuthorityDoc(
  value: string,
): value is ArchitectureAuthorityDoc {
  return (ARCHITECTURE_AUTHORITY_DOCS as readonly string[]).includes(value);
}

export function isEngineeringTransitionAllowed(
  from: EngineeringLifecycle,
  to: EngineeringLifecycle,
): boolean {
  return ALLOWED_TRANSITIONS[from].includes(to);
}

/**
 * Deterministic consistency verification.
 * Rejects incomplete records (missing proposal / evidence / validation / authority).
 */
export function verifyEngineeringConsistency(
  record: EngineeringChangeRecord,
  context: EngineeringConsistencyContext,
): ConsistencyResult {
  const errors: ConsistencyError[] = [];
  const proposalById = new Map(
    context.proposals.map((p) => [p.proposalId, p]),
  );
  const evidenceSet = new Set(context.evidenceIds);

  if (record.proposalIds.length === 0) {
    errors.push("missing_proposal");
  }
  if (record.commits.length === 0 || !record.commits.every((c) => COMMIT_RE.test(c))) {
    errors.push("missing_commit");
  }
  if (record.affectedModules.length === 0) {
    errors.push("missing_module");
  }
  if (record.affectedTests.length === 0) {
    errors.push("missing_test");
  }
  if (
    record.architectureDocuments.length === 0 ||
    !record.architectureDocuments.every(isArchitectureAuthorityDoc)
  ) {
    errors.push("missing_architecture_authority");
  }

  for (const proposalId of record.proposalIds) {
    const proposal = proposalById.get(proposalId);
    if (!proposal) {
      errors.push("missing_proposal");
      continue;
    }
    const validationStatus = evaluateValidationContract(
      proposal.validation,
      proposal.opportunityIds,
      proposal.expectedImprovements.length,
    );
    if (validationStatus !== "complete") {
      errors.push("proposal_validation_incomplete");
    }
    if (proposal.opportunityIds.length === 0) {
      errors.push("orphan_opportunity");
    }
    if (proposal.validation.replaySessionIds.length === 0) {
      errors.push("orphan_replay");
    }
  }

  const evidenceIds = record.validationEvidence.evidenceSnapshotIds;
  if (evidenceIds.length === 0) {
    errors.push("missing_evidence");
  } else {
    for (const id of evidenceIds) {
      if (!evidenceSet.has(id)) {
        errors.push("missing_evidence");
        break;
      }
    }
  }

  if (!record.validationEvidence.proposalValidationComplete) {
    errors.push("missing_validation");
  }
  if (record.validationEvidence.architectureAuthorityIds.length === 0) {
    errors.push("missing_architecture_authority");
  }

  // Deduplicate errors while preserving order.
  const unique = [...new Set(errors)];
  return {
    status: unique.length === 0 ? "complete" : "incomplete",
    errors: unique,
  };
}

/**
 * Full release traceability chain:
 * Commit → Proposal → Opportunity → Evidence → Replay → Interaction traces
 */
export function buildReleaseTraceability(
  record: EngineeringChangeRecord,
  context: EngineeringConsistencyContext,
): ReleaseTraceability {
  const missing: TraceabilityGap[] = [];
  const proposalById = new Map(
    context.proposals.map((p) => [p.proposalId, p]),
  );

  if (record.commits.length === 0) {
    missing.push("missing_commit");
  }
  if (record.proposalIds.length === 0) {
    missing.push("missing_proposal");
  }

  const opportunityIds = new Set<string>();
  const evidenceSnapshotIds = new Set<string>();
  const replaySessionIds = new Set<string>();

  for (const proposalId of record.proposalIds) {
    const proposal = proposalById.get(proposalId);
    if (!proposal) {
      missing.push("missing_proposal");
      continue;
    }
    const validationStatus = evaluateValidationContract(
      proposal.validation,
      proposal.opportunityIds,
      proposal.expectedImprovements.length,
    );
    if (validationStatus !== "complete") {
      missing.push("incomplete_proposal_validation");
    }
    for (const id of proposal.opportunityIds) {
      opportunityIds.add(id);
    }
    for (const id of proposal.evidenceSnapshotIds) {
      evidenceSnapshotIds.add(id);
    }
    for (const id of proposal.validation.replaySessionIds) {
      replaySessionIds.add(id);
    }
  }

  for (const id of record.validationEvidence.opportunityIds) {
    opportunityIds.add(id);
  }
  for (const id of record.validationEvidence.evidenceSnapshotIds) {
    evidenceSnapshotIds.add(id);
  }
  for (const id of record.validationEvidence.replaySessionIds) {
    replaySessionIds.add(id);
  }

  if (opportunityIds.size === 0) {
    missing.push("missing_opportunity");
  }
  if (evidenceSnapshotIds.size === 0) {
    missing.push("missing_evidence");
  }
  if (replaySessionIds.size === 0) {
    missing.push("missing_replay");
  }

  const uniqueMissing = [...new Set(missing)];
  const interactionSessionIds = [...replaySessionIds].sort((a, b) =>
    a.localeCompare(b),
  );

  return {
    changeId: record.changeId,
    commits: [...record.commits].sort((a, b) => a.localeCompare(b)),
    proposalIds: [...record.proposalIds].sort((a, b) => a.localeCompare(b)),
    opportunityIds: [...opportunityIds].sort((a, b) => a.localeCompare(b)),
    evidenceSnapshotIds: [...evidenceSnapshotIds].sort((a, b) =>
      a.localeCompare(b),
    ),
    replaySessionIds: [...replaySessionIds].sort((a, b) => a.localeCompare(b)),
    interactionSessionIds,
    complete: uniqueMissing.length === 0,
    missing: uniqueMissing,
  };
}

/**
 * Build a draft engineering record from proposals.
 * Reuses proposal / evidence / opportunity / replay identifiers — does not mint duplicates.
 */
export function buildEngineeringRecordFromProposals(
  proposals: ExperienceChangeProposal[],
  options: {
    commits: string[];
    architectureDocuments: ArchitectureAuthorityDoc[];
    affectedModules: string[];
    affectedTests: string[];
    releaseImpact?: ReleaseImpact;
  },
): EngineeringChangeRecord | null {
  if (proposals.length === 0) {
    return null;
  }

  const proposalIds = [
    ...new Set(proposals.map((p) => p.proposalId)),
  ].sort((a, b) => a.localeCompare(b));
  const commits = [
    ...new Set(options.commits.filter((c) => COMMIT_RE.test(c))),
  ].sort((a, b) => a.localeCompare(b));
  const architectureDocuments = [
    ...new Set(
      options.architectureDocuments.filter(isArchitectureAuthorityDoc),
    ),
  ].sort((a, b) => a.localeCompare(b)) as ArchitectureAuthorityDoc[];
  const affectedModules = [
    ...new Set(options.affectedModules.filter((m) => PATH_RE.test(m))),
  ].sort((a, b) => a.localeCompare(b));
  const affectedTests = [
    ...new Set(options.affectedTests.filter((t) => PATH_RE.test(t))),
  ].sort((a, b) => a.localeCompare(b));

  const opportunityIds = [
    ...new Set(proposals.flatMap((p) => p.opportunityIds)),
  ].sort((a, b) => a.localeCompare(b));
  const evidenceSnapshotIds = [
    ...new Set(proposals.flatMap((p) => p.evidenceSnapshotIds)),
  ].sort((a, b) => a.localeCompare(b));
  const replaySessionIds = [
    ...new Set(proposals.flatMap((p) => p.validation.replaySessionIds)),
  ].sort((a, b) => a.localeCompare(b));

  if (
    proposalIds.length === 0 ||
    commits.length === 0 ||
    architectureDocuments.length === 0 ||
    affectedModules.length === 0 ||
    affectedTests.length === 0 ||
    evidenceSnapshotIds.length === 0 ||
    replaySessionIds.length === 0 ||
    opportunityIds.length === 0
  ) {
    return null;
  }

  const proposalValidationComplete = proposals.every((p) => {
    return (
      evaluateValidationContract(
        p.validation,
        p.opportunityIds,
        p.expectedImprovements.length,
      ) === "complete"
    );
  });

  const validationEvidence: EngineeringValidationEvidence = {
    proposalIds: [...proposalIds],
    evidenceSnapshotIds: [...evidenceSnapshotIds],
    opportunityIds: [...opportunityIds],
    replaySessionIds: [...replaySessionIds],
    proposalValidationComplete,
    architectureAuthorityIds: [...architectureDocuments],
  };

  const changeId = `eng-${fnv1a(
    `${proposalIds.join(",")}|${commits.join(",")}|${architectureDocuments.join(",")}`,
  )}`;

  const record: EngineeringChangeRecord = {
    schemaVersion: 1,
    changeId,
    proposalIds,
    commits,
    architectureDocuments,
    affectedModules,
    affectedTests,
    validationEvidence,
    releaseImpact: options.releaseImpact ?? "dev_tooling",
    state: "draft",
    consistencyStatus: "incomplete",
  };

  // Consistency against the provided proposals; evidence ids assumed present when building.
  const consistency = verifyEngineeringConsistency(record, {
    proposals,
    evidenceIds: evidenceSnapshotIds,
  });
  record.consistencyStatus = consistency.status;
  return record;
}

export function loadEngineeringBundle(
  store: ExperienceStoreAdapter,
): EngineeringBundle {
  const parsed = loadJsonBundle(
    store,
    ENGINEERING_STORAGE_KEY,
    emptyBundle,
    (value): value is EngineeringBundle =>
      !!value &&
      typeof value === "object" &&
      (value as EngineeringBundle).schemaVersion === 1 &&
      Array.isArray((value as EngineeringBundle).records) &&
      Array.isArray((value as EngineeringBundle).history),
  );
  return {
    schemaVersion: 1,
    records: parsed.records.slice(-MAX_ENGINEERING_RECORDS),
    history: parsed.history.slice(-MAX_ENGINEERING_HISTORY),
  };
}

function saveEngineeringBundle(
  store: ExperienceStoreAdapter,
  bundle: EngineeringBundle,
): void {
  saveJsonBundle(store, ENGINEERING_STORAGE_KEY, {
    schemaVersion: 1,
    records: bundle.records.slice(-MAX_ENGINEERING_RECORDS),
    history: bundle.history.slice(-MAX_ENGINEERING_HISTORY),
  });
}

function freezeEntry(entry: EngineeringHistoryEntry): EngineeringHistoryEntry {
  return Object.freeze({ ...entry, schemaVersion: 1 as const });
}

function nextHistorySeq(bundle: EngineeringBundle, changeId: string): number {
  let max = 0;
  for (const entry of bundle.history) {
    if (entry.changeId === changeId && entry.seq > max) {
      max = entry.seq;
    }
  }
  return max + 1;
}

function appendHistory(
  bundle: EngineeringBundle,
  entry: EngineeringHistoryEntry,
): void {
  bundle.history = [...bundle.history, freezeEntry(entry)];
}

export function listEngineeringRecords(
  store: ExperienceStoreAdapter,
): EngineeringChangeRecord[] {
  return loadEngineeringBundle(store).records.map((r) => ({ ...r }));
}

export function getEngineeringRecord(
  store: ExperienceStoreAdapter,
  changeId: string,
): EngineeringChangeRecord | null {
  const found = loadEngineeringBundle(store).records.find(
    (r) => r.changeId === changeId,
  );
  return found ? { ...found } : null;
}

export function listEngineeringHistory(
  store: ExperienceStoreAdapter,
  changeId: string,
): EngineeringHistoryEntry[] {
  return loadEngineeringBundle(store)
    .history.filter((h) => h.changeId === changeId)
    .map((h) => ({ ...h }));
}

export function listAllEngineeringHistory(
  store: ExperienceStoreAdapter,
): EngineeringHistoryEntry[] {
  return loadEngineeringBundle(store).history.map((h) => ({ ...h }));
}

/**
 * Persist a draft record. Rejects incomplete consistency (does not store).
 * Idempotent on changeId.
 */
export function persistEngineeringRecord(
  store: ExperienceStoreAdapter,
  record: EngineeringChangeRecord,
  context: EngineeringConsistencyContext,
  options?: { now?: number },
):
  | { ok: true; record: EngineeringChangeRecord }
  | { ok: false; error: "consistency_incomplete"; consistency: ConsistencyResult } {
  const bundle = loadEngineeringBundle(store);
  const existing = bundle.records.find((r) => r.changeId === record.changeId);
  if (existing) {
    return { ok: true, record: { ...existing } };
  }

  const consistency = verifyEngineeringConsistency(record, context);
  if (consistency.status !== "complete") {
    return { ok: false, error: "consistency_incomplete", consistency };
  }

  const authority =
    record.architectureDocuments[0] ?? "46_Engineering_Governance.md";
  const stored: EngineeringChangeRecord = {
    ...record,
    consistencyStatus: "complete",
    state: "draft",
  };

  bundle.records = [...bundle.records, stored];
  const seq = nextHistorySeq(bundle, stored.changeId);
  appendHistory(bundle, {
    schemaVersion: 1,
    entryId: `ehist-${stored.changeId}-${seq}`,
    changeId: stored.changeId,
    seq,
    t: options?.now ?? Date.now(),
    previousState: null,
    newState: "draft",
    reason: "created_from_proposals",
    authorityReference: authority,
  });
  saveEngineeringBundle(store, bundle);
  return { ok: true, record: { ...stored } };
}

export type EngineeringTransitionResult =
  | { ok: true; record: EngineeringChangeRecord }
  | { ok: false; error: EngineeringTransitionError };

/**
 * Manual lifecycle transition. No automatic advancement.
 * `released` requires complete consistency and complete release traceability.
 */
export function transitionEngineeringRecord(
  store: ExperienceStoreAdapter,
  changeId: string,
  nextState: EngineeringLifecycle,
  context: EngineeringConsistencyContext,
  options: {
    authorityReference: ArchitectureAuthorityDoc;
    reason?: EngineeringReason;
    now?: number;
  },
): EngineeringTransitionResult {
  const bundle = loadEngineeringBundle(store);
  const index = bundle.records.findIndex((r) => r.changeId === changeId);
  if (index < 0) {
    return { ok: false, error: "not_found" };
  }

  const current = bundle.records[index]!;
  if (current.state === "released") {
    return { ok: false, error: "released" };
  }
  if (!isEngineeringTransitionAllowed(current.state, nextState)) {
    return { ok: false, error: "invalid_transition" };
  }
  if (!isArchitectureAuthorityDoc(options.authorityReference)) {
    return { ok: false, error: "consistency_incomplete" };
  }

  const consistency = verifyEngineeringConsistency(current, context);
  if (consistency.status !== "complete") {
    return { ok: false, error: "consistency_incomplete" };
  }

  if (nextState === "released") {
    const trace = buildReleaseTraceability(current, context);
    if (!trace.complete) {
      return { ok: false, error: "traceability_incomplete" };
    }
  }

  const reason =
    options.reason ??
    REASON_FOR_TRANSITION[`${current.state}->${nextState}`] ??
    "mark_implemented";

  const updated: EngineeringChangeRecord = {
    ...current,
    state: nextState,
    consistencyStatus: "complete",
  };

  const records = [...bundle.records];
  records[index] = updated;
  bundle.records = records;

  const seq = nextHistorySeq(bundle, changeId);
  appendHistory(bundle, {
    schemaVersion: 1,
    entryId: `ehist-${changeId}-${seq}`,
    changeId,
    seq,
    t: options.now ?? Date.now(),
    previousState: current.state,
    newState: nextState,
    reason,
    authorityReference: options.authorityReference,
  });

  saveEngineeringBundle(store, bundle);
  return { ok: true, record: { ...updated } };
}

export function assertEngineeringHistoryImmutable(
  before: EngineeringHistoryEntry[],
  after: EngineeringHistoryEntry[],
): boolean {
  if (after.length < before.length) {
    return false;
  }
  for (let i = 0; i < before.length; i++) {
    const a = before[i]!;
    const b = after[i]!;
    if (
      a.entryId !== b.entryId ||
      a.seq !== b.seq ||
      a.previousState !== b.previousState ||
      a.newState !== b.newState ||
      a.reason !== b.reason ||
      a.authorityReference !== b.authorityReference ||
      a.t !== b.t
    ) {
      return false;
    }
  }
  return true;
}

/** True when every released record has complete traceability (no orphans). */
export function assertNoOrphanReleasedRecords(
  store: ExperienceStoreAdapter,
  context: EngineeringConsistencyContext,
): boolean {
  const released = listEngineeringRecords(store).filter(
    (r) => r.state === "released",
  );
  return released.every(
    (r) => buildReleaseTraceability(r, context).complete,
  );
}

export function clearEngineeringStore(store: ExperienceStoreAdapter): void {
  clearJsonKey(store, ENGINEERING_STORAGE_KEY);
}

export function defaultEngineeringStore(): ExperienceStoreAdapter {
  return browserStore() ?? memoryStore();
}
