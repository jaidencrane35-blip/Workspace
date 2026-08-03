/**
 * ExperienceEvidence — derived metrics only (no raw interaction events).
 * Persisted separately from Sprint 51 traces.
 */

import { fnv1a } from "./devHash";
import type { ExperienceDestination, ExperienceSession } from "./experienceEvents";
import {
  browserStore,
  memoryStore,
  type ExperienceStoreAdapter,
} from "./experienceStore";
import { analyzeTraces, type TraceAggregate } from "./traceAnalysis";

export const EVIDENCE_STORAGE_KEY = "ws.dev.experience.evidence.v1";
export const MAX_EVIDENCE_SNAPSHOTS = 40;

export interface ExperienceEvidenceMetrics {
  sessionCount: number;
  medianTimeToConfidenceMs: number;
  meanFrictionScore: number;
  medianFrictionScore: number;
  frictionMin: number;
  frictionMax: number;
  frictionP25: number;
  frictionP75: number;
  hesitationHotspotCount: number;
  topHesitationDestination: ExperienceDestination | null;
  topHesitationMedianGapMs: number;
  navigationLoopCount: number;
  abandonedFlowTotal: number;
  abandonedSave: number;
  abandonedContinue: number;
  recoverySuccessRate: number;
  recoveryCount: number;
  interruptionCount: number;
  replayCount: number;
  replayDivergenceRate: number;
  saveSuccessTotal: number;
  continueSuccessTotal: number;
}

export interface ExperienceEvidence {
  schemaVersion: 1;
  evidenceId: string;
  /** Stable fingerprint of source session ids + metric digest (no wall clock). */
  fingerprint: string;
  /** Opaque snapshot tag — allowlisted short token only. */
  tag: string;
  sourceSessionIds: string[];
  metrics: ExperienceEvidenceMetrics;
  /** Derived hotspot summary (destinations + numbers only). */
  hotspots: Array<{
    destination: ExperienceDestination;
    medianGapMs: number;
    samples: number;
  }>;
  /** Derived loop summary (allowlisted pattern strings + counts). */
  loops: Array<{ pattern: string; count: number }>;
}

export type MetricDirection = "lower_better" | "higher_better";

export type MetricVerdict = "improved" | "unchanged" | "regressed";

export interface MeasurableMetric {
  key: keyof ExperienceEvidenceMetrics;
  direction: MetricDirection;
  /** Absolute epsilon for "unchanged". */
  epsilon: number;
}

/** Measured metrics eligible for baseline comparison. */
export const COMPARABLE_METRICS: readonly MeasurableMetric[] = [
  { key: "medianTimeToConfidenceMs", direction: "lower_better", epsilon: 1 },
  { key: "meanFrictionScore", direction: "lower_better", epsilon: 0.0001 },
  { key: "medianFrictionScore", direction: "lower_better", epsilon: 0.0001 },
  { key: "frictionP75", direction: "lower_better", epsilon: 0.0001 },
  { key: "hesitationHotspotCount", direction: "lower_better", epsilon: 0 },
  { key: "topHesitationMedianGapMs", direction: "lower_better", epsilon: 1 },
  { key: "navigationLoopCount", direction: "lower_better", epsilon: 0 },
  { key: "abandonedFlowTotal", direction: "lower_better", epsilon: 0 },
  { key: "recoverySuccessRate", direction: "higher_better", epsilon: 0.0001 },
  { key: "replayDivergenceRate", direction: "lower_better", epsilon: 0.0001 },
] as const;

export interface MetricComparison {
  key: keyof ExperienceEvidenceMetrics;
  baseline: number;
  candidate: number;
  delta: number;
  verdict: MetricVerdict;
  direction: MetricDirection;
}

export interface EvidenceComparison {
  baselineId: string;
  candidateId: string;
  metrics: MetricComparison[];
  summary: {
    improved: number;
    unchanged: number;
    regressed: number;
  };
}

interface EvidenceBundle {
  schemaVersion: 1;
  snapshots: ExperienceEvidence[];
  /** evidenceId marked as active baseline, if any. */
  baselineId: string | null;
  /** Replay invocation count recorded by dashboard/tools (local). */
  replayInvocations: number;
}

function emptyBundle(): EvidenceBundle {
  return {
    schemaVersion: 1,
    snapshots: [],
    baselineId: null,
    replayInvocations: 0,
  };
}

const LABEL_RE = /^[a-z0-9_.:-]{1,64}$/i;

function sanitizeLabel(label: string | undefined): string {
  if (label && LABEL_RE.test(label)) {
    return label;
  }
  return "snapshot";
}

export function aggregateToEvidence(
  aggregate: TraceAggregate,
  options?: { tag?: string; evidenceId?: string },
): ExperienceEvidence {
  const top = aggregate.hesitationHotspots[0] ?? null;
  const metrics: ExperienceEvidenceMetrics = {
    sessionCount: aggregate.sessionCount,
    medianTimeToConfidenceMs: aggregate.medianTimeToConfidenceMs,
    meanFrictionScore: aggregate.meanFrictionScore,
    medianFrictionScore: aggregate.medianFrictionScore,
    frictionMin: aggregate.frictionMin,
    frictionMax: aggregate.frictionMax,
    frictionP25: aggregate.frictionP25,
    frictionP75: aggregate.frictionP75,
    hesitationHotspotCount: aggregate.hesitationHotspots.length,
    topHesitationDestination: top?.destination ?? null,
    topHesitationMedianGapMs: top?.medianGapMs ?? 0,
    navigationLoopCount: aggregate.navigationLoops.reduce(
      (sum, loop) => sum + loop.count,
      0,
    ),
    abandonedFlowTotal: aggregate.abandonedFlows.total,
    abandonedSave: aggregate.abandonedFlows.save,
    abandonedContinue: aggregate.abandonedFlows.continue,
    recoverySuccessRate: aggregate.recovery.successRate,
    recoveryCount: aggregate.recovery.recovered,
    interruptionCount: aggregate.recovery.interrupted,
    replayCount: aggregate.replay.sessionsReplayed,
    replayDivergenceRate: aggregate.replay.divergenceRate,
    saveSuccessTotal: aggregate.successes.save,
    continueSuccessTotal: aggregate.successes.continue,
  };

  const fingerprintPayload = JSON.stringify({
    sessionIds: aggregate.sessionIds,
    metrics,
  });
  const fingerprint = fnv1a(fingerprintPayload);
  const evidenceId =
    options?.evidenceId && LABEL_RE.test(options.evidenceId)
      ? options.evidenceId
      : `evd-${fingerprint}`;

  return {
    schemaVersion: 1,
    evidenceId,
    fingerprint,
    tag: sanitizeLabel(options?.tag),
    sourceSessionIds: [...aggregate.sessionIds],
    metrics,
    hotspots: aggregate.hesitationHotspots.map((h) => ({
      destination: h.destination,
      medianGapMs: h.medianGapMs,
      samples: h.samples,
    })),
    loops: aggregate.navigationLoops.map((l) => ({
      pattern: l.pattern,
      count: l.count,
    })),
  };
}

/**
 * Build ExperienceEvidence from stored sessions. Deterministic for the same sessions.
 */
export function buildEvidenceFromSessions(
  sessions: ExperienceSession[],
  options?: { tag?: string; evidenceId?: string },
): ExperienceEvidence {
  return aggregateToEvidence(analyzeTraces(sessions), options);
}

export function verdictForMetric(
  baseline: number,
  candidate: number,
  direction: MetricDirection,
  epsilon: number,
): MetricVerdict {
  const delta = candidate - baseline;
  if (Math.abs(delta) <= epsilon) {
    return "unchanged";
  }
  if (direction === "lower_better") {
    return delta < 0 ? "improved" : "regressed";
  }
  return delta > 0 ? "improved" : "regressed";
}

/**
 * Compare two evidence snapshots on measured metrics only.
 */
export function compareEvidence(
  baseline: ExperienceEvidence,
  candidate: ExperienceEvidence,
): EvidenceComparison {
  const metrics: MetricComparison[] = COMPARABLE_METRICS.map((spec) => {
    const baseVal = Number(baseline.metrics[spec.key]);
    const candVal = Number(candidate.metrics[spec.key]);
    return {
      key: spec.key,
      baseline: baseVal,
      candidate: candVal,
      delta: Number((candVal - baseVal).toFixed(4)),
      verdict: verdictForMetric(
        baseVal,
        candVal,
        spec.direction,
        spec.epsilon,
      ),
      direction: spec.direction,
    };
  });

  const summary = {
    improved: metrics.filter((m) => m.verdict === "improved").length,
    unchanged: metrics.filter((m) => m.verdict === "unchanged").length,
    regressed: metrics.filter((m) => m.verdict === "regressed").length,
  };

  return {
    baselineId: baseline.evidenceId,
    candidateId: candidate.evidenceId,
    metrics,
    summary,
  };
}

export function loadEvidenceBundle(
  store: ExperienceStoreAdapter,
): EvidenceBundle {
  const raw = store.getItem(EVIDENCE_STORAGE_KEY);
  if (!raw) {
    return emptyBundle();
  }
  try {
    const parsed = JSON.parse(raw) as EvidenceBundle;
    if (parsed?.schemaVersion !== 1 || !Array.isArray(parsed.snapshots)) {
      return emptyBundle();
    }
    return {
      schemaVersion: 1,
      snapshots: parsed.snapshots.slice(-MAX_EVIDENCE_SNAPSHOTS),
      baselineId:
        typeof parsed.baselineId === "string" ? parsed.baselineId : null,
      replayInvocations:
        typeof parsed.replayInvocations === "number"
          ? parsed.replayInvocations
          : 0,
    };
  } catch {
    return emptyBundle();
  }
}

function saveEvidenceBundle(
  store: ExperienceStoreAdapter,
  bundle: EvidenceBundle,
): void {
  const next: EvidenceBundle = {
    schemaVersion: 1,
    snapshots: bundle.snapshots.slice(-MAX_EVIDENCE_SNAPSHOTS),
    baselineId: bundle.baselineId,
    replayInvocations: bundle.replayInvocations,
  };
  store.setItem(EVIDENCE_STORAGE_KEY, JSON.stringify(next));
}

export function listEvidenceSnapshots(
  store: ExperienceStoreAdapter,
): ExperienceEvidence[] {
  return loadEvidenceBundle(store).snapshots;
}

export function persistEvidenceSnapshot(
  store: ExperienceStoreAdapter,
  evidence: ExperienceEvidence,
): void {
  const bundle = loadEvidenceBundle(store);
  const index = bundle.snapshots.findIndex(
    (s) => s.evidenceId === evidence.evidenceId,
  );
  if (index >= 0) {
    bundle.snapshots[index] = evidence;
  } else {
    bundle.snapshots.push(evidence);
  }
  saveEvidenceBundle(store, bundle);
}

export function setEvidenceBaseline(
  store: ExperienceStoreAdapter,
  evidenceId: string | null,
): void {
  const bundle = loadEvidenceBundle(store);
  bundle.baselineId = evidenceId;
  saveEvidenceBundle(store, bundle);
}

export function getEvidenceBaseline(
  store: ExperienceStoreAdapter,
): ExperienceEvidence | null {
  const bundle = loadEvidenceBundle(store);
  if (!bundle.baselineId) {
    return null;
  }
  return (
    bundle.snapshots.find((s) => s.evidenceId === bundle.baselineId) ?? null
  );
}

export function recordReplayInvocation(store: ExperienceStoreAdapter): number {
  const bundle = loadEvidenceBundle(store);
  bundle.replayInvocations += 1;
  saveEvidenceBundle(store, bundle);
  return bundle.replayInvocations;
}

export function getReplayInvocationCount(
  store: ExperienceStoreAdapter,
): number {
  return loadEvidenceBundle(store).replayInvocations;
}

export function clearEvidenceStore(store: ExperienceStoreAdapter): void {
  store.removeItem(EVIDENCE_STORAGE_KEY);
}

export function defaultEvidenceStore(): ExperienceStoreAdapter {
  return browserStore() ?? memoryStore();
}
