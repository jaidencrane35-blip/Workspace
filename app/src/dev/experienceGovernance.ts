/**
 * Experience Change Governance — deterministic proposals between evidence and implementation.
 * No automatic promotion. No AI decision-making. No user content.
 */

import { fnv1a } from "./devHash";
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
import type { ExperienceEvidenceMetrics, MetricDirection } from "./experienceEvidence";
import type {
  ExperienceOpportunity,
  OpportunityWorkflow,
} from "./experienceImprovement";

export const GOVERNANCE_STORAGE_KEY = "ws.dev.experience.governance.v1";
export const MAX_PROPOSALS = 50;
export const MAX_HISTORY_ENTRIES = 2000;

export type ProposalLifecycle =
  | "draft"
  | "review"
  | "accepted"
  | "implemented"
  | "validated"
  | "closed";

export type ImplementationScope =
  | "dev_tooling"
  | "experience_surface"
  | "instrumentation"
  | "documentation"
  | "none";

export type AffectedComponent =
  | "home"
  | "save"
  | "resume"
  | "pilot"
  | "help"
  | "shell"
  | "dock"
  | "dev_overlay"
  | "unknown";

export type ValidationCriterion =
  | "has_evidence_baseline"
  | "has_replay_references"
  | "has_success_metric"
  | "has_expected_improvements"
  | "opportunities_linked";

export type GovernanceReason =
  | "created_from_opportunities"
  | "submit_for_review"
  | "accept"
  | "reject_to_draft"
  | "mark_implemented"
  | "mark_validated"
  | "close";

export type ValidationStatus = "incomplete" | "complete";

export type TransitionError =
  | "not_found"
  | "invalid_transition"
  | "validation_incomplete"
  | "missing_evidence"
  | "missing_replay"
  | "closed";

export interface ExpectedMetricImprovement {
  metric: keyof ExperienceEvidenceMetrics;
  direction: MetricDirection;
  /** Positive magnitude of expected improvement. */
  targetDelta: number;
  baselineValue: number;
}

export interface ProposalValidationContract {
  evidenceBaselineId: string;
  replaySessionIds: string[];
  criteria: ValidationCriterion[];
  successMetric: keyof ExperienceEvidenceMetrics;
  successDirection: MetricDirection;
  /** Required improvement magnitude on successMetric vs baseline. */
  successThresholdDelta: number;
}

export interface ExperienceChangeProposal {
  schemaVersion: 1;
  proposalId: string;
  opportunityIds: string[];
  evidenceSnapshotIds: string[];
  workflows: OpportunityWorkflow[];
  affectedComponents: AffectedComponent[];
  expectedImprovements: ExpectedMetricImprovement[];
  confidence: number;
  implementationScope: ImplementationScope;
  state: ProposalLifecycle;
  validation: ProposalValidationContract;
  validationStatus: ValidationStatus;
}

/** Immutable history record — append-only; never mutated after write. */
export interface GovernanceHistoryEntry {
  schemaVersion: 1;
  entryId: string;
  proposalId: string;
  seq: number;
  /** Epoch ms supplied by caller (injectable for tests). */
  t: number;
  previousState: ProposalLifecycle | null;
  newState: ProposalLifecycle;
  reason: GovernanceReason;
  evidenceReferenceId: string;
}

interface GovernanceBundle {
  schemaVersion: 1;
  proposals: ExperienceChangeProposal[];
  history: GovernanceHistoryEntry[];
}

const LIFECYCLE_ORDER: readonly ProposalLifecycle[] = [
  "draft",
  "review",
  "accepted",
  "implemented",
  "validated",
  "closed",
] as const;

/** Manual transitions only — no automatic promotion. */
const ALLOWED_TRANSITIONS: Record<
  ProposalLifecycle,
  readonly ProposalLifecycle[]
> = {
  draft: ["review"],
  review: ["accepted", "draft"],
  accepted: ["implemented"],
  implemented: ["validated"],
  validated: ["closed"],
  closed: [],
};

const REASON_FOR_TRANSITION: Partial<
  Record<`${ProposalLifecycle}->${ProposalLifecycle}`, GovernanceReason>
> = {
  "draft->review": "submit_for_review",
  "review->accepted": "accept",
  "review->draft": "reject_to_draft",
  "accepted->implemented": "mark_implemented",
  "implemented->validated": "mark_validated",
  "validated->closed": "close",
};

const ID_RE = /^[a-z0-9_.:-]{1,96}$/i;

function emptyBundle(): GovernanceBundle {
  return { schemaVersion: 1, proposals: [], history: [] };
}

function metricDirection(
  metric: keyof ExperienceEvidenceMetrics,
): MetricDirection {
  if (metric === "recoverySuccessRate") {
    return "higher_better";
  }
  return "lower_better";
}

function workflowToComponents(
  workflow: OpportunityWorkflow,
): AffectedComponent[] {
  switch (workflow) {
    case "save":
      return ["save"];
    case "continue":
      return ["resume"];
    case "navigation":
      return ["dock", "shell"];
    case "confidence":
      return ["home", "shell"];
    case "recovery":
      return ["resume"];
    case "replay":
      return ["dev_overlay"];
    case "friction":
      return ["shell"];
    default:
      return ["unknown"];
  }
}

function scopeForWorkflows(
  workflows: OpportunityWorkflow[],
): ImplementationScope {
  if (workflows.every((w) => w === "replay")) {
    return "dev_tooling";
  }
  if (workflows.includes("save") || workflows.includes("continue")) {
    return "experience_surface";
  }
  if (workflows.includes("friction") || workflows.includes("navigation")) {
    return "experience_surface";
  }
  return "none";
}

const REQUIRED_CRITERIA: readonly ValidationCriterion[] = [
  "has_evidence_baseline",
  "has_replay_references",
  "has_success_metric",
  "has_expected_improvements",
  "opportunities_linked",
] as const;

/**
 * A proposal cannot advance past draft unless the validation contract is complete.
 * Subjective fields are not consulted — only measurable linkage.
 */
export function evaluateValidationContract(
  validation: ProposalValidationContract,
  opportunityIds: string[],
  expectedImprovementCount: number,
): ValidationStatus {
  if (!validation.evidenceBaselineId || !ID_RE.test(validation.evidenceBaselineId)) {
    return "incomplete";
  }
  if (validation.replaySessionIds.length === 0) {
    return "incomplete";
  }
  if (!validation.successMetric || validation.successThresholdDelta <= 0) {
    return "incomplete";
  }
  if (expectedImprovementCount <= 0) {
    return "incomplete";
  }
  if (opportunityIds.length === 0) {
    return "incomplete";
  }
  for (const req of REQUIRED_CRITERIA) {
    if (!validation.criteria.includes(req)) {
      return "incomplete";
    }
  }
  return "complete";
}

/**
 * Build a draft proposal from opportunities.
 * Requires evidence + replay linkage. Does not persist. Does not auto-advance.
 */
export function buildProposalFromOpportunities(
  opportunities: ExperienceOpportunity[],
  options?: {
    implementationScope?: ImplementationScope;
    evidenceBaselineId?: string;
  },
): ExperienceChangeProposal | null {
  if (opportunities.length === 0) {
    return null;
  }

  const opportunityIds = [
    ...new Set(opportunities.map((o) => o.opportunityId)),
  ].sort((a, b) => a.localeCompare(b));
  const evidenceSnapshotIds = [
    ...new Set(opportunities.flatMap((o) => o.supportingEvidenceIds)),
  ].sort((a, b) => a.localeCompare(b));
  const replaySessionIds = [
    ...new Set(opportunities.flatMap((o) => o.replaySessionIds)),
  ].sort((a, b) => a.localeCompare(b));
  const workflows = [
    ...new Set(opportunities.map((o) => o.workflow)),
  ].sort((a, b) => a.localeCompare(b)) as OpportunityWorkflow[];

  if (evidenceSnapshotIds.length === 0 || replaySessionIds.length === 0) {
    return null;
  }

  const affectedComponents = [
    ...new Set(workflows.flatMap((w) => workflowToComponents(w))),
  ].sort((a, b) => a.localeCompare(b)) as AffectedComponent[];

  const expectedImprovements: ExpectedMetricImprovement[] = opportunities
    .map((o) => ({
      metric: o.metric,
      direction: metricDirection(o.metric),
      targetDelta: Number(Math.max(0, o.excess).toFixed(4)),
      baselineValue: o.observedValue,
    }))
    .filter((e) => e.targetDelta > 0)
    .sort((a, b) => a.metric.localeCompare(b.metric));

  // Dedupe by metric — keep max targetDelta.
  const byMetric = new Map<string, ExpectedMetricImprovement>();
  for (const item of expectedImprovements) {
    const prev = byMetric.get(item.metric);
    if (!prev || item.targetDelta > prev.targetDelta) {
      byMetric.set(item.metric, item);
    }
  }
  const uniqueImprovements = [...byMetric.values()].sort((a, b) =>
    a.metric.localeCompare(b.metric),
  );

  if (uniqueImprovements.length === 0) {
    return null;
  }

  const primary = uniqueImprovements[0]!;
  const evidenceBaselineId =
    options?.evidenceBaselineId && ID_RE.test(options.evidenceBaselineId)
      ? options.evidenceBaselineId
      : evidenceSnapshotIds[0]!;

  const validation: ProposalValidationContract = {
    evidenceBaselineId,
    replaySessionIds: [...replaySessionIds],
    criteria: [
      "has_evidence_baseline",
      "has_replay_references",
      "has_success_metric",
      "has_expected_improvements",
      "opportunities_linked",
    ],
    successMetric: primary.metric,
    successDirection: primary.direction,
    successThresholdDelta: primary.targetDelta,
  };

  const confidence = Number(
    (
      opportunities.reduce((sum, o) => sum + o.confidence, 0) /
      opportunities.length
    ).toFixed(4),
  );

  const proposalId = `prop-${fnv1a(
    `${opportunityIds.join(",")}|${evidenceSnapshotIds.join(",")}|${primary.metric}`,
  )}`;

  const validationStatus = evaluateValidationContract(
    validation,
    opportunityIds,
    uniqueImprovements.length,
  );

  return {
    schemaVersion: 1,
    proposalId,
    opportunityIds,
    evidenceSnapshotIds,
    workflows,
    affectedComponents,
    expectedImprovements: uniqueImprovements,
    confidence,
    implementationScope:
      options?.implementationScope ?? scopeForWorkflows(workflows),
    state: "draft",
    validation,
    validationStatus,
  };
}

export function isTransitionAllowed(
  from: ProposalLifecycle,
  to: ProposalLifecycle,
): boolean {
  return ALLOWED_TRANSITIONS[from].includes(to);
}

export function loadGovernanceBundle(
  store: ExperienceStoreAdapter,
): GovernanceBundle {
  const parsed = loadJsonBundle(
    store,
    GOVERNANCE_STORAGE_KEY,
    emptyBundle,
    (value): value is GovernanceBundle =>
      !!value &&
      typeof value === "object" &&
      (value as GovernanceBundle).schemaVersion === 1 &&
      Array.isArray((value as GovernanceBundle).proposals) &&
      Array.isArray((value as GovernanceBundle).history),
  );
  return {
    schemaVersion: 1,
    proposals: parsed.proposals.slice(-MAX_PROPOSALS),
    // History is append-only; keep tail if over cap (oldest dropped only by ring).
    history: parsed.history.slice(-MAX_HISTORY_ENTRIES),
  };
}

function saveGovernanceBundle(
  store: ExperienceStoreAdapter,
  bundle: GovernanceBundle,
): void {
  saveJsonBundle(store, GOVERNANCE_STORAGE_KEY, {
    schemaVersion: 1,
    proposals: bundle.proposals.slice(-MAX_PROPOSALS),
    history: bundle.history.slice(-MAX_HISTORY_ENTRIES),
  });
}

function freezeEntry(entry: GovernanceHistoryEntry): GovernanceHistoryEntry {
  return Object.freeze({ ...entry, schemaVersion: 1 as const });
}

function nextHistorySeq(
  bundle: GovernanceBundle,
  proposalId: string,
): number {
  let max = 0;
  for (const entry of bundle.history) {
    if (entry.proposalId === proposalId && entry.seq > max) {
      max = entry.seq;
    }
  }
  return max + 1;
}

function appendHistory(
  bundle: GovernanceBundle,
  entry: GovernanceHistoryEntry,
): void {
  // Never mutate prior entries — only concatenate.
  bundle.history = [...bundle.history, freezeEntry(entry)];
}

export function listProposals(
  store: ExperienceStoreAdapter,
): ExperienceChangeProposal[] {
  return loadGovernanceBundle(store).proposals.map((p) => ({ ...p }));
}

export function getProposal(
  store: ExperienceStoreAdapter,
  proposalId: string,
): ExperienceChangeProposal | null {
  const found = loadGovernanceBundle(store).proposals.find(
    (p) => p.proposalId === proposalId,
  );
  return found ? { ...found } : null;
}

/** Immutable history for a proposal (copy; source entries are frozen). */
export function listProposalHistory(
  store: ExperienceStoreAdapter,
  proposalId: string,
): GovernanceHistoryEntry[] {
  return loadGovernanceBundle(store)
    .history.filter((h) => h.proposalId === proposalId)
    .map((h) => ({ ...h }));
}

export function listAllHistory(
  store: ExperienceStoreAdapter,
): GovernanceHistoryEntry[] {
  return loadGovernanceBundle(store).history.map((h) => ({ ...h }));
}

/**
 * Persist a new draft proposal and append creation history.
 * Idempotent on proposalId (returns existing without rewriting history).
 */
export function persistProposal(
  store: ExperienceStoreAdapter,
  proposal: ExperienceChangeProposal,
  options?: { now?: number },
): ExperienceChangeProposal {
  const bundle = loadGovernanceBundle(store);
  const existing = bundle.proposals.find(
    (p) => p.proposalId === proposal.proposalId,
  );
  if (existing) {
    return { ...existing };
  }

  const validationStatus = evaluateValidationContract(
    proposal.validation,
    proposal.opportunityIds,
    proposal.expectedImprovements.length,
  );
  const stored: ExperienceChangeProposal = {
    ...proposal,
    validationStatus,
    state: "draft",
  };

  bundle.proposals = [...bundle.proposals, stored];
  const seq = nextHistorySeq(bundle, stored.proposalId);
  const t = options?.now ?? Date.now();
  appendHistory(bundle, {
    schemaVersion: 1,
    entryId: `hist-${stored.proposalId}-${seq}`,
    proposalId: stored.proposalId,
    seq,
    t,
    previousState: null,
    newState: "draft",
    reason: "created_from_opportunities",
    evidenceReferenceId: stored.validation.evidenceBaselineId,
  });
  saveGovernanceBundle(store, bundle);
  return { ...stored };
}

export type TransitionResult =
  | { ok: true; proposal: ExperienceChangeProposal }
  | { ok: false; error: TransitionError };

/**
 * Manually advance lifecycle. No automatic promotion.
 * Appends an immutable history entry on success.
 */
export function transitionProposal(
  store: ExperienceStoreAdapter,
  proposalId: string,
  nextState: ProposalLifecycle,
  options: {
    evidenceReferenceId: string;
    reason?: GovernanceReason;
    now?: number;
  },
): TransitionResult {
  const bundle = loadGovernanceBundle(store);
  const index = bundle.proposals.findIndex((p) => p.proposalId === proposalId);
  if (index < 0) {
    return { ok: false, error: "not_found" };
  }

  const current = bundle.proposals[index]!;
  if (current.state === "closed") {
    return { ok: false, error: "closed" };
  }
  if (!isTransitionAllowed(current.state, nextState)) {
    return { ok: false, error: "invalid_transition" };
  }

  const validationStatus = evaluateValidationContract(
    current.validation,
    current.opportunityIds,
    current.expectedImprovements.length,
  );
  // Advancing into review/accepted/validated requires a complete validation contract.
  const requiresValidation =
    nextState === "review" ||
    nextState === "accepted" ||
    nextState === "validated";
  if (requiresValidation && validationStatus !== "complete") {
    return { ok: false, error: "validation_incomplete" };
  }
  if (!current.validation.evidenceBaselineId) {
    return { ok: false, error: "missing_evidence" };
  }
  if (current.validation.replaySessionIds.length === 0) {
    return { ok: false, error: "missing_replay" };
  }
  if (!options.evidenceReferenceId || !ID_RE.test(options.evidenceReferenceId)) {
    return { ok: false, error: "missing_evidence" };
  }

  const reason =
    options.reason ??
    REASON_FOR_TRANSITION[`${current.state}->${nextState}`] ??
    "submit_for_review";

  const updated: ExperienceChangeProposal = {
    ...current,
    state: nextState,
    validationStatus,
  };

  const proposals = [...bundle.proposals];
  proposals[index] = updated;
  bundle.proposals = proposals;

  const seq = nextHistorySeq(bundle, proposalId);
  appendHistory(bundle, {
    schemaVersion: 1,
    entryId: `hist-${proposalId}-${seq}`,
    proposalId,
    seq,
    t: options.now ?? Date.now(),
    previousState: current.state,
    newState: nextState,
    reason,
    evidenceReferenceId: options.evidenceReferenceId,
  });

  saveGovernanceBundle(store, bundle);
  return { ok: true, proposal: { ...updated } };
}

/** Verify history entries for a proposal were never rewritten (seq contiguous, entryIds stable). */
export function assertHistoryImmutable(
  before: GovernanceHistoryEntry[],
  after: GovernanceHistoryEntry[],
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
      a.evidenceReferenceId !== b.evidenceReferenceId ||
      a.t !== b.t
    ) {
      return false;
    }
  }
  return true;
}

export function clearGovernanceStore(store: ExperienceStoreAdapter): void {
  clearJsonKey(store, GOVERNANCE_STORAGE_KEY);
}

export function defaultGovernanceStore(): ExperienceStoreAdapter {
  return browserStore() ?? memoryStore();
}

export function lifecycleIndex(state: ProposalLifecycle): number {
  return LIFECYCLE_ORDER.indexOf(state);
}
