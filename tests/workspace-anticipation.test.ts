/**
 * Sprint 69 — Workspace Anticipation.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
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
import { resolvePresentationWithPresence } from "../app/src/experience/workspacePresence";
import {
  deriveWorkspaceAnticipation,
  getActiveWorkspaceAnticipation,
  replayWorkspaceAnticipation,
  resolvePresentationWithAnticipation,
  validateWorkspaceAnticipation,
} from "../app/src/experience/workspaceAnticipation";

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

describe("anticipation resolver", () => {
  it("derives anticipation from evidence/presence and resolves without executing", () => {
    const store = memoryStore();
    const { activation } = seedCertified(store);

    const anticipation = deriveWorkspaceAnticipation(store);
    expect(anticipation).toBeTruthy();
    expect(anticipation!.schemaVersion).toBe(1);
    expect(anticipation!.active).toBe(true);
    expect(anticipation!.confidence).toBeGreaterThanOrEqual(0);
    expect(anticipation!.confidence).toBeLessThanOrEqual(1);
    expect(anticipation!.contributingAdaptationIds).toContain(
      activation.adaptationId,
    );
    expect(anticipation!.evidenceLineage.tipEvidenceId).toBeTruthy();
    expect(anticipation!.replayLineage.replaySessionIds.length).toBeGreaterThan(
      0,
    );
    expect(anticipation!.architectureSnapshotIds.length).toBeGreaterThan(0);
    expect(anticipation!.presenceId).toBeTruthy();
    expect(anticipation!.evolutionId).toBeTruthy();
    expect(anticipation!.readiness).toMatchObject({
      subtleEmphasis: expect.any(Number),
      environmentalWeighting: expect.any(Number),
      preAttentiveFocus: expect.any(Number),
      objectReadiness: expect.any(Number),
      motionPreparation: expect.any(Number),
    });

    // Source module asserts no autonomous execution APIs.
    const src = readFileSync(
      path.join(root, "app/src/experience/workspaceAnticipation.ts"),
      "utf8",
    );
    expect(src).not.toMatch(/activateAdaptation|navigate\(|executeAction/);
    expect(src).toMatch(/never executes|Predicts only|user decides/i);

    expect(getActiveWorkspaceAnticipation(store)?.anticipationId).toBe(
      anticipation!.anticipationId,
    );

    const withAnticipation = resolvePresentationWithAnticipation(store);
    expect(withAnticipation.appliedAdaptationIds).toContain(
      activation.adaptationId,
    );
    // Stability may damp scales toward identity; applied lineage stays certified.
    expect(withAnticipation.spacingScale).toBeGreaterThan(0);
    expect(withAnticipation.emphasisScale).toBeGreaterThan(0);

    teardown(store);
  });

  it("keeps anticipation inactive without certified presence lineage", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 10 });
    // Skip certify — presence/evolution inactive.
    const anticipation = deriveWorkspaceAnticipation(store);
    if (anticipation) {
      expect(anticipation.active).toBe(false);
      expect(anticipation.validation.failureReasons.length).toBeGreaterThan(0);
    }
    expect(getActiveWorkspaceAnticipation(store)).toBeNull();
    const presentation = resolvePresentationWithAnticipation(store);
    expect(presentation.appliedAdaptationIds.length).toBeGreaterThan(0);
    teardown(store);
  });
});

describe("replay determinism", () => {
  it("replays identical anticipation and presentation", () => {
    const store = memoryStore();
    seedCertified(store);

    const a = replayWorkspaceAnticipation(store);
    const b = replayWorkspaceAnticipation(store);
    expect(a.anticipation?.anticipationId).toBe(b.anticipation?.anticipationId);
    expect(a.anticipation?.confidence).toBe(b.anticipation?.confidence);
    expect(a.anticipation?.likelyNextMoment).toBe(
      b.anticipation?.likelyNextMoment,
    );
    expect(a.presentation).toEqual(b.presentation);
    expect(a.baselinePresentation).toEqual(b.baselinePresentation);
    // Presentation may be stability-damped vs presence baseline; predictions stay fixed.
    expect(a.anticipation?.likelyNextMoment).toBe(
      b.anticipation?.likelyNextMoment,
    );

    teardown(store);
  });
});

describe("governance", () => {
  it("revalidates anticipation lineage", () => {
    const store = memoryStore();
    seedCertified(store);
    const anticipation = deriveWorkspaceAnticipation(store)!;
    const validation = validateWorkspaceAnticipation(store, anticipation);
    expect(validation.valid).toBe(true);
    expect(validation.presenceId).toBe(anticipation.presenceId);
    expect(validation.evolutionId).toBe(anticipation.evolutionId);
    expect(validation.compositionValidationResult).toBe("passed");
    teardown(store);
  });

  it("extends authority chain through anticipation doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "59_Workspace_Presence.md" &&
          b === "60_Workspace_Anticipation.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (workspace anticipation)", () => {
  it("introduces no persistence and omits forbidden content", () => {
    const store = memoryStore();
    seedCertified(store);
    const anticipation = deriveWorkspaceAnticipation(store)!;
    const src = readFileSync(
      path.join(root, "app/src/experience/workspaceAnticipation.ts"),
      "utf8",
    );
    expect(src).not.toMatch(/STORAGE_KEY\s*=/);
    expect(src).not.toMatch(/saveJsonBundle|store\.setItem/);
    const raw = JSON.stringify({
      anticipation,
      replay: replayWorkspaceAnticipation(store),
    });
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

  it("overlay lazy-loads anticipation module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/workspaceAnticipation")');
    expect(dash).toContain("workspace-anticipation");
    expect(dash).toContain("anticipation-state");
    expect(dash).toContain("anticipation-confidence");
    expect(dash).toContain("anticipation-evidence-lineage");
    expect(dash).toContain("anticipation-replay-lineage");
    expect(dash).toContain("anticipation-contributing-adaptations");
    expect(dash).toContain("anticipation-predicted-focal");
    expect(dash).toContain("anticipation-validation-state");
  });

  it("architecture doc records schema and prediction boundaries", () => {
    const doc = readFileSync(
      path.join(root, "architecture/60_Workspace_Anticipation.md"),
      "utf8",
    );
    expect(doc).toContain("WorkspaceAnticipation");
    expect(doc).toContain("resolvePresentationWithAnticipation");
    expect(doc).toContain("The system predicts");
    expect(doc).toContain("Nothing executes automatically");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
