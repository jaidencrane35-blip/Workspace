/**
 * P23.S2 — Substitution Prohibition Enforcement.
 *
 * Proves that a request fulfilled by knowledge never resolves to a desktop
 * effect chosen from action-shaped vocabulary, that enforcement can only remove
 * an effect and never introduce one, and that genuine desktop work is untouched.
 */
import { beforeEach, describe, expect, it, vi } from "vitest";

const invoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({ invoke }));

import { comprehend } from "../app/src/lib/goalContract";
import {
  resolveIntent,
  resolveIntentWithGoal,
  type IntentAction,
} from "../app/src/lib/intentBridge";
import {
  enforceSubstitutionProhibition,
  hasPositiveOutcomeEvidence,
  isAnswerOnlyGoal,
} from "../app/src/lib/substitutionProhibition";
import { CAPABILITY_GRAPH } from "../app/src/lib/capabilityRegistry";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

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

describe("P23.S2 substitution prohibition", () => {
  beforeEach(() => {
    resetWorkspaceContext();
    invoke.mockClear();
  });

  it("1. KNOW does not substitute a desktop effect for a missing answer", () => {
    // A question that merely contains "minimize" must not collapse the surface.
    const { goal, action } = resolveIntentWithGoal("What does minimize mean?");
    expect(goal.outcome).toBe("KNOW");
    expect(action.kind).not.toBe("collapse");

    // The command form still works.
    resetWorkspaceContext();
    expect(resolveIntent("Collapse").kind).toBe("collapse");
  });

  it("2. COMPUTE keeps its answer and opens nothing", () => {
    const { goal, action } = resolveIntentWithGoal("What is 15% of 80?");
    expect(goal.outcome).toBe("COMPUTE");
    expect(goal.mode).toBe("computation");
    expect(action.kind).toBe("unknown");
    expect(action.reply).toBe("12");
    expect(isEffect(action)).toBe(false);
  });

  it("2b. a genuine compute-and-act request is not treated as answer-only", () => {
    const { goal, action } = resolveIntentWithGoal("What is 15% of 80 and open Notepad");
    expect(goal.outcome).toBe("COMPUTE");
    expect(goal.mode).toBe("hybrid");
    expect(isAnswerOnlyGoal(goal)).toBe(false);
    expect(action.kind).toBe("appOpen");
  });

  it("3. SOCIAL answers conversationally, with no effect and no desktop suggestion", () => {
    for (const utterance of ["How are you?", "Hey, how are you?", "How's your day?"]) {
      resetWorkspaceContext();
      const { goal, action } = resolveIntentWithGoal(utterance);
      expect(goal.outcome).toBe("SOCIAL");
      expect(isEffect(action)).toBe(false);
      expect(action.kind).toBe("unknown");
      if (action.kind !== "unknown") return;
      expect(action.suggestion).toBeUndefined();
      expect(action.reply).not.toMatch(/desktop step|try a desktop|open |launch /i);
    }
  });

  it("4. information questions containing desktop vocabulary stay observational", () => {
    const cases: Array<[string, string]> = [
      ["What windows do I have open?", "winEnumerate"],
      ["Can you tell me what is open on my desktop?", "winEnumerate"],
      ["Which browsers are available?", "browserStatus"],
      ["Where is the Save button in Notepad?", "winFindControl"],
    ];
    for (const [utterance, expected] of cases) {
      resetWorkspaceContext();
      const { action } = resolveIntentWithGoal(utterance);
      expect(action.kind, utterance).toBe(expected);
    }

    resetWorkspaceContext();
    expect(comprehend("What windows do I have open?").outcome).toBe(
      "PERCEIVE_MACHINE",
    );
  });

  it("5. genuine desktop commands remain functional", () => {
    const cases: Array<[string, string]> = [
      ["Open Notepad.", "appOpen"],
      ["Take a screenshot", "screenshotDesktop"],
      ["Where was I?", "navigate"],
      ["Prepare my coding workspace with Cursor and Notepad.", "prepareCodingWorkspace"],
    ];
    for (const [utterance, expected] of cases) {
      resetWorkspaceContext();
      expect(resolveIntent(utterance).kind, utterance).toBe(expected);
    }
  });

  it("6. hybrid goals keep their desktop work", () => {
    const { goal, action } = resolveIntentWithGoal(
      "Open Pictures, open Screenshots, and count the pictures.",
    );
    expect(goal.mode).toBe("hybrid");
    expect(isAnswerOnlyGoal(goal)).toBe(false);
    expect(isEffect(action)).toBe(true);
  });
});

describe("P23.S2 Queensland live-failure regression", () => {
  beforeEach(() => {
    resetWorkspaceContext();
    invoke.mockClear();
  });

  const QUEENSLAND = [
    "What time is it in Queensland?",
    "What is the time in Queensland, Australia?",
  ];

  it("comprehends KNOW and requests no desktop effect", () => {
    for (const utterance of QUEENSLAND) {
      resetWorkspaceContext();
      const goal = comprehend(utterance);
      expect(goal.outcome).toBe("KNOW");
      expect(goal.mode).toBe("information");
      expect(goal.domain).toBe("time");
      // Comprehension names no desktop work of its own.
      expect(goal.clauses).toHaveLength(0);
      expect(goal.targets.every((t) => t.role === "place")).toBe(true);
      expect(invoke).not.toHaveBeenCalled();
    }
  });

  it("never lets the outcome alone imply opening a browser", () => {
    for (const utterance of QUEENSLAND) {
      resetWorkspaceContext();
      const goal = comprehend(utterance);
      // The outcome by itself resolves to a truthful limitation, not an effect.
      const refused = enforceSubstitutionProhibition(goal, {
        kind: "browserOpen",
        url: "https://chatgpt.com/?q=x",
        reply: "Opening ChatGPT.",
      });
      expect(refused.kind).toBe("unknown");
      expect(refused.reply).toMatch(/current time in Queensland/i);

      // It is permitted only when routing marked it as its own decision.
      const routed = enforceSubstitutionProhibition(goal, {
        kind: "browserOpen",
        url: "https://chatgpt.com/?q=x",
        reply: "Opening ChatGPT.",
        informationHandoff: true,
      });
      expect(routed.kind).toBe("browserOpen");
    }
  });

  it("introduces no new browser-opening behaviour", () => {
    for (const utterance of QUEENSLAND) {
      resetWorkspaceContext();
      const { goal, action } = resolveIntentWithGoal(utterance);
      if (action.kind === "browserOpen") {
        // Any surviving handoff came from existing routing, not from this layer.
        expect(action.informationHandoff).toBe(true);
      } else {
        expect(isEffect(action)).toBe(false);
      }
      expect(hasPositiveOutcomeEvidence(goal)).toBe(true);
    }
  });
});

describe("P23.S2 authority boundary", () => {
  beforeEach(() => {
    resetWorkspaceContext();
    invoke.mockClear();
  });

  const CORPUS = [
    "What time is it in Queensland?",
    "What is 15% of 80?",
    "How are you?",
    "What does minimize mean?",
    "What windows do I have open?",
    "Open Notepad.",
    "Open Pictures, open Screenshots, and count the pictures.",
    "Take a screenshot",
  ];

  const PROBE_ACTIONS: IntentAction[] = [
    { kind: "unknown", reply: "nothing yet" },
    { kind: "appOpen", query: "Notepad", reply: "Opening Notepad." },
    { kind: "collapse", reply: "Collapsing." },
    { kind: "browserOpen", url: "https://example.com", reply: "Opening." },
    { kind: "winEnumerate", reply: "Checking windows." },
  ];

  it("can only remove an effect, never introduce one (no Meaning → Effect)", () => {
    for (const utterance of CORPUS) {
      resetWorkspaceContext();
      const goal = comprehend(utterance);
      for (const probe of PROBE_ACTIONS) {
        const out = enforceSubstitutionProhibition(goal, probe);
        const unchanged = out === probe;
        expect(unchanged || SPEAKING.has(out.kind)).toBe(true);
        if (!unchanged) {
          expect(isEffect(out)).toBe(false);
        }
      }
    }
  });

  it("performs no IPC", () => {
    for (const utterance of CORPUS) {
      resetWorkspaceContext();
      const goal = comprehend(utterance);
      enforceSubstitutionProhibition(goal, { kind: "collapse", reply: "x" });
    }
    expect(invoke).not.toHaveBeenCalled();
  });

  it("keeps the Goal Contract free of capability and provider identity", () => {
    const ids = CAPABILITY_GRAPH.map((node) => node.id);
    for (const utterance of CORPUS) {
      resetWorkspaceContext();
      const goal = comprehend(utterance);
      const decided = JSON.stringify({
        mode: goal.mode,
        outcome: goal.outcome,
        domain: goal.domain,
        requestedResult: goal.requestedResult,
        roles: goal.targets.map((t) => t.role),
        evidence: goal.evidence,
      }).toLowerCase();
      for (const id of ids) {
        expect(decided).not.toContain(id);
      }
      expect(decided).not.toContain("chatgpt");
      expect(decided).not.toContain("provider");
    }
  });

  it("does not duplicate the existing external handoff", () => {
    // Enforcement never constructs a handoff of its own; it only honours the
    // mark set by existing routing.
    resetWorkspaceContext();
    const goal = comprehend("Who wrote Hamlet?");
    const out = enforceSubstitutionProhibition(goal, {
      kind: "unknown",
      reply: "nothing",
      softMiss: true,
    });
    expect(JSON.stringify(out).toLowerCase()).not.toContain("chatgpt");
    expect(out.kind).toBe("unknown");
  });
});
