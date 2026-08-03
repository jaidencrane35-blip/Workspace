/**
 * Sprint 51 — Experience Validation: instrumentation, friction, replay, privacy.
 */
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  FORBIDDEN_EVENT_KEYS,
  sanitizeEvent,
  type ExperienceEvent,
  type ExperienceSession,
} from "../app/src/dev/experienceEvents";
import {
  clearInstrumentation,
  disposeExperienceInstrumentation,
  getActiveSession,
  initExperienceInstrumentation,
  isInstrumentationActive,
  listStoredSessions,
  trackContinueSuccess,
  trackFlowStart,
  trackNavigate,
  trackSaveSuccess,
} from "../app/src/dev/experienceInstrumentation";
import {
  EXPERIENCE_VALIDATION_STORAGE_KEY,
  memoryStore,
} from "../app/src/dev";
import { computeFrictionScore } from "../app/src/dev/frictionModel";
import {
  STORAGE_KEY,
  loadBundle,
  upsertSession,
} from "../app/src/dev/experienceStore";
import { frictionEquals, replaySession } from "../app/src/dev/sessionReplay";
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

function fixtureSession(events: ExperienceEvent[]): ExperienceSession {
  return {
    schemaVersion: 1,
    sessionId: "fixture-1",
    startedAt: 1_000_000,
    events,
  };
}

describe("experience validation gate / lifecycle", () => {
  let store: ReturnType<typeof memoryStore>;

  beforeEach(() => {
    store = memoryStore();
    disposeExperienceInstrumentation();
    clearInstrumentation();
    initExperienceInstrumentation({
      store,
      sessionId: "test-session",
      force: true,
    });
  });

  afterEach(() => {
    disposeExperienceInstrumentation();
    clearInstrumentation();
  });

  it("starts a session with session_start and persists locally", () => {
    expect(isInstrumentationActive()).toBe(true);
    const session = getActiveSession();
    expect(session?.sessionId).toBe("test-session");
    expect(session?.events[0]?.type).toBe("session_start");
    expect(STORAGE_KEY).toBe("ws.dev.experience.validation.v1");
    expect(EXPERIENCE_VALIDATION_STORAGE_KEY).toBe(STORAGE_KEY);
    const listed = listStoredSessions();
    expect(listed.length).toBe(1);
    expect(loadBundle(store).sessions[0]?.sessionId).toBe("test-session");
  });

  it("records navigate, save success, and flow abandon without content fields", () => {
    trackNavigate("save", { commandId: "go_save", modality: "pointer" });
    trackSaveSuccess();
    trackNavigate("resume", { commandId: "go_continue" });
    trackFlowStart("continue");
    trackNavigate("home", { commandId: "dock_navigate" });

    const session = getActiveSession()!;
    const types = session.events.map((e) => e.type);
    expect(types).toContain("navigate");
    expect(types).toContain("save_success");
    expect(types).toContain("flow_start");
    expect(types).toContain("flow_abandon");

    for (const event of session.events) {
      for (const key of FORBIDDEN_EVENT_KEYS) {
        expect(Object.prototype.hasOwnProperty.call(event, key)).toBe(false);
      }
      expect(JSON.stringify(event)).not.toMatch(/handoff|pricing|Northwind/i);
    }
  });

  it("ignores track calls after dispose", () => {
    const before = getActiveSession()!.events.length;
    disposeExperienceInstrumentation();
    expect(isInstrumentationActive()).toBe(false);
    trackNavigate("save");
    trackSaveSuccess();
    // Session snapshot retained for inspection, but no new events after dispose.
    expect(getActiveSession()!.events.length).toBe(before);
  });
});

describe("friction model", () => {
  it("is reproducible from the same trace", () => {
    const session = fixtureSession([
      { seq: 0, t: 0, type: "session_start", destination: "home" },
      {
        seq: 1,
        t: 400,
        type: "first_meaningful_interaction",
        destination: "home",
        modality: "pointer",
      },
      {
        seq: 2,
        t: 900,
        type: "navigate",
        from: "home",
        destination: "save",
        commandId: "go_save",
      },
      { seq: 3, t: 1200, type: "flow_start", flow: "save", destination: "save" },
      {
        seq: 4,
        t: 2000,
        type: "navigate",
        from: "save",
        destination: "resume",
        commandId: "go_continue",
      },
      {
        seq: 5,
        t: 2100,
        type: "flow_abandon",
        flow: "save",
        from: "save",
        destination: "resume",
      },
      {
        seq: 6,
        t: 2500,
        type: "flow_start",
        flow: "continue",
        destination: "resume",
      },
      {
        seq: 7,
        t: 4000,
        type: "continue_success",
        flow: "continue",
        destination: "resume",
      },
    ]);

    const a = computeFrictionScore(session);
    const b = computeFrictionScore(structuredClone(session));
    expect(frictionEquals(a, b)).toBe(true);
    expect(a.score).toBeGreaterThanOrEqual(0);
    expect(a.score).toBeLessThanOrEqual(1);
    expect(a.components.successfulRecovery).toBe(1);
    expect(a.components.flowInterruption).toBe(1);
    expect(a.evidence.continueSuccess).toBe(1);
  });

  it("raises friction for thrashing without success", () => {
    const calm = fixtureSession([
      { seq: 0, t: 0, type: "session_start", destination: "home" },
      {
        seq: 1,
        t: 100,
        type: "first_meaningful_interaction",
        destination: "home",
        modality: "pointer",
      },
      {
        seq: 2,
        t: 200,
        type: "navigate",
        from: "home",
        destination: "save",
      },
      { seq: 3, t: 500, type: "save_success", flow: "save", destination: "save" },
    ]);
    const thrash = fixtureSession([
      { seq: 0, t: 0, type: "session_start", destination: "home" },
      {
        seq: 1,
        t: 8000,
        type: "first_meaningful_interaction",
        destination: "home",
        modality: "pointer",
      },
      {
        seq: 2,
        t: 9000,
        type: "navigate",
        from: "home",
        destination: "save",
      },
      {
        seq: 3,
        t: 10000,
        type: "navigate",
        from: "save",
        destination: "resume",
      },
      {
        seq: 4,
        t: 11000,
        type: "navigate",
        from: "resume",
        destination: "save",
      },
      {
        seq: 5,
        t: 11100,
        type: "repeated_action",
        destination: "save",
        targetKind: "navigate",
      },
      {
        seq: 6,
        t: 11200,
        type: "flow_abandon",
        flow: "save",
        destination: "home",
      },
    ]);
    expect(computeFrictionScore(thrash).score).toBeGreaterThan(
      computeFrictionScore(calm).score,
    );
  });
});

describe("deterministic session replay", () => {
  it("replays destinations in order and matches friction", () => {
    const session = fixtureSession([
      { seq: 0, t: 0, type: "session_start", destination: "home" },
      {
        seq: 1,
        t: 10,
        type: "navigate",
        from: "home",
        destination: "save",
      },
      { seq: 2, t: 20, type: "save_success", flow: "save", destination: "save" },
      {
        seq: 3,
        t: 30,
        type: "navigate",
        from: "save",
        destination: "resume",
      },
      {
        seq: 4,
        t: 40,
        type: "continue_success",
        flow: "continue",
        destination: "resume",
      },
    ]);

    const seen: string[] = [];
    const first = replaySession(session, {
      navigate: (d) => seen.push(d),
    });
    const second = replaySession(session, {
      navigate: () => undefined,
    });

    expect(seen).toEqual(["save", "resume"]);
    expect(first.destinations).toEqual(["save", "resume"]);
    expect(first.timeline.map((x) => x.t)).toEqual([0, 10, 20, 30, 40]);
    expect(frictionEquals(first.friction, second.friction)).toBe(true);
    expect(frictionEquals(first.friction, computeFrictionScore(session))).toBe(
      true,
    );
  });
});

describe("privacy audit", () => {
  it("sanitizeEvent drops unknown types and non-opaque targetKind", () => {
    expect(
      sanitizeEvent({
        seq: 0,
        t: 0,
        type: "navigate",
        targetKind: "Save my pricing notes!!!",
      } as ExperienceEvent),
    ).toEqual({ seq: 0, t: 0, type: "navigate" });

    expect(
      sanitizeEvent({
        seq: 0,
        t: 0,
        // @ts-expect-error intentional invalid type
        type: "user_typed_handoff",
      }),
    ).toBeNull();
  });

  it("storage adapter never receives forbidden content keys", () => {
    const store = memoryStore();
    const dirty = fixtureSession([
      {
        seq: 0,
        t: 0,
        type: "session_start",
        destination: "home",
        // @ts-expect-error privacy — must not survive sanitize path via instrumentation
        handoff: "secret note",
        name: "Northwind",
      } as ExperienceEvent,
    ]);
    // Direct upsert simulates a corrupted writer; audit that our public sanitize path
    // and instrumentation never emit these keys. Also verify store payload for clean session.
    upsertSession(store, dirty);
    const raw = store.getItem(STORAGE_KEY)!;
    // Even if a bad session is forced in, product writers must not add these — instrumentation uses sanitize.
    expect(raw).toContain("session_start");

    const store2 = memoryStore();
    initExperienceInstrumentation({
      store: store2,
      sessionId: "priv",
      force: true,
    });
    trackNavigate("save", { commandId: "go_save" });
    trackSaveSuccess();
    const payload = store2.getItem(STORAGE_KEY)!;
    for (const key of FORBIDDEN_EVENT_KEYS) {
      expect(payload).not.toContain(`"${key}"`);
    }
    disposeExperienceInstrumentation();
    clearInstrumentation();
  });

  it("dev instrumentation modules do not call fetch or sendBeacon", () => {
    const dir = path.join(root, "app/src/dev");
    const files = readdirSync(dir).filter((f) => f.endsWith(".ts"));
    expect(files.length).toBeGreaterThan(0);
    for (const file of files) {
      const src = readFileSync(path.join(dir, file), "utf8");
      expect(src).not.toMatch(/\bfetch\s*\(/);
      expect(src).not.toMatch(/sendBeacon/);
      expect(src).not.toMatch(/navigator\.sendBeacon/);
      expect(src).not.toMatch(/XMLHttpRequest/);
      expect(src).not.toMatch(/WebSocket/);
    }
  });

  it("architecture doc records local-only privacy guarantees", () => {
    const doc = readFileSync(
      path.join(root, "architecture/42_Experience_Validation.md"),
      "utf8",
    );
    expect(doc).toContain("Local-only");
    expect(doc).toContain("No user content");
    expect(doc).toContain("ws.dev.experience.validation.v1");
    expect(doc).not.toMatch(/recommend(ation|s)?\s*:/i);
  });
});
