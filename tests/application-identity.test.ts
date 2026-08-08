/**
 * P23.S6 — Observed Application Identity.
 *
 * Proves that the application behind a window reaches the Owner only when
 * Windows actually reported it. Every application name in an answer here is a
 * value carried by the observation; remove it from the observation and the
 * answer must stop claiming one. Window questions stay window questions.
 */
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { beforeEach, describe, expect, it, vi } from "vitest";

/** Proof boundary: the single IPC entry, as established in P23.S3–S5. */
const { invokeIpc } = vi.hoisted(() => ({ invokeIpc: vi.fn() }));
vi.mock("../app/src/lib/ipc", () => ({
  invokeIpc,
  IpcCommandError: class extends Error {
    code = "mock";
  },
}));

import { comprehend } from "../app/src/lib/goalContract";
import { resolveIntentWithGoal } from "../app/src/lib/intentBridge";
import { composeObservationAnswer } from "../app/src/lib/answerSource";
import { handleOperatorUtterance } from "../app/src/lib/operator/intelligence";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

/**
 * Synthetic desktop windows. Titles and applications are invented for this
 * file, so anything that appears in an answer was carried there by the
 * observation and could not have come from the repository.
 */
interface ObservedWindow {
  title: string;
  application?: string | null;
}

const QUOKKA: ObservedWindow = {
  title: "Quokka Ledger",
  application: "quokka-editor.exe",
};
const ZARNAK: ObservedWindow = {
  title: "Zarnak Notes",
  application: "zarnak-browser.exe",
};

/** One desktop, observed either way, exactly as the Kernel composes it. */
function desktopIs(windows: ObservedWindow[], activeIndex = 0): void {
  invokeIpc.mockReset();
  invokeIpc.mockImplementation(async (_command: string, payload: unknown) => {
    const operation = (payload as { intent?: { operation?: string } })?.intent
      ?.operation;
    const items = windows.map((window, index) => ({
      hwnd: `0x${index}`,
      title: window.title,
      processId: 1000 + index,
      processName: window.application ?? null,
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

async function replyTo(utterance: string): Promise<string> {
  const outcome = await handleOperatorUtterance(utterance);
  return outcome.kind === "reply" ? outcome.text : `«${outcome.kind}»`;
}

function invokedCommands(): string[] {
  return invokeIpc.mock.calls.map((call) => call[0] as string);
}

function sentIntents(): Array<Record<string, unknown>> {
  return invokeIpc.mock.calls.map(
    (call) => (call[1] as { intent: Record<string, unknown> }).intent,
  );
}

beforeEach(() => {
  resetWorkspaceContext();
  desktopIs([QUOKKA, ZARNAK]);
});

describe("P23.S6 the application comes from the observation", () => {
  it("1-2. answers with the application the observation supplied", async () => {
    desktopIs([QUOKKA]);
    expect(await replyTo("Which application am I using?")).toBe(
      "You’re using quokka-editor.exe — its window is “Quokka Ledger”.",
    );
  });

  it("9. answers 'what application is active?' with the observed application", async () => {
    desktopIs([QUOKKA]);
    expect(await replyTo("What application is active?")).toBe(
      "The active application is quokka-editor.exe — its window is “Quokka Ledger”.",
    );
  });

  it("4. changes the application answer when the observed application changes", async () => {
    desktopIs([{ title: "Quokka Ledger", application: "quokka-editor.exe" }]);
    const first = await replyTo("Which application am I using?");

    resetWorkspaceContext();
    desktopIs([{ title: "Quokka Ledger", application: "zarnak-browser.exe" }]);
    const second = await replyTo("Which application am I using?");

    expect(first).toContain("quokka-editor.exe");
    expect(second).toContain("zarnak-browser.exe");
    expect(first).not.toBe(second);
  });

  it("5. does not let the window title move the application answer", async () => {
    desktopIs([{ title: "Quokka Ledger", application: "quokka-editor.exe" }]);
    const first = await replyTo("Which application am I using?");

    resetWorkspaceContext();
    desktopIs([
      { title: "Something Else Entirely", application: "quokka-editor.exe" },
    ]);
    const second = await replyTo("Which application am I using?");

    // Same application, different window: the identity claim is unchanged.
    expect(first).toContain("quokka-editor.exe");
    expect(second).toContain("quokka-editor.exe");
    expect(second).toContain("Something Else Entirely");
  });

  it("13. reports the gap truthfully when Windows named no application", async () => {
    desktopIs([{ title: "Quokka Ledger", application: null }]);
    const text = await replyTo("Which application am I using?");

    expect(text).toContain("Quokka Ledger");
    expect(text).toMatch(/didn’t tell me which application/);
    expect(text).not.toMatch(/\.exe/);
    // The title is never promoted into an application claim.
    expect(text).not.toMatch(/using Quokka Ledger|application is Quokka/);
  });

  it("14. stops claiming an application the moment the observation stops carrying one", async () => {
    desktopIs([QUOKKA]);
    const named = await replyTo("Which application am I using?");

    resetWorkspaceContext();
    desktopIs([{ title: QUOKKA.title, application: null }]);
    const unnamed = await replyTo("Which application am I using?");

    expect(named).toContain("quokka-editor.exe");
    expect(unnamed).not.toContain("quokka-editor.exe");
    expect(unnamed).not.toMatch(/\.exe/);
  });
});

describe("P23.S6 windows and applications stay different questions", () => {
  it("7. answers the window question with the window", async () => {
    desktopIs([QUOKKA]);
    expect(await replyTo("What window am I using?")).toBe(
      "You’re using “Quokka Ledger” right now.",
    );
  });

  it("8. keeps 'which window is active?' window-oriented", async () => {
    desktopIs([QUOKKA]);
    const text = await replyTo("Which window is active?");
    expect(text).toBe("The active window is “Quokka Ledger”.");
    expect(text).not.toContain("quokka-editor.exe");
  });

  it("10-11. lists open windows by title, without repeating the application", async () => {
    desktopIs([
      { title: "Quokka Ledger", application: "quokka-editor.exe" },
      { title: "Quokka Drafts", application: "quokka-editor.exe" },
    ]);
    // Two windows of one application: titles already tell them apart.
    expect(await replyTo("What windows are open?")).toBe(
      "You’ve got 2 windows open: Quokka Ledger and Quokka Drafts.",
    );
  });

  it("12. distinguishes identical titles by their observed application", async () => {
    desktopIs([
      { title: "Untitled", application: "quokka-editor.exe" },
      { title: "Untitled", application: "zarnak-browser.exe" },
    ]);
    expect(await replyTo("What windows are open?")).toBe(
      "You’ve got 2 windows open: Untitled (quokka-editor.exe) and " +
        "Untitled (zarnak-browser.exe).",
    );
  });

  it("leaves identical titles alone when no application was observed", async () => {
    desktopIs([
      { title: "Untitled", application: null },
      { title: "Untitled", application: null },
    ]);
    const text = await replyTo("What windows are open?");
    expect(text).toBe("You’ve got 2 windows open: Untitled and Untitled.");
    expect(text).not.toMatch(/\(/);
  });

  it("6. resolves the grounded follow-up to the observed application", async () => {
    desktopIs([QUOKKA, ZARNAK], 1);
    await handleOperatorUtterance("What windows are open?");

    // Context says which observed thing is meant; the observation says what it is.
    expect(await replyTo("Which one am I using?")).toBe(
      "You’re using “Zarnak Notes” right now.",
    );
  });
});

describe("P23.S6 nothing is inferred and nothing happens", () => {
  it("3. carries no application dictionary or title-based inference", () => {
    const source = readFileSync(
      path.join(root, "app/src/lib/observationAnswerSource.ts"),
      "utf8",
    );
    expect(source).not.toMatch(/quokka|zarnak/i);
    expect(source).not.toMatch(/chrome|cursor|explorer|notepad|firefox|edge|code\.exe/i);
    expect(source).not.toMatch(/\.exe/);
    // Identity is read from the observation, never assigned out of a title.
    expect(source).not.toMatch(/(?:application|process)\w*\s*[=:]\s*[^\n]*title/i);
    expect(source).toContain("processName");
  });

  it("15-16. never derives identity from a URL or from capability metadata", async () => {
    desktopIs([
      { title: "Zarnak Search — https://example.invalid/q", application: null },
    ]);
    const text = await replyTo("Which application am I using?");
    // Quoting the observed title is truthful; reading an application out of the
    // URL inside it would not be.
    expect(text).toMatch(/didn’t tell me which application/);
    expect(text).not.toMatch(/\.exe/);
    // Neither shape of application claim this source can make appears.
    expect(text).not.toMatch(/You’re using \S|The active application is/);
  });

  it("17. causes no desktop effect during an application question", async () => {
    for (const utterance of [
      "Which application am I using?",
      "What application is active?",
    ]) {
      resetWorkspaceContext();
      desktopIs([QUOKKA]);
      await handleOperatorUtterance(utterance);

      expect(invokedCommands()).toEqual(["execute_capability_intent"]);
      const [intent] = sentIntents();
      expect(intent).toMatchObject({ domain: "window", operation: "active" });
      expect(intent.operation).not.toMatch(
        /focus|open|minimi|maximi|snap|close|arrange|restore|kill|terminate/,
      );
    }
  });

  it("18. reaches no browser and no external model", async () => {
    desktopIs([QUOKKA]);
    const text = (await replyTo("Which application am I using?")).toLowerCase();
    expect(text).not.toMatch(/chatgpt|http|browser|search/);
  });

  it("19-20. keeps meaning free of identity and the ladder free of IPC", () => {
    const serialized = JSON.stringify(
      comprehend("Which application am I using?"),
    ).toLowerCase();
    for (const token of ["provider", "capability", "process", "kernel", ".exe"]) {
      expect(serialized).not.toContain(token);
    }

    for (const rel of [
      "app/src/lib/answerSource.ts",
      "app/src/lib/observationAnswerSource.ts",
    ]) {
      const source = readFileSync(path.join(root, rel), "utf8");
      expect(source).not.toMatch(/@tauri-apps|invokeIpc|invoke\(/);
      expect(source).not.toMatch(/execute_capability_intent|execute_window_operation/);
    }
  });

  it("21. asks only for the observation, so permission stays the Kernel's decision", async () => {
    invokeIpc.mockReset();
    invokeIpc.mockResolvedValue({
      ok: false,
      message: "I can’t do that without permission.",
      status: "failed",
      domain: "window",
      operation: "active",
      items: null,
    });

    const text = await replyTo("Which application am I using?");
    expect(text).toBe("I can’t do that without permission.");
    expect(text).not.toMatch(/\.exe/);
  });

  it("composes nothing from an observation that reports no window", () => {
    const goal = comprehend("Which application am I using?");
    expect(composeObservationAnswer(goal, { ok: true, items: [] })).toBeNull();
    expect(
      composeObservationAnswer(goal, {
        ok: false,
        items: [{ title: "Quokka Ledger", processName: "quokka-editor.exe" }],
      }),
    ).toBeNull();
  });

  it("still routes application questions through the active-window observation", () => {
    const { goal, action } = resolveIntentWithGoal("Which application am I using?");
    expect(goal.outcome).toBe("PERCEIVE_MACHINE");
    expect(action.kind).toBe("winActive");
  });
});
