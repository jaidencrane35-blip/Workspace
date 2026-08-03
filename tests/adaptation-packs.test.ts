/**
 * Sprint 64 — Certified adaptation packs.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  listAdaptations,
  resolvePresentationConfiguration,
} from "../app/src/experience/workspaceAdaptation";
import {
  clearExperimentStore,
} from "../app/src/experience/adaptationExperiments";
import {
  clearProductionActivationStore,
  runFirstProductionAdaptation,
} from "../app/src/experience/productionAdaptation";
import {
  certifyAdaptationSet,
  clearCertificationStore,
} from "../app/src/experience/adaptationCertification";
import {
  PACK_STORAGE_KEY,
  activateAdaptationPack,
  certifyAdaptationPack,
  clearAdaptationPackStore,
  deactivateAdaptationPack,
  getActiveAdaptationPack,
  listAdaptationPacks,
  packActivationReady,
} from "../app/src/experience/adaptationPacks";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

function seedCertifiedActive(store: ReturnType<typeof memoryStore>) {
  const activation = runFirstProductionAdaptation(store, { now: 10 });
  expect(activation.outcome).toBe("activated");
  const cert = certifyAdaptationSet(store, { now: 100 });
  expect(cert.ok).toBe(true);
  return { activation, cert };
}

describe("pack certification", () => {
  it("certifies a pack from certified adaptations", () => {
    const store = memoryStore();
    const { activation, cert } = seedCertifiedActive(store);

    const result = certifyAdaptationPack(store, { now: 200 });
    expect(result.ok).toBe(true);
    expect(result.failureReasons).toEqual([]);
    expect(result.pack).toBeTruthy();
    expect(result.pack!.rolloutStatus).toBe("certified");
    expect(result.pack!.version).toBe(1);
    expect(result.pack!.adaptationIds).toContain(activation.adaptationId);
    expect(result.pack!.certificationIds).toContain(
      cert.certification!.certificationId,
    );
    expect(result.pack!.compositionHash).toBeTruthy();
    expect(result.pack!.stabilitySummary.adaptationCount).toBeGreaterThan(0);
    expect(result.pack!.evidenceSummary.tipEvidenceId).toBeTruthy();

    expect(listAdaptationPacks(store)).toHaveLength(1);

    // Duplicate identical pack content rejected; no write.
    const dup = certifyAdaptationPack(store, { now: 300 });
    expect(dup.ok).toBe(false);
    expect(dup.failureReasons).toContain("duplicate_pack");
    expect(listAdaptationPacks(store)).toHaveLength(1);

    clearAdaptationPackStore(store);
    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });

  it("fails when adaptations are not certified", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 10 });
    // Skip certifyAdaptationSet.
    const result = certifyAdaptationPack(store, { now: 20 });
    expect(result.ok).toBe(false);
    expect(result.failureReasons).toContain("adaptation_not_certified");
    expect(listAdaptationPacks(store)).toHaveLength(0);

    clearAdaptationPackStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });
});

describe("pack activation", () => {
  it("activates and deactivates a pack via existing adaptation pathway", () => {
    const store = memoryStore();
    seedCertifiedActive(store);
    const certified = certifyAdaptationPack(store, { now: 200 });
    expect(certified.ok).toBe(true);
    const pack = certified.pack!;
    expect(packActivationReady(store, pack)).toBe(true);

    // Clear production actives, then activate through pack pathway.
    expect(
      deactivateAdaptationPack(store, pack.packId, pack.version).ok,
    ).toBe(true);
    for (const id of pack.adaptationIds) {
      expect(
        listAdaptations(store).find((a) => a.adaptationId === id)?.rolloutState,
      ).not.toBe("active");
    }

    const activated = activateAdaptationPack(
      store,
      pack.packId,
      pack.version,
    );
    expect(activated.ok).toBe(true);
    expect(activated.activatedIds).toEqual(pack.adaptationIds);
    expect(getActiveAdaptationPack(store)?.packId).toBe(pack.packId);
    expect(getActiveAdaptationPack(store)?.version).toBe(pack.version);

    for (const id of pack.adaptationIds) {
      expect(
        listAdaptations(store).find((a) => a.adaptationId === id)?.rolloutState,
      ).toBe("active");
    }
    expect(
      resolvePresentationConfiguration(listAdaptations(store))
        .appliedAdaptationIds,
    ).toEqual([...pack.adaptationIds].sort((a, b) => a.localeCompare(b)));

    const off = deactivateAdaptationPack(store, pack.packId, pack.version);
    expect(off.ok).toBe(true);
    expect(getActiveAdaptationPack(store)).toBeNull();
    for (const id of pack.adaptationIds) {
      expect(
        listAdaptations(store).find((a) => a.adaptationId === id)?.rolloutState,
      ).not.toBe("active");
    }

    clearAdaptationPackStore(store);
    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });

  it("keeps pack records immutable after activation", () => {
    const store = memoryStore();
    seedCertifiedActive(store);
    const certified = certifyAdaptationPack(store, { now: 200 });
    const before = listAdaptationPacks(store)[0]!;
    activateAdaptationPack(store, before.packId, before.version);
    const after = listAdaptationPacks(store)[0]!;
    expect(after).toEqual(before);
    expect(after.rolloutStatus).toBe("certified");

    clearAdaptationPackStore(store);
    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });
});

describe("governance + authority", () => {
  it("extends authority chain through packs doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "54_Adaptation_Certification.md" &&
          b === "55_Adaptation_Packs.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (adaptation packs)", () => {
  it("pack storage omits forbidden content keys", () => {
    const store = memoryStore();
    seedCertifiedActive(store);
    certifyAdaptationPack(store, { now: 200 });
    const raw = store.getItem(PACK_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
    clearAdaptationPackStore(store);
    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
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

  it("overlay lazy-loads packs module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/adaptationPacks")');
    expect(dash).toContain("adaptation-packs");
    expect(dash).toContain("run-pack-certification");
    expect(dash).toContain("active-pack");
  });

  it("architecture doc records pack schema and activation model", () => {
    const doc = readFileSync(
      path.join(root, "architecture/55_Adaptation_Packs.md"),
      "utf8",
    );
    expect(doc).toContain("WorkspaceAdaptationPack");
    expect(doc).toContain("certifyAdaptationPack");
    expect(doc).toContain("activateAdaptationPack");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
