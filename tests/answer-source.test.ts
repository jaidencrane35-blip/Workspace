/**
 * P23.S3 — Answer Source Ladder: local time and date.
 *
 * Proves that a comprehended informational request is answered by a trusted
 * local source instead of being handed to an external one, that the answer
 * comes from the runtime's authoritative time-zone database rather than a
 * hard-coded offset, and that nothing about answering reaches the desktop.
 */
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { beforeEach, describe, expect, it, vi } from "vitest";

// Stands in for the single IPC entry. `@tauri-apps/api` resolves outside this
// workspace package and cannot be replaced, so mocking it would silently
// no-op and every "no IPC" assertion below would be vacuous.
const { invoke } = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("../app/src/lib/ipc", () => ({
  invokeIpc: invoke,
  IpcCommandError: class extends Error {},
}));

import { comprehend } from "../app/src/lib/goalContract";
import {
  resolveIntent,
  resolveIntentWithGoal,
  type IntentAction,
} from "../app/src/lib/intentBridge";
import {
  ANSWER_RUNGS,
  answerForGoal,
  resolveAnswer,
} from "../app/src/lib/answerSource";
import {
  dateIn,
  resolveRegionZone,
  temporalAnswerSource,
  timeIn,
} from "../app/src/lib/temporalAnswerSource";
import { classifyIntelligenceKind } from "../app/src/lib/intelligenceRouting";
import { handleOperatorUtterance } from "../app/src/lib/operator/intelligence";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const SPEAKING = new Set([
  "unknown",
  "capabilityExplain",
  "browserExplain",
  "voiceExplain",
  "voiceStatus",
]);

function isEffect(action: IntentAction): boolean {
  return !SPEAKING.has(action.kind);
}

/** The clock may tick between the answer and the assertion. */
function plausibleTimes(zone: string): string[] {
  const now = Date.now();
  return [-60_000, 0, 60_000].map((delta) => timeIn(zone, new Date(now + delta)));
}

function expectLocalTimeAnswer(utterance: string, zone: string, place: RegExp) {
  resetWorkspaceContext();
  invoke.mockClear();
  const { goal, action } = resolveIntentWithGoal(utterance);

  expect(goal.outcome).toBe("KNOW");
  expect(goal.domain).toBe("time");
  expect(action.kind).toBe("unknown");
  expect(isEffect(action)).toBe(false);
  expect(action.reply).toMatch(place);
  expect(action.reply.toLowerCase()).not.toMatch(/chatgpt|http|browser/);
  expect(plausibleTimes(zone).some((t) => action.reply.includes(t))).toBe(true);
  expect(invoke).not.toHaveBeenCalled();
  return action;
}

describe("P23.S3 local time and date answers", () => {
  beforeEach(() => {
    resetWorkspaceContext();
    invoke.mockClear();
  });

  it("1. answers the time in Queensland locally", () => {
    expectLocalTimeAnswer(
      "What time is it in Queensland?",
      "Australia/Brisbane",
      /Queensland/,
    );
  });

  it("2. answers the current time in Brisbane from the same source", () => {
    expectLocalTimeAnswer(
      "What's the current time in Brisbane?",
      "Australia/Brisbane",
      /Brisbane/,
    );
  });

  it("3. answers QLD from the same source", () => {
    expectLocalTimeAnswer(
      "What is the time right now in QLD?",
      "Australia/Brisbane",
      /Queensland/,
    );
  });

  it("4. answers the date in Queensland from the same source", () => {
    const { goal, action } = resolveIntentWithGoal(
      "What date is it in Queensland?",
    );
    expect(goal.requestedResult).toBe("current date");
    expect(goal.domain).toBe("time");
    expect(action.kind).toBe("unknown");
    expect(action.reply).toContain(dateIn("Australia/Brisbane"));
    expect(action.reply).toMatch(/Queensland/);
    expect(invoke).not.toHaveBeenCalled();
  });

  it("5. resolves Western Australia to Australia/Perth", () => {
    expect(resolveRegionZone("western australia")).toEqual({
      kind: "zone",
      zone: "Australia/Perth",
      label: "Western Australia",
    });
    expectLocalTimeAnswer(
      "What time is it in Western Australia?",
      "Australia/Perth",
      /Western Australia/,
    );
  });

  it("6. resolves bare WA only when the conversation is already Australian", () => {
    // Alone it is genuinely ambiguous and must not be guessed.
    expect(resolveRegionZone("wa").kind).toBe("ambiguous");

    // After an Australian region, continuity makes it unambiguous.
    resolveIntentWithGoal("What time is it in Queensland?");
    const resolved = resolveRegionZone("wa");
    expect(resolved).toEqual({
      kind: "zone",
      zone: "Australia/Perth",
      label: "Western Australia",
    });
  });

  it("7. keeps existing time-zone aliases working", () => {
    for (const [phrase, zone] of [
      ["perth", "Australia/Perth"],
      ["sydney", "Australia/Sydney"],
      ["adelaide", "Australia/Adelaide"],
      ["tokyo", "Asia/Tokyo"],
      ["london", "Europe/London"],
      ["utc", "UTC"],
    ] as const) {
      const resolved = resolveRegionZone(phrase);
      expect(resolved.kind).toBe("zone");
      expect(resolved.kind === "zone" && resolved.zone).toBe(zone);
    }
    // An IANA identifier the Owner typed directly is honoured as given.
    expect(resolveRegionZone("australia/brisbane")).toMatchObject({
      kind: "zone",
      zone: "Australia/Brisbane",
    });
    expect(resolveIntent("what time is it in Perth").reply).toMatch(/Perth/i);
  });

  it("8. takes daylight saving from the authoritative source, not an offset", () => {
    // Brisbane never observes DST; Sydney does. Same instant, both seasons.
    const january = new Date("2026-01-15T00:00:00Z");
    const july = new Date("2026-07-15T00:00:00Z");

    expect(timeIn("Australia/Sydney", january)).not.toBe(
      timeIn("Australia/Brisbane", january),
    );
    expect(timeIn("Australia/Sydney", july)).toBe(
      timeIn("Australia/Brisbane", july),
    );
    expect(timeIn("UTC", january)).toBe("12:00 AM");

    const source = readFileSync(
      path.join(root, "app/src/lib/temporalAnswerSource.ts"),
      "utf8",
    );
    expect(source).toContain("Intl.DateTimeFormat");
    expect(source).not.toMatch(/getTimezoneOffset|UTC\+|60 \* 60 \* 1000/);
  });

  it("9. never invents an answer for an ambiguous place", () => {
    expect(resolveRegionZone("georgia").kind).toBe("ambiguous");
    const { action } = resolveIntentWithGoal("What time is it in Georgia?");
    expect(action.reply.toLowerCase()).not.toMatch(/\bit’s \d/);
    expect(classifyIntelligenceKind("What time is it in WA?")).toBe(
      "CLARIFICATION",
    );
    expect(resolveIntent("What time is it in WA?").reply.toLowerCase()).toMatch(
      /western australia|washington/,
    );
  });

  it("10. leaves unsupported knowledge questions to the existing external path", () => {
    for (const utterance of [
      "What time is it in Ulaanbaatar?",
      "How long from Rockhampton to Gladstone?",
    ]) {
      resetWorkspaceContext();
      const { goal, action } = resolveIntentWithGoal(utterance);
      expect(answerForGoal(goal)).toBeNull();
      if (action.kind === "browserOpen") {
        expect(action.informationHandoff).toBe(true);
      }
    }
  });

  it("18. opens no browser for a local time or date request", () => {
    for (const utterance of [
      "What time is it in Queensland?",
      "What's the date in Queensland?",
      "What time is it in Brisbane?",
      "Can you tell me the time in Queensland?",
    ]) {
      resetWorkspaceContext();
      const action = resolveIntent(utterance);
      expect(action.kind).not.toBe("browserOpen");
      expect(action.kind).not.toBe("appOpen");
      expect(action.reply.toLowerCase()).not.toContain("chatgpt");
    }
  });

  it("reasons from question shape rather than exact strings", () => {
    for (const utterance of [
      "What time is it in Queensland?",
      "What's the current time in Queensland?",
      "Can you tell me the time in Queensland?",
      "Do you know the time in Queensland?",
      "What is the time right now in Queensland?",
    ]) {
      resetWorkspaceContext();
      const { goal, action } = resolveIntentWithGoal(utterance);
      expect(goal.requestedResult).toBe("current time");
      expect(action.reply).toMatch(/Queensland/);
    }
  });
});

describe("P23.S3 Queensland — the exact Owner failure", () => {
  const UTTERANCE = "What is the time in Queensland, Australia?";

  beforeEach(() => {
    resetWorkspaceContext();
    invoke.mockClear();
  });

  it("answers through the real Conversation façade without any IPC", async () => {
    const outcome = await handleOperatorUtterance(UTTERANCE);
    expect(outcome.kind).toBe("reply");
    const text = outcome.kind === "reply" ? outcome.text : "";
    expect(text).toMatch(/^It’s \d{1,2}:\d{2} (AM|PM) in Queensland\.$/);
    expect(invoke).not.toHaveBeenCalled();
  });

  it("answers locally end to end and reaches no external source", () => {
    const goal = comprehend(UTTERANCE);
    expect(goal.outcome).toBe("KNOW");
    expect(goal.domain).toBe("time");
    expect(goal.requestedResult).toBe("current time");
    expect(goal.subject).toMatch(/queensland/);

    const region = resolveRegionZone(goal.subject);
    expect(region).toEqual({
      kind: "zone",
      zone: "Australia/Brisbane",
      label: "Queensland",
    });

    const answer = answerForGoal(goal);
    expect(answer?.rung).toBe("deterministic-local");
    expect(answer?.sourceId).toBe("local-clock");

    const { action } = resolveIntentWithGoal(UTTERANCE);
    expect(action.kind).toBe("unknown");
    expect(action.reply).toMatch(/^It’s \d{1,2}:\d{2} (AM|PM) in Queensland\.$/);
    expect(
      plausibleTimes("Australia/Brisbane").some((t) => action.reply.includes(t)),
    ).toBe(true);

    // browserOpen = false, ChatGPT handoff = false, desktop effect = none.
    expect(action.kind === ("browserOpen" as IntentAction["kind"])).toBe(false);
    expect(action.reply.toLowerCase()).not.toContain("chatgpt");
    expect(isEffect(action)).toBe(false);
    expect(invoke).not.toHaveBeenCalled();
  });
});

describe("P23.S3 preserved behaviour", () => {
  beforeEach(() => {
    resetWorkspaceContext();
    invoke.mockClear();
  });

  it("11. leaves genuine desktop requests as desktop requests", () => {
    expect(resolveIntent("Open Notepad.").kind).toBe("appOpen");
    expect(resolveIntent("Take a screenshot").kind).toBe("screenshotDesktop");
    expect(isEffect(resolveIntent("What windows do I have open?"))).toBe(true);
  });

  it("12. keeps the P23.S2 substitution prohibition intact", () => {
    // A question containing "minimize" still never collapses the surface,
    // while the command form still does.
    expect(resolveIntent("What does minimize mean?").kind).not.toBe("collapse");
    resetWorkspaceContext();
    expect(resolveIntent("Collapse").kind).toBe("collapse");

    // A goal the ladder cannot answer is still refused, never substituted.
    resetWorkspaceContext();
    const { goal, action } = resolveIntentWithGoal("How's your day?");
    expect(goal.outcome).toBe("SOCIAL");
    expect(isEffect(action)).toBe(false);
  });

  it("13. keeps social conversation conversational", () => {
    for (const utterance of ["How are you?", "How's your day?"]) {
      resetWorkspaceContext();
      const action = resolveIntent(utterance);
      expect(isEffect(action)).toBe(false);
      expect(action.reply.toLowerCase()).not.toContain("chatgpt");
    }
  });

  it("14. keeps computation computational", () => {
    expect(resolveIntent("What is 15% of 80?").reply).toBe("12");
    expect(resolveIntent("convert 100 c to f").reply).toMatch(/212/);
  });
});

describe("P23.S3 authority boundary", () => {
  const LADDER_FILES = [
    "app/src/lib/answerSource.ts",
    "app/src/lib/temporalAnswerSource.ts",
  ];

  it("15. performs no desktop IPC from any answer source", () => {
    for (const rel of LADDER_FILES) {
      const source = readFileSync(path.join(root, rel), "utf8");
      expect(source).not.toMatch(/@tauri-apps|invoke\(|execute_capability_intent/);
    }
  });

  it("16. selects no Kernel capability and constructs no action", () => {
    for (const rel of LADDER_FILES) {
      const source = readFileSync(path.join(root, rel), "utf8");
      expect(source).not.toMatch(/kind:\s*"(appOpen|browserOpen|win[A-Z]|screenshot)/);
      expect(source).not.toMatch(/CapabilityIntent|capabilityRegistry|executionPlan/);
    }
    // The ladder yields text, never an action the Kernel could execute.
    const answer = resolveAnswer(comprehend("What time is it in Brisbane?"));
    expect(Object.keys(answer ?? {}).sort()).toEqual(["rung", "sourceId", "text"]);
  });

  it("17. keeps the Goal Contract free of provider and capability identity", () => {
    const goal = comprehend("What time is it in Queensland?");
    const serialized = JSON.stringify(goal).toLowerCase();
    for (const token of [
      "provider",
      "capability",
      "browseropen",
      "appopen",
      "chatgpt",
      "australia/brisbane",
      "local-clock",
    ]) {
      expect(serialized).not.toContain(token);
    }
  });

  it("declares the full ladder while implementing only the local rung", () => {
    expect(ANSWER_RUNGS).toEqual([
      "deterministic-local",
      "workspace-system",
      "capability-observation",
      "authorized-external",
      "honest-limitation",
    ]);
    expect(temporalAnswerSource.rung).toBe("deterministic-local");
    // A source declines rather than answering outside its contract.
    expect(temporalAnswerSource.covers(comprehend("How are you?"))).toBe(false);
    expect(temporalAnswerSource.answer(comprehend("Open Notepad."))).toBeNull();
  });
});
