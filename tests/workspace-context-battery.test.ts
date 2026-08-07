/**
 * P16.37 — Workspace Context hostile battery (≥500 contextual interactions).
 * Multi-turn continuity only — not alias / grammar / Goal Resolution expansion.
 */
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { resolveIntentWithEvidence } from "../app/src/lib/intentPipeline";
import {
  getWorkspaceContext,
  resetWorkspaceContext,
} from "../app/src/lib/workspaceContext";

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

type TurnExpect = {
  core: string;
  kinds: string[];
  contextHit?: boolean;
};

type Scenario = {
  id: string;
  setup: string[];
  turns: TurnExpect[];
};

const SCENARIOS: Scenario[] = [
  {
    id: "chrome-beside-again-close",
    setup: ["Open Chrome"],
    turns: [
      {
        core: "Put it beside Cursor",
        kinds: ["browserOpenBeside"],
        contextHit: true,
      },
      { core: "Do that again", kinds: ["browserOpenBeside"], contextHit: true },
      { core: "Close that", kinds: ["appClose"], contextHit: true },
    ],
  },
  {
    id: "chatgpt-other-find",
    setup: ["Open ChatGPT beside Cursor"],
    turns: [
      { core: "Open the other one", kinds: ["winFocus"], contextHit: true },
      { core: "Find it", kinds: ["winFocus"], contextHit: true },
      { core: "Focus that window", kinds: ["winFocus"], contextHit: true },
    ],
  },
  {
    id: "youtube-continuity",
    setup: ["Open YouTube"],
    turns: [
      {
        core: "Put it beside Cursor",
        kinds: ["browserOpenBeside"],
        contextHit: true,
      },
      {
        core: "I'm still working",
        kinds: ["navigate"],
        contextHit: true,
      },
      {
        core: "Bring everything back",
        kinds: ["navigate"],
        contextHit: true,
      },
      {
        core: "Continue what I was doing",
        kinds: ["navigate"],
        contextHit: true,
      },
    ],
  },
  {
    id: "notepad-again-go-back",
    setup: ["Open Notepad"],
    turns: [
      { core: "Do it again", kinds: ["appOpen", "appLaunch"], contextHit: true },
      { core: "Go back", kinds: ["navigate"], contextHit: true },
      { core: "Finish this", kinds: ["navigate"], contextHit: true },
    ],
  },
  {
    id: "two-apps-other",
    setup: ["Open Chrome", "Open Notepad"],
    turns: [
      { core: "Open the other one", kinds: ["winFocus"], contextHit: true },
      { core: "Close that", kinds: ["appClose"], contextHit: true },
      { core: "Again", kinds: ["appClose"], contextHit: true },
    ],
  },
  {
    id: "github-resume-family",
    setup: ["Open GitHub"],
    turns: [
      {
        core: "Place it beside Cursor",
        kinds: ["browserOpenBeside"],
        contextHit: true,
      },
      { core: "Resume what I was doing", kinds: ["navigate"], contextHit: true },
      { core: "Take me where I was", kinds: ["navigate"] },
    ],
  },
  {
    id: "empty-again-clarify",
    setup: [],
    turns: [
      { core: "Do that again", kinds: ["unknown"], contextHit: true },
      { core: "Close that", kinds: ["unknown"], contextHit: true },
      { core: "Open the other one", kinds: ["unknown"], contextHit: true },
      {
        core: "Put it beside Cursor",
        kinds: ["unknown"],
        contextHit: true,
      },
    ],
  },
  {
    id: "focus-then-that",
    setup: ["Focus Chrome"],
    turns: [
      { core: "Close it", kinds: ["appClose"], contextHit: true },
      { core: "Find it", kinds: ["unknown", "winFocus"] },
    ],
  },
  {
    id: "edge-move-continue",
    setup: ["Open Edge"],
    turns: [
      { core: "Bring that window", kinds: ["winFocus"], contextHit: true },
      { core: "Continue", kinds: ["navigate", "navigateNamed", "unknown"] },
      { core: "I'm done", kinds: ["unknown"] },
    ],
  },
  {
    id: "store-repeat",
    setup: ["Open Microsoft Store"],
    turns: [
      { core: "Once more", kinds: ["appOpen", "appLaunch"], contextHit: true },
      { core: "Repeat that", kinds: ["appOpen", "appLaunch"], contextHit: true },
      { core: "Dismiss that", kinds: ["appClose"], contextHit: true },
    ],
  },
];

describe("P16.37 Workspace Context battery", () => {
  it("resolves ≥500 contextual multi-turn interactions deterministically", () => {
    let total = 0;
    let matched = 0;
    let contextHits = 0;
    let clarifyOk = 0;
    let internals = 0;
    const failures: string[] = [];

    for (const scenario of SCENARIOS) {
      const turnVariants = scenario.turns.map((t) => ({
        ...t,
        variants: expand(t.core),
      }));
      const maxLen = Math.max(...turnVariants.map((t) => t.variants.length));

      for (let i = 0; i < maxLen; i++) {
        resetWorkspaceContext();
        for (const s of scenario.setup) {
          resolveIntent(s);
        }

        for (const turn of turnVariants) {
          const utterance = turn.variants[i % turn.variants.length]!;
          total += 1;
          const beforeTurn = getWorkspaceContext().turn;
          const evidence = resolveIntentWithEvidence(utterance);
          const action = evidence.action;
          const ctxStage = evidence.stages.find(
            (s) => s.stage === "workspace_context",
          );

          if (/Provider|Registry|Kernel|WinRT/i.test(action.reply ?? "")) {
            internals += 1;
            failures.push(`${scenario.id}: ${utterance} → internals`);
            continue;
          }

          if (turn.contextHit) {
            if (ctxStage?.hit) contextHits += 1;
            else {
              failures.push(
                `${scenario.id}: ${utterance} expected context hit (turn=${beforeTurn})`,
              );
            }
          }

          if (turn.kinds.includes(action.kind)) {
            matched += 1;
            if (action.kind === "unknown") clarifyOk += 1;
          } else {
            failures.push(
              `${scenario.id}: ${utterance} → ${action.kind} (want ${turn.kinds.join("|")})`,
            );
          }
        }
      }
    }

    const rate = matched / total;
    expect(total).toBeGreaterThanOrEqual(500);
    expect(internals).toBe(0);
    expect(rate).toBeGreaterThanOrEqual(0.9);
    expect(contextHits).toBeGreaterThan(100);
    expect(clarifyOk).toBeGreaterThan(0);
    if (failures.length) {
      expect(failures.slice(0, 12), failures.slice(0, 12).join("\n")).toEqual(
        [],
      );
    }
  });

  it("falsifies Goal Resolution alone on multi-turn pronouns", () => {
    resetWorkspaceContext();
    resolveIntent("Open Chrome");
    const beside = resolveIntent("Put it beside Cursor");
    expect(beside.kind).toBe("browserOpenBeside");
    expect(getWorkspaceContext().lastAppQuery || getWorkspaceContext().lastUrl).toBeTruthy();

    const again = resolveIntent("Do that again");
    expect(again.kind).toBe("browserOpenBeside");

    const close = resolveIntent("Close that");
    expect(close.kind).toBe("appClose");
    expect("query" in close && close.query).not.toMatch(/^(that|it)$/i);
  });
});
