/**
 * Sprint 53 — Evidence-driven improvement engine.
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
  type ExperienceEvidence,
} from "../app/src/dev/experienceEvidence";
import {
  detectOpportunities,
  evolveBaselines,
  evolutionVerdictFor,
  opportunitiesHaveReplayLinkage,
} from "../app/src/dev/experienceImprovement";
import {
  initExperienceInstrumentation,
  disposeExperienceInstrumentation,
  clearInstrumentation,
  listStoredSessions,
  replayStoredSession,
  trackNavigate,
  trackSaveSuccess,
} from "../app/src/dev/experienceInstrumentation";
import { memoryStore } from "../app/src/dev/experienceStore";

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

const calm = session("s-calm", [
  { seq: 0, t: 0, type: "session_start", destination: "home" },
  {
    seq: 1,
    t: 120,
    type: "first_meaningful_interaction",
    destination: "home",
    modality: "pointer",
  },
  nav(2, 200, "home", "save"),
  { seq: 3, t: 250, type: "flow_start", flow: "save", destination: "save" },
  { seq: 4, t: 400, type: "save_success", flow: "save", destination: "save" },
]);

const thrash = session("s-thrash", [
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
  {
    seq: 8,
    t: 16100,
    type: "repeated_action",
    destination: "resume",
    targetKind: "navigate",
  },
]);

describe("opportunity detection", () => {
  it("is deterministic for identical evidence", () => {
    const evidence = buildEvidenceFromSessions([thrash], { tag: "t1" });
    const a = detectOpportunities([evidence]);
    const b = detectOpportunities([evidence]);
    expect(a).toEqual(b);
    expect(a.length).toBeGreaterThan(0);
    expect(a.every((o) => o.opportunityId.startsWith("opp-"))).toBe(true);
  });

  it("emits metric-bound opportunities without natural language", () => {
    const evidence = buildEvidenceFromSessions([thrash], { tag: "t2" });
    const opps = detectOpportunities([evidence]);
    const metrics = new Set(opps.map((o) => o.metric));
    expect(metrics.has("abandonedSave") || metrics.has("meanFrictionScore")).toBe(
      true,
    );
    for (const opp of opps) {
      expect(opp.workflow).toMatch(
        /^(save|continue|navigation|confidence|recovery|replay|friction)$/,
      );
      expect(["low", "medium", "high"]).toContain(opp.severity);
      expect(opp.confidence).toBeGreaterThanOrEqual(0);
      expect(opp.confidence).toBeLessThanOrEqual(1);
      expect(opp.reproducibilityScore).toBeGreaterThanOrEqual(0);
      expect(opp.reproducibilityScore).toBeLessThanOrEqual(1);
      expect(opp.supportingEvidenceIds).toContain(evidence.evidenceId);
      const keys = Object.keys(opp);
      for (const forbidden of FORBIDDEN_EVENT_KEYS) {
        expect(keys).not.toContain(forbidden);
      }
    }
  });

  it("does not flag calm evidence for abandon / loop thresholds", () => {
    const evidence = buildEvidenceFromSessions([calm], { tag: "calm" });
    const opps = detectOpportunities([evidence]);
    expect(opps.some((o) => o.metric === "abandonedSave")).toBe(false);
    expect(opps.some((o) => o.metric === "navigationLoopCount")).toBe(false);
  });

  it("skips opportunities when no replay session ids exist", () => {
    const emptySessions: ExperienceEvidence = {
      ...buildEvidenceFromSessions([calm], { tag: "x" }),
      sourceSessionIds: [],
      metrics: {
        ...buildEvidenceFromSessions([calm], { tag: "x" }).metrics,
        abandonedSave: 3,
        abandonedFlowTotal: 3,
      },
    };
    expect(detectOpportunities([emptySessions])).toEqual([]);
  });
});

describe("longitudinal baseline evolution", () => {
  it("classifies improving / stable / degrading with significance floors", () => {
    expect(evolutionVerdictFor(10, 10.1, "lower_better", 1)).toBe("stable");
    expect(evolutionVerdictFor(10, 5, "lower_better", 1)).toBe("improving");
    expect(evolutionVerdictFor(10, 20, "lower_better", 1)).toBe("degrading");
    expect(evolutionVerdictFor(0.2, 0.5, "higher_better", 0.05)).toBe(
      "improving",
    );
  });

  it("detects meaningful improvement from thrash → calm timeline", () => {
    const worse = buildEvidenceFromSessions([thrash], { tag: "w" });
    const better = buildEvidenceFromSessions([calm], { tag: "b" });
    const evolution = evolveBaselines([worse, better]);
    expect(evolution.snapshotCount).toBe(2);
    expect(evolution).toEqual(evolveBaselines([worse, better]));
    expect(evolution.summary.improving).toBeGreaterThan(0);
    const friction = evolution.metrics.find((m) => m.key === "meanFrictionScore");
    expect(friction?.verdict).toBe("improving");
    expect(evolution.improvements.some((m) => m.key === "meanFrictionScore")).toBe(
      true,
    );
  });

  it("ignores insignificant variation", () => {
    const a = buildEvidenceFromSessions([calm], { tag: "a" });
    const b: ExperienceEvidence = {
      ...a,
      evidenceId: "evd-near",
      fingerprint: "near0001",
      tag: "near",
      metrics: {
        ...a.metrics,
        // Tiny friction nudge below longitudinal floor (0.03 abs).
        meanFrictionScore: Number((a.metrics.meanFrictionScore + 0.001).toFixed(4)),
      },
    };
    const evolution = evolveBaselines([a, b]);
    const friction = evolution.metrics.find((m) => m.key === "meanFrictionScore");
    expect(friction?.verdict).toBe("stable");
  });
});

describe("replay linkage", () => {
  it("links every opportunity to replayable session ids", () => {
    const evidence = buildEvidenceFromSessions([thrash, calm], { tag: "link" });
    const opps = detectOpportunities([evidence]);
    expect(opportunitiesHaveReplayLinkage(opps)).toBe(true);
    expect(opps.every((o) => o.replaySessionIds.length > 0)).toBe(true);
  });

  it("replays linked sessions from the existing trace store without duplication", () => {
    const store = memoryStore();
    disposeExperienceInstrumentation();
    clearInstrumentation();
    initExperienceInstrumentation({
      store,
      sessionId: "s-live",
      force: true,
    });
    trackNavigate("save", { commandId: "go_save" });
    trackSaveSuccess();
    const live = listStoredSessions();
    expect(live.some((s) => s.sessionId === "s-live")).toBe(true);

    const evidence = buildEvidenceFromSessions(live, { tag: "live" });
    // Force a threshold cross while preserving session ids.
    const forced: ExperienceEvidence = {
      ...evidence,
      metrics: {
        ...evidence.metrics,
        abandonedSave: 2,
        abandonedFlowTotal: 2,
        meanFrictionScore: 0.5,
      },
    };
    const opps = detectOpportunities([forced]);
    expect(opps.length).toBeGreaterThan(0);
    for (const opp of opps) {
      for (const sessionId of opp.replaySessionIds) {
        const result = replayStoredSession(sessionId);
        expect(result).not.toBeNull();
        expect(result?.sessionId).toBe(sessionId);
      }
    }
    disposeExperienceInstrumentation();
    clearInstrumentation();
  });
});

describe("privacy audit (improvement engine)", () => {
  it("serialized opportunities omit forbidden content keys", () => {
    const evidence = buildEvidenceFromSessions([thrash], { tag: "priv" });
    const raw = JSON.stringify(detectOpportunities([evidence]));
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind|recommend/i);
  });

  it("dev modules still have no network telemetry", () => {
    const dir = path.join(root, "app/src/dev");
    const files = readdirSync(dir).filter((f) => /\.(ts|tsx)$/.test(f));
    for (const file of files) {
      const src = readFileSync(path.join(dir, file), "utf8");
      expect(src).not.toMatch(/\bfetch\s*\(/);
      expect(src).not.toMatch(/sendBeacon/);
      expect(src).not.toMatch(/XMLHttpRequest/);
      expect(src).not.toMatch(/WebSocket/);
    }
  });

  it("overlay lazy-loads the improvement engine", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("./experienceImprovement")');
    const main = readFileSync(path.join(root, "app/src/main.tsx"), "utf8");
    expect(main).toContain("import.meta.env.DEV");
    expect(main).not.toMatch(/from\s+["'].*experienceImprovement["']/);
  });

  it("architecture doc records opportunity schema and privacy", () => {
    const doc = readFileSync(
      path.join(root, "architecture/44_Experience_Improvement_Model.md"),
      "utf8",
    );
    expect(doc).toContain("ExperienceOpportunity");
    expect(doc).toContain("LONGITUDINAL_SIGNIFICANCE");
    expect(doc).toContain("replaySessionIds");
    expect(doc).toMatch(/Evidence only|No design guidance/i);
  });
});
