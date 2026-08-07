/**
 * P16.36 — Goal Resolution hostile battery (≥500 natural requests).
 * Measures refinement, clarification, candidate ranking — not alias growth.
 */
import { describe, expect, it } from "vitest";
import {
  resolveIntent,
  resolveIntentBeforeGoalResolution,
} from "../app/src/lib/intentBridge";
import { resolveIntentWithEvidence } from "../app/src/lib/intentPipeline";
import { resolveGoal } from "../app/src/lib/goalResolution";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

const LEAD = ["", "please ", "can you ", "could you ", "i think "];
const TAIL = ["", " please", " for me", " now", ""];

function expand(core: string): string[] {
  const out: string[] = [];
  for (const l of LEAD) {
    for (const t of TAIL) {
      const s = `${l}${core}${t}`.replace(/\s+/g, " ").trim();
      out.push(s);
      out.push(`${s}.`);
      out.push(`${s}?`);
    }
  }
  return [...new Set(out)];
}

type Expect =
  | { family: "resume"; kinds: string[] }
  | { family: "clarify"; kinds: string[]; clarify: true }
  | { family: "beside"; kinds: string[]; multi: true }
  | { family: "locate"; kinds: string[] }
  | { family: "done"; kinds: string[] }
  | { family: "discovery"; kinds: string[] }
  | { family: "open"; kinds: string[] };

const GROUPS: Array<{ cores: string[]; expect: Expect }> = [
  {
    cores: [
      "Where was I",
      "I need everything back",
      "I was coding",
      "I need my workspace",
      "Get everything back",
      "I need my desk back",
      "I'm trying to continue",
      "Take me where I was",
    ],
    expect: { family: "resume", kinds: ["navigate"] },
  },
  {
    cores: [
      "I've lost it",
      "I lost it",
      "I'm looking for something",
      "Looking for something",
      "Find it",
      "Where did it go",
    ],
    expect: { family: "clarify", kinds: ["unknown"], clarify: true },
  },
  {
    cores: [
      "I need YouTube beside Cursor",
      "Open ChatGPT beside Cursor",
      "I want YouTube next to Cursor",
    ],
    expect: { family: "beside", kinds: ["browserOpenBeside"], multi: true },
  },
  {
    cores: [
      "I've got Chrome somewhere",
      "I was just using Chrome",
      "Where is Cursor",
      "Bring back my browser",
    ],
    expect: { family: "locate", kinds: ["winFocus"] },
  },
  {
    cores: ["I'm done", "I'm done with this", "I'm finished with this"],
    expect: { family: "done", kinds: ["unknown"] },
  },
  {
    cores: [
      "What can you do",
      "Show me everything you know how to control",
      "Show me everything you can control",
    ],
    expect: { family: "discovery", kinds: ["capabilityExplain"] },
  },
  {
    cores: ["Open ChatGPT", "Take me to YouTube", "Open Notepad", "Focus Chrome"],
    expect: {
      family: "open",
      kinds: [
        "browserOpen",
        "browserOpenFocus",
        "appOpen",
        "appLaunch",
        "winFocus",
      ],
    },
  },
];

describe("P16.36 Goal Resolution battery", () => {
  it("classifies ≥500 natural requests with goal resolution evidence", () => {
    let total = 0;
    let matched = 0;
    let refined = 0;
    let withCandidates = 0;
    let misunderstood = 0;
    const failures: string[] = [];

    for (const group of GROUPS) {
      for (const core of group.cores) {
        for (const utterance of expand(core)) {
          total += 1;
          // Single-turn Goal Resolution evidence — isolate from session Context.
          resetWorkspaceContext();
          const before = resolveIntentBeforeGoalResolution(utterance);
          const goal = resolveGoal(utterance, before);
          const action = resolveIntent(utterance);
          resetWorkspaceContext();
          const evidence = resolveIntentWithEvidence(utterance);

          if (goal.refined) refined += 1;
          if (goal.candidates.length >= 1) withCandidates += 1;

          if (/Provider|Registry|Kernel|WinRT/i.test(action.reply ?? "")) {
            misunderstood += 1;
            failures.push(`${utterance} → internals`);
            continue;
          }
          if (
            "query" in action &&
            typeof action.query === "string" &&
            /\.exe$/i.test(action.query)
          ) {
            misunderstood += 1;
            failures.push(`${utterance} → exe`);
            continue;
          }

          const okKind = group.expect.kinds.includes(action.kind);
          const okClarify =
            group.expect.family !== "clarify" ||
            (goal.needsClarification && action.kind === "unknown");
          const okMulti =
            group.expect.family !== "beside" ||
            evidence.plan.multiStep === true;

          if (okKind && okClarify && okMulti) {
            matched += 1;
          } else {
            failures.push(
              `${utterance} → ${action.kind} (want ${group.expect.kinds.join("|")})`,
            );
          }

          expect(
            evidence.stages.some((s) => s.stage === "goal_resolution" && s.hit),
          ).toBe(true);
          expect(goal.intendedOutcome.length).toBeGreaterThan(5);
          expect(goal.completionCriteria.length).toBeGreaterThan(0);
          expect(goal.recoveryStrategy.length).toBeGreaterThan(5);
        }
      }
    }

    expect(total).toBeGreaterThanOrEqual(500);
    expect(withCandidates / total).toBe(1);
    expect(matched / total).toBeGreaterThanOrEqual(0.9);
    expect(misunderstood / total).toBeLessThan(0.02);
    // Goal Resolution must refine a measurable share of underspecified resumes/locates.
    expect(refined).toBeGreaterThan(50);
    if (failures.length > 0 && matched / total < 0.9) {
      expect(failures.slice(0, 20)).toEqual([]);
    }
  });

  it("falsifies Situation Goals alone for underspecified Owner phrases", () => {
    const cases = [
      { u: "I've lost it.", kind: "unknown", clarify: true },
      { u: "Where was I?", kind: "navigate" },
      { u: "I need everything back.", kind: "navigate" },
      { u: "I was coding.", kind: "navigate" },
      { u: "I need my workspace.", kind: "navigate" },
      { u: "I'm looking for something.", kind: "unknown", clarify: true },
      { u: "I need YouTube beside Cursor.", kind: "browserOpenBeside" },
    ];
    for (const c of cases) {
      const before = resolveIntentBeforeGoalResolution(c.u);
      const goal = resolveGoal(c.u, before);
      const after = resolveIntent(c.u);
      expect(after.kind, c.u).toBe(c.kind);
      expect(goal.candidates.length, c.u).toBeGreaterThanOrEqual(1);
      expect(goal.selectedPlan.steps.length, c.u).toBeGreaterThan(0);
      if (c.clarify) {
        expect(goal.needsClarification, c.u).toBe(true);
        expect(goal.missingInformation.length, c.u).toBeGreaterThan(0);
      }
      if (c.kind === "navigate") {
        expect(goal.refined || before.kind === "navigate", c.u).toBe(true);
      }
    }
  });

  it("ranks beside compositions with a fallback candidate plan", () => {
    const goal = resolveGoal(
      "I need YouTube beside Cursor",
      resolveIntentBeforeGoalResolution("I need YouTube beside Cursor"),
    );
    expect(goal.candidates.length).toBeGreaterThanOrEqual(2);
    expect(goal.candidates[0]!.id).toBe("primary");
    expect(goal.candidates.some((c) => c.id === "open-only")).toBe(true);
    expect(goal.selectedPlan.multiStep).toBe(true);
  });
});
