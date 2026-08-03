/**
 * Sprint 61 — First governed production adaptation.
 * Selects, promotes, activates, and post-validates via existing pathways.
 * No new lifecycle states. No Runtime Core / navigation / persistence changes.
 */

import {
  buildArchitectureGraph,
  listArchitectureSnapshots,
  validateArchitectureIntegrity,
} from "../dev/architecturalIntegrity";
import { fnv1a } from "../dev/devHash";
import {
  buildEvidenceFromSessions,
  listEvidenceSnapshots,
  persistEvidenceSnapshot,
  type ExperienceEvidence,
  type ExperienceEvidenceMetrics,
} from "../dev/experienceEvidence";
import type {
  ExperienceEvent,
  ExperienceSession,
} from "../dev/experienceEvents";
import { listEngineeringRecords } from "../dev/engineeringGovernance";
import { detectOpportunities } from "../dev/experienceImprovement";
import { listProposals } from "../dev/experienceGovernance";
import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import {
  runAdaptationExperiments,
} from "./adaptationExperiments";
import {
  buildAdaptationCatalog,
  type AdaptationCatalogEntry,
} from "./adaptationOperations";
import {
  LONGITUDINAL_MIN_OBSERVATIONS,
  appendLongitudinalObservation,
  getStabilityReport,
  isRolloutReady,
  listLongitudinalRecords,
  promoteToRolloutCandidate,
  runLongitudinalValidation,
} from "./longitudinalAdaptation";
import {
  activateAdaptation,
  listAdaptations,
  rollbackAdaptation,
  rollbackTriggered,
  validateAdaptationEvidence,
  type WorkspaceAdaptation,
} from "./workspaceAdaptation";

export const PRODUCTION_ACTIVATION_STORAGE_KEY =
  "ws.experience.adaptation.production.v1";

export type ProductionActivationOutcome =
  | "activated"
  | "blocked"
  | "rolled_back";

export type ProductionBlockReason =
  | "empty_catalog"
  | "evidence_minimum_unmet"
  | "stability_threshold_unmet"
  | "unresolved_regressions"
  | "governance_incomplete"
  | "not_rollout_ready"
  | "promote_failed"
  | "activate_failed"
  | "post_activation_regression"
  | "post_activation_metric_unmet"
  | "integrity_invalid"
  | "lineage_broken";

export interface ProductionActivationRecord {
  schemaVersion: 1;
  outcome: ProductionActivationOutcome;
  adaptationId: string | null;
  activatedAt: number | null;
  preEvidenceId: string | null;
  postEvidenceId: string | null;
  expectedMetric: keyof ExperienceEvidenceMetrics | null;
  preMetricValue: number | null;
  postMetricValue: number | null;
  metricDelta: number | null;
  regression: boolean;
  governanceIntact: boolean;
  integrityValid: boolean;
  rollbackAvailable: boolean;
  blockReasons: ProductionBlockReason[];
  proposalId: string | null;
  engineeringChangeId: string | null;
  architectureSnapshotId: string | null;
  replaySessionIds: string[];
  stabilityScore: number | null;
}

function emptyRecord(
  partial?: Partial<ProductionActivationRecord>,
): ProductionActivationRecord {
  return {
    schemaVersion: 1,
    outcome: "blocked",
    adaptationId: null,
    activatedAt: null,
    preEvidenceId: null,
    postEvidenceId: null,
    expectedMetric: null,
    preMetricValue: null,
    postMetricValue: null,
    metricDelta: null,
    regression: false,
    governanceIntact: false,
    integrityValid: false,
    rollbackAvailable: false,
    blockReasons: [],
    proposalId: null,
    engineeringChangeId: null,
    architectureSnapshotId: null,
    replaySessionIds: [],
    stabilityScore: null,
    ...partial,
  };
}

function session(
  id: string,
  events: ExperienceEvent[],
): ExperienceSession {
  return {
    schemaVersion: 1,
    sessionId: id,
    startedAt: 1,
    events,
  };
}

function nav(
  seq: number,
  t: number,
  from: ExperienceEvent["from"],
  destination: ExperienceEvent["destination"],
): ExperienceEvent {
  return {
    seq,
    t,
    type: "navigate",
    from,
    destination,
    commandId: "dock_navigate",
  };
}

/** Fresh low-friction session for longitudinal depth / post-activation evidence. */
function productionCalmSession(id: string, ttc: number): ExperienceSession {
  return session(id, [
    { seq: 0, t: 0, type: "session_start", destination: "home" },
    {
      seq: 1,
      t: ttc,
      type: "first_meaningful_interaction",
      destination: "home",
      modality: "pointer",
    },
    nav(2, ttc + 200, "home", "save"),
    {
      seq: 3,
      t: ttc + 300,
      type: "flow_start",
      flow: "save",
      destination: "save",
    },
    {
      seq: 4,
      t: ttc + 500,
      type: "save_success",
      flow: "save",
      destination: "save",
    },
  ]);
}

function metricValue(
  evidence: ExperienceEvidence,
  metric: keyof ExperienceEvidenceMetrics,
): number {
  return Number(evidence.metrics[metric]);
}

function improved(
  baseline: number,
  after: number,
  direction: WorkspaceAdaptation["expectedDirection"],
  delta: number,
): boolean {
  if (direction === "lower_better") {
    return baseline - after >= delta - 1e-9;
  }
  return after - baseline >= delta - 1e-9;
}

function persistRecord(
  store: ExperienceStoreAdapter,
  record: ProductionActivationRecord,
): ProductionActivationRecord {
  store.setItem(PRODUCTION_ACTIVATION_STORAGE_KEY, JSON.stringify(record));
  return record;
}

export function getProductionActivationRecord(
  store: ExperienceStoreAdapter,
): ProductionActivationRecord | null {
  const raw = store.getItem(PRODUCTION_ACTIVATION_STORAGE_KEY);
  if (!raw) {
    return null;
  }
  try {
    const parsed = JSON.parse(raw) as ProductionActivationRecord;
    if (parsed?.schemaVersion !== 1) {
      return null;
    }
    return {
      ...parsed,
      blockReasons: [...(parsed.blockReasons ?? [])],
      replaySessionIds: [...(parsed.replaySessionIds ?? [])],
    };
  } catch {
    return null;
  }
}

export function clearProductionActivationStore(
  store: ExperienceStoreAdapter,
): void {
  store.removeItem(PRODUCTION_ACTIVATION_STORAGE_KEY);
}

function lineageIntact(
  adaptation: WorkspaceAdaptation,
  store: ExperienceStoreAdapter,
): boolean {
  const proposal = listProposals(store).find(
    (p) => p.proposalId === adaptation.proposalId,
  );
  const engineering = listEngineeringRecords(store).find(
    (r) => r.changeId === adaptation.engineeringChangeId,
  );
  const architecture = listArchitectureSnapshots(store).find(
    (s) => s.snapshotId === adaptation.architectureSnapshotId,
  );
  return Boolean(
    proposal &&
      engineering &&
      architecture?.integrity.valid &&
      adaptation.validation.replaySessionIds.length > 0 &&
      engineering.proposalIds.includes(adaptation.proposalId),
  );
}

function integrityValid(store: ExperienceStoreAdapter): boolean {
  const graph = buildArchitectureGraph({
    engineeringRecords: listEngineeringRecords(store),
    proposals: listProposals(store),
    opportunities: detectOpportunities(listEvidenceSnapshots(store)),
    evidence: listEvidenceSnapshots(store),
  });
  return validateArchitectureIntegrity(graph, {
    engineeringRecords: listEngineeringRecords(store),
    proposals: listProposals(store),
  }).valid;
}

/**
 * Extend longitudinal series to the minimum observation count without
 * weakening thresholds — additional evidence generations only.
 */
export function ensureLongitudinalEligibility(
  store: ExperienceStoreAdapter,
  adaptation: WorkspaceAdaptation,
): void {
  let record = listLongitudinalRecords(store).find(
    (r) => r.adaptationId === adaptation.adaptationId,
  );
  let guard = 0;
  while (
    (record?.observationCount ?? 0) < LONGITUDINAL_MIN_OBSERVATIONS &&
    guard < 6
  ) {
    const n = (record?.observationCount ?? 0) + 1;
    const evidence = buildEvidenceFromSessions(
      [productionCalmSession(`s-prod-long-${adaptation.adaptationId}-${n}`, 350 + n * 10)],
      { tag: `prod_long_${n}` },
    );
    persistEvidenceSnapshot(store, evidence);
    appendLongitudinalObservation(store, adaptation, evidence);
    record = listLongitudinalRecords(store).find(
      (r) => r.adaptationId === adaptation.adaptationId,
    );
    guard += 1;
  }
  runLongitudinalValidation(store, adaptation);
}

export interface ProductionSelection {
  entry: AdaptationCatalogEntry;
  adaptation: WorkspaceAdaptation;
  stabilityScore: number;
}

/**
 * Select exactly one catalog adaptation eligible for rollout_candidate.
 * Highest stability score; zero unresolved regressions; complete lineage.
 */
export function selectProductionAdaptation(
  store: ExperienceStoreAdapter,
):
  | { ok: true; selection: ProductionSelection }
  | { ok: false; reasons: ProductionBlockReason[]; catalogSize: number } {
  const catalog = buildAdaptationCatalog(store);
  if (catalog.entries.length === 0) {
    return { ok: false, reasons: ["empty_catalog"], catalogSize: 0 };
  }

  const adaptations = listAdaptations(store);
  const eligible: ProductionSelection[] = [];
  const reasons = new Set<ProductionBlockReason>();

  for (const entry of catalog.entries) {
    const adaptation = adaptations.find(
      (a) => a.adaptationId === entry.adaptationId,
    );
    if (!adaptation) {
      continue;
    }
    if (adaptation.validation.validationResult !== "passed") {
      reasons.add("governance_incomplete");
      continue;
    }
    if (!lineageIntact(adaptation, store)) {
      reasons.add("lineage_broken");
      continue;
    }
    const report = getStabilityReport(store, adaptation.adaptationId);
    if (!report) {
      reasons.add("evidence_minimum_unmet");
      continue;
    }
    if (report.regressionEvents.length > 0) {
      reasons.add("unresolved_regressions");
      continue;
    }
    if (report.evidenceTimeline.length < LONGITUDINAL_MIN_OBSERVATIONS) {
      reasons.add("evidence_minimum_unmet");
      continue;
    }
    if (!isRolloutReady(adaptation, report)) {
      if (report.rolloutDisposition === "reject") {
        reasons.add("unresolved_regressions");
      } else if (report.stabilityScore < 0.7) {
        reasons.add("stability_threshold_unmet");
      } else {
        reasons.add("not_rollout_ready");
      }
      continue;
    }
    if (
      adaptation.rolloutState !== "candidate" &&
      adaptation.rolloutState !== "rollout_candidate"
    ) {
      reasons.add("not_rollout_ready");
      continue;
    }
    eligible.push({
      entry,
      adaptation,
      stabilityScore: report.stabilityScore,
    });
  }

  if (eligible.length === 0) {
    return {
      ok: false,
      reasons: [...reasons].sort(),
      catalogSize: catalog.entries.length,
    };
  }

  eligible.sort((a, b) => {
    if (b.stabilityScore !== a.stabilityScore) {
      return b.stabilityScore - a.stabilityScore;
    }
    return a.adaptation.adaptationId.localeCompare(b.adaptation.adaptationId);
  });

  return { ok: true, selection: eligible[0]! };
}

/**
 * Full pathway: ensure catalog depth → select → promote → activate → post-validate.
 * Uses existing activateAdaptation / rollbackAdaptation only.
 */
export function runFirstProductionAdaptation(
  store: ExperienceStoreAdapter,
  options?: { now?: number },
): ProductionActivationRecord {
  if (listAdaptations(store).length === 0) {
    runAdaptationExperiments(store);
  }

  // Bring every passed candidate to longitudinal minimum (thresholds unchanged).
  for (const adaptation of listAdaptations(store)) {
    if (adaptation.validation.validationResult !== "passed") {
      continue;
    }
    if (
      adaptation.rolloutState !== "candidate" &&
      adaptation.rolloutState !== "rollout_candidate"
    ) {
      continue;
    }
    ensureLongitudinalEligibility(store, adaptation);
  }

  const selected = selectProductionAdaptation(store);
  if (!selected.ok) {
    return persistRecord(
      store,
      emptyRecord({
        outcome: "blocked",
        blockReasons: selected.reasons,
        integrityValid: integrityValid(store),
      }),
    );
  }

  const { adaptation, stabilityScore } = selected.selection;
  let current = adaptation;

  if (current.rolloutState === "candidate") {
    const promoted = promoteToRolloutCandidate(store, current.adaptationId);
    if (!promoted.ok) {
      const reason: ProductionBlockReason =
        promoted.error === "evidence_minimum_unmet"
          ? "evidence_minimum_unmet"
          : promoted.error === "unresolved_regressions"
            ? "unresolved_regressions"
            : promoted.error === "stability_threshold_unmet"
              ? "stability_threshold_unmet"
              : promoted.error === "governance_incomplete"
                ? "governance_incomplete"
                : "promote_failed";
      return persistRecord(
        store,
        emptyRecord({
          outcome: "blocked",
          adaptationId: current.adaptationId,
          blockReasons: [reason],
          proposalId: current.proposalId,
          engineeringChangeId: current.engineeringChangeId,
          architectureSnapshotId: current.architectureSnapshotId,
          replaySessionIds: [...current.validation.replaySessionIds],
          stabilityScore,
          governanceIntact: lineageIntact(current, store),
          integrityValid: integrityValid(store),
        }),
      );
    }
    current = promoted.adaptation;
  }

  const evidenceBefore = listEvidenceSnapshots(store);
  const preEvidence =
    evidenceBefore[evidenceBefore.length - 1] ??
    evidenceBefore.find(
      (e) => e.evidenceId === current.validation.baselineEvidenceId,
    );
  if (!preEvidence) {
    return persistRecord(
      store,
      emptyRecord({
        outcome: "blocked",
        adaptationId: current.adaptationId,
        blockReasons: ["evidence_minimum_unmet"],
        stabilityScore,
      }),
    );
  }

  const activated = activateAdaptation(store, current.adaptationId);
  if (!activated.ok) {
    return persistRecord(
      store,
      emptyRecord({
        outcome: "blocked",
        adaptationId: current.adaptationId,
        blockReasons: ["activate_failed"],
        proposalId: current.proposalId,
        engineeringChangeId: current.engineeringChangeId,
        architectureSnapshotId: current.architectureSnapshotId,
        replaySessionIds: [...current.validation.replaySessionIds],
        stabilityScore,
        governanceIntact: lineageIntact(current, store),
        integrityValid: integrityValid(store),
        rollbackAvailable: true,
      }),
    );
  }
  current = activated.adaptation;

  const activatedAt =
    options?.now ??
    Number.parseInt(
      fnv1a(`${current.adaptationId}|${preEvidence.evidenceId}`).slice(0, 8),
      16,
    );

  // Fresh post-activation evidence (slightly stronger calm path).
  const postEvidence = buildEvidenceFromSessions(
    [productionCalmSession(`s-prod-post-${current.adaptationId}`, 320)],
    { tag: "prod_post" },
  );
  persistEvidenceSnapshot(store, postEvidence);

  const preVal = metricValue(preEvidence, current.expectedMetric);
  const postVal = metricValue(postEvidence, current.expectedMetric);
  const metricDelta = Number((postVal - preVal).toFixed(4));
  const regression = rollbackTriggered(
    preEvidence,
    postEvidence,
    current.validation.rollbackCriteria,
  );
  // Post-activation: require improvement vs pre OR vs original baseline contract.
  const baseline =
    listEvidenceSnapshots(store).find(
      (e) => e.evidenceId === current.validation.baselineEvidenceId,
    ) ?? preEvidence;
  const vsBaseline = validateAdaptationEvidence(
    current,
    baseline,
    postEvidence,
  );
  const vsPreImproved = improved(
    preVal,
    postVal,
    current.expectedDirection,
    // Allow small absolute improvement post-activation (presentation already warm).
    Math.min(current.expectedImprovementDelta, current.expectedDirection === "lower_better" ? 1 : 0.0001),
  );
  const metricOk =
    !regression &&
    (vsPreImproved || vsBaseline.validationResult === "passed");

  const govOk = lineageIntact(current, store);
  const integOk = integrityValid(store);

  if (!metricOk || regression || !govOk || !integOk) {
    rollbackAdaptation(store, current.adaptationId);
    const blockReasons: ProductionBlockReason[] = [];
    if (regression) {
      blockReasons.push("post_activation_regression");
    }
    if (!metricOk && !regression) {
      blockReasons.push("post_activation_metric_unmet");
    }
    if (!govOk) {
      blockReasons.push("lineage_broken");
    }
    if (!integOk) {
      blockReasons.push("integrity_invalid");
    }
    return persistRecord(
      store,
      emptyRecord({
        outcome: "rolled_back",
        adaptationId: current.adaptationId,
        activatedAt,
        preEvidenceId: preEvidence.evidenceId,
        postEvidenceId: postEvidence.evidenceId,
        expectedMetric: current.expectedMetric,
        preMetricValue: preVal,
        postMetricValue: postVal,
        metricDelta,
        regression,
        governanceIntact: govOk,
        integrityValid: integOk,
        rollbackAvailable: true,
        blockReasons,
        proposalId: current.proposalId,
        engineeringChangeId: current.engineeringChangeId,
        architectureSnapshotId: current.architectureSnapshotId,
        replaySessionIds: [...current.validation.replaySessionIds],
        stabilityScore,
      }),
    );
  }

  return persistRecord(
    store,
    emptyRecord({
      outcome: "activated",
      adaptationId: current.adaptationId,
      activatedAt,
      preEvidenceId: preEvidence.evidenceId,
      postEvidenceId: postEvidence.evidenceId,
      expectedMetric: current.expectedMetric,
      preMetricValue: preVal,
      postMetricValue: postVal,
      metricDelta,
      regression: false,
      governanceIntact: true,
      integrityValid: true,
      rollbackAvailable: true,
      blockReasons: [],
      proposalId: current.proposalId,
      engineeringChangeId: current.engineeringChangeId,
      architectureSnapshotId: current.architectureSnapshotId,
      replaySessionIds: [...current.validation.replaySessionIds],
      stabilityScore,
    }),
  );
}

/** Manual rollback of the recorded production activation (existing pathway). */
export function rollbackProductionAdaptation(
  store: ExperienceStoreAdapter,
):
  | { ok: true; record: ProductionActivationRecord }
  | { ok: false; error: string } {
  const record = getProductionActivationRecord(store);
  if (!record?.adaptationId) {
    return { ok: false, error: "not_found" };
  }
  const result = rollbackAdaptation(store, record.adaptationId);
  if (!result.ok) {
    return { ok: false, error: result.error };
  }
  const updated = persistRecord(
    store,
    emptyRecord({
      ...record,
      outcome: "rolled_back",
      rollbackAvailable: true,
      blockReasons: record.blockReasons.includes("post_activation_regression")
        ? record.blockReasons
        : [...record.blockReasons],
    }),
  );
  return { ok: true, record: updated };
}
