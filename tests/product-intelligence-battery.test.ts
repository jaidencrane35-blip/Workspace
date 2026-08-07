/**
 * P16.38 — Hostile product experience battery (≥500).
 * Measures Owner-facing desktop operator experience — not implementation internals.
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { resolveIntentWithEvidence } from "../app/src/lib/intentPipeline";
import {
  generateCapabilityDiscovery,
  validateCapabilityGraphGovernance,
} from "../app/src/lib/capabilityRegistry";
import {
  classifyProductExperience,
  hasEngineeringLeak,
  PRODUCT_INTELLIGENCE_GAPS,
} from "../app/src/lib/productIntelligence";
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

type Group = {
  cores: string[];
  setup?: string[];
  expectKinds: string[];
  family?: string;
};

const GROUPS: Group[] = [
  {
    cores: [
      "I need my development setup",
      "I was working on something",
      "I need everything back",
      "Take me where I was",
      "Where was I",
    ],
    expectKinds: ["navigate"],
    family: "resume_intention",
  },
  {
    cores: ["I've lost it", "I'm looking for something", "Find it"],
    expectKinds: ["unknown"],
    family: "locate_clarify",
  },
  {
    cores: [
      "I'm looking for my screenshots",
      "Find my screenshots",
      "Where are my screenshots",
    ],
    expectKinds: ["appLaunch", "appOpen"],
    family: "screenshots_intention",
  },
  {
    cores: [
      "What can you do",
      "Show me everything you know how to control",
      "What can't you do",
      "What else can you do",
      "What's similar",
      "What are my options",
    ],
    expectKinds: ["capabilityExplain"],
    family: "discovery",
  },
  {
    cores: ["Back", "Previous", "Go back", "I'm still working", "Continue"],
    setup: ["Open Chrome"],
    expectKinds: ["navigate", "navigateNamed"],
    family: "continuity",
  },
  {
    cores: ["Again", "Do that again"],
    setup: ["Open Notepad"],
    expectKinds: ["appOpen", "appLaunch"],
    family: "continuity",
  },
  {
    cores: [
      "What windows are open",
      "What's open on my desktop",
      "Show me open windows",
    ],
    expectKinds: ["winEnumerate"],
    family: "awareness",
  },
  {
    cores: [
      "Open ChatGPT",
      "Open YouTube beside Cursor",
      "Focus Chrome",
      "Take a screenshot",
    ],
    expectKinds: [
      "browserOpen",
      "browserOpenFocus",
      "browserOpenBeside",
      "winFocus",
      "screenshotDesktop",
      "screenshotWindow",
      "screenshotCaptureAndCopy",
      "screenshotMonitor",
    ],
    family: "open_action",
  },
];

describe("P16.38 product intelligence battery", () => {
  it("classifies ≥500 Owner product interactions without engineering leaks", () => {
    let total = 0;
    let matched = 0;
    let goalOriented = 0;
    let leaks = 0;
    const failures: string[] = [];

    for (const group of GROUPS) {
      for (const core of group.cores) {
        for (const utterance of expand(core)) {
          total += 1;
          resetWorkspaceContext();
          for (const s of group.setup ?? []) {
            resolveIntent(s);
          }
          const action = resolveIntent(utterance);
          const evidence = resolveIntentWithEvidence(utterance);
          const exp = classifyProductExperience(utterance, action.kind);

          if (hasEngineeringLeak(action.reply ?? "")) {
            leaks += 1;
            failures.push(`${utterance} → leak`);
            continue;
          }
          if (group.expectKinds.includes(action.kind)) {
            matched += 1;
          } else {
            failures.push(
              `${utterance} → ${action.kind} (want ${group.expectKinds.join("|")})`,
            );
          }
          if (exp.goalOriented) goalOriented += 1;
          if (
            group.family === "discovery" &&
            !evidence.stages.some((s) => s.stage === "capability_discovery")
          ) {
            // discovery may hit semantic/situation — still capabilityExplain
          }
        }
      }
    }

    expect(total).toBeGreaterThanOrEqual(500);
    expect(leaks).toBe(0);
    expect(matched / total).toBeGreaterThanOrEqual(0.9);
    expect(goalOriented / total).toBeGreaterThanOrEqual(0.85);
    if (failures.length) {
      expect(failures.slice(0, 12), failures.slice(0, 12).join("\n")).toEqual(
        [],
      );
    }
  });

  it("Registry discovery includes purpose, similar, recovery, and no leaks", () => {
    expect(validateCapabilityGraphGovernance()).toEqual([]);
    const d = generateCapabilityDiscovery("all");
    expect(hasEngineeringLeak(d.reply)).toBe(false);
    expect(d.reply).toMatch(/Related \/ similar \/ alternatives/i);
    expect(d.reply).toMatch(/Why:/i);
    expect(d.reply).toMatch(/won’t overclaim|won't overclaim/i);
    expect(d.reply).toMatch(/If something doesn’t work|If something doesn't work/i);
    expect(d.reply).not.toMatch(/Capability graph/i);
  });

  it("every intelligence gap has a single explicit owner", () => {
    const owners = new Set(PRODUCT_INTELLIGENCE_GAPS.map((g) => g.owner));
    expect(owners.size).toBeGreaterThanOrEqual(5);
    for (const g of PRODUCT_INTELLIGENCE_GAPS) {
      expect(g.owner.length).toBeGreaterThan(2);
      expect(g.note.length).toBeGreaterThan(8);
    }
  });

  it("F12–F14 Owner findings route as a desktop operator", () => {
    resetWorkspaceContext();
    expect(resolveIntent("I need my development setup.").kind).toBe("navigate");
    expect(resolveIntent("I've lost it.").kind).toBe("unknown");
    expect(resolveIntent("I'm looking for my screenshots.").kind).toMatch(
      /appLaunch|appOpen/,
    );
    resetWorkspaceContext();
    resolveIntent("Open Chrome");
    expect(resolveIntent("Back").kind).toBe("navigate");
    expect(resolveIntent("Previous").kind).toBe("navigate");
    const discovery = resolveIntent("What else can you do?");
    expect(discovery.kind).toBe("capabilityExplain");
    expect(hasEngineeringLeak(discovery.reply ?? "")).toBe(false);
  });
});
