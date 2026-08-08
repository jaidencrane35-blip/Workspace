/**
 * P23.S5 — Active Window Answer.
 *
 * Proves that a second semantic observation need can be answered through the
 * same architecture: the Owner asks which window they are in, the comprehended
 * need reaches the existing authorized active-window observation, and the
 * observation itself becomes the answer. Nothing about the answer is written
 * into the source, and asking a question still changes nothing on the desktop.
 */
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { beforeEach, describe, expect, it, vi } from "vitest";

/**
 * Proof boundary, unchanged from P23.S4: `@tauri-apps/api` resolves outside
 * this workspace package, so the stand-in sits one layer in, at the single IPC
 * entry. Everything above it is the real product path — intent resolution,
 * grounding, the Conversation façade, the Capability Runtime bridge, payload
 * mapping, and answer composition. Below it is Rust.
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
import {
  activeWindowAnswerSource,
  openWindowsAnswerSource,
} from "../app/src/lib/observationAnswerSource";
import { handleOperatorUtterance } from "../app/src/lib/operator/intelligence";
import { toCapabilityIntent } from "../app/src/lib/operator";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const ACTIVE_WINDOW_QUESTIONS = [
  "Which window is active?",
  "What's the active window?",
  "What window am I using?",
  "Which application am I using?",
];

/**
 * One synthetic desktop, observed two ways — exactly as the Kernel composes
 * each observation. Both answers must be derived from this and nothing else,
 * so changing the desktop here is the only way to change what Workspace says.
 */
function desktopIs(titles: string[], activeIndex = 0): void {
  invokeIpc.mockReset();
  invokeIpc.mockImplementation(async (_command: string, payload: unknown) => {
    const operation = (payload as { intent?: { operation?: string } })?.intent
      ?.operation;
    const items = titles.map((title, index) => ({
      hwnd: `0x${index}`,
      title,
      processId: 1000 + index,
      minimized: false,
      focused: index === activeIndex,
      x: 0,
      y: 0,
      width: 800,
      height: 600,
      monitorIndex: 0,
    }));

    if (operation === "active") {
      const active = items[activeIndex];
      return active
        ? {
            ok: true,
            message: `Active window: “${active.title}”.`,
            status: "active",
            domain: "window",
            operation: "active",
            target: active.title,
            items: [active],
          }
        : {
            ok: false,
            message: "No active window.",
            status: "not_found",
            domain: "window",
            operation: "active",
            items: null,
          };
    }

    return {
      ok: true,
      message: `Open windows (${items.length}).`,
      status: "enumerated",
      domain: "window",
      operation: "enumerate",
      items,
    };
  });
}

/** Every IPC command this turn reached, in order. */
function invokedCommands(): string[] {
  return invokeIpc.mock.calls.map((call) => call[0] as string);
}

function sentIntents(): Array<Record<string, unknown>> {
  return invokeIpc.mock.calls.map(
    (call) => (call[1] as { intent: Record<string, unknown> }).intent,
  );
}

async function replyTo(utterance: string): Promise<string> {
  const outcome = await handleOperatorUtterance(utterance);
  return outcome.kind === "reply" ? outcome.text : `«${outcome.kind}»`;
}

beforeEach(() => {
  resetWorkspaceContext();
  desktopIs(["Quokka Ledger", "Zarnak Notes", "Verdigris Atlas"]);
});

describe("P23.S5 the active window becomes an answer", () => {
  it("1-2. answers which window is active from the active-window observation", async () => {
    for (const utterance of ["Which window is active?", "What's the active window?"]) {
      resetWorkspaceContext();
      desktopIs(["Quokka Ledger", "Zarnak Notes"], 1);

      const { goal, action } = resolveIntentWithGoal(utterance);
      expect(goal.outcome).toBe("PERCEIVE_MACHINE");
      expect(goal.mode).toBe("observation");
      expect(observationNeededFor(goal)).toBe("active-window");

      // The comprehended need reaches the existing authorized observation.
      expect(action.kind).toBe("winActive");
      expect(toCapabilityIntent(action)).toEqual({
        domain: "window",
        operation: "active",
      });

      resetWorkspaceContext();
      expect(await replyTo(utterance)).toBe(
        "The active window is “Zarnak Notes”.",
      );
    }
  });

  it("3. answers the same question asked from the Owner's side", async () => {
    desktopIs(["Quokka Ledger", "Zarnak Notes"], 1);
    expect(await replyTo("What window am I using?")).toBe(
      "You’re using “Zarnak Notes” right now.",
    );
  });

  it("4. will not name an application this observation did not report", async () => {
    // This desktop reports titles and no owning application, which Windows
    // genuinely does for protected processes. P23.S6 gave the observation a
    // place to carry identity; it must stay empty rather than be filled in.
    desktopIs(["Verdigris Atlas"]);
    const text = await replyTo("Which application am I using?");

    expect(text).toContain("Verdigris Atlas");
    expect(text).toMatch(/didn’t tell me which application/);
    // The title is not quietly promoted into an application name.
    expect(text).not.toMatch(/using Verdigris Atlas|application is Verdigris/);
  });

  it("names the window the observation reports as focused, not the first one", async () => {
    desktopIs(["Quokka Ledger", "Zarnak Notes", "Verdigris Atlas"], 2);
    expect(await replyTo("Which window is active?")).toBe(
      "The active window is “Verdigris Atlas”.",
    );
  });

  it("7. changes its answer when the observation changes", async () => {
    desktopIs(["Quokka Ledger"]);
    const first = await replyTo("Which window is active?");

    resetWorkspaceContext();
    desktopIs(["Zarnak Notes"]);
    const second = await replyTo("Which window is active?");

    expect(first).toBe("The active window is “Quokka Ledger”.");
    expect(second).toBe("The active window is “Zarnak Notes”.");
    expect(first).not.toBe(second);
  });

  it("8. speaks a title that exists nowhere in the repository", async () => {
    desktopIs(["Thorn & Marrow — draft 14"]);
    const text = await replyTo("What's the active window?");
    expect(text).toContain("Thorn & Marrow — draft 14");

    const source = readFileSync(
      path.join(root, "app/src/lib/observationAnswerSource.ts"),
      "utf8",
    );
    expect(source).not.toMatch(/thorn|marrow|quokka|zarnak|verdigris/i);
    expect(source).not.toMatch(/chrome|cursor|explorer|notepad|firefox|edge/i);
  });

  it("reports an unobservable active window truthfully", async () => {
    desktopIs([]);
    const text = await replyTo("Which window is active?");
    expect(text).toBe("No active window.");
    expect(text).not.toMatch(/The active window is/);
  });

  it("composes nothing from an empty, unreported, or refused observation", () => {
    const goal = comprehend("Which window is active?");
    expect(composeObservationAnswer(goal, { ok: true, items: [] })).toBeNull();
    expect(composeObservationAnswer(goal, { ok: true, items: null })).toBeNull();
    expect(
      composeObservationAnswer(goal, {
        ok: false,
        items: [{ title: "Quokka Ledger", focused: true }],
      }),
    ).toBeNull();
  });
});

describe("P23.S5 “which one?” is grounded, never guessed", () => {
  it("5. answers the follow-up once the conversation is about windows", async () => {
    desktopIs(["Quokka Ledger", "Zarnak Notes", "Verdigris Atlas"], 1);

    const opening = await replyTo("What windows are open?");
    expect(opening).toContain("Quokka Ledger");

    const { goal, action } = resolveIntentWithGoal("Which one am I using?");
    expect(goal.outcome).toBe("PERCEIVE_MACHINE");
    expect(goal.evidence).toContain(
      "grounded_reference=prior_desktop_observation",
    );
    expect(action.kind).toBe("winActive");

    resetWorkspaceContext();
    desktopIs(["Quokka Ledger", "Zarnak Notes", "Verdigris Atlas"], 1);
    await handleOperatorUtterance("What windows are open?");
    expect(await replyTo("Which one am I using?")).toBe(
      "You’re using “Zarnak Notes” right now.",
    );
  });

  it("6. invents nothing when there is nothing to ground against", async () => {
    for (const utterance of ["Which one?", "Which one am I using?"]) {
      resetWorkspaceContext();
      desktopIs(["Quokka Ledger"]);

      const { goal, action } = resolveIntentWithGoal(utterance);
      expect(goal.outcome).toBe("KNOW");
      expect(observationNeededFor(goal)).toBeNull();
      expect(action.kind).toBe("unknown");

      resetWorkspaceContext();
      desktopIs(["Quokka Ledger"]);
      const text = await replyTo(utterance);
      expect(text).not.toContain("Quokka Ledger");
      expect(invokedCommands()).toEqual([]);
    }
  });

  it("keeps a question, an active-window question, and an effect distinct", () => {
    // Grounding may refine meaning into perception — never into an effect.
    resetWorkspaceContext();
    desktopIs(["Quokka Ledger"]);
    resolveIntentWithGoal("What windows are open?");

    const followUp = resolveIntentWithGoal("Which one am I using?");
    expect(followUp.action.kind).toBe("winActive");

    const asked = resolveIntentWithGoal("Which window is active?");
    expect(asked.action.kind).toBe("winActive");

    const effect = resolveIntentWithGoal("Open that one.");
    expect(effect.goal.outcome).toBe("REACH_STATE");
    expect(effect.action.kind).not.toBe("winActive");
    expect(effect.action.kind).not.toBe("winEnumerate");
  });
});

describe("P23.S5 nothing else happens", () => {
  it("9. causes no desktop effect beyond the observation itself", async () => {
    for (const utterance of ACTIVE_WINDOW_QUESTIONS) {
      resetWorkspaceContext();
      desktopIs(["Quokka Ledger"]);
      await handleOperatorUtterance(utterance);

      expect(invokedCommands()).toEqual(["execute_capability_intent"]);
      const [intent] = sentIntents();
      expect(intent).toMatchObject({ domain: "window", operation: "active" });
      // Never focus, open, minimise, maximise, arrange, or collapse.
      expect(intent.operation).not.toMatch(
        /focus|open|minimi|maximi|snap|close|arrange|restore/,
      );
      // Nothing is asked of any window either: no target, geometry, or text.
      const arguments_ = Object.entries(intent).filter(
        ([field]) => field !== "domain" && field !== "operation",
      );
      expect(arguments_.filter(([, value]) => value !== null)).toEqual([]);
    }
  });

  it("10. reaches no browser and no external model", async () => {
    for (const utterance of ACTIVE_WINDOW_QUESTIONS) {
      resetWorkspaceContext();
      desktopIs(["Quokka Ledger"]);
      const { action } = resolveIntentWithGoal(utterance);
      expect(action.kind).not.toBe("browserOpen");
      expect(action.kind).not.toBe("appOpen");
      expect(action.kind).not.toBe("collapse");

      resetWorkspaceContext();
      desktopIs(["Quokka Ledger"]);
      const text = (await replyTo(utterance)).toLowerCase();
      expect(text).not.toMatch(/chatgpt|http|browser/);
    }
  });

  it("13. leaves the open-window answer working", async () => {
    desktopIs(["Quokka Ledger", "Zarnak Notes"]);
    const { goal, action } = resolveIntentWithGoal("What windows are open?");
    expect(observationNeededFor(goal)).toBe("open-windows");
    expect(action.kind).toBe("winEnumerate");

    resetWorkspaceContext();
    desktopIs(["Quokka Ledger", "Zarnak Notes"]);
    expect(await replyTo("What windows are open?")).toBe(
      "You’ve got 2 windows open: Quokka Ledger and Zarnak Notes.",
    );
  });

  it("18. leaves existing desktop actions unchanged", () => {
    const unchanged: Array<[string, IntentAction["kind"]]> = [
      ["Open Notepad.", "appOpen"],
      ["Take a screenshot", "screenshotDesktop"],
      ["Minimize Chrome", "winMinimize"],
      ["Collapse", "collapse"],
    ];
    for (const [utterance, kind] of unchanged) {
      resetWorkspaceContext();
      expect(resolveIntent(utterance).kind).toBe(kind);
    }
  });
});

describe("P23.S5 authority boundary", () => {
  it("11. keeps the Goal Contract free of capability and provider identity", () => {
    resetWorkspaceContext();
    resolveIntentWithGoal("What windows are open?");
    const grounded = resolveIntentWithGoal("Which one am I using?").goal;

    for (const goal of [...ACTIVE_WINDOW_QUESTIONS.map(comprehend), grounded]) {
      const serialized = JSON.stringify(goal).toLowerCase();
      for (const token of [
        "provider",
        "capability",
        "winactive",
        "c-obs",
        "operation",
        "execute_capability_intent",
        "kernel",
      ]) {
        expect(serialized).not.toContain(token);
      }
    }
  });

  it("12. performs no IPC and names no capability in the answer-source layer", () => {
    for (const rel of [
      "app/src/lib/answerSource.ts",
      "app/src/lib/observationAnswerSource.ts",
    ]) {
      const source = readFileSync(path.join(root, rel), "utf8");
      expect(source).not.toMatch(/@tauri-apps|invokeIpc|invoke\(/);
      expect(source).not.toMatch(/execute_capability_intent|execute_window_operation/);
      expect(source).not.toMatch(/C-OBS-\d|winActive|winEnumerate/);
    }
  });

  it("has one source per semantic need, so there is nothing to select between", () => {
    const ladder = readFileSync(
      path.join(root, "app/src/lib/answerSource.ts"),
      "utf8",
    );
    const declared = /export type ObservationNeed =([^;]+);/
      .exec(ladder)![1]
      .split("|")
      .map((member) => member.trim().replace(/"/g, ""))
      .filter(Boolean);

    const registered = [openWindowsAnswerSource.need, activeWindowAnswerSource.need];
    expect(new Set(registered).size).toBe(registered.length);
    expect([...declared].sort()).toEqual([...registered].sort());
  });

  it("keeps the two observation sources mutually exclusive", () => {
    const corpus = [
      ...ACTIVE_WINDOW_QUESTIONS,
      "What windows are currently open?",
      "Which windows are open?",
      "What applications are open?",
      "What windows do I have open?",
      "Can you tell me what's open on my desktop?",
      "What's running right now?",
      "Which one?",
      "Open that one.",
      "Minimize Chrome",
      "What time is it?",
    ];
    for (const utterance of corpus) {
      const goal = comprehend(utterance);
      const covering = [openWindowsAnswerSource, activeWindowAnswerSource].filter(
        (source) => source.covers(goal),
      );
      expect(covering.length).toBeLessThanOrEqual(1);
    }
  });
});
