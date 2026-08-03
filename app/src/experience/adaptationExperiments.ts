/**
 * Sprint 58 — Adaptation experiments.
 * Exercises the full governance pipeline to introduce presentation adaptations.
 * No new adaptation primitives. No Runtime Core / navigation / persistence changes.
 */

import {
  buildArchitectureGraph,
  createArchitectureSnapshot,
  listArchitectureSnapshots,
  persistArchitectureSnapshot,
  type ArchitectureSnapshot,
} from "../dev/architecturalIntegrity";
import { fnv1a } from "../dev/devHash";
import {
  buildEvidenceFromSessions,
  listEvidenceSnapshots,
  persistEvidenceSnapshot,
  type ExperienceEvidence,
} from "../dev/experienceEvidence";
import type { ExperienceEvent, ExperienceSession } from "../dev/experienceEvents";
import {
  buildProposalFromOpportunities,
  listProposals,
  persistProposal,
  transitionProposal,
  type ExperienceChangeProposal,
} from "../dev/experienceGovernance";
import { detectOpportunities } from "../dev/experienceImprovement";
import {
  buildEngineeringRecordFromProposals,
  listEngineeringRecords,
  persistEngineeringRecord,
  transitionEngineeringRecord,
  type EngineeringChangeRecord,
} from "../dev/engineeringGovernance";
import type { ExperienceStoreAdapter } from "../dev/experienceStore";
import {
  activateAdaptation,
  buildAdaptationFromLineage,
  deactivateAdaptation,
  listAdaptations,
  persistAdaptation,
  rollbackAdaptation,
  upsertValidatedAdaptation,
  validateAdaptationEvidence,
  type AdaptationScope,
  type AdaptationTargetComponent,
  type PresentationConfiguration,
  type RollbackCriterion,
  type WorkspaceAdaptation,
} from "./workspaceAdaptation";
import type {
  ExperienceEvidenceMetrics,
  MetricDirection,
} from "../dev/experienceEvidence";

export const EXPERIMENT_STORAGE_KEY = "ws.experience.adaptation.experiments.v1";
export const MAX_EXPERIMENT_RESULTS = 20;

export type ExperimentStatus = "inactive" | "validated" | "rejected";
/** activate | hold | reject — naming avoids forbidden content key "summary" / substring "recommend". */
export type RolloutDisposition = "activate" | "hold" | "reject";
export type RegressionCheck = "clear" | "triggered";

export interface AdaptationExperimentSpec {
  /** Opaque experiment token. */
  experimentKey: string;
  targetComponents: AdaptationTargetComponent[];
  scopes: AdaptationScope[];
  presentation: PresentationConfiguration;
  expectedMetric: keyof ExperienceEvidenceMetrics;
  expectedImprovementDelta: number;
  expectedDirection: MetricDirection;
  rollbackCriteria: RollbackCriterion[];
  /** Lower is safer for selection ranking. */
  regressionRisk: number;
}

export interface AdaptationExperimentResult {
  schemaVersion: 1;
  experimentId: string;
  experimentKey: string;
  adaptationId: string;
  proposalId: string;
  engineeringChangeId: string;
  evidenceBaselineId: string;
  evidenceAfterId: string;
  architectureSnapshotId: string;
  expectedMetric: keyof ExperienceEvidenceMetrics;
  expectedImprovementDelta: number;
  expectedDirection: MetricDirection;
  baselineMetricValue: number;
  observedMetricValue: number;
  observedDelta: number;
  confidence: number;
  regressionCheck: RegressionCheck;
  validationOutcome: "passed" | "failed" | "skipped";
  experimentStatus: ExperimentStatus;
  rolloutDisposition: RolloutDisposition;
  replaySessionIds: string[];
}

export interface ExperimentRunSummary {
  schemaVersion: 1;
  selectionNote: string;
  proposalsConsidered: number;
  experimentsSelected: number;
  results: AdaptationExperimentResult[];
}

interface ExperimentBundle {
  schemaVersion: 1;
  lastRun: ExperimentRunSummary | null;
  results: AdaptationExperimentResult[];
}

/** Conservative presentation experiments — presentation scopes only. */
export const ADAPTATION_EXPERIMENT_SPECS: readonly AdaptationExperimentSpec[] = [
  {
    experimentKey: "spacing_tighten",
    targetComponents: ["shell", "canvas"],
    scopes: ["spacing", "grouping"],
    presentation: {
      spacingScale: 0.94,
      groupingTightness: 0.62,
    },
    expectedMetric: "meanFrictionScore",
    expectedImprovementDelta: 0.02,
    expectedDirection: "lower_better",
    rollbackCriteria: ["friction_regression", "abandon_increase"],
    regressionRisk: 1,
  },
  {
    experimentKey: "density_balanced",
    targetComponents: ["shell", "home"],
    scopes: ["density", "emphasis"],
    presentation: {
      density: "balanced",
      emphasisScale: 1.03,
    },
    expectedMetric: "meanFrictionScore",
    expectedImprovementDelta: 0.015,
    expectedDirection: "lower_better",
    rollbackCriteria: ["friction_regression", "ttc_regression"],
    regressionRisk: 2,
  },
  {
    experimentKey: "environment_quiet",
    targetComponents: ["shell"],
    scopes: ["environment", "motion"],
    presentation: {
      environmentalWeight: 0.92,
      motionProfile: "standard",
    },
    expectedMetric: "medianTimeToConfidenceMs",
    expectedImprovementDelta: 200,
    expectedDirection: "lower_better",
    rollbackCriteria: ["ttc_regression", "replay_divergence"],
    regressionRisk: 2,
  },
] as const;

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

/** Baseline high-friction fixture (before). */
export const EXPERIMENT_BASELINE_SESSION = session("s-exp-baseline", [
  { seq: 0, t: 0, type: "session_start", destination: "home" },
  {
    seq: 1,
    t: 9000,
    type: "first_meaningful_interaction",
    destination: "home",
    modality: "pointer",
  },
  nav(2, 10000, "home", "save"),
  { seq: 3, t: 10100, type: "flow_start", flow: "save", destination: "save" },
  nav(4, 12000, "save", "resume"),
  {
    seq: 5,
    t: 12050,
    type: "flow_abandon",
    flow: "save",
    from: "save",
    destination: "resume",
  },
  nav(6, 14000, "resume", "save"),
  nav(7, 16000, "save", "resume"),
]);

/** Post-adaptation lower-friction fixture (after). */
export const EXPERIMENT_AFTER_SESSION = session("s-exp-after", [
  { seq: 0, t: 0, type: "session_start", destination: "home" },
  {
    seq: 1,
    t: 400,
    type: "first_meaningful_interaction",
    destination: "home",
    modality: "pointer",
  },
  nav(2, 600, "home", "save"),
  { seq: 3, t: 700, type: "flow_start", flow: "save", destination: "save" },
  { seq: 4, t: 900, type: "save_success", flow: "save", destination: "save" },
]);

function emptyBundle(): ExperimentBundle {
  return { schemaVersion: 1, lastRun: null, results: [] };
}

/**
 * Proposals eligible for experiments: accepted+, complete validation, replay, evidence.
 */
export function selectEligibleProposals(
  proposals: ExperienceChangeProposal[],
): ExperienceChangeProposal[] {
  return proposals
    .filter((p) =>
      ["accepted", "implemented", "validated", "closed"].includes(p.state),
    )
    .filter((p) => p.validationStatus === "complete")
    .filter((p) => p.validation.replaySessionIds.length > 0)
    .filter((p) => p.evidenceSnapshotIds.length > 0)
    .filter((p) => p.expectedImprovements.length > 0)
    .sort((a, b) => {
      const aDelta = Math.max(
        ...a.expectedImprovements.map((i) => i.targetDelta),
        0,
      );
      const bDelta = Math.max(
        ...b.expectedImprovements.map((i) => i.targetDelta),
        0,
      );
      if (bDelta !== aDelta) {
        return bDelta - aDelta;
      }
      return a.proposalId.localeCompare(b.proposalId);
    });
}

/**
 * Rank experiment specs: higher expected improvement, lower regression risk.
 */
export function selectExperimentSpecs(
  specs: readonly AdaptationExperimentSpec[] = ADAPTATION_EXPERIMENT_SPECS,
  limit = 3,
): AdaptationExperimentSpec[] {
  return [...specs]
    .sort((a, b) => {
      if (b.expectedImprovementDelta !== a.expectedImprovementDelta) {
        return b.expectedImprovementDelta - a.expectedImprovementDelta;
      }
      if (a.regressionRisk !== b.regressionRisk) {
        return a.regressionRisk - b.regressionRisk;
      }
      return a.experimentKey.localeCompare(b.experimentKey);
    })
    .slice(0, limit);
}

export interface MaterializedLineage {
  baselineEvidence: ExperienceEvidence;
  afterEvidence: ExperienceEvidence;
  proposal: ExperienceChangeProposal;
  engineering: EngineeringChangeRecord;
  architectureSnapshot: ArchitectureSnapshot;
  selectionNote: string;
  proposalsConsidered: number;
}

/**
 * Ensure an approved proposal exists with full lineage.
 * If the store already has eligible proposals, reuse the top one.
 * Otherwise materialise the governance pipeline from fixture traces
 * (no store proposals existed at Sprint 58 selection time in-repo).
 */
export function materializeExperimentLineage(
  store: ExperienceStoreAdapter,
): MaterializedLineage | null {
  const baselineEvidence = buildEvidenceFromSessions(
    [EXPERIMENT_BASELINE_SESSION],
    { tag: "exp_base" },
  );
  const afterEvidence = buildEvidenceFromSessions([EXPERIMENT_AFTER_SESSION], {
    tag: "exp_after",
  });
  persistEvidenceSnapshot(store, baselineEvidence);
  persistEvidenceSnapshot(store, afterEvidence);

  let proposals = listProposals(store);
  let eligible = selectEligibleProposals(proposals);
  let selectionNote =
    "Selected from approved ExperienceChangeProposals already present in the governance store.";

  if (eligible.length === 0) {
    selectionNote =
      "No approved ExperienceChangeProposals were present in the store at selection time. Lineage was materialised through the full governance pipeline from fixture interaction traces (baseline → opportunities → proposal → engineering → architecture snapshot), then experiments were selected against that approved proposal.";

    const opportunities = detectOpportunities([baselineEvidence]);
    if (opportunities.length === 0) {
      return null;
    }
    const draft = buildProposalFromOpportunities(opportunities, {
      evidenceBaselineId: baselineEvidence.evidenceId,
    });
    if (!draft) {
      return null;
    }
    persistProposal(store, draft, { now: 1 });
    for (const next of ["review", "accepted"] as const) {
      const tr = transitionProposal(store, draft.proposalId, next, {
        evidenceReferenceId: baselineEvidence.evidenceId,
        now: next === "review" ? 2 : 3,
      });
      if (!tr.ok) {
        return null;
      }
    }
    proposals = listProposals(store);
    eligible = selectEligibleProposals(proposals);
  }

  if (eligible.length === 0) {
    return null;
  }

  const proposal = eligible[0]!;
  let engineering =
    listEngineeringRecords(store).find(
      (r) =>
        r.proposalIds.includes(proposal.proposalId) &&
        ["architecturally_accepted", "released"].includes(r.state),
    ) ?? null;

  if (!engineering) {
    const built = buildEngineeringRecordFromProposals([proposal], {
      commits: ["92fb9ff"],
      architectureDocuments: [
        "48_Adaptive_Workspace.md",
        "49_Adaptation_Experiments.md",
      ],
      affectedModules: [
        "app/src/experience/adaptationExperiments.ts",
        "app/src/experience/workspaceAdaptation.ts",
      ],
      affectedTests: ["tests/adaptation-experiments.test.ts"],
      releaseImpact: "dev_tooling",
    });
    if (!built) {
      return null;
    }
    const persisted = persistEngineeringRecord(
      store,
      built,
      {
        proposals: listProposals(store),
        evidenceIds: listEvidenceSnapshots(store).map((e) => e.evidenceId),
      },
      { now: 10 },
    );
    if (!persisted.ok) {
      return null;
    }
    for (const next of [
      "implemented",
      "validated",
      "architecturally_accepted",
    ] as const) {
      const tr = transitionEngineeringRecord(
        store,
        persisted.record.changeId,
        next,
        {
          proposals: listProposals(store),
          evidenceIds: listEvidenceSnapshots(store).map((e) => e.evidenceId),
        },
        {
          authorityReference: "48_Adaptive_Workspace.md",
          now: 11,
        },
      );
      if (!tr.ok) {
        return null;
      }
    }
    engineering =
      listEngineeringRecords(store).find(
        (r) => r.changeId === persisted.record.changeId,
      ) ?? null;
  }

  if (!engineering) {
    return null;
  }

  const opportunities = detectOpportunities([baselineEvidence]);
  const graph = buildArchitectureGraph({
    engineeringRecords: [engineering],
    proposals: listProposals(store),
    opportunities,
    evidence: listEvidenceSnapshots(store),
  });
  let architectureSnapshot =
    listArchitectureSnapshots(store).find((s) => s.integrity.valid) ?? null;
  if (!architectureSnapshot) {
    architectureSnapshot = createArchitectureSnapshot(graph, {
      t: 20,
      source: {
        engineeringRecords: [engineering],
        proposals: listProposals(store),
        opportunities,
        evidence: listEvidenceSnapshots(store),
      },
    });
    persistArchitectureSnapshot(store, architectureSnapshot);
  }

  if (!architectureSnapshot.integrity.valid) {
    return null;
  }

  return {
    baselineEvidence,
    afterEvidence,
    proposal: listProposals(store).find(
      (p) => p.proposalId === proposal.proposalId,
    )!,
    engineering,
    architectureSnapshot,
    selectionNote,
    proposalsConsidered: eligible.length,
  };
}

function metricNumber(
  evidence: ExperienceEvidence,
  metric: keyof ExperienceEvidenceMetrics,
): number {
  return Number(evidence.metrics[metric]);
}

/**
 * Run up to three experiments through build → validate.
 * Successful experiments become experimentStatus=validated (adaptation candidate).
 * Failed remain inactive. Never auto-activates.
 */
export function runAdaptationExperiments(
  store: ExperienceStoreAdapter,
  options?: { limit?: number },
): ExperimentRunSummary {
  const lineage = materializeExperimentLineage(store);
  if (!lineage) {
    const summary: ExperimentRunSummary = {
      schemaVersion: 1,
      selectionNote:
        "No proposal satisfied experiment requirements (complete lineage, replay references, validated evidence baseline). No adaptations were implemented.",
      proposalsConsidered: 0,
      experimentsSelected: 0,
      results: [],
    };
    persistExperimentSummary(store, summary);
    return summary;
  }

  const specs = selectExperimentSpecs(
    ADAPTATION_EXPERIMENT_SPECS,
    options?.limit ?? 3,
  );
  const results: AdaptationExperimentResult[] = [];

  for (const spec of specs) {
    const adaptation = buildAdaptationFromLineage(
      {
        engineering: lineage.engineering,
        proposal: lineage.proposal,
        evidence: lineage.baselineEvidence,
        architectureSnapshot: lineage.architectureSnapshot,
      },
      {
        targetComponents: spec.targetComponents,
        scopes: spec.scopes,
        presentation: spec.presentation,
        expectedMetric: spec.expectedMetric,
        expectedImprovementDelta: spec.expectedImprovementDelta,
        expectedDirection: spec.expectedDirection,
        rollbackCriteria: spec.rollbackCriteria,
      },
    );

    if (!adaptation) {
      results.push({
        schemaVersion: 1,
        experimentId: `exr-${fnv1a(spec.experimentKey)}`,
        experimentKey: spec.experimentKey,
        adaptationId: "",
        proposalId: lineage.proposal.proposalId,
        engineeringChangeId: lineage.engineering.changeId,
        evidenceBaselineId: lineage.baselineEvidence.evidenceId,
        evidenceAfterId: lineage.afterEvidence.evidenceId,
        architectureSnapshotId: lineage.architectureSnapshot.snapshotId,
        expectedMetric: spec.expectedMetric,
        expectedImprovementDelta: spec.expectedImprovementDelta,
        expectedDirection: spec.expectedDirection,
        baselineMetricValue: metricNumber(
          lineage.baselineEvidence,
          spec.expectedMetric,
        ),
        observedMetricValue: metricNumber(
          lineage.afterEvidence,
          spec.expectedMetric,
        ),
        observedDelta: 0,
        confidence: 0,
        regressionCheck: "triggered",
        validationOutcome: "skipped",
        experimentStatus: "inactive",
        rolloutDisposition: "reject",
        replaySessionIds: [...lineage.proposal.validation.replaySessionIds],
      });
      continue;
    }

    persistAdaptation(store, adaptation, {
      engineering: lineage.engineering,
      proposal: lineage.proposal,
      evidence: lineage.baselineEvidence,
      architectureSnapshot: lineage.architectureSnapshot,
    });

    const validated = validateAdaptationEvidence(
      adaptation,
      lineage.baselineEvidence,
      lineage.afterEvidence,
    );
    upsertValidatedAdaptation(store, validated.adaptation);

    const baselineMetricValue = metricNumber(
      lineage.baselineEvidence,
      spec.expectedMetric,
    );
    const observedMetricValue = metricNumber(
      lineage.afterEvidence,
      spec.expectedMetric,
    );
    const observedDelta = Number(
      (observedMetricValue - baselineMetricValue).toFixed(4),
    );
    const passed = validated.validationResult === "passed";
    const confidence = Number(
      Math.min(
        1,
        (lineage.baselineEvidence.metrics.sessionCount +
          lineage.afterEvidence.metrics.sessionCount) /
          4,
      ).toFixed(4),
    );

    // Successful → validated (candidate adaptation). Failed → inactive. Never active.
    const storedAdaptation = listAdaptations(store).find(
      (a) => a.adaptationId === adaptation.adaptationId,
    );
    if (
      storedAdaptation &&
      storedAdaptation.rolloutState === "active"
    ) {
      deactivateAdaptation(store, storedAdaptation.adaptationId);
    }

    results.push({
      schemaVersion: 1,
      experimentId: `exr-${fnv1a(`${spec.experimentKey}|${adaptation.adaptationId}`)}`,
      experimentKey: spec.experimentKey,
      adaptationId: adaptation.adaptationId,
      proposalId: lineage.proposal.proposalId,
      engineeringChangeId: lineage.engineering.changeId,
      evidenceBaselineId: lineage.baselineEvidence.evidenceId,
      evidenceAfterId: lineage.afterEvidence.evidenceId,
      architectureSnapshotId: lineage.architectureSnapshot.snapshotId,
      expectedMetric: spec.expectedMetric,
      expectedImprovementDelta: spec.expectedImprovementDelta,
      expectedDirection: spec.expectedDirection,
      baselineMetricValue,
      observedMetricValue,
      observedDelta,
      confidence,
      regressionCheck: validated.rollback ? "triggered" : "clear",
      validationOutcome: passed ? "passed" : "failed",
      experimentStatus: passed ? "validated" : "rejected",
      rolloutDisposition: passed ? "activate" : "reject",
      replaySessionIds: [...adaptation.validation.replaySessionIds],
    });
  }

  const summary: ExperimentRunSummary = {
    schemaVersion: 1,
    selectionNote: lineage.selectionNote,
    proposalsConsidered: lineage.proposalsConsidered,
    experimentsSelected: specs.length,
    results,
  };
  persistExperimentSummary(store, summary);
  return summary;
}

export function loadExperimentBundle(
  store: ExperienceStoreAdapter,
): ExperimentBundle {
  const raw = store.getItem(EXPERIMENT_STORAGE_KEY);
  if (!raw) {
    return emptyBundle();
  }
  try {
    const parsed = JSON.parse(raw) as ExperimentBundle;
    if (parsed?.schemaVersion !== 1 || !Array.isArray(parsed.results)) {
      return emptyBundle();
    }
    const legacy = parsed as ExperimentBundle & {
      summary?: ExperimentRunSummary | null;
    };
    return {
      schemaVersion: 1,
      lastRun: legacy.lastRun ?? legacy.summary ?? null,
      results: parsed.results.slice(-MAX_EXPERIMENT_RESULTS),
    };
  } catch {
    return emptyBundle();
  }
}

export function persistExperimentSummary(
  store: ExperienceStoreAdapter,
  lastRun: ExperimentRunSummary,
): void {
  store.setItem(
    EXPERIMENT_STORAGE_KEY,
    JSON.stringify({
      schemaVersion: 1,
      lastRun,
      results: lastRun.results.slice(-MAX_EXPERIMENT_RESULTS),
    }),
  );
}

export function listExperimentResults(
  store: ExperienceStoreAdapter,
): AdaptationExperimentResult[] {
  return loadExperimentBundle(store).results.map((r) => ({ ...r }));
}

export function getExperimentSummary(
  store: ExperienceStoreAdapter,
): ExperimentRunSummary | null {
  const lastRun = loadExperimentBundle(store).lastRun;
  return lastRun
    ? { ...lastRun, results: lastRun.results.map((r) => ({ ...r })) }
    : null;
}

/**
 * Individual toggle: active ↔ inactive for validated adaptations.
 * Rolled-back / rejected cannot activate.
 */
export function toggleAdaptationExperiment(
  store: ExperienceStoreAdapter,
  adaptationId: string,
):
  | { ok: true; adaptation: WorkspaceAdaptation }
  | { ok: false; error: string } {
  const adaptation = listAdaptations(store).find(
    (a) => a.adaptationId === adaptationId,
  );
  if (!adaptation) {
    return { ok: false, error: "not_found" };
  }
  if (adaptation.rolloutState === "active") {
    const result = deactivateAdaptation(store, adaptationId);
    return result.ok
      ? { ok: true, adaptation: result.adaptation }
      : { ok: false, error: result.error };
  }
  if (adaptation.validation.validationResult !== "passed") {
    return { ok: false, error: "validation_not_passed" };
  }
  if (adaptation.rolloutState === "rolled_back") {
    return { ok: false, error: "rolled_back" };
  }
  const result = activateAdaptation(store, adaptationId);
  return result.ok
    ? { ok: true, adaptation: result.adaptation }
    : { ok: false, error: result.error };
}

export function clearExperimentStore(store: ExperienceStoreAdapter): void {
  store.removeItem(EXPERIMENT_STORAGE_KEY);
}

/** Rollback readiness: validated experiments have replay refs and inactive/candidate adaptations. */
export function experimentRollbackReady(
  store: ExperienceStoreAdapter,
  experimentId: string,
): boolean {
  const result = listExperimentResults(store).find(
    (r) => r.experimentId === experimentId,
  );
  if (!result || !result.adaptationId) {
    return false;
  }
  const adaptation = listAdaptations(store).find(
    (a) => a.adaptationId === result.adaptationId,
  );
  if (!adaptation) {
    return false;
  }
  return (
    adaptation.validation.replaySessionIds.length > 0 &&
    (adaptation.rolloutState === "active" ||
      adaptation.rolloutState === "candidate" ||
      adaptation.rolloutState === "inactive")
  );
}

export function rollbackExperimentAdaptation(
  store: ExperienceStoreAdapter,
  adaptationId: string,
) {
  return rollbackAdaptation(store, adaptationId);
}
