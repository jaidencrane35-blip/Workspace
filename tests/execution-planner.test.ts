/**
 * P16.34 — Execution planning before CapabilityIntent leaves Intent Layer.
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import {
  buildExecutionPlan,
  summarizeExecutionPlan,
} from "../app/src/lib/executionPlanner";
import { resolveIntentWithEvidence } from "../app/src/lib/intentPipeline";
import {
  CAPABILITY_GRAPH,
  describeCapability,
  generateCapabilityDiscovery,
} from "../app/src/lib/capabilityRegistry";

const HOSTILE: Array<{
  utterance: string;
  accept: (action: ReturnType<typeof resolveIntent>) => boolean;
  multiStep?: boolean;
}> = [
  {
    utterance: "I've got Chrome somewhere",
    accept: (a) => a.kind === "winFocus",
  },
  {
    utterance: "I was just using ChatGPT",
    accept: (a) => a.kind === "winFocus",
  },
  {
    utterance: "I need YouTube next to Cursor",
    accept: (a) =>
      a.kind === "browserOpenBeside" &&
      "beside" in a &&
      /cursor/i.test(a.beside),
    multiStep: true,
  },
  {
    utterance: "Open the browser I had before",
    accept: (a) => a.kind === "winFocus" && /history|find|browser/i.test(a.reply),
  },
  {
    utterance: "Find my screenshots",
    accept: (a) =>
      a.kind === "appLaunch" &&
      "query" in a &&
      /pictures/i.test(String(a.query)),
  },
  {
    utterance: "Show me everything you can do",
    accept: (a) =>
      a.kind === "capabilityExplain" &&
      /won’t overclaim|If something doesn’t work|Why:/i.test(a.reply),
  },
  {
    utterance: "Take me where I was",
    accept: (a) =>
      a.kind === "navigate" && "view" in a && a.view === "resume",
  },
  {
    utterance: "I need my work setup",
    accept: (a) =>
      a.kind === "navigate" && "view" in a && a.view === "resume",
  },
  {
    utterance: "Open ChatGPT beside Cursor",
    accept: (a) => a.kind === "browserOpenBeside",
    multiStep: true,
  },
];

describe("P16.34 execution planner + capability intelligence", () => {
  it("reasons hostile cognitive phrases without inventing executables", () => {
    const failures: string[] = [];
    for (const row of HOSTILE) {
      const action = resolveIntent(row.utterance);
      if (
        "query" in action &&
        typeof action.query === "string" &&
        /\.exe$/i.test(action.query)
      ) {
        failures.push(`${row.utterance} → exe ${action.query}`);
        continue;
      }
      if (/Provider|Registry|Kernel|WinRT/i.test(action.reply ?? "")) {
        failures.push(`${row.utterance} → internals`);
        continue;
      }
      if (!row.accept(action)) {
        failures.push(`${row.utterance} → ${action.kind}: ${action.reply}`);
      }
    }
    expect(failures).toEqual([]);
  });

  it("builds multi-step execution plans before beside composition", () => {
    const action = resolveIntent("Open ChatGPT beside Cursor.");
    const plan = buildExecutionPlan("Open ChatGPT beside Cursor.", action);
    expect(plan.multiStep).toBe(true);
    expect(plan.steps.length).toBeGreaterThanOrEqual(4);
    expect(plan.capabilities).toContain("browser");
    expect(plan.steps.some((s) => /locate/i.test(s.goal))).toBe(true);
    expect(plan.steps.some((s) => s.status === "delegated_to_kernel")).toBe(
      true,
    );
    expect(summarizeExecutionPlan(plan)).toMatch(/Locate|layout|Open/i);
  });

  it("records execution_plan stage in pipeline evidence", () => {
    const evidence = resolveIntentWithEvidence("I need YouTube next to Cursor");
    expect(evidence.plan.multiStep).toBe(true);
    expect(
      evidence.stages.some((s) => s.stage === "execution_plan" && s.hit),
    ).toBe(true);
    expect(evidence.action.kind).toBe("browserOpenBeside");
  });

  it("requires every Capability Registry node to be fully self-describing", () => {
    for (const node of CAPABILITY_GRAPH) {
      expect(node.arguments.length, node.id).toBeGreaterThan(0);
      expect(node.failureRecovery.length, node.id).toBeGreaterThan(10);
      expect(node.limitations.length, node.id).toBeGreaterThan(0);
      expect(node.examples.length, node.id).toBeGreaterThan(0);
      expect(node.requirements.length, node.id).toBeGreaterThan(0);
      const described = describeCapability(node.id);
      expect(described).toBeTruthy();
      expect(described!).toMatch(/Arguments:|Won’t:|If it fails:/);
      expect(described!).not.toMatch(/Provider|Kernel|WinRT/);
    }
    const discovery = generateCapabilityDiscovery("all");
    expect(discovery.reply).toMatch(/If something doesn’t work:/);
    expect(discovery.reply).toMatch(/Args —|Needs —/);
  });
});
