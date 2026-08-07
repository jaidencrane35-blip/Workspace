/**
 * P16.37 — Hostile per-layer validation.
 * Each cognitive layer must survive independently with measurable pass rates.
 */
import { describe, expect, it } from "vitest";
import { parseDesktopIntent } from "../app/src/lib/intentGrammar";
import { resolveSituationGoal } from "../app/src/lib/situationGoals";
import {
  applyGoalResolution,
  resolveGoal,
} from "../app/src/lib/goalResolution";
import {
  resolveFromWorkspaceContext,
  resetWorkspaceContext,
  commitWorkspaceContext,
} from "../app/src/lib/workspaceContext";
import { buildExecutionPlan } from "../app/src/lib/executionPlanner";
import {
  CAPABILITY_GRAPH,
  generateRecoveryGuidance,
  validateCapabilityGraphGovernance,
} from "../app/src/lib/capabilityRegistry";
import {
  resolveIntent,
  resolveIntentBeforeGoalResolution,
} from "../app/src/lib/intentBridge";
import { resolveIntentWithEvidence } from "../app/src/lib/intentPipeline";

function rate(pass: number, total: number): number {
  return total === 0 ? 0 : pass / total;
}

describe("P16.37 hostile cognitive layers", () => {
  it("Grammar layer — structured intents without inventing", () => {
    const cases: Array<{ u: string; expectHit: boolean }> = [
      { u: "Open Chrome", expectHit: true },
      { u: "Focus Cursor", expectHit: true },
      { u: "Snap Chrome left", expectHit: true },
      { u: "Maximize Cursor", expectHit: true },
      { u: "asdfghjkl zxcvbnm", expectHit: false },
      { u: "!!@@##", expectHit: false },
      { u: "Do that again", expectHit: false },
    ];
    let pass = 0;
    for (const c of cases) {
      const g = parseDesktopIntent(c.u);
      const ok =
        c.expectHit === Boolean(g) &&
        !/\.exe/i.test(JSON.stringify(g ?? {}));
      if (ok) pass += 1;
    }
    expect(rate(pass, cases.length)).toBeGreaterThanOrEqual(0.85);
  });

  it("Situation Goals layer — high-level only, no pronoun ownership", () => {
    const cases = [
      { u: "Continue where I left off", expectHit: true },
      { u: "Open Chrome", expectHit: false },
      { u: "Close that", expectHit: false },
      { u: "Do that again", expectHit: false },
    ];
    let pass = 0;
    for (const c of cases) {
      const hit = Boolean(resolveSituationGoal(c.u));
      if (hit === c.expectHit) pass += 1;
    }
    expect(rate(pass, cases.length)).toBe(1);
  });

  it("Goal Resolution layer — clarify unbound locate; keep concrete", () => {
    const clarify = resolveGoal(
      "Find it",
      resolveIntentBeforeGoalResolution("Find it"),
    );
    expect(clarify.needsClarification).toBe(true);

    const concrete = applyGoalResolution("Find it", {
      kind: "winFocus",
      query: "Google Chrome",
      reply: "Looking for Chrome.",
    });
    expect(concrete.kind).toBe("winFocus");
    if (concrete.kind === "winFocus") {
      expect(concrete.query).toBe("Google Chrome");
    }
  });

  it("Workspace Context layer — continuity without prior state clarifies", () => {
    resetWorkspaceContext();
    const empty = resolveFromWorkspaceContext("Close that");
    expect(empty?.action.kind).toBe("unknown");

    commitWorkspaceContext("Open Chrome", {
      kind: "appOpen",
      query: "Google Chrome",
      reply: "Opening Chrome.",
    });
    const close = resolveFromWorkspaceContext("Close that");
    expect(close?.action.kind).toBe("appClose");
  });

  it("Execution Planning layer — plans never invent capabilities", () => {
    const ids = new Set(CAPABILITY_GRAPH.map((n) => n.id));
    const actions = [
      resolveIntent("Open ChatGPT beside Cursor"),
      resolveIntent("Open Notepad"),
      resolveIntent("Take a screenshot"),
    ];
    let pass = 0;
    for (const action of actions) {
      const plan = buildExecutionPlan("plan", action);
      const ok = plan.capabilities.every((c) => ids.has(c));
      if (ok && plan.steps.length > 0) pass += 1;
    }
    expect(rate(pass, actions.length)).toBe(1);
  });

  it("Capability Registry governance — every node complete", () => {
    const errors = validateCapabilityGraphGovernance();
    expect(errors).toEqual([]);
  });

  it("Recovery layer — guidance never claims invented success", () => {
    const g = generateRecoveryGuidance("teleport to mars");
    expect(g.reply).toMatch(/won’t invent|can't do that|can’t do that/i);
    expect(g.reply).not.toMatch(/successfully teleported/i);
  });

  it("Conversation layer — no Provider/Registry jargon in replies", () => {
    const samples = [
      "Open Chrome",
      "Find it",
      "What can you do",
      "Do that again",
      "I'm still working",
    ];
    let pass = 0;
    for (const u of samples) {
      resetWorkspaceContext();
      if (u === "Do that again" || u === "Find it") {
        resolveIntent("Open Chrome");
      }
      const action = resolveIntent(u);
      if (!/Provider|Registry|Kernel|WinRT|HRESULT/i.test(action.reply ?? "")) {
        pass += 1;
      }
    }
    expect(rate(pass, samples.length)).toBe(1);
  });

  it("Desktop Awareness layer — pipeline records context stage", () => {
    resetWorkspaceContext();
    resolveIntent("Open YouTube");
    const evidence = resolveIntentWithEvidence("Put it beside Cursor");
    const ctx = evidence.stages.find((s) => s.stage === "workspace_context");
    expect(ctx?.hit).toBe(true);
    expect(evidence.action.kind).toBe("browserOpenBeside");
  });
});
