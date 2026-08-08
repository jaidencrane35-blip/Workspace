/**
 * P23.S1 — Outcome-First Comprehension (Goal Contract).
 *
 * Proves that comprehension preserves the Owner's outcome rather than the
 * command they happened to phrase it with, and that it selects no providers
 * and performs no desktop effects.
 */
import { beforeEach, describe, expect, it, vi } from "vitest";

const invoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({ invoke }));

import { comprehend, type GoalContract } from "../app/src/lib/goalContract";
import { resolveIntent, resolveIntentWithGoal } from "../app/src/lib/intentBridge";
import { CAPABILITY_GRAPH } from "../app/src/lib/capabilityRegistry";
import {
  getWorkspaceContext,
  resetWorkspaceContext,
} from "../app/src/lib/workspaceContext";

function targetPhrases(goal: GoalContract): string[] {
  return goal.targets.map((t) => (t.canonical ?? t.phrase).toLowerCase());
}

describe("P23.S1 Goal Contract — outcome-first comprehension", () => {
  beforeEach(() => {
    resetWorkspaceContext();
    invoke.mockClear();
  });

  it("1. asks for the time: information outcome, no desktop goal", () => {
    const goal = comprehend("What time is it in Queensland?");
    expect(goal.outcome).toBe("KNOW");
    expect(goal.mode).toBe("information");
    expect(goal.domain).toBe("time");
    expect(goal.requestedResult).toBe("current time");
    expect(goal.subject).toBe("queensland");
    expect(goal.clarificationNeeded).toBe(false);
  });

  it("2. asks to be shown a place: visual-location outcome, distinct from case 1", () => {
    const know = comprehend("What is the time in Queensland, Australia?");
    const show = comprehend("Show me where Queensland, Australia is.");

    expect(show.outcome).toBe("PERCEIVE_WORLD");
    expect(show.mode).toBe("show");
    expect(show.requestedResult).toBe("location");
    expect(show.subject).toBe("queensland, australia");

    // The two requests must not be indistinguishable.
    expect(show.outcome).not.toBe(know.outcome);
    expect(show.mode).not.toBe(know.mode);
  });

  it("3. preserves both semantic targets of a desktop request", () => {
    const goal = comprehend("Open File Explorer and go to Pictures.");
    expect(goal.outcome).toBe("REACH_STATE");
    expect(goal.mode).toBe("action");
    expect(goal.compound).toBe(true);
    expect(targetPhrases(goal)).toEqual(
      expect.arrayContaining(["file explorer", "pictures"]),
    );
  });

  it("4. keeps the final requested result of a compound goal", () => {
    const goal = comprehend("Open Pictures, open Screenshots, and count the pictures.");
    // The goal is the count, not the first thing that must be opened.
    expect(goal.outcome).toBe("KNOW");
    expect(goal.mode).toBe("hybrid");
    expect(goal.requestedResult).toBe("count");
    expect(goal.compound).toBe(true);
    expect(goal.clauses.length).toBeGreaterThanOrEqual(3);
    expect(targetPhrases(goal)).toEqual(
      expect.arrayContaining(["pictures", "screenshots"]),
    );
  });

  it("5. reads observe-then-act as one hybrid goal", () => {
    const goal = comprehend("Find the latest screenshot and open it in Paint.");
    expect(goal.outcome).toBe("REACH_STATE");
    expect(goal.mode).toBe("hybrid");
    expect(goal.compound).toBe(true);
    expect(targetPhrases(goal)).toEqual(expect.arrayContaining(["paint"]));
    // "it" is bound by the antecedent named earlier in the same sentence.
    const pronoun = goal.references.find((r) => /it\b/.test(r.phrase));
    expect(pronoun?.resolved).toBe(true);
    expect(goal.clarificationNeeded).toBe(false);
  });

  it("6. treats ordinary conversation as conversation", () => {
    for (const utterance of ["How are you?", "Hey, how are you?"]) {
      const goal = comprehend(utterance);
      expect(goal.outcome).toBe("SOCIAL");
      expect(goal.mode).toBe("conversation");
      expect(goal.domain).toBe("conversation");
      expect(goal.targets).toHaveLength(0);
    }
  });

  it("7. marks a context-dependent reference and invents no target", () => {
    const goal = comprehend("Open that folder.");
    expect(goal.outcome).toBe("REACH_STATE");
    expect(goal.references.map((r) => r.phrase)).toContain("that folder");
    expect(goal.references[0]?.resolved).toBe(false);
    expect(goal.clarificationNeeded).toBe(true);
    expect(goal.targets.filter((t) => t.role !== "state")).toHaveLength(0);
  });

  it("7b. resolves the same reference once session context holds a referent", () => {
    resolveIntent("Open Notepad");
    expect(getWorkspaceContext().lastAppQuery).toBeTruthy();

    const goal = comprehend("Open that folder.");
    expect(goal.references[0]?.resolved).toBe(true);
    expect(goal.references[0]?.referent).toBeTruthy();
  });

  it("8. reports ambiguous “it” as unresolved rather than inventing a subject", () => {
    const goal = comprehend("Open it.");
    expect(goal.references.map((r) => r.phrase)).toContain("open it");
    expect(goal.references[0]?.resolved).toBe(false);
    expect(goal.clarificationNeeded).toBe(true);
    expect(goal.subject).toBeNull();
    expect(goal.uncertainties.join(" ")).toMatch(/unresolved reference/);
  });

  it("9. reports an unknown target as unrecognised rather than inventing one", () => {
    const goal = comprehend("Open Blorptron 9000.");
    expect(goal.outcome).toBe("REACH_STATE");
    const target = goal.targets.find((t) => t.role === "subject");
    expect(target?.phrase).toBe("blorptron 9000");
    expect(target?.recognized).toBe(false);
    expect(target?.canonical).toBeNull();
    expect(goal.uncertainties.join(" ")).toMatch(/unrecognised target/);
  });

  it("10. leaves existing single-step desktop behaviour unchanged", () => {
    expect(resolveIntent("Open Notepad").kind).toBe("appOpen");
    resetWorkspaceContext();
    expect(resolveIntent("What windows are open?").kind).toBe("winEnumerate");
    resetWorkspaceContext();
    const prepare = resolveIntent(
      "Prepare my coding workspace with Cursor and Notepad.",
    );
    expect(prepare.kind).toBe("prepareCodingWorkspace");
  });

  it("keeps requested end-state for a compound media goal", () => {
    const goal = comprehend(
      "Open YouTube, search for lofi, play it, put it fullscreen and make sure the volume is on.",
    );
    expect(goal.compound).toBe(true);
    const states = goal.targets.filter((t) => t.role === "state").map((t) => t.phrase);
    expect(states).toEqual(
      expect.arrayContaining(["fullscreen", "audio enabled", "playing"]),
    );
    expect(targetPhrases(goal)).toEqual(expect.arrayContaining(["youtube"]));
  });

  it("reads a machine question as observation even when it contains “open”", () => {
    const goal = comprehend("What windows do I have open?");
    expect(goal.outcome).toBe("PERCEIVE_MACHINE");
    expect(goal.mode).toBe("observation");
    expect(goal.domain).toBe("desktop");
  });

  it("reads arithmetic as computation, never as a desktop goal", () => {
    const goal = comprehend("How much is 3 tonnes at $87 per 200kg?");
    expect(goal.outcome).toBe("COMPUTE");
    expect(goal.mode).toBe("computation");
    expect(goal.outcome).not.toBe("REACH_STATE");
  });
});

describe("P23.S1 authority boundary", () => {
  beforeEach(() => {
    resetWorkspaceContext();
    invoke.mockClear();
  });

  const corpus = [
    "What time is it in Queensland?",
    "Show me where Queensland is.",
    "Open File Explorer and go to Pictures.",
    "Open Pictures, open Screenshots, and count the pictures.",
    "Find the latest screenshot and open it in Paint.",
    "How are you?",
    "Open that folder.",
    "Open it.",
    "Open Blorptron 9000.",
    "Open Notepad",
  ];

  it("performs no IPC and therefore no desktop effect", () => {
    for (const utterance of corpus) {
      comprehend(utterance);
    }
    expect(invoke).not.toHaveBeenCalled();
  });

  it("selects no capability, provider, or execution step", () => {
    const ids = CAPABILITY_GRAPH.map((node) => node.id);
    for (const utterance of corpus) {
      const goal = comprehend(utterance);
      // Only the fields Workspace decides — Owner phrases are excluded, since
      // the Owner may legitimately say a word that matches a capability id.
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
      for (const token of [
        "provider",
        "execute_capability_intent",
        "capabilityid",
        "domain\":\"browser",
      ]) {
        expect(decided).not.toContain(token);
      }
    }
  });

  it("never invents a canonical identity for an unrecognised target", () => {
    for (const utterance of corpus) {
      for (const target of comprehend(utterance).targets) {
        if (!target.recognized) {
          expect(target.canonical).toBeNull();
        }
      }
    }
  });

  it("exposes only meaning fields on the contract", () => {
    const goal = comprehend("Open File Explorer and go to Pictures.");
    expect(Object.keys(goal).sort()).toEqual(
      [
        "clarificationNeeded",
        "clauses",
        "compound",
        "domain",
        "evidence",
        "mode",
        "normalized",
        "outcome",
        "references",
        "requestedResult",
        "subject",
        "targets",
        "uncertainties",
        "utterance",
      ].sort(),
    );
  });
});

describe("P23.S1 preservation", () => {
  beforeEach(() => {
    resetWorkspaceContext();
    invoke.mockClear();
  });

  it("carries the comprehended goal alongside the resolved action", () => {
    const { goal, action } = resolveIntentWithGoal(
      "Open Pictures, open Screenshots, and count the pictures.",
    );
    expect(action).toBeTruthy();
    expect(goal.outcome).toBe("KNOW");
    expect(goal.requestedResult).toBe("count");
  });

  it("preserves the goal in Workspace Context instead of discarding it", () => {
    resolveIntent("Open File Explorer and go to Pictures.");
    const stored = getWorkspaceContext().currentGoal;
    expect(stored).not.toBeNull();
    expect(stored?.outcome).toBe("REACH_STATE");
    expect(stored?.compound).toBe(true);
    expect(stored?.utterance).toBe("Open File Explorer and go to Pictures.");
  });
});
