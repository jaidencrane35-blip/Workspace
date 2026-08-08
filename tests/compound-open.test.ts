/**
 * P21.S2 — Compound Goal Decomposition.
 */
import { beforeEach, describe, expect, it } from "vitest";
import { resolveCompoundOpen } from "../app/src/lib/compoundOpen";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { toCapabilityIntent } from "../app/src/lib/operator/intentMap";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

describe("P21.S2 Compound Goal Decomposition", () => {
  beforeEach(() => {
    resetWorkspaceContext();
  });

  it("opens Cursor and Chrome as a compound intent", () => {
    const action = resolveIntent("Open Cursor and Chrome.");
    expect(action.kind).toBe("compoundOpen");
    if (action.kind !== "compoundOpen") return;
    expect(action.targets).toHaveLength(2);
    expect(action.targets[0]).toMatchObject({ kind: "app", label: "Cursor" });
    expect(action.targets[1]).toMatchObject({ kind: "app", label: "Chrome" });
    const ipc = toCapabilityIntent(action);
    expect(ipc).toMatchObject({
      domain: "application",
      operation: "open_compound",
    });
    expect(ipc?.text).toContain("app:Cursor");
    expect(ipc?.text).toContain("app:Google Chrome");
  });

  it("opens Chrome and Notepad as a compound intent", () => {
    const action = resolveIntent("Open Chrome and Notepad.");
    expect(action.kind).toBe("compoundOpen");
    if (action.kind !== "compoundOpen") return;
    expect(action.targets.map((t) => t.label)).toEqual(["Chrome", "Notepad"]);
  });

  it("keeps browser beside Cursor on the beside path (not compound and)", () => {
    const action = resolveIntent("Open Browser beside Cursor.");
    expect(action.kind).toBe("browserOpenBeside");
    if (action.kind !== "browserOpenBeside") return;
    expect(action.beside.toLowerCase()).toMatch(/cursor/);
  });

  it("does not invent unresolved compound targets", () => {
    const action = resolveIntent("Open Cursor and TotallyUnknownApp999.");
    expect(action.kind).toBe("unknown");
    expect(action.reply.toLowerCase()).not.toMatch(/opening “cursor” and/);
  });

  it("leaves single-intent open behaviour unchanged", () => {
    expect(resolveIntent("Open Notepad.")).toMatchObject({
      kind: "appOpen",
      query: "Notepad",
    });
    expect(resolveIntent("Open ChatGPT.")).toMatchObject({
      kind: "browserOpen",
      url: "https://chatgpt.com",
    });
  });

  it("helper encodes only when every part resolves", () => {
    const ok = resolveCompoundOpen("Cursor and Chrome");
    expect(ok?.encode).toBe("app:Cursor|app:Google Chrome");
    expect(resolveCompoundOpen("Cursor and foobarbaz")).toBeNull();
  });
});
