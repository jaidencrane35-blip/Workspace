/**
 * P23.S4 — Observation Answer Bridge.
 *
 * Proves that a question about the desktop's current state is answered from an
 * authorized Kernel observation: the comprehended need reaches the existing
 * window observation, the observation result becomes the answer, and every word
 * of that answer comes from the observation rather than from anything written
 * into the source.
 */
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { beforeEach, describe, expect, it, vi } from "vitest";

/**
 * Proof boundary. `@tauri-apps/api` resolves outside this workspace package and
 * cannot be replaced by `vi.mock`, so the stand-in sits one layer in, at the
 * single IPC entry. Everything above it is the real product path: intent
 * resolution, the Conversation façade, the Capability Runtime bridge and its
 * payload mapping, and answer composition. Below it — the native transport and
 * the Kernel's own window enumeration — is Rust, covered by Rust tests and by
 * Owner Product Proof.
 */
const { invokeIpc } = vi.hoisted(() => ({ invokeIpc: vi.fn() }));
vi.mock("../app/src/lib/ipc", () => ({
  invokeIpc,
  IpcCommandError: class extends Error {
    code = "mock";
  },
}));

import { comprehend } from "../app/src/lib/goalContract";
import {
  resolveIntent,
  resolveIntentWithGoal,
  type IntentAction,
} from "../app/src/lib/intentBridge";
import {
  composeObservationAnswer,
  observationNeededFor,
} from "../app/src/lib/answerSource";
import { openWindowsAnswerSource } from "../app/src/lib/observationAnswerSource";
import { handleOperatorUtterance } from "../app/src/lib/operator/intelligence";
import {
  isBannedProviderCommand,
  toCapabilityIntent,
} from "../app/src/lib/operator";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const OPEN_WINDOW_QUESTIONS = [
  "What windows are currently open?",
  "Which windows are open?",
  "What applications are open?",
  "What windows do I have open?",
  "Can you tell me what's open on my desktop?",
];

/** An authorized observation exactly as the Kernel composes it. */
function observationOf(titles: string[]) {
  return {
    ok: true,
    message: `Open windows (${titles.length}):\n${titles.map((t) => `• ${t}`).join("\n")}`,
    status: "enumerated",
    domain: "window",
    operation: "enumerate",
    items: titles.map((title, index) => ({
      hwnd: `0x${index}`,
      title,
      processId: 1000 + index,
      minimized: false,
      focused: index === 0,
      x: 0,
      y: 0,
      width: 800,
      height: 600,
      monitorIndex: 0,
    })),
  };
}

function respondWith(titles: string[]) {
  invokeIpc.mockReset();
  invokeIpc.mockResolvedValue(observationOf(titles));
}

/** Every IPC command this turn reached, in order. */
function invokedCommands(): string[] {
  return invokeIpc.mock.calls.map((call) => call[0] as string);
}

beforeEach(() => {
  resetWorkspaceContext();
  respondWith(["Chrome", "Cursor", "File Explorer"]);
});

describe("P23.S4 desktop questions become observations", () => {
  it("1-4. answers every supported phrasing from the window observation", async () => {
    for (const utterance of OPEN_WINDOW_QUESTIONS) {
      resetWorkspaceContext();
      respondWith(["Chrome", "Cursor", "File Explorer"]);

      const { goal, action } = resolveIntentWithGoal(utterance);
      expect(goal.outcome).toBe("PERCEIVE_MACHINE");
      expect(goal.mode).toBe("observation");
      expect(observationNeededFor(goal)).toBe("open-windows");

      // The comprehended need reaches the existing authorized observation.
      expect(action.kind).toBe("winEnumerate");
      expect(toCapabilityIntent(action)).toEqual({
        domain: "window",
        operation: "enumerate",
      });

      resetWorkspaceContext();
      const outcome = await handleOperatorUtterance(utterance);
      expect(outcome.kind).toBe("reply");
      const text = outcome.kind === "reply" ? outcome.text : "";
      expect(text).toBe(
        "You’ve got 3 windows open: Chrome, Cursor, and File Explorer.",
      );
    }
  });

  it("5. leaves the active-window question to its existing support", () => {
    const goal = comprehend("Which window is active?");
    expect(openWindowsAnswerSource.covers(goal)).toBe(false);
    expect(observationNeededFor(goal)).toBeNull();

    // Unchanged: it was already served, by a different observation.
    const action = resolveIntent("Which window is active?");
    expect(action.kind).toBe("winActive");
    expect(toCapabilityIntent(action)).toEqual({
      domain: "window",
      operation: "active",
    });
  });

  it("6. reflects the windows actually observed, not a written-in list", async () => {
    // Titles that appear nowhere in the repository: the answer can only contain
    // them if it was derived from the observation result.
    const observed = ["Quokka Ledger", "Zarnak Notes", "Verdigris Atlas"];
    respondWith(observed);

    const outcome = await handleOperatorUtterance("What windows are open?");
    const text = outcome.kind === "reply" ? outcome.text : "";
    for (const title of observed) {
      expect(text).toContain(title);
    }
    expect(text).not.toMatch(/Chrome|Cursor|Explorer/);

    // And the source itself names no application.
    const source = readFileSync(
      path.join(root, "app/src/lib/observationAnswerSource.ts"),
      "utf8",
    );
    expect(source).not.toMatch(/chrome|cursor|explorer|notepad|firefox|edge/i);
  });

  it("7. changes its answer when the observation changes", async () => {
    respondWith(["Solitaire"]);
    const first = await handleOperatorUtterance("What windows are open?");

    resetWorkspaceContext();
    respondWith(["Solitaire", "Calculator"]);
    const second = await handleOperatorUtterance("What windows are open?");

    const firstText = first.kind === "reply" ? first.text : "";
    const secondText = second.kind === "reply" ? second.text : "";
    expect(firstText).toBe("You’ve got one window open: Solitaire.");
    expect(secondText).toBe(
      "You’ve got 2 windows open: Solitaire and Calculator.",
    );
    expect(firstText).not.toBe(secondText);
  });

  it("summarises a crowded desktop instead of listing everything", async () => {
    respondWith(["A", "B", "C", "D", "E", "F", "G", "H"]);
    const outcome = await handleOperatorUtterance("What windows are open?");
    const text = outcome.kind === "reply" ? outcome.text : "";
    expect(text).toBe("You’ve got 8 windows open: A, B, C, D, E, F, and 2 more.");
  });
});

describe("P23.S4 nothing else happens", () => {
  it("8. causes no desktop effect beyond the observation itself", async () => {
    for (const utterance of OPEN_WINDOW_QUESTIONS) {
      resetWorkspaceContext();
      respondWith(["Chrome"]);
      await handleOperatorUtterance(utterance);

      expect(invokedCommands()).toEqual(["execute_capability_intent"]);
      const sent = invokeIpc.mock.calls[0][1] as {
        intent: { domain: string; operation: string };
      };
      expect(sent.intent).toMatchObject({
        domain: "window",
        operation: "enumerate",
      });
      // Never focus, open, minimise, arrange, or collapse.
      expect(sent.intent.operation).not.toMatch(
        /focus|open|minimi|maximi|snap|close|arrange/,
      );
    }
  });

  it("9. reaches no browser and no external model", async () => {
    for (const utterance of OPEN_WINDOW_QUESTIONS) {
      resetWorkspaceContext();
      respondWith(["Chrome"]);
      const { action } = resolveIntentWithGoal(utterance);
      expect(action.kind).not.toBe("browserOpen");
      expect(action.kind).not.toBe("appOpen");
      expect(action.kind).not.toBe("collapse");

      const outcome = await handleOperatorUtterance(utterance);
      const text = outcome.kind === "reply" ? outcome.text.toLowerCase() : "";
      expect(text).not.toMatch(/chatgpt|http|browser/);
    }
  });

  it("12. reports a refused observation truthfully and invents nothing", async () => {
    invokeIpc.mockReset();
    invokeIpc.mockResolvedValue({
      ok: false,
      message: "I can’t do that without permission.",
      status: "failed",
      domain: "window",
      operation: "enumerate",
      items: null,
    });

    const outcome = await handleOperatorUtterance("What windows are open?");
    const text = outcome.kind === "reply" ? outcome.text : "";
    expect(text).toBe("I can’t do that without permission.");
    expect(text).not.toMatch(/You’ve got/);
  });

  it("composes nothing from an empty or unreported observation", () => {
    const goal = comprehend("What windows are open?");
    expect(composeObservationAnswer(goal, { ok: true, items: [] })).toBeNull();
    expect(composeObservationAnswer(goal, { ok: true, items: null })).toBeNull();
    expect(
      composeObservationAnswer(goal, {
        ok: false,
        items: [{ title: "Chrome" }],
      }),
    ).toBeNull();
  });
});

describe("P23.S4 authority boundary", () => {
  const LADDER_FILES = [
    "app/src/lib/answerSource.ts",
    "app/src/lib/observationAnswerSource.ts",
  ];

  it("10. keeps the Goal Contract free of capability and provider identity", () => {
    for (const utterance of OPEN_WINDOW_QUESTIONS) {
      const serialized = JSON.stringify(comprehend(utterance)).toLowerCase();
      for (const token of [
        "provider",
        "capability",
        "winenumerate",
        "enumerate",
        "execute_capability_intent",
        "kernel",
      ]) {
        expect(serialized).not.toContain(token);
      }
    }
  });

  it("11. performs no IPC from the answer-source layer", () => {
    for (const rel of LADDER_FILES) {
      const source = readFileSync(path.join(root, rel), "utf8");
      expect(source).not.toMatch(/@tauri-apps|invokeIpc|invoke\(/);
      expect(source).not.toMatch(/execute_capability_intent|execute_window_operation/);
    }
  });

  it("13. leaves the Kernel as the only observer", () => {
    // Provider-specific window IPC remains banned from the Conversation façade.
    expect(isBannedProviderCommand("execute_window_operation")).toBe(true);

    const facade = readFileSync(
      path.join(root, "app/src/lib/operator/intelligence.ts"),
      "utf8",
    );
    expect(facade).toContain("executeCapabilityIntent");
    expect(facade).not.toMatch(/execute_window_operation|enumerate/);

    // The bridge asks for one observation and cannot ask for anything else.
    const contract = readFileSync(
      path.join(root, "app/src/lib/answerSource.ts"),
      "utf8",
    );
    expect(contract).toMatch(/ObservationNeed\s*=\s*"open-windows";/);
  });

  it("18. never overrides an action the Owner actually requested", () => {
    const unchanged: Array<[string, IntentAction["kind"]]> = [
      ["Open Notepad.", "appOpen"],
      ["Take a screenshot", "screenshotDesktop"],
      ["Which window is active?", "winActive"],
      ["Collapse", "collapse"],
    ];
    for (const [utterance, kind] of unchanged) {
      resetWorkspaceContext();
      expect(resolveIntent(utterance).kind).toBe(kind);
    }

    // An explicit effect on a window stays an effect, never an observation.
    resetWorkspaceContext();
    const minimize = resolveIntent("Minimize Chrome");
    expect(minimize.kind).not.toBe("winEnumerate");
  });
});
