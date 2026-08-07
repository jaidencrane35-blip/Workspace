/**
 * P16.39 — Hostile Product Operator Intelligence battery (≥750).
 * Measures operator-activity understanding — not command tables.
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import {
  classifyOperatorActivity,
  resolveSituationGoal,
} from "../app/src/lib/situationGoals";
import {
  hasEngineeringLeak,
  STATE_VS_GOAL_OWNERS,
  PRODUCT_INTELLIGENCE_GAPS,
} from "../app/src/lib/productIntelligence";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";
import { validateCapabilityGraphGovernance } from "../app/src/lib/capabilityRegistry";

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

type Group = {
  cores: string[];
  activity: ReturnType<typeof classifyOperatorActivity>;
  expectKinds: string[];
};

const GROUPS: Group[] = [
  {
    activity: "session_resume",
    cores: [
      "I need to get back into work",
      "Set me up",
      "I'm starting my day",
      "Take me back",
      "Back to work",
      "I need my development setup",
      "I was working on something",
      "Get started",
    ],
    expectKinds: ["navigate"],
  },
  {
    activity: "work_mode",
    cores: [
      "I'm coding",
      "I'm debugging",
      "I'm researching",
      "I'm reviewing",
      "I'm writing documentation",
      "I'm doing research",
      "I'm doing code review",
    ],
    expectKinds: ["navigate"],
  },
  {
    activity: "session_end",
    cores: [
      "I'm finished",
      "I'm done",
      "I'm taking a break",
      "Taking a break",
      "I'm done with this",
    ],
    expectKinds: ["unknown"],
  },
  {
    activity: "discovery",
    cores: [
      "Show me everything you know how to control",
      "Tell me everything you can control",
    ],
    expectKinds: ["capabilityExplain"],
  },
  {
    activity: "screenshots",
    cores: [
      "I'm looking for my screenshots",
      "Find my screenshots",
      "Where are my screenshots",
    ],
    expectKinds: ["appLaunch", "appOpen"],
  },
  {
    activity: "none",
    cores: [
      "Open ChatGPT",
      "Focus Chrome",
      "What windows are open",
      "Let's continue",
      "Continue",
    ],
    expectKinds: [
      "browserOpen",
      "browserOpenFocus",
      "winFocus",
      "winEnumerate",
      "navigate",
      "navigateNamed",
    ],
  },
];

describe("P16.39 operator activity battery", () => {
  it("understands ≥750 operator-activity interactions without engineering leaks", () => {
    let total = 0;
    let matched = 0;
    let activityOk = 0;
    let leaks = 0;
    let outsideRefusal = 0;
    const failures: string[] = [];

    for (const group of GROUPS) {
      for (const core of group.cores) {
        for (const utterance of expand(core)) {
          total += 1;
          resetWorkspaceContext();
          const family = classifyOperatorActivity(utterance);
          const action = resolveIntent(utterance);

          if (hasEngineeringLeak(action.reply ?? "")) {
            leaks += 1;
            failures.push(`${utterance} → leak`);
            continue;
          }
          if (/outside what i can operate/i.test(action.reply ?? "")) {
            outsideRefusal += 1;
            failures.push(`${utterance} → outside-refusal`);
            continue;
          }
          if (family === group.activity) activityOk += 1;
          else {
            failures.push(
              `${utterance} → activity ${family} (want ${group.activity})`,
            );
          }
          if (group.expectKinds.includes(action.kind)) matched += 1;
          else {
            failures.push(
              `${utterance} → ${action.kind} (want ${group.expectKinds.join("|")})`,
            );
          }
        }
      }
    }

    expect(total).toBeGreaterThanOrEqual(750);
    expect(leaks).toBe(0);
    expect(outsideRefusal).toBe(0);
    expect(matched / total).toBeGreaterThanOrEqual(0.9);
    expect(activityOk / total).toBeGreaterThanOrEqual(0.9);
    if (failures.length) {
      expect(failures.slice(0, 12), failures.slice(0, 12).join("\n")).toEqual(
        [],
      );
    }
  });

  it("F15 Owner findings route as operator states via Situation Goals", () => {
    const cases: Array<{ u: string; kind: string; activity: string }> = [
      { u: "I need to get back into work.", kind: "navigate", activity: "session_resume" },
      { u: "Set me up.", kind: "navigate", activity: "session_resume" },
      { u: "I'm starting my day.", kind: "navigate", activity: "session_resume" },
      { u: "I'm coding.", kind: "navigate", activity: "work_mode" },
      { u: "I'm debugging.", kind: "navigate", activity: "work_mode" },
      { u: "I'm researching.", kind: "navigate", activity: "work_mode" },
      { u: "I'm writing documentation.", kind: "navigate", activity: "work_mode" },
      { u: "I'm reviewing.", kind: "navigate", activity: "work_mode" },
      { u: "I'm taking a break.", kind: "unknown", activity: "session_end" },
      { u: "I'm finished.", kind: "unknown", activity: "session_end" },
      { u: "Take me back.", kind: "navigate", activity: "session_resume" },
    ];
    for (const c of cases) {
      resetWorkspaceContext();
      expect(classifyOperatorActivity(c.u), c.u).toBe(c.activity);
      expect(resolveSituationGoal(c.u)?.kind ?? resolveIntent(c.u).kind, c.u).toBe(
        c.kind,
      );
      const action = resolveIntent(c.u);
      expect(action.kind, c.u).toBe(c.kind);
      expect(hasEngineeringLeak(action.reply ?? "")).toBe(false);
      expect(action.reply ?? "").not.toMatch(/outside what i can operate/i);
    }
  });

  it("proves State vs Goal owners are distinct and Registry governance holds", () => {
    const owners = Object.values(STATE_VS_GOAL_OWNERS);
    expect(new Set(owners).size).toBe(owners.length);
    expect(STATE_VS_GOAL_OWNERS.userGoal).toMatch(/Situation Goals/);
    expect(STATE_VS_GOAL_OWNERS.conversationState).toMatch(/Context/);
    expect(STATE_VS_GOAL_OWNERS.capabilityState).toMatch(/Registry/);
    expect(STATE_VS_GOAL_OWNERS.executionState).toMatch(/Execution/);
    expect(validateCapabilityGraphGovernance()).toEqual([]);
    expect(
      PRODUCT_INTELLIGENCE_GAPS.some((g) => g.id === "operator-activities"),
    ).toBe(true);
  });
});
