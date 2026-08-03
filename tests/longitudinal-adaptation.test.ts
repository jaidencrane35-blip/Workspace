/**
 * Sprint 59 — Longitudinal adaptation validation.
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
  listEvidenceSnapshots,
  persistEvidenceSnapshot,
} from "../app/src/dev/experienceEvidence";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  ADAPTATION_STORAGE_KEY,
  activateAdaptation,
  listAdaptations,
} from "../app/src/experience/workspaceAdaptation";
import {
  runAdaptationExperiments,
  clearExperimentStore,
} from "../app/src/experience/adaptationExperiments";
import {
  LONGITUDINAL_MIN_OBSERVATIONS,
  LONGITUDINAL_STABILITY_THRESHOLD,
  analyzeAdaptationStability,
  appendLongitudinalObservation,
  getLongitudinalRecord,
  getStabilityReport,
  isRolloutReady,
  listRolloutCandidates,
  promoteToRolloutCandidate,
  runLongitudinalValidation,
} from "../app/src/experience/longitudinalAdaptation";

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

function calmSession(id: string, ttc = 400): ExperienceSession {
  return session(id, [
    { seq: 0, t: 0, type: "session_start", destination: "home" },
    {
      seq: 1,
      t: ttc,
      type: "first_meaningful_interaction",
      destination: "home",
      modality: "pointer",
    },
    nav(2, ttc + 200, "home", "save"),
    {
      seq: 3,
      t: ttc + 300,
      type: "flow_start",
      flow: "save",
      destination: "save",
    },
    {
      seq: 4,
      t: ttc + 500,
      type: "save_success",
      flow: "save",
      destination: "save",
    },
  ]);
}

describe("longitudinal stability analysis", () => {
  it("scores multi-generation improvement deterministically", () => {
    const store = memoryStore();
    const summary = runAdaptationExperiments(store);
    const adaptation = listAdaptations(store).find(
      (a) => a.adaptationId === summary.results[0]!.adaptationId,
    )!;
    expect(adaptation.rolloutState).toBe("candidate");

    const gen2 = buildEvidenceFromSessions([calmSession("s-long-2", 380)], {
      tag: "long_g2",
    });
    const gen3 = buildEvidenceFromSessions([calmSession("s-long-3", 360)], {
      tag: "long_g3",
    });
    persistEvidenceSnapshot(store, gen2);
    persistEvidenceSnapshot(store, gen3);

    appendLongitudinalObservation(store, adaptation, gen2);
    const { report } = appendLongitudinalObservation(store, adaptation, gen3);

    expect(report.evidenceTimeline.length).toBeGreaterThanOrEqual(
      LONGITUDINAL_MIN_OBSERVATIONS,
    );
    expect(report.sampleCount).toBeGreaterThanOrEqual(2);
    expect(report.stabilityScore).toBeGreaterThanOrEqual(
      LONGITUDINAL_STABILITY_THRESHOLD,
    );
    expect(report.regressionEvents).toEqual([]);
    expect(report.rolloutDisposition).toBe("rollout_candidate");
    expect(report.confidenceEvolution.length).toBe(report.evidenceTimeline.length);

    const snaps = listEvidenceSnapshots(store);
    const again = analyzeAdaptationStability(
      adaptation,
      report.evidenceTimeline.map(
        (id) => snaps.find((e) => e.evidenceId === id)!,
      ),
    );
    expect(again.stabilityScore).toBe(report.stabilityScore);
    expect(again.rolloutDisposition).toBe(report.rolloutDisposition);

    clearExperimentStore(store);
  });

  it("holds when evidence minimum is unmet", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    const adaptation = listAdaptations(store)[0]!;
    const { report } = runLongitudinalValidation(store, adaptation);
    // Experiments seed baseline + after only (2 observations).
    expect(report.evidenceTimeline.length).toBeLessThan(
      LONGITUDINAL_MIN_OBSERVATIONS,
    );
    expect(report.rolloutDisposition).toBe("hold");
    expect(isRolloutReady(adaptation, report)).toBe(false);
    clearExperimentStore(store);
  });
});

describe("rollout readiness", () => {
  it("promotes to rollout_candidate only when longitudinal gates pass", () => {
    const store = memoryStore();
    const summary = runAdaptationExperiments(store);
    const adaptationId = summary.results[0]!.adaptationId;
    let adaptation = listAdaptations(store).find(
      (a) => a.adaptationId === adaptationId,
    )!;

    const blocked = promoteToRolloutCandidate(store, adaptationId);
    expect(blocked.ok).toBe(false);
    if (!blocked.ok) {
      expect(blocked.error).toBe("evidence_minimum_unmet");
    }

    const gen2 = buildEvidenceFromSessions([calmSession("s-ready-2")], {
      tag: "ready_g2",
    });
    const gen3 = buildEvidenceFromSessions([calmSession("s-ready-3")], {
      tag: "ready_g3",
    });
    persistEvidenceSnapshot(store, gen2);
    persistEvidenceSnapshot(store, gen3);
    appendLongitudinalObservation(store, adaptation, gen2);
    appendLongitudinalObservation(store, adaptation, gen3);

    const promoted = promoteToRolloutCandidate(store, adaptationId);
    expect(promoted.ok).toBe(true);
    if (promoted.ok) {
      expect(promoted.adaptation.rolloutState).toBe("rollout_candidate");
      expect(promoted.report.rolloutDisposition).toBe("rollout_candidate");
    }

    adaptation = listAdaptations(store).find(
      (a) => a.adaptationId === adaptationId,
    )!;
    expect(adaptation.rolloutState).toBe("rollout_candidate");
    expect(listRolloutCandidates(store).map((a) => a.adaptationId)).toContain(
      adaptationId,
    );

    // Activation remains manual.
    expect(
      listAdaptations(store).every((a) => a.rolloutState !== "active"),
    ).toBe(true);
    const activated = activateAdaptation(store, adaptationId);
    expect(activated.ok).toBe(true);
    if (activated.ok) {
      expect(activated.adaptation.rolloutState).toBe("active");
    }

    clearExperimentStore(store);
  });

  it("rejects promotion when regressions are present", () => {
    const store = memoryStore();
    const summary = runAdaptationExperiments(store);
    const adaptation = listAdaptations(store).find(
      (a) => a.adaptationId === summary.results[0]!.adaptationId,
    )!;

    const thrash = session("s-regressed", [
      { seq: 0, t: 0, type: "session_start", destination: "home" },
      {
        seq: 1,
        t: 12000,
        type: "first_meaningful_interaction",
        destination: "home",
        modality: "pointer",
      },
      nav(2, 13000, "home", "save"),
      {
        seq: 3,
        t: 13100,
        type: "flow_start",
        flow: "save",
        destination: "save",
      },
      nav(4, 15000, "save", "resume"),
      {
        seq: 5,
        t: 15050,
        type: "flow_abandon",
        flow: "save",
        from: "save",
        destination: "resume",
      },
      nav(6, 16000, "resume", "save"),
      nav(7, 17000, "save", "home"),
      nav(8, 18000, "home", "save"),
    ]);
    const gen2 = buildEvidenceFromSessions([calmSession("s-ok-2")], {
      tag: "reg_g2",
    });
    const bad = buildEvidenceFromSessions([thrash], { tag: "reg_bad" });
    persistEvidenceSnapshot(store, gen2);
    persistEvidenceSnapshot(store, bad);
    appendLongitudinalObservation(store, adaptation, gen2);
    const { report } = appendLongitudinalObservation(store, adaptation, bad);

    expect(report.regressionEvents.length).toBeGreaterThan(0);
    expect(report.rolloutDisposition).toBe("reject");
    const promoted = promoteToRolloutCandidate(store, adaptation.adaptationId);
    expect(promoted.ok).toBe(false);
    if (!promoted.ok) {
      expect(["unresolved_regressions", "stability_threshold_unmet"]).toContain(
        promoted.error,
      );
    }
    clearExperimentStore(store);
  });
});

describe("governance + authority", () => {
  it("extends authority chain through longitudinal doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "49_Adaptation_Experiments.md" &&
          b === "50_Longitudinal_Adaptation_Validation.md",
      ),
    ).toBe(true);
  });

  it("records longitudinal indexes without duplicating metrics", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    const adaptation = listAdaptations(store)[0]!;
    const gen2 = buildEvidenceFromSessions([calmSession("s-idx-2")], {
      tag: "idx_g2",
    });
    persistEvidenceSnapshot(store, gen2);
    appendLongitudinalObservation(store, adaptation, gen2);
    const record = getLongitudinalRecord(store, adaptation.adaptationId)!;
    const report = getStabilityReport(store, adaptation.adaptationId)!;
    expect(record.baselineEvidenceId).toBeTruthy();
    expect(record.latestEvidenceId).toBeTruthy();
    expect(record.observationCount).toBe(report.evidenceTimeline.length);
    const raw = store.getItem(ADAPTATION_STORAGE_KEY)!;
    expect(raw).toContain("longitudinalRecords");
    expect(raw).toContain("stabilityReports");
    // Longitudinal records store ids + scores only (no nested metrics object).
    expect(JSON.parse(raw).longitudinalRecords[0]).not.toHaveProperty("metrics");
    clearExperimentStore(store);
  });
});

describe("privacy audit (longitudinal)", () => {
  it("adaptation storage omits forbidden content keys", () => {
    const store = memoryStore();
    runAdaptationExperiments(store);
    const adaptation = listAdaptations(store)[0]!;
    const gen2 = buildEvidenceFromSessions([calmSession("s-priv-2")], {
      tag: "priv_g2",
    });
    persistEvidenceSnapshot(store, gen2);
    appendLongitudinalObservation(store, adaptation, gen2);
    const raw = store.getItem(ADAPTATION_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
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

  it("overlay lazy-loads longitudinal module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/longitudinalAdaptation")');
    expect(dash).toContain("longitudinal-adaptation");
  });

  it("architecture doc records thresholds and lifecycle", () => {
    const doc = readFileSync(
      path.join(root, "architecture/50_Longitudinal_Adaptation_Validation.md"),
      "utf8",
    );
    expect(doc).toContain("stabilityScore");
    expect(doc).toContain("rollout_candidate");
    expect(doc).toContain("LONGITUDINAL_STABILITY_THRESHOLD");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
