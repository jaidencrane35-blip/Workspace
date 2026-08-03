/**
 * Sprint 52 — Experience Evidence pipeline: aggregation, comparison, replay, privacy.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import type { ExperienceEvent, ExperienceSession } from "../app/src/dev/experienceEvents";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import {
  EVIDENCE_STORAGE_KEY,
  buildEvidenceFromSessions,
  clearEvidenceStore,
  compareEvidence,
  getEvidenceBaseline,
  listEvidenceSnapshots,
  persistEvidenceSnapshot,
  setEvidenceBaseline,
  verdictForMetric,
} from "../app/src/dev/experienceEvidence";
import { memoryStore } from "../app/src/dev/experienceStore";
import { replaySession } from "../app/src/dev/sessionReplay";
import { analyzeTraces } from "../app/src/dev/traceAnalysis";

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
  return { seq, t, type: "navigate", from, destination, commandId: "dock_navigate" };
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

describe("trace analysis aggregation", () => {
  it("is deterministic for the same sessions regardless of input order", () => {
    const a = analyzeTraces([calm, thrash]);
    const b = analyzeTraces([thrash, calm]);
    expect(a).toEqual(b);
    expect(a.sessionCount).toBe(2);
    expect(a.sessionIds).toEqual(["s-calm", "s-thrash"]);
    expect(a.medianTimeToConfidenceMs).toBeGreaterThan(0);
    expect(a.frictionScores.length).toBe(2);
    expect(a.abandonedFlows.total).toBeGreaterThan(0);
    expect(a.replay.sessionsReplayed).toBe(2);
    expect(a.replay.divergenceRate).toBe(0);
  });

  it("detects navigation loops and hesitation hotspots without content", () => {
    const looped = session("s-loop", [
      { seq: 0, t: 0, type: "session_start", destination: "home" },
      {
        seq: 1,
        t: 100,
        type: "first_meaningful_interaction",
        destination: "home",
        modality: "pointer",
      },
      nav(2, 200, "home", "save"),
      nav(3, 1500, "save", "resume"),
      nav(4, 1600, "resume", "save"),
    ]);
    const agg = analyzeTraces([looped]);
    expect(agg.navigationLoops.some((l) => l.pattern === "save→resume→save")).toBe(
      true,
    );
    expect(agg.hesitationHotspots.length).toBeGreaterThan(0);
    const blob = JSON.stringify(agg);
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(blob).not.toContain(`"${key}"`);
    }
  });
});

describe("ExperienceEvidence model", () => {
  it("stores derived metrics only and persists separately from traces", () => {
    const store = memoryStore();
    const evidence = buildEvidenceFromSessions([calm, thrash], {
      tag: "baseline_a",
    });
    expect(evidence.schemaVersion).toBe(1);
    expect(evidence.sourceSessionIds).toEqual(["s-calm", "s-thrash"]);
    expect(evidence.metrics.sessionCount).toBe(2);
    expect("events" in evidence).toBe(false);
    persistEvidenceSnapshot(store, evidence);
    expect(store.getItem(EVIDENCE_STORAGE_KEY)).toBeTruthy();
    expect(store.getItem("ws.dev.experience.validation.v1")).toBeNull();
    const listed = listEvidenceSnapshots(store);
    expect(listed).toHaveLength(1);
    expect(listed[0]?.fingerprint).toBe(evidence.fingerprint);
  });

  it("rebuilds identical evidence for the same sessions", () => {
    const a = buildEvidenceFromSessions([calm, thrash], { tag: "x" });
    const b = buildEvidenceFromSessions([thrash, calm], { tag: "x" });
    expect(a.fingerprint).toBe(b.fingerprint);
    expect(a.metrics).toEqual(b.metrics);
  });
});

describe("baseline comparison / regression", () => {
  it("classifies improved / unchanged / regressed deterministically", () => {
    expect(verdictForMetric(10, 10, "lower_better", 0)).toBe("unchanged");
    expect(verdictForMetric(10, 8, "lower_better", 0)).toBe("improved");
    expect(verdictForMetric(10, 12, "lower_better", 0)).toBe("regressed");
    expect(verdictForMetric(0.2, 0.5, "higher_better", 0.0001)).toBe("improved");
    expect(verdictForMetric(0.5, 0.2, "higher_better", 0.0001)).toBe("regressed");
  });

  it("compares two evidence snapshots with stable summary counts", () => {
    const baseline = buildEvidenceFromSessions([thrash], { tag: "base" });
    const candidate = buildEvidenceFromSessions([calm], { tag: "cand" });
    const first = compareEvidence(baseline, candidate);
    const second = compareEvidence(baseline, candidate);
    expect(first).toEqual(second);
    expect(
      first.summary.improved +
        first.summary.unchanged +
        first.summary.regressed,
    ).toBe(first.metrics.length);
    expect(first.summary.improved).toBeGreaterThan(0);
    // Calm should reduce mean friction vs thrash.
    const friction = first.metrics.find((m) => m.key === "meanFrictionScore");
    expect(friction?.verdict).toBe("improved");
  });

  it("tracks baseline selection in the evidence store", () => {
    const store = memoryStore();
    const evidence = buildEvidenceFromSessions([calm], { tag: "b1" });
    persistEvidenceSnapshot(store, evidence);
    setEvidenceBaseline(store, evidence.evidenceId);
    expect(getEvidenceBaseline(store)?.evidenceId).toBe(evidence.evidenceId);
    clearEvidenceStore(store);
    expect(listEvidenceSnapshots(store)).toHaveLength(0);
  });
});

describe("replay compatibility", () => {
  it("replay remains compatible with evidence aggregation divergence check", () => {
    const agg = analyzeTraces([calm]);
    expect(agg.replay.divergent).toBe(0);
    const replay = replaySession(calm);
    expect(replay.destinations).toEqual(["save"]);
    const evidence = buildEvidenceFromSessions([calm]);
    expect(evidence.metrics.replayDivergenceRate).toBe(0);
    expect(evidence.metrics.replayCount).toBe(1);
  });
});

describe("privacy audit (evidence pipeline)", () => {
  it("evidence JSON never contains forbidden content keys", () => {
    const store = memoryStore();
    const evidence = buildEvidenceFromSessions([calm, thrash], {
      tag: "priv",
    });
    persistEvidenceSnapshot(store, evidence);
    const raw = store.getItem(EVIDENCE_STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${key}"`);
    }
    expect(raw).not.toMatch(/handoff|pricing|Northwind/i);
  });

  it("dev modules do not introduce network telemetry", () => {
    const dir = path.join(root, "app/src/dev");
    const files = readdirSync(dir).filter((f) => /\.(ts|tsx)$/.test(f));
    expect(files.length).toBeGreaterThan(0);
    for (const file of files) {
      const src = readFileSync(path.join(dir, file), "utf8");
      expect(src).not.toMatch(/\bfetch\s*\(/);
      expect(src).not.toMatch(/sendBeacon/);
      expect(src).not.toMatch(/XMLHttpRequest/);
      expect(src).not.toMatch(/WebSocket/);
    }
  });

  it("dashboard mounts only behind DEV dynamic import in main.tsx", () => {
    const main = readFileSync(path.join(root, "app/src/main.tsx"), "utf8");
    expect(main).toContain("import.meta.env.DEV");
    expect(main).toContain('import("./dev/mountExperienceEvidence")');
    expect(main).not.toMatch(
      /from\s+["'].*ExperienceEvidenceDashboard["']/,
    );
  });

  it("architecture doc records evidence-only schema and privacy", () => {
    const doc = readFileSync(
      path.join(root, "architecture/43_Experience_Evidence_Model.md"),
      "utf8",
    );
    expect(doc).toContain("ExperienceEvidence");
    expect(doc).toContain("ws.dev.experience.evidence.v1");
    expect(doc).toContain("Local-only");
    expect(doc).toMatch(/No recommendations|Evidence only/i);
  });
});
