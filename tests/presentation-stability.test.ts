/**
 * Sprint 71 — Presentation Stability.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  persistEvidenceSnapshot,
  type ExperienceEvidence,
} from "../app/src/dev/experienceEvidence";
import { clearExperimentStore } from "../app/src/experience/adaptationExperiments";
import {
  certifyAdaptationSet,
  clearCertificationStore,
} from "../app/src/experience/adaptationCertification";
import { clearAdaptationPackStore } from "../app/src/experience/adaptationPacks";
import {
  clearProductionActivationStore,
  runFirstProductionAdaptation,
} from "../app/src/experience/productionAdaptation";
import {
  deriveWorkspaceAnticipation,
  resolvePresentationWithAnticipation,
} from "../app/src/experience/workspaceAnticipation";
import {
  applyStabilityToPresentation,
  deriveWorkspacePresentationStability,
  measurePresentationStability,
  replayPresentationStability,
} from "../app/src/experience/workspacePresentationStability";
import { identityPresentation } from "../app/src/experience/workspaceAdaptation";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

function seedCertified(store: ReturnType<typeof memoryStore>) {
  const activation = runFirstProductionAdaptation(store, { now: 10 });
  expect(activation.outcome).toBe("activated");
  const cert = certifyAdaptationSet(store, { now: 100 });
  expect(cert.ok).toBe(true);
  return { activation, cert };
}

function teardown(store: ReturnType<typeof memoryStore>) {
  clearAdaptationPackStore(store);
  clearCertificationStore(store);
  clearProductionActivationStore(store);
  clearExperimentStore(store);
}

function evidenceAt(
  id: string,
  friction: number,
  hesitationMs: number,
): ExperienceEvidence {
  return {
    schemaVersion: 1,
    evidenceId: id,
    fingerprint: `fp-${id}`,
    tag: `tag_${id}`,
    sourceSessionIds: [`s-${id}`],
    metrics: {
      sessionCount: 3,
      medianTimeToConfidenceMs: 100,
      meanFrictionScore: friction,
      medianFrictionScore: friction,
      frictionMin: friction,
      frictionMax: friction,
      frictionP25: friction,
      frictionP75: friction,
      hesitationHotspotCount: 1,
      topHesitationDestination: "home",
      topHesitationMedianGapMs: hesitationMs,
      navigationLoopCount: 0,
      abandonedFlowTotal: 0,
      abandonedSave: 0,
      abandonedContinue: 0,
      recoverySuccessRate: 1,
      recoveryCount: 0,
      interruptionCount: 0,
      replayCount: 1,
      replayDivergenceRate: 0.05,
      saveSuccessTotal: 1,
      continueSuccessTotal: 1,
    },
    hotspots: [{ destination: "home", medianGapMs: hesitationMs, samples: 3 }],
    loops: [],
  };
}

describe("presentation stability measurement", () => {
  it("measures lower variance for calm evidence series", () => {
    const calm = [
      evidenceAt("c1", 0.2, 100),
      evidenceAt("c2", 0.21, 110),
      evidenceAt("c3", 0.19, 90),
    ];
    const volatile = [
      evidenceAt("v1", 0.1, 50),
      evidenceAt("v2", 0.8, 1500),
      evidenceAt("v3", 0.2, 200),
    ];
    const calmM = measurePresentationStability(calm);
    const volatileM = measurePresentationStability(volatile);
    expect(calmM.presentationVariance).toBeLessThan(
      volatileM.presentationVariance,
    );
    expect(calmM.presentationStabilityScore).toBeGreaterThan(
      volatileM.presentationStabilityScore,
    );
    expect(calmM.varianceTrend.length).toBe(3);
  });

  it("damps presentation toward identity when stability active", () => {
    const extreme = {
      ...identityPresentation(),
      emphasisScale: 1.5,
      environmentalWeight: 1.5,
      spacingScale: 1.4,
      motionProfile: "expressive" as const,
      appliedAdaptationIds: ["adapt-x"],
    };
    const stability = {
      schemaVersion: 1 as const,
      stabilityId: "stability-test",
      presentationStabilityScore: 0.9,
      environmentalStability: 0.9,
      motionContinuity: 0.9,
      focalStability: 0.9,
      transitionConsistency: 0.9,
      presentationVariance: 0.01,
      varianceTrend: [0.01],
      evidenceLineage: { evidenceSnapshotIds: [], tipEvidenceId: null },
      replayLineage: { replaySessionIds: [], bundleReplayInvocations: 0 },
      calibrationLineageId: "cal",
      anticipationLineageId: "ant",
      architectureSnapshotIds: ["arch"],
      motionDamping: 0.6,
      transitionCadence: 0.5,
      emphasisSmoothing: 0.5,
      environmentalInterpolation: 0.5,
      focalSettling: 0.6,
      atmosphericContinuity: 0.5,
      active: true,
      validation: {
        valid: true,
        failureReasons: [],
        compositionValidationResult: "passed" as const,
        composedStabilityScore: 1,
      },
    };
    const damped = applyStabilityToPresentation(extreme, stability);
    expect(damped.emphasisScale).toBeLessThan(extreme.emphasisScale!);
    expect(damped.environmentalWeight).toBeLessThan(
      extreme.environmentalWeight!,
    );
    expect(damped.appliedAdaptationIds).toEqual(["adapt-x"]);
    expect(applyStabilityToPresentation(extreme, { ...stability, active: false }))
      .toEqual(extreme);
  });
});

describe("resolver integration", () => {
  it("applies stability internally without changing predictions", () => {
    const store = memoryStore();
    seedCertified(store);
    persistEvidenceSnapshot(store, evidenceAt("s1", 0.2, 100));
    persistEvidenceSnapshot(store, evidenceAt("s2", 0.2, 100));
    persistEvidenceSnapshot(store, evidenceAt("s3", 0.21, 105));

    const anticipation = deriveWorkspaceAnticipation(store)!;
    const beforeMoments = {
      next: anticipation.likelyNextMoment,
      continue: anticipation.likelyContinuationTarget,
      focal: anticipation.likelyFocalRegion,
    };
    const resolved = resolvePresentationWithAnticipation(store);
    const again = deriveWorkspaceAnticipation(store)!;
    expect(again.likelyNextMoment).toBe(beforeMoments.next);
    expect(again.likelyContinuationTarget).toBe(beforeMoments.continue);
    expect(again.likelyFocalRegion).toBe(beforeMoments.focal);
    expect(resolved.appliedAdaptationIds.length).toBeGreaterThan(0);

    const stability = deriveWorkspacePresentationStability(store, {
      anticipationLineageId: anticipation.anticipationId,
    });
    expect(stability).toBeTruthy();
    expect(stability!.evidenceLineage.tipEvidenceId).toBeTruthy();
    expect(stability!.architectureSnapshotIds.length).toBeGreaterThan(0);
    // Calibration lineage always referenced when calibration object exists.
    expect(stability!.calibrationLineageId).toBeTruthy();

    teardown(store);
  });
});

describe("replay determinism", () => {
  it("replays identical stability measurement", () => {
    const store = memoryStore();
    seedCertified(store);
    persistEvidenceSnapshot(store, evidenceAt("r1", 0.2, 100));
    persistEvidenceSnapshot(store, evidenceAt("r2", 0.2, 100));

    const a = replayPresentationStability(store);
    const b = replayPresentationStability(store);
    expect(a.stability).toEqual(b.stability);
    expect(a.measurement).toEqual(b.measurement);
    expect(
      resolvePresentationWithAnticipation(store),
    ).toEqual(resolvePresentationWithAnticipation(store));

    teardown(store);
  });
});

describe("governance", () => {
  it("extends authority chain through presentation stability doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "61_Workspace_Calibration.md" &&
          b === "62_Presentation_Stability.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (presentation stability)", () => {
  it("introduces no persistence and omits forbidden content", () => {
    const store = memoryStore();
    seedCertified(store);
    const src = readFileSync(
      path.join(root, "app/src/experience/workspacePresentationStability.ts"),
      "utf8",
    );
    expect(src).not.toMatch(/STORAGE_KEY\s*=/);
    expect(src).not.toMatch(/saveJsonBundle|store\.setItem/);
    const raw = JSON.stringify(replayPresentationStability(store));
    for (const forbidden of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${forbidden}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
    teardown(store);
  });

  it("experience modules introduce no network telemetry", () => {
    const dir = path.join(root, "app/src/experience");
    for (const file of readdirSync(dir).filter((f) => /\.(ts|tsx)$/.test(f))) {
      const src = readFileSync(path.join(dir, file), "utf8");
      expect(src).not.toMatch(/\bfetch\s*\(/);
      expect(src).not.toMatch(/sendBeacon/);
      expect(src).not.toMatch(/XMLHttpRequest/);
      expect(src).not.toMatch(/WebSocket/);
    }
  });

  it("overlay lazy-loads stability module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain(
      'import("../experience/workspacePresentationStability")',
    );
    expect(dash).toContain("presentation-stability");
    expect(dash).toContain("stability-score");
    expect(dash).toContain("stability-variance-trend");
    expect(dash).toContain("stability-transition-consistency");
    expect(dash).toContain("stability-environmental-continuity");
    expect(dash).toContain("stability-evidence-lineage");
    expect(dash).toContain("stability-replay-lineage");
  });

  it("architecture doc records variance model", () => {
    const doc = readFileSync(
      path.join(root, "architecture/62_Presentation_Stability.md"),
      "utf8",
    );
    expect(doc).toContain("WorkspacePresentationStability");
    expect(doc).toContain("presentationVariance");
    expect(doc).toContain("No new resolver stage");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
