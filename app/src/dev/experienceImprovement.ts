/**
 * Evidence-driven improvement engine.
 * Deterministic opportunities + longitudinal baseline evolution.
 * No natural-language speculation. No production behaviour changes.
 */

import { fnv1a } from "./devHash";
import {
  COMPARABLE_METRICS,
  type ExperienceEvidence,
  type ExperienceEvidenceMetrics,
  type MetricDirection,
} from "./experienceEvidence";

export type OpportunityWorkflow =
  | "save"
  | "continue"
  | "navigation"
  | "confidence"
  | "recovery"
  | "replay"
  | "friction";

export type OpportunitySeverity = "low" | "medium" | "high";

export interface ExperienceOpportunity {
  schemaVersion: 1;
  opportunityId: string;
  metric: keyof ExperienceEvidenceMetrics;
  workflow: OpportunityWorkflow;
  /** Sessions contributing via supporting evidence. */
  evidenceCount: number;
  severity: OpportunitySeverity;
  /** 0–1 sample-size confidence. */
  confidence: number;
  /** 0–1; 1 − replayDivergenceRate of supporting evidence. */
  reproducibilityScore: number;
  supportingEvidenceIds: string[];
  /** Session ids for deterministic replay (existing trace store — not duplicated). */
  replaySessionIds: string[];
  observedValue: number;
  threshold: number;
  /** Signed excess beyond threshold (positive = worse for lower_better). */
  excess: number;
}

export type EvolutionVerdict = "improving" | "stable" | "degrading";

export interface MetricEvolution {
  key: keyof ExperienceEvidenceMetrics;
  direction: MetricDirection;
  verdict: EvolutionVerdict;
  first: number;
  last: number;
  delta: number;
  significanceFloor: number;
}

export interface BaselineEvolution {
  evidenceIds: string[];
  snapshotCount: number;
  metrics: MetricEvolution[];
  summary: {
    improving: number;
    stable: number;
    degrading: number;
  };
  improvements: MetricEvolution[];
  regressions: MetricEvolution[];
}

interface OpportunityRule {
  metric: keyof ExperienceEvidenceMetrics;
  workflow: OpportunityWorkflow;
  direction: MetricDirection;
  threshold: number;
  minSessions: number;
  active?: (metrics: ExperienceEvidenceMetrics) => boolean;
}

/**
 * Absolute thresholds for opportunity emission.
 * Opportunity only when the metric crosses the threshold in the worse direction.
 */
export const OPPORTUNITY_RULES: readonly OpportunityRule[] = [
  {
    metric: "medianTimeToConfidenceMs",
    workflow: "confidence",
    direction: "lower_better",
    threshold: 3000,
    minSessions: 1,
  },
  {
    metric: "meanFrictionScore",
    workflow: "friction",
    direction: "lower_better",
    threshold: 0.25,
    minSessions: 1,
  },
  {
    metric: "medianFrictionScore",
    workflow: "friction",
    direction: "lower_better",
    threshold: 0.25,
    minSessions: 1,
  },
  {
    metric: "frictionP75",
    workflow: "friction",
    direction: "lower_better",
    threshold: 0.4,
    minSessions: 1,
  },
  {
    metric: "hesitationHotspotCount",
    workflow: "navigation",
    direction: "lower_better",
    threshold: 2,
    minSessions: 1,
  },
  {
    metric: "topHesitationMedianGapMs",
    workflow: "navigation",
    direction: "lower_better",
    threshold: 2000,
    minSessions: 1,
    active: (m) => m.hesitationHotspotCount >= 1,
  },
  {
    metric: "navigationLoopCount",
    workflow: "navigation",
    direction: "lower_better",
    threshold: 1,
    minSessions: 1,
  },
  {
    metric: "abandonedSave",
    workflow: "save",
    direction: "lower_better",
    threshold: 1,
    minSessions: 1,
  },
  {
    metric: "abandonedContinue",
    workflow: "continue",
    direction: "lower_better",
    threshold: 1,
    minSessions: 1,
  },
  {
    metric: "recoverySuccessRate",
    workflow: "recovery",
    direction: "higher_better",
    threshold: 0.5,
    minSessions: 1,
    active: (m) => m.interruptionCount >= 1,
  },
  {
    metric: "replayDivergenceRate",
    workflow: "replay",
    direction: "lower_better",
    threshold: 0,
    minSessions: 1,
    active: (m) => m.replayCount >= 1 && m.replayDivergenceRate > 0,
  },
] as const;

/**
 * Longitudinal significance floors — ignore noise below these.
 * Larger than pairwise comparison epsilons in ExperienceEvidence.
 */
export const LONGITUDINAL_SIGNIFICANCE: Readonly<
  Record<
    string,
    { abs: number; rel: number }
  >
> = {
  medianTimeToConfidenceMs: { abs: 250, rel: 0.1 },
  meanFrictionScore: { abs: 0.03, rel: 0.08 },
  medianFrictionScore: { abs: 0.03, rel: 0.08 },
  frictionP75: { abs: 0.04, rel: 0.08 },
  hesitationHotspotCount: { abs: 1, rel: 0 },
  topHesitationMedianGapMs: { abs: 200, rel: 0.1 },
  navigationLoopCount: { abs: 1, rel: 0 },
  abandonedFlowTotal: { abs: 1, rel: 0 },
  recoverySuccessRate: { abs: 0.1, rel: 0.15 },
  replayDivergenceRate: { abs: 0.05, rel: 0 },
};

function crossedThreshold(
  value: number,
  threshold: number,
  direction: MetricDirection,
): boolean {
  if (direction === "lower_better") {
    return value > threshold;
  }
  return value < threshold;
}

function excessBeyond(
  value: number,
  threshold: number,
  direction: MetricDirection,
): number {
  if (direction === "lower_better") {
    return Number((value - threshold).toFixed(4));
  }
  return Number((threshold - value).toFixed(4));
}

function severityFor(
  value: number,
  threshold: number,
  direction: MetricDirection,
): OpportunitySeverity {
  const ratio =
    direction === "lower_better"
      ? value / Math.max(threshold, 1e-9)
      : threshold / Math.max(value, 1e-9);
  if (ratio < 1.25) {
    return "low";
  }
  if (ratio < 2) {
    return "medium";
  }
  return "high";
}

function confidenceFor(sessionCount: number): number {
  return Number(Math.min(1, sessionCount / 5).toFixed(4));
}

function severityRank(severity: OpportunitySeverity): number {
  if (severity === "high") return 0;
  if (severity === "medium") return 1;
  return 2;
}

/**
 * Detect opportunities from one or more evidence snapshots.
 * Identical evidence input → identical opportunity list.
 * Requires replaySessionIds (from evidence source sessions); skips otherwise.
 */
export function detectOpportunities(
  evidenceList: ExperienceEvidence[],
): ExperienceOpportunity[] {
  if (evidenceList.length === 0) {
    return [];
  }

  // Primary signal = last snapshot in caller timeline order.
  const primary = evidenceList[evidenceList.length - 1]!;
  const supportingIds = [
    ...new Set(evidenceList.map((e) => e.evidenceId)),
  ].sort((a, b) => a.localeCompare(b));
  const replaySessionIds = [
    ...new Set(evidenceList.flatMap((e) => e.sourceSessionIds)),
  ].sort((a, b) => a.localeCompare(b));

  if (replaySessionIds.length === 0) {
    return [];
  }

  const metrics = primary.metrics;
  const evidenceCount = metrics.sessionCount;
  const reproducibilityScore = Number(
    Math.max(0, Math.min(1, 1 - metrics.replayDivergenceRate)).toFixed(4),
  );

  const opportunities: ExperienceOpportunity[] = [];

  for (const rule of OPPORTUNITY_RULES) {
    if (evidenceCount < rule.minSessions) {
      continue;
    }
    if (rule.active && !rule.active(metrics)) {
      continue;
    }
    const observedValue = Number(metrics[rule.metric]);
    if (!crossedThreshold(observedValue, rule.threshold, rule.direction)) {
      continue;
    }

    const opportunityId = `opp-${fnv1a(`${rule.metric}|${rule.workflow}|${rule.threshold}`)}`;
    opportunities.push({
      schemaVersion: 1,
      opportunityId,
      metric: rule.metric,
      workflow: rule.workflow,
      evidenceCount,
      severity: severityFor(observedValue, rule.threshold, rule.direction),
      confidence: confidenceFor(evidenceCount),
      reproducibilityScore,
      supportingEvidenceIds: supportingIds,
      replaySessionIds: [...replaySessionIds],
      observedValue,
      threshold: rule.threshold,
      excess: excessBeyond(observedValue, rule.threshold, rule.direction),
    });
  }

  opportunities.sort((a, b) => {
    const sr = severityRank(a.severity) - severityRank(b.severity);
    if (sr !== 0) {
      return sr;
    }
    return a.opportunityId.localeCompare(b.opportunityId);
  });

  return opportunities;
}

function significanceFloor(
  first: number,
  abs: number,
  rel: number,
): number {
  return Math.max(abs, Math.abs(first) * rel);
}

export function evolutionVerdictFor(
  first: number,
  last: number,
  direction: MetricDirection,
  floor: number,
): EvolutionVerdict {
  const delta = last - first;
  if (Math.abs(delta) <= floor) {
    return "stable";
  }
  if (direction === "lower_better") {
    return delta < 0 ? "improving" : "degrading";
  }
  return delta > 0 ? "improving" : "degrading";
}

/**
 * Compare an ordered series of evidence snapshots (caller timeline order).
 * Uses first vs last value per comparable metric with longitudinal floors.
 * Requires ≥2 snapshots; otherwise all metrics report stable with empty delta.
 */
export function evolveBaselines(
  snapshots: ExperienceEvidence[],
): BaselineEvolution {
  const ordered = [...snapshots];
  const evidenceIds = ordered.map((s) => s.evidenceId);

  if (ordered.length < 2) {
    const metrics: MetricEvolution[] = COMPARABLE_METRICS.map((spec) => {
      const value = ordered[0] ? Number(ordered[0].metrics[spec.key]) : 0;
      return {
        key: spec.key,
        direction: spec.direction,
        verdict: "stable" as const,
        first: value,
        last: value,
        delta: 0,
        significanceFloor: 0,
      };
    });
    return {
      evidenceIds,
      snapshotCount: ordered.length,
      metrics,
      summary: { improving: 0, stable: metrics.length, degrading: 0 },
      improvements: [],
      regressions: [],
    };
  }

  const firstSnap = ordered[0]!;
  const lastSnap = ordered[ordered.length - 1]!;

  const metrics: MetricEvolution[] = COMPARABLE_METRICS.map((spec) => {
    const first = Number(firstSnap.metrics[spec.key]);
    const last = Number(lastSnap.metrics[spec.key]);
    const sig = LONGITUDINAL_SIGNIFICANCE[spec.key] ?? {
      abs: spec.epsilon,
      rel: 0.05,
    };
    const floor = significanceFloor(first, sig.abs, sig.rel);
    const verdict = evolutionVerdictFor(
      first,
      last,
      spec.direction,
      floor,
    );
    return {
      key: spec.key,
      direction: spec.direction,
      verdict,
      first,
      last,
      delta: Number((last - first).toFixed(4)),
      significanceFloor: Number(floor.toFixed(4)),
    };
  });

  const improvements = metrics.filter((m) => m.verdict === "improving");
  const regressions = metrics.filter((m) => m.verdict === "degrading");
  const summary = {
    improving: improvements.length,
    stable: metrics.filter((m) => m.verdict === "stable").length,
    degrading: regressions.length,
  };

  return {
    evidenceIds,
    snapshotCount: ordered.length,
    metrics,
    summary,
    improvements,
    regressions,
  };
}

/** Ensure every opportunity carries at least one replay session id. */
export function opportunitiesHaveReplayLinkage(
  opportunities: ExperienceOpportunity[],
): boolean {
  return opportunities.every((o) => o.replaySessionIds.length > 0);
}
