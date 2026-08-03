/**
 * Sprint 59 — Longitudinal adaptation validation.
 * Stability across multiple ExperienceEvidence snapshots.
 * No new adaptation primitives. No Runtime Core / navigation / persistence changes.
 */

import {
  compareEvidence,
  listEvidenceSnapshots,
  type ExperienceEvidence,
  type ExperienceEvidenceMetrics,
} from "../dev/experienceEvidence";
import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import {
  loadAdaptationBundle,
  rollbackTriggered,
  saveAdaptationBundle,
  upsertValidatedAdaptation,
  type AdaptationStabilityReport,
  type AdaptationTransitionError,
  type LongitudinalAdaptationRecord,
  type WorkspaceAdaptation,
} from "./workspaceAdaptation";

export const LONGITUDINAL_MIN_OBSERVATIONS = 3;
export const LONGITUDINAL_STABILITY_THRESHOLD = 0.7;
export const LONGITUDINAL_MAX_UNRESOLVED_REGRESSIONS = 0;

export type {
  AdaptationStabilityReport,
  LongitudinalAdaptationRecord,
};

export type LongitudinalTransitionError =
  | AdaptationTransitionError
  | "stability_threshold_unmet"
  | "unresolved_regressions"
  | "evidence_minimum_unmet"
  | "governance_incomplete";

function clamp01(n: number): number {
  return Math.min(1, Math.max(0, n));
}

function round4(n: number): number {
  return Number(n.toFixed(4));
}

function metricValue(
  evidence: ExperienceEvidence,
  metric: keyof ExperienceEvidenceMetrics,
): number {
  return Number(evidence.metrics[metric]);
}

function confidenceOf(evidence: ExperienceEvidence): number {
  return round4(clamp01(evidence.metrics.sessionCount / 4));
}

function improvedEnough(
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

function populationVariance(values: number[]): number {
  if (values.length === 0) {
    return 0;
  }
  const mean = values.reduce((a, b) => a + b, 0) / values.length;
  const sumSq = values.reduce((a, b) => a + (b - mean) ** 2, 0);
  return sumSq / values.length;
}

function resolveTimeline(
  adaptation: WorkspaceAdaptation,
  evidenceById: Map<string, ExperienceEvidence>,
  orderedIds: string[],
): ExperienceEvidence[] {
  const timeline: ExperienceEvidence[] = [];
  for (const id of orderedIds) {
    const snap = evidenceById.get(id);
    if (snap) {
      timeline.push(snap);
    }
  }
  if (timeline.length === 0) {
    const baseline = evidenceById.get(adaptation.validation.baselineEvidenceId);
    if (baseline) {
      timeline.push(baseline);
    }
  }
  return timeline;
}

/**
 * Deterministic stability analysis over an evidence timeline.
 * Metrics are read from ExperienceEvidence — never duplicated.
 */
export function analyzeAdaptationStability(
  adaptation: WorkspaceAdaptation,
  timeline: ExperienceEvidence[],
): AdaptationStabilityReport {
  const evidenceTimeline = timeline.map((e) => e.evidenceId);
  if (timeline.length === 0) {
    return {
      schemaVersion: 1,
      adaptationId: adaptation.adaptationId,
      sampleCount: 0,
      stabilityScore: 0,
      confidenceTrend: "stable",
      regressionEvents: [],
      rolloutDisposition: "reject",
      metricVariance: 0,
      improvementConsistency: 0,
      regressionFrequency: 0,
      confidenceEvolution: [],
      evidenceTimeline,
    };
  }

  const baseline = timeline[0]!;
  const samples = timeline.slice(1);
  const sampleCount = samples.length;
  const baseVal = metricValue(baseline, adaptation.expectedMetric);
  const sampleValues = samples.map((s) =>
    metricValue(s, adaptation.expectedMetric),
  );

  const confidenceEvolution = timeline.map(confidenceOf);
  let improvedCount = 0;
  const regressionEvents: AdaptationStabilityReport["regressionEvents"] = [];

  for (const sample of samples) {
    const afterVal = metricValue(sample, adaptation.expectedMetric);
    if (
      improvedEnough(
        baseVal,
        afterVal,
        adaptation.expectedDirection,
        adaptation.expectedImprovementDelta,
      )
    ) {
      improvedCount += 1;
    }

    const rolled = rollbackTriggered(
      baseline,
      sample,
      adaptation.validation.rollbackCriteria,
    );
    const cmp = compareEvidence(baseline, sample);
    const metricCmp = cmp.metrics.find(
      (m) => m.key === adaptation.expectedMetric,
    );
    if (rolled || metricCmp?.verdict === "regressed") {
      let criterion: AdaptationStabilityReport["regressionEvents"][number]["criterion"] =
        "metric_regression";
      if (rolled) {
        for (const c of adaptation.validation.rollbackCriteria) {
          if (rollbackTriggered(baseline, sample, [c])) {
            criterion = c;
            break;
          }
        }
      }
      regressionEvents.push({
        evidenceId: sample.evidenceId,
        criterion,
        metric: adaptation.expectedMetric,
        baselineValue: baseVal,
        observedValue: afterVal,
      });
    }
  }

  const improvementConsistency =
    sampleCount === 0 ? 0 : round4(improvedCount / sampleCount);
  const rawVariance = populationVariance(sampleValues);
  const norm = Math.max(Math.abs(baseVal), 1e-6);
  const metricVariance = round4(clamp01(rawVariance / (norm * norm)));
  const regressionFrequency =
    sampleCount === 0
      ? 0
      : round4(regressionEvents.length / sampleCount);
  const stabilityScore = round4(
    clamp01(
      improvementConsistency * (1 - metricVariance) * (1 - regressionFrequency),
    ),
  );

  const half = Math.floor(confidenceEvolution.length / 2);
  const early =
    confidenceEvolution.slice(0, Math.max(half, 1)).reduce((a, b) => a + b, 0) /
    Math.max(half, 1);
  const lateVals = confidenceEvolution.slice(half);
  const late =
    lateVals.length === 0
      ? early
      : lateVals.reduce((a, b) => a + b, 0) / lateVals.length;
  let confidenceTrend: AdaptationStabilityReport["confidenceTrend"] = "stable";
  if (late - early > 0.05) {
    confidenceTrend = "rising";
  } else if (early - late > 0.05) {
    confidenceTrend = "falling";
  }

  const governanceComplete =
    adaptation.validation.validationResult === "passed" &&
    adaptation.validation.replaySessionIds.length > 0 &&
    Boolean(adaptation.engineeringChangeId) &&
    Boolean(adaptation.proposalId) &&
    Boolean(adaptation.architectureSnapshotId);

  const observationCount = timeline.length;
  const evidenceMinimumMet =
    observationCount >= LONGITUDINAL_MIN_OBSERVATIONS && sampleCount >= 2;
  const unresolved = regressionEvents.length;
  const stabilityMet = stabilityScore >= LONGITUDINAL_STABILITY_THRESHOLD;

  let rolloutDisposition: AdaptationStabilityReport["rolloutDisposition"] =
    "hold";
  if (!governanceComplete) {
    rolloutDisposition = "reject";
  } else if (unresolved > LONGITUDINAL_MAX_UNRESOLVED_REGRESSIONS) {
    rolloutDisposition = "reject";
  } else if (!evidenceMinimumMet || !stabilityMet) {
    rolloutDisposition = "hold";
  } else {
    rolloutDisposition = "rollout_candidate";
  }

  return {
    schemaVersion: 1,
    adaptationId: adaptation.adaptationId,
    sampleCount,
    stabilityScore,
    confidenceTrend,
    regressionEvents,
    rolloutDisposition,
    metricVariance,
    improvementConsistency,
    regressionFrequency,
    confidenceEvolution,
    evidenceTimeline,
  };
}

export function buildLongitudinalRecord(
  adaptation: WorkspaceAdaptation,
  timeline: ExperienceEvidence[],
  report: AdaptationStabilityReport,
): LongitudinalAdaptationRecord {
  const ids = timeline.map((e) => e.evidenceId);
  const baselineEvidenceId =
    ids[0] ?? adaptation.validation.baselineEvidenceId;
  const latestEvidenceId = ids[ids.length - 1] ?? baselineEvidenceId;
  const intermediateEvidenceIds = ids.slice(1, -1);
  return {
    schemaVersion: 1,
    adaptationId: adaptation.adaptationId,
    baselineEvidenceId,
    intermediateEvidenceIds,
    latestEvidenceId,
    observationCount: ids.length,
    stabilityScore: report.stabilityScore,
    regressionCount: report.regressionEvents.length,
  };
}

export function listLongitudinalRecords(
  store: ExperienceStoreAdapter,
): LongitudinalAdaptationRecord[] {
  return loadAdaptationBundle(store).longitudinalRecords.map((r) => ({
    ...r,
    intermediateEvidenceIds: [...r.intermediateEvidenceIds],
  }));
}

export function listStabilityReports(
  store: ExperienceStoreAdapter,
): AdaptationStabilityReport[] {
  return loadAdaptationBundle(store).stabilityReports.map((r) => ({
    ...r,
    regressionEvents: r.regressionEvents.map((e) => ({ ...e })),
    confidenceEvolution: [...r.confidenceEvolution],
    evidenceTimeline: [...r.evidenceTimeline],
  }));
}

export function getLongitudinalRecord(
  store: ExperienceStoreAdapter,
  adaptationId: string,
): LongitudinalAdaptationRecord | null {
  return (
    listLongitudinalRecords(store).find(
      (r) => r.adaptationId === adaptationId,
    ) ?? null
  );
}

export function getStabilityReport(
  store: ExperienceStoreAdapter,
  adaptationId: string,
): AdaptationStabilityReport | null {
  return (
    listStabilityReports(store).find((r) => r.adaptationId === adaptationId) ??
    null
  );
}

function upsertLongitudinalArtefacts(
  store: ExperienceStoreAdapter,
  record: LongitudinalAdaptationRecord,
  report: AdaptationStabilityReport,
): void {
  const bundle = loadAdaptationBundle(store);
  const recIdx = bundle.longitudinalRecords.findIndex(
    (r) => r.adaptationId === record.adaptationId,
  );
  if (recIdx >= 0) {
    const next = [...bundle.longitudinalRecords];
    next[recIdx] = record;
    bundle.longitudinalRecords = next;
  } else {
    bundle.longitudinalRecords = [...bundle.longitudinalRecords, record];
  }
  const repIdx = bundle.stabilityReports.findIndex(
    (r) => r.adaptationId === report.adaptationId,
  );
  if (repIdx >= 0) {
    const next = [...bundle.stabilityReports];
    next[repIdx] = report;
    bundle.stabilityReports = next;
  } else {
    bundle.stabilityReports = [...bundle.stabilityReports, report];
  }
  saveAdaptationBundle(store, bundle);
}

/**
 * Append an evidence snapshot to the adaptation's longitudinal series and re-analyse.
 */
export function appendLongitudinalObservation(
  store: ExperienceStoreAdapter,
  adaptation: WorkspaceAdaptation,
  evidence: ExperienceEvidence,
): {
  record: LongitudinalAdaptationRecord;
  report: AdaptationStabilityReport;
} {
  const existing = getLongitudinalRecord(store, adaptation.adaptationId);
  const evidenceById = new Map(
    listEvidenceSnapshots(store).map((e) => [e.evidenceId, e]),
  );
  evidenceById.set(evidence.evidenceId, evidence);

  const orderedIds: string[] = [];
  if (existing) {
    orderedIds.push(existing.baselineEvidenceId);
    for (const id of existing.intermediateEvidenceIds) {
      if (!orderedIds.includes(id)) {
        orderedIds.push(id);
      }
    }
    if (
      existing.latestEvidenceId &&
      !orderedIds.includes(existing.latestEvidenceId)
    ) {
      orderedIds.push(existing.latestEvidenceId);
    }
  } else {
    const baselineId = adaptation.validation.baselineEvidenceId;
    if (baselineId) {
      orderedIds.push(baselineId);
    }
  }
  if (!orderedIds.includes(evidence.evidenceId)) {
    orderedIds.push(evidence.evidenceId);
  }

  const timeline = resolveTimeline(adaptation, evidenceById, orderedIds);
  const report = analyzeAdaptationStability(adaptation, timeline);
  const record = buildLongitudinalRecord(adaptation, timeline, report);
  upsertLongitudinalArtefacts(store, record, report);

  // Demote rollout_candidate when regressions appear.
  if (
    adaptation.rolloutState === "rollout_candidate" &&
    report.regressionEvents.length > LONGITUDINAL_MAX_UNRESOLVED_REGRESSIONS
  ) {
    upsertValidatedAdaptation(store, {
      ...adaptation,
      rolloutState: "candidate",
    });
  }

  return { record, report };
}

/**
 * Re-run stability over the recorded timeline (or baseline-only).
 */
export function runLongitudinalValidation(
  store: ExperienceStoreAdapter,
  adaptation: WorkspaceAdaptation,
): {
  record: LongitudinalAdaptationRecord;
  report: AdaptationStabilityReport;
} {
  const existing = getLongitudinalRecord(store, adaptation.adaptationId);
  const evidenceById = new Map(
    listEvidenceSnapshots(store).map((e) => [e.evidenceId, e]),
  );
  const orderedIds = existing
    ? [
        existing.baselineEvidenceId,
        ...existing.intermediateEvidenceIds,
        existing.latestEvidenceId,
      ].filter((id, i, arr) => id && arr.indexOf(id) === i)
    : [adaptation.validation.baselineEvidenceId].filter(Boolean);

  const timeline = resolveTimeline(adaptation, evidenceById, orderedIds);
  const report = analyzeAdaptationStability(adaptation, timeline);
  const record = buildLongitudinalRecord(adaptation, timeline, report);
  upsertLongitudinalArtefacts(store, record, report);

  if (
    adaptation.rolloutState === "rollout_candidate" &&
    report.regressionEvents.length > LONGITUDINAL_MAX_UNRESOLVED_REGRESSIONS
  ) {
    upsertValidatedAdaptation(store, {
      ...adaptation,
      rolloutState: "candidate",
    });
  }

  return { record, report };
}

export function isRolloutReady(
  adaptation: WorkspaceAdaptation,
  report: AdaptationStabilityReport,
): boolean {
  return (
    adaptation.validation.validationResult === "passed" &&
    report.rolloutDisposition === "rollout_candidate" &&
    report.stabilityScore >= LONGITUDINAL_STABILITY_THRESHOLD &&
    report.regressionEvents.length <= LONGITUDINAL_MAX_UNRESOLVED_REGRESSIONS &&
    report.evidenceTimeline.length >= LONGITUDINAL_MIN_OBSERVATIONS
  );
}

/**
 * candidate → rollout_candidate when longitudinal thresholds are met.
 * Never auto-activates.
 */
export function promoteToRolloutCandidate(
  store: ExperienceStoreAdapter,
  adaptationId: string,
):
  | { ok: true; adaptation: WorkspaceAdaptation; report: AdaptationStabilityReport }
  | { ok: false; error: LongitudinalTransitionError } {
  const bundle = loadAdaptationBundle(store);
  const index = bundle.adaptations.findIndex(
    (a) => a.adaptationId === adaptationId,
  );
  if (index < 0) {
    return { ok: false, error: "not_found" };
  }
  const current = bundle.adaptations[index]!;
  if (current.validation.validationResult !== "passed") {
    return { ok: false, error: "validation_not_passed" };
  }
  if (
    current.rolloutState !== "candidate" &&
    current.rolloutState !== "rollout_candidate"
  ) {
    return { ok: false, error: "invalid_state" };
  }
  if (
    !current.engineeringChangeId ||
    !current.proposalId ||
    !current.architectureSnapshotId ||
    current.validation.replaySessionIds.length === 0
  ) {
    return { ok: false, error: "governance_incomplete" };
  }

  const { report } = runLongitudinalValidation(store, current);
  if (report.evidenceTimeline.length < LONGITUDINAL_MIN_OBSERVATIONS) {
    return { ok: false, error: "evidence_minimum_unmet" };
  }
  if (report.regressionEvents.length > LONGITUDINAL_MAX_UNRESOLVED_REGRESSIONS) {
    return { ok: false, error: "unresolved_regressions" };
  }
  if (report.stabilityScore < LONGITUDINAL_STABILITY_THRESHOLD) {
    return { ok: false, error: "stability_threshold_unmet" };
  }
  if (report.rolloutDisposition !== "rollout_candidate") {
    return { ok: false, error: "stability_threshold_unmet" };
  }

  const updated: WorkspaceAdaptation = {
    ...current,
    rolloutState: "rollout_candidate",
  };
  const fresh = loadAdaptationBundle(store);
  const idx = fresh.adaptations.findIndex((a) => a.adaptationId === adaptationId);
  if (idx < 0) {
    return { ok: false, error: "not_found" };
  }
  const next = [...fresh.adaptations];
  next[idx] = updated;
  fresh.adaptations = next;
  saveAdaptationBundle(store, fresh);
  return { ok: true, adaptation: { ...updated }, report };
}

export function listRolloutCandidates(
  store: ExperienceStoreAdapter,
): WorkspaceAdaptation[] {
  return loadAdaptationBundle(store).adaptations.filter(
    (a) => a.rolloutState === "rollout_candidate",
  );
}
