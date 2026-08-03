/**
 * Sprint 65 — Governance consolidation compatibility.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  CERTIFICATION_STORAGE_KEY,
  certifyAdaptationSet,
  clearCertificationStore,
  hashAdaptationSet,
} from "../app/src/experience/adaptationCertification";
import {
  PACK_STORAGE_KEY,
  certifyAdaptationPack,
  clearAdaptationPackStore,
} from "../app/src/experience/adaptationPacks";
import {
  PRODUCTION_ACTIVATION_STORAGE_KEY,
  clearProductionActivationStore,
  runFirstProductionAdaptation,
} from "../app/src/experience/productionAdaptation";
import { clearExperimentStore } from "../app/src/experience/adaptationExperiments";
import { ADAPTATION_STORAGE_KEY } from "../app/src/experience/workspaceAdaptation";
import { validateComposition } from "../app/src/experience/adaptationComposition";
import {
  GOVERNANCE_ID_RE,
  loadJsonBundle,
  saveJsonBundle,
  stablePayloadHash,
  storeArchitectureIntegrityValid,
  tipOf,
} from "../app/src/dev/governancePrimitives";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("governance consolidation compatibility", () => {
  it("preserves storage keys and schemas through certify → pack pathway", () => {
    const store = memoryStore();
    const activation = runFirstProductionAdaptation(store, { now: 10 });
    expect(activation.outcome).toBe("activated");

    const cert = certifyAdaptationSet(store, { now: 100 });
    expect(cert.ok).toBe(true);
    const pack = certifyAdaptationPack(store, { now: 200 });
    expect(pack.ok).toBe(true);

    // Keys unchanged.
    expect(store.getItem(CERTIFICATION_STORAGE_KEY)).toBeTruthy();
    expect(store.getItem(PACK_STORAGE_KEY)).toBeTruthy();
    expect(store.getItem(PRODUCTION_ACTIVATION_STORAGE_KEY)).toBeTruthy();
    expect(store.getItem(ADAPTATION_STORAGE_KEY)).toBeTruthy();

    const certBundle = JSON.parse(store.getItem(CERTIFICATION_STORAGE_KEY)!);
    expect(certBundle.schemaVersion).toBe(1);
    expect(Array.isArray(certBundle.certifications)).toBe(true);
    expect(certBundle.certifications[0].certificationId).toBe(
      cert.certification!.certificationId,
    );

    const packBundle = JSON.parse(store.getItem(PACK_STORAGE_KEY)!);
    expect(packBundle.schemaVersion).toBe(1);
    expect(Array.isArray(packBundle.packs)).toBe(true);
    expect(packBundle.packs[0].packId).toBe(pack.pack!.packId);
    expect(packBundle.packs[0].compositionHash).toBe(pack.pack!.compositionHash);

    clearAdaptationPackStore(store);
    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });

  it("keeps governance validation equivalent after consolidation", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 10 });
    const composition = validateComposition(store);
    expect(composition.validationResult).toBe("passed");
    expect(composition.governanceIntact).toBe(true);
    expect(storeArchitectureIntegrityValid(store)).toBe(true);

    const a = certifyAdaptationSet(store, { now: 100 });
    const b = certifyAdaptationSet(store, { now: 200 });
    expect(a.ok).toBe(true);
    expect(b.ok).toBe(true);
    expect(b.certification!.previousCertificationId).toBe(
      a.certification!.certificationId,
    );

    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });

  it("shared primitives are deterministic", () => {
    expect(stablePayloadHash(["a", "b"])).toBe(stablePayloadHash(["a", "b"]));
    expect(hashAdaptationSet(["x"], "e", "a", 1)).toBe(
      stablePayloadHash(["x", "e", "a", "1"]),
    );
    expect(tipOf([1, 2, 3])).toBe(3);
    expect(tipOf([])).toBeNull();
    expect(GOVERNANCE_ID_RE.test("adapt-abc")).toBe(true);

    const store = memoryStore();
    saveJsonBundle(store, "ws.test.consol.v1", { schemaVersion: 1, items: [1] });
    const loaded = loadJsonBundle(
      store,
      "ws.test.consol.v1",
      () => ({ schemaVersion: 1 as const, items: [] as number[] }),
      (p): p is { schemaVersion: 1; items: number[] } =>
        !!p &&
        typeof p === "object" &&
        (p as { schemaVersion: number }).schemaVersion === 1 &&
        Array.isArray((p as { items: unknown }).items),
    );
    expect(loaded.items).toEqual([1]);
  });
});

describe("governance + authority", () => {
  it("extends authority chain through consolidation doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "55_Adaptation_Packs.md" &&
          b === "56_Governance_Consolidation.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (governance consolidation)", () => {
  it("primitives module introduces no network telemetry", () => {
    const dir = path.join(root, "app/src/dev");
    for (const file of readdirSync(dir).filter((f) => /\.(ts|tsx)$/.test(f))) {
      const src = readFileSync(path.join(dir, file), "utf8");
      expect(src).not.toMatch(/\bfetch\s*\(/);
      expect(src).not.toMatch(/sendBeacon/);
      expect(src).not.toMatch(/XMLHttpRequest/);
      expect(src).not.toMatch(/WebSocket/);
    }
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

  it("consolidated stores still omit forbidden content keys", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 10 });
    certifyAdaptationSet(store, { now: 100 });
    certifyAdaptationPack(store, { now: 200 });
    for (const key of [
      CERTIFICATION_STORAGE_KEY,
      PACK_STORAGE_KEY,
      PRODUCTION_ACTIVATION_STORAGE_KEY,
      ADAPTATION_STORAGE_KEY,
    ]) {
      const raw = store.getItem(key)!;
      for (const forbidden of FORBIDDEN_EVENT_KEYS) {
        expect(raw).not.toContain(`"${forbidden}"`);
      }
      expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
    }
    clearAdaptationPackStore(store);
    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });

  it("architecture doc records consolidation inventory", () => {
    const doc = readFileSync(
      path.join(root, "architecture/56_Governance_Consolidation.md"),
      "utf8",
    );
    expect(doc).toContain("governancePrimitives");
    expect(doc).toContain("governanceStore");
    expect(doc).toContain("storeArchitectureIntegrityValid");
    expect(doc).toContain("Preserved public contracts");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
