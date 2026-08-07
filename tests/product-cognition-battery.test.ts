/**
 * P16.35 — Product Cognition hostile battery (≥250 natural requests).
 * Classifies supported / partially_supported / unsupported / misunderstood.
 * Does not expand Intent aliases — measures cognition quality.
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { resolveIntentWithEvidence } from "../app/src/lib/intentPipeline";
import {
  assessCognition,
  type CognitionExpectation,
  type CognitionOutcome,
} from "../app/src/lib/productCognition";
import {
  CAPABILITY_GRAPH,
  describeCapability,
} from "../app/src/lib/capabilityRegistry";

const LEAD = ["", "please ", "can you ", "could you ", "would you "];
const TAIL = ["", " please", " for me", " now"];

function expand(core: string): string[] {
  const out: string[] = [];
  for (const l of LEAD) {
    for (const t of TAIL) {
      const s = `${l}${core}${t}`.replace(/\s+/g, " ").trim();
      out.push(s);
      out.push(`${s}.`);
    }
  }
  return [...new Set(out)];
}

const CASES: Array<{ cores: string[]; expectation: CognitionExpectation }> = [
  {
    cores: [
      "I've got Chrome somewhere",
      "I've got ChatGPT somewhere",
      "I was just using Chrome",
      "I was just using ChatGPT",
      "Where is Cursor",
      "Bring back my browser",
    ],
    expectation: { family: "locate", kinds: ["winFocus"] },
  },
  {
    cores: [
      "I need YouTube beside ChatGPT",
      "I want YouTube next to Cursor",
      "Open ChatGPT beside Cursor",
      "I need ChatGPT next to Cursor",
    ],
    expectation: { family: "beside", kinds: ["browserOpenBeside"] },
  },
  {
    cores: [
      "Show me everything you can do",
      "Show me everything you can control",
      "Show me everything you know how to control",
      "What can you do",
      "How can you help me",
    ],
    expectation: { family: "discovery", kinds: ["capabilityExplain"] },
  },
  {
    cores: [
      "Take me back",
      "Take me where I was",
      "I need to continue where I left off",
      "I want my development setup",
      "I want my coding environment",
      "I need my work setup",
      "I was working on something",
      "I need everything ready",
    ],
    expectation: { family: "resume", kinds: ["navigate"] },
  },
  {
    cores: [
      "Find my screenshots",
      "I'm looking for yesterday's screenshots",
      "Show me the folder with screenshots",
      "Locate my screenshots",
    ],
    expectation: {
      family: "screenshots_folder",
      kinds: ["appLaunch"],
    },
  },
  {
    cores: ["I'm done with this", "I'm done", "I'm finished with this"],
    expectation: { family: "end_session", kinds: ["unknown"] },
  },
  {
    cores: ["Take me to YouTube", "Open ChatGPT", "Open GitHub"],
    expectation: {
      family: "open_site",
      kinds: ["browserOpen", "browserOpenFocus"],
    },
  },
  {
    cores: ["Open Notepad", "Launch Cursor", "Open Microsoft Store"],
    expectation: {
      family: "open_app",
      kinds: ["appOpen", "appLaunch", "appOpenMaximize"],
    },
  },
  {
    cores: [
      "Maximise Cursor",
      "Restore Cursor",
      "Focus Chrome",
      "Put Chrome on the other monitor",
    ],
    expectation: {
      family: "window_ops",
      kinds: [
        "winMaximize",
        "winRestore",
        "winFocus",
        "winMoveMonitor",
        "winMinimize",
      ],
    },
  },
  {
    cores: ["What windows are open", "Which windows are open"],
    expectation: { family: "enumerate", kinds: ["winEnumerate"] },
  },
];

describe("P16.35 product cognition battery", () => {
  it("classifies ≥250 natural desktop requests without inventing executables", () => {
    const tallies: Record<CognitionOutcome, number> = {
      supported: 0,
      partially_supported: 0,
      unsupported: 0,
      misunderstood: 0,
    };
    const failures: string[] = [];
    let total = 0;

    for (const group of CASES) {
      for (const core of group.cores) {
        for (const utterance of expand(core)) {
          total += 1;
          const evidence = resolveIntentWithEvidence(utterance);
          const assessment = assessCognition(
            utterance,
            evidence.action,
            group.expectation,
            evidence.plan,
          );
          tallies[assessment.outcome] += 1;
          if (assessment.outcome === "misunderstood") {
            failures.push(
              `${utterance} → ${assessment.kind} (${assessment.reason})`,
            );
          }
        }
      }
    }

    expect(total).toBeGreaterThanOrEqual(250);
    // Misunderstandings (wrong routing / invent) must stay rare.
    expect(tallies.misunderstood / total).toBeLessThan(0.05);
    // Supported + honest partial should dominate Owner situation families.
    expect(
      (tallies.supported + tallies.partially_supported) / total,
    ).toBeGreaterThanOrEqual(0.85);
    if (failures.length > 0 && tallies.misunderstood / total >= 0.05) {
      expect(failures.slice(0, 15)).toEqual([]);
    }
  });

  it("routes Owner situation goals without proposal misclassification", () => {
    for (const utterance of [
      "Show me everything you know how to control",
      "I'm looking for yesterday's screenshots",
      "I want my development setup",
      "I was working on something",
      "I'm done with this",
      "I need to continue where I left off",
    ]) {
      const action = resolveIntent(utterance);
      expect(action.kind, utterance).not.toBe("proposal");
      expect(action.reply).not.toMatch(/Provider|Registry|Kernel/i);
    }
  });

  it("requires Capability Graph discoverability / similar / alternatives metadata", () => {
    for (const node of CAPABILITY_GRAPH) {
      expect(node.discoverability.length, node.id).toBeGreaterThan(8);
      expect(Array.isArray(node.similar), node.id).toBe(true);
      expect(Array.isArray(node.alternatives), node.id).toBe(true);
      const d = describeCapability(node.id)!;
      expect(d).toMatch(/Purpose:|Discover:|Arguments:/);
    }
  });
});
