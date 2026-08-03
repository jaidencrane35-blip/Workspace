/**
 * Sprint 67 — Workspace Memory Evolution.
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
import {
  activateAdaptationPack,
  certifyAdaptationPack,
  clearAdaptationPackStore,
} from "../app/src/experience/adaptationPacks";
import {
  clearProductionActivationStore,
  runFirstProductionAdaptation,
} from "../app/src/experience/productionAdaptation";
import {
  listAdaptations,
  resolvePresentationConfiguration,
} from "../app/src/experience/workspaceAdaptation";
import {
  deriveActiveMemoryEvolution,
  getActiveMemoryEvolution,
  listMemoryEvolutions,
  replayMemoryEvolution,
  resolvePresentationFromRuntime,
  validateMemoryEvolution,
} from "../app/src/experience/workspaceMemoryEvolution";

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

describe("evolution resolver", () => {
  it("derives active evolution from certified adaptations and resolves presentation", () => {
    const store = memoryStore();
    const { activation } = seedCertified(store);

    const evolution = deriveActiveMemoryEvolution(store);
    expect(evolution).toBeTruthy();
    expect(evolution!.schemaVersion).toBe(1);
    expect(evolution!.originatingAdaptationIds).toContain(
      activation.adaptationId,
    );
    expect(evolution!.evidenceLineage.evidenceSnapshotIds.length).toBeGreaterThan(
      0,
    );
    expect(
      evolution!.certificationLineage.certificationIds.length,
    ).toBeGreaterThan(0);
    expect(evolution!.architectureSnapshotIds.length).toBeGreaterThan(0);
    expect(evolution!.presentationDelta).toMatchObject({
      spacingScale: expect.any(Number),
      emphasisScale: expect.any(Number),
      groupingTightness: expect.any(Number),
      environmentalWeight: expect.any(Number),
    });
    expect(evolution!.active).toBe(true);

    const active = getActiveMemoryEvolution(store);
    expect(active?.evolutionId).toBe(evolution!.evolutionId);

    const presentation = resolvePresentationFromRuntime(store);
    expect(presentation.appliedAdaptationIds).toContain(
      activation.adaptationId,
    );

    // Matches composing the same active set.
    const baseline = resolvePresentationConfiguration(listAdaptations(store));
    expect(presentation.spacingScale).toBe(baseline.spacingScale);
    expect(presentation.emphasisScale).toBe(baseline.emphasisScale);
    expect(presentation.appliedAdaptationIds).toEqual(
      baseline.appliedAdaptationIds,
    );

    teardown(store);
  });

  it("keeps invalid evolution inactive", () => {
    const store = memoryStore();
    // Adaptations without certification → no certified members.
    runFirstProductionAdaptation(store, { now: 10 });
    // Skip certifyAdaptationSet.
    const evolution = deriveActiveMemoryEvolution(store);
    // May be null (no certified members) or inactive.
    if (evolution) {
      expect(evolution.active).toBe(false);
      expect(evolution.validation.failureReasons.length).toBeGreaterThan(0);
    }
    expect(getActiveMemoryEvolution(store)).toBeNull();

    // Fallback still resolves active adaptations.
    const presentation = resolvePresentationFromRuntime(store);
    expect(presentation.appliedAdaptationIds.length).toBeGreaterThan(0);

    teardown(store);
  });

  it("uses active pack pathway when pack is activated", () => {
    const store = memoryStore();
    seedCertified(store);
    const packResult = certifyAdaptationPack(store, { now: 200 });
    expect(packResult.ok).toBe(true);
    const activated = activateAdaptationPack(
      store,
      packResult.pack!.packId,
      packResult.pack!.version,
    );
    expect(activated.ok).toBe(true);

    const evolution = getActiveMemoryEvolution(store);
    expect(evolution).toBeTruthy();
    expect(evolution!.packId).toBe(packResult.pack!.packId);
    expect(evolution!.packVersion).toBe(packResult.pack!.version);
    expect(evolution!.active).toBe(true);

    teardown(store);
  });
});

describe("replay determinism", () => {
  it("replays identical evolution id and presentation", () => {
    const store = memoryStore();
    seedCertified(store);

    const a = replayMemoryEvolution(store);
    const b = replayMemoryEvolution(store);
    expect(a.evolution?.evolutionId).toBe(b.evolution?.evolutionId);
    expect(a.presentation).toEqual(b.presentation);
    expect(a.evolution?.presentationDelta).toEqual(
      b.evolution?.presentationDelta,
    );

    const history = listMemoryEvolutions(store);
    expect(history.length).toBeGreaterThan(0);
    expect(history[0]!.evolutionId).toBeTruthy();

    teardown(store);
  });
});

describe("governance", () => {
  it("revalidates lineage on stored evolution", () => {
    const store = memoryStore();
    seedCertified(store);
    const evolution = deriveActiveMemoryEvolution(store)!;
    const validation = validateMemoryEvolution(store, evolution);
    expect(validation.valid).toBe(true);
    expect(validation.replaySessionIds.length).toBeGreaterThan(0);
    expect(validation.compositionValidationResult).toBe("passed");
    teardown(store);
  });

  it("extends authority chain through memory evolution doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "57_Real_World_Adaptation_Validation.md" &&
          b === "58_Workspace_Memory_Evolution.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (workspace memory evolution)", () => {
  it("introduces no persistence keys and omits forbidden content", () => {
    const store = memoryStore();
    seedCertified(store);
    const evolution = deriveActiveMemoryEvolution(store)!;
    const src = readFileSync(
      path.join(root, "app/src/experience/workspaceMemoryEvolution.ts"),
      "utf8",
    );
    expect(src).not.toMatch(/STORAGE_KEY\s*=/);
    expect(src).not.toMatch(/saveJsonBundle|store\.setItem/);
    const raw = JSON.stringify({
      evolution,
      history: listMemoryEvolutions(store),
      presentation: resolvePresentationFromRuntime(store),
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

  it("overlay lazy-loads evolution module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/workspaceMemoryEvolution")');
    expect(dash).toContain("workspace-memory-evolution");
    expect(dash).toContain("active-memory-evolution");
    expect(dash).toContain("evolution-history");
    expect(dash).toContain("evolution-presentation-delta");
    expect(dash).toContain("evolution-evidence-lineage");
    expect(dash).toContain("evolution-replay-lineage");
  });

  it("architecture doc records schema and resolver ordering", () => {
    const doc = readFileSync(
      path.join(root, "architecture/58_Workspace_Memory_Evolution.md"),
      "utf8",
    );
    expect(doc).toContain("WorkspaceMemoryEvolution");
    expect(doc).toContain("resolvePresentationFromRuntime");
    expect(doc).toContain("Active Adaptation Pack");
    expect(doc).toContain("presentationDelta");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
