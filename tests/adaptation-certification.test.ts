/**
 * Sprint 63 — Adaptation regression certification.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import type {
  ExperienceEvent,
  ExperienceSession,
} from "../app/src/dev/experienceEvents";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import {
  buildEvidenceFromSessions,
  persistEvidenceSnapshot,
} from "../app/src/dev/experienceEvidence";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  clearProductionActivationStore,
  runFirstProductionAdaptation,
} from "../app/src/experience/productionAdaptation";
import {
  CERTIFICATION_STORAGE_KEY,
  certifyAdaptationSet,
  clearCertificationStore,
  compareCertifications,
  getLatestCertification,
  listCertifications,
} from "../app/src/experience/adaptationCertification";
import { clearExperimentStore } from "../app/src/experience/adaptationExperiments";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

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

const thrash = session("s-cert-thrash", [
  { seq: 0, t: 0, type: "session_start", destination: "home" },
  {
    seq: 1,
    t: 12000,
    type: "first_meaningful_interaction",
    destination: "home",
    modality: "pointer",
  },
  nav(2, 13000, "home", "save"),
  { seq: 3, t: 13100, type: "flow_start", flow: "save", destination: "save" },
  nav(4, 15000, "save", "resume"),
  {
    seq: 5,
    t: 15050,
    type: "flow_abandon",
    flow: "save",
    from: "save",
    destination: "resume",
  },
  nav(6, 17000, "resume", "save"),
  nav(7, 19000, "save", "home"),
  nav(8, 20000, "home", "save"),
]);

describe("certification gate", () => {
  it("certifies a governed active set when gate conditions hold", () => {
    const store = memoryStore();
    const activation = runFirstProductionAdaptation(store, { now: 10 });
    expect(activation.outcome).toBe("activated");

    const result = certifyAdaptationSet(store, { now: 100 });
    expect(result.ok).toBe(true);
    expect(result.failureReasons).toEqual([]);
    expect(result.certification).toBeTruthy();
    expect(result.certification!.adaptationIds).toContain(
      activation.adaptationId,
    );
    expect(result.certification!.governanceValid).toBe(true);
    expect(result.certification!.integrityValid).toBe(true);
    expect(result.certification!.regressionStatus).toBe("clear");
    expect(result.certification!.evidenceSnapshotId).toBeTruthy();
    expect(result.certification!.architectureSnapshotId).toBeTruthy();
    expect(result.certification!.engineeringChangeIds.length).toBeGreaterThan(0);
    expect(result.certification!.certifiedMetrics.meanFrictionScore).toBeTypeOf(
      "number",
    );

    expect(listCertifications(store)).toHaveLength(1);
    expect(getLatestCertification(store)?.certificationId).toBe(
      result.certification!.certificationId,
    );

    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });

  it("fails without writing when evidence is incomplete", () => {
    const store = memoryStore();
    const result = certifyAdaptationSet(store, { now: 1 });
    expect(result.ok).toBe(false);
    expect(result.failureReasons).toContain("evidence_incomplete");
    expect(result.certification).toBeNull();
    expect(listCertifications(store)).toHaveLength(0);
  });

  it("appends a second certification when regression-free", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 10 });
    const first = certifyAdaptationSet(store, { now: 100 });
    expect(first.ok).toBe(true);

    const second = certifyAdaptationSet(store, { now: 200 });
    expect(second.ok).toBe(true);
    expect(second.certification!.previousCertificationId).toBe(
      first.certification!.certificationId,
    );
    expect(listCertifications(store)).toHaveLength(2);
    // Immutability: first record unchanged.
    expect(listCertifications(store)[0]).toEqual(first.certification);

    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });
});

describe("regression comparison", () => {
  it("detects metric regression and blocks certification", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 10 });
    const first = certifyAdaptationSet(store, { now: 100 });
    expect(first.ok).toBe(true);
    const beforeCount = listCertifications(store).length;

    // Tip evidence becomes high-friction — metric regression vs prior cert.
    const bad = buildEvidenceFromSessions([thrash], { tag: "cert_bad" });
    persistEvidenceSnapshot(store, bad);

    const blocked = certifyAdaptationSet(store, { now: 300 });
    expect(blocked.ok).toBe(false);
    expect(blocked.failureReasons).toContain("regression_detected");
    expect(blocked.certification).toBeNull();
    expect(listCertifications(store)).toHaveLength(beforeCount);

    expect(blocked.comparison).toBeTruthy();
    expect(blocked.comparison!.regressionFree).toBe(false);
    expect(
      blocked.comparison!.regressions.some((r) => r.cause === "metric_regression"),
    ).toBe(true);

    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });

  it("compareCertifications reports deterministic causes", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 10 });
    const a = certifyAdaptationSet(store, { now: 100 });
    expect(a.ok).toBe(true);

    const worse = buildEvidenceFromSessions([thrash], { tag: "cert_cmp" });
    persistEvidenceSnapshot(store, worse);
    // Build a synthetic current candidate via failed gate comparison.
    const gate = certifyAdaptationSet(store, { now: 400 });
    expect(gate.ok).toBe(false);
    expect(gate.comparison).toBeTruthy();

    const cmp = compareCertifications(
      a.certification!,
      {
        ...a.certification!,
        certificationId: "acert-synthetic",
        evidenceSnapshotId: worse.evidenceId,
        certifiedMetrics: {
          ...a.certification!.certifiedMetrics,
          meanFrictionScore:
            (a.certification!.certifiedMetrics.meanFrictionScore ?? 0) + 0.5,
          medianTimeToConfidenceMs:
            (a.certification!.certifiedMetrics.medianTimeToConfidenceMs ?? 0) +
            5000,
        },
        composedStabilityScore:
          (a.certification!.composedStabilityScore ?? 1) - 0.2,
        integrityValid: false,
        governanceValid: false,
      },
      null,
      null,
    );
    expect(cmp.regressionFree).toBe(false);
    expect(cmp.regressions.some((r) => r.cause === "metric_regression")).toBe(
      true,
    );
    expect(cmp.regressions.some((r) => r.cause === "stability_regression")).toBe(
      true,
    );
    expect(
      cmp.regressions.some((r) => r.cause === "integrity_regression"),
    ).toBe(true);
    expect(
      cmp.regressions.some((r) => r.cause === "governance_regression"),
    ).toBe(true);

    clearCertificationStore(store);
    clearProductionActivationStore(store);
    clearExperimentStore(store);
  });
});

describe("governance + authority", () => {
  it("extends authority chain through certification doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "53_Adaptation_Composition.md" &&
          b === "54_Adaptation_Certification.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (adaptation certification)", () => {
  it("certification storage omits forbidden content keys", () => {
    const store = memoryStore();
    runFirstProductionAdaptation(store, { now: 10 });
    certifyAdaptationSet(store, { now: 100 });
    const raw = store.getItem(CERTIFICATION_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
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

  it("overlay lazy-loads certification module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/adaptationCertification")');
    expect(dash).toContain("adaptation-certification");
    expect(dash).toContain("run-adaptation-certification");
    expect(dash).toContain("certification-history");
  });

  it("architecture doc records certification schema and gate", () => {
    const doc = readFileSync(
      path.join(root, "architecture/54_Adaptation_Certification.md"),
      "utf8",
    );
    expect(doc).toContain("AdaptationCertification");
    expect(doc).toContain("certifyAdaptationSet");
    expect(doc).toContain("compareCertifications");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
