/**
 * Sprint 68 — Workspace Presence.
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
import { resolvePresentationFromRuntime } from "../app/src/experience/workspaceMemoryEvolution";
import {
  deriveWorkspacePresence,
  getActiveWorkspacePresence,
  presenceContributorsExist,
  replayWorkspacePresence,
  resolvePresentationWithPresence,
  validateWorkspacePresence,
} from "../app/src/experience/workspacePresence";

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

describe("presence resolver", () => {
  it("derives presence from certified evolution and resolves presentation", () => {
    const store = memoryStore();
    const { activation } = seedCertified(store);

    const presence = deriveWorkspacePresence(store);
    expect(presence).toBeTruthy();
    expect(presence!.schemaVersion).toBe(1);
    expect(presence!.active).toBe(true);
    expect(presence!.contributingAdaptationIds).toContain(
      activation.adaptationId,
    );
    expect(presence!.evolutionId).toBeTruthy();
    expect(presence!.certificationIds.length).toBeGreaterThan(0);
    expect(presence!.environmentalCalm).toBeGreaterThanOrEqual(0);
    expect(presence!.environmentalCalm).toBeLessThanOrEqual(1);
    expect(presence!.resolvedEnvironment).toMatchObject({
      lighting: expect.any(Number),
      spacingRhythm: expect.any(Number),
      atmosphericIntensity: expect.any(Number),
      motionCadence: expect.any(Number),
      focalEmphasis: expect.any(Number),
      depthWeighting: expect.any(Number),
    });

    expect(getActiveWorkspacePresence(store)?.presenceId).toBe(
      presence!.presenceId,
    );

    const withPresence = resolvePresentationWithPresence(store);
    const baseline = resolvePresentationFromRuntime(store);
    expect(withPresence.spacingScale).toBe(baseline.spacingScale);
    expect(withPresence.emphasisScale).toBe(baseline.emphasisScale);
    expect(withPresence.environmentalWeight).toBe(baseline.environmentalWeight);
    expect(withPresence.appliedAdaptationIds).toContain(activation.adaptationId);
    expect(presenceContributorsExist(store)).toBe(true);

    teardown(store);
  });

  it("keeps presence inactive without certification lineage", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 10 });
    // Skip certify — evolution inactive ⇒ presence inactive.
    const presence = deriveWorkspacePresence(store);
    if (presence) {
      expect(presence.active).toBe(false);
      expect(presence.validation.failureReasons.length).toBeGreaterThan(0);
    }
    expect(getActiveWorkspacePresence(store)).toBeNull();

    const presentation = resolvePresentationWithPresence(store);
    expect(presentation.appliedAdaptationIds.length).toBeGreaterThan(0);

    teardown(store);
  });
});

describe("replay determinism", () => {
  it("replays identical presence and presentation", () => {
    const store = memoryStore();
    seedCertified(store);

    const a = replayWorkspacePresence(store);
    const b = replayWorkspacePresence(store);
    expect(a.presence?.presenceId).toBe(b.presence?.presenceId);
    expect(a.presentation).toEqual(b.presentation);
    expect(a.baselinePresentation).toEqual(b.baselinePresentation);
    expect(a.presentation.spacingScale).toBe(
      a.baselinePresentation.spacingScale,
    );
    expect(a.evolution?.evolutionId).toBe(b.evolution?.evolutionId);

    teardown(store);
  });
});

describe("governance", () => {
  it("revalidates presence lineage", () => {
    const store = memoryStore();
    seedCertified(store);
    const presence = deriveWorkspacePresence(store)!;
    const validation = validateWorkspacePresence(store, presence);
    expect(validation.valid).toBe(true);
    expect(validation.evolutionId).toBe(presence.evolutionId);
    expect(validation.compositionValidationResult).toBe("passed");
    teardown(store);
  });

  it("extends authority chain through presence doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "58_Workspace_Memory_Evolution.md" &&
          b === "59_Workspace_Presence.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (workspace presence)", () => {
  it("introduces no persistence and omits forbidden content", () => {
    const store = memoryStore();
    seedCertified(store);
    const presence = deriveWorkspacePresence(store)!;
    const src = readFileSync(
      path.join(root, "app/src/experience/workspacePresence.ts"),
      "utf8",
    );
    expect(src).not.toMatch(/STORAGE_KEY\s*=/);
    expect(src).not.toMatch(/saveJsonBundle|store\.setItem/);
    const raw = JSON.stringify({
      presence,
      replay: replayWorkspacePresence(store),
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

  it("overlay lazy-loads presence module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/workspacePresence")');
    expect(dash).toContain("workspace-presence");
    expect(dash).toContain("active-presence-profile");
    expect(dash).toContain("presence-contributing-adaptations");
    expect(dash).toContain("presence-evolution-lineage");
    expect(dash).toContain("presence-resolved-environment");
    expect(dash).toContain("presence-replay-comparison");
    expect(dash).toContain("presence-validation-status");
  });

  it("architecture doc records schema and resolver ordering", () => {
    const doc = readFileSync(
      path.join(root, "architecture/59_Workspace_Presence.md"),
      "utf8",
    );
    expect(doc).toContain("WorkspacePresence");
    expect(doc).toContain("resolvePresentationWithPresence");
    expect(doc).toContain("Workspace Memory Evolution");
    expect(doc).toContain("resolvedEnvironment");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
