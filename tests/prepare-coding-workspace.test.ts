/**
 * C-PROC-002 — Prepare Coding Workspace (Intent + IPC mapping regression).
 */
import { beforeEach, describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import { toCapabilityIntent } from "../app/src/lib/operator/intentMap";
import {
  matchPrepareCodingWorkspacePhrase,
  resolvePrepareCodingWorkspace,
} from "../app/src/lib/prepareCodingWorkspace";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

describe("C-PROC-002 Prepare Coding Workspace", () => {
  beforeEach(() => {
    resetWorkspaceContext();
  });

  it("resolves Cursor and Notepad completely for Product Proof phrasing", () => {
    const action = resolveIntent(
      "Prepare my coding workspace with Cursor and Notepad.",
    );
    expect(action.kind).toBe("prepareCodingWorkspace");
    if (action.kind !== "prepareCodingWorkspace") return;
    expect(action.targets).toHaveLength(2);
    expect(action.targets.map((t) => t.label)).toEqual(["Cursor", "Notepad"]);
    expect(action.encode).toBe("app:Cursor|app:Notepad");
    const ipc = toCapabilityIntent(action);
    expect(ipc).toMatchObject({
      domain: "application",
      operation: "prepare_coding_workspace",
    });
    expect(ipc?.text).toBe("app:Cursor|app:Notepad");
    expect(ipc?.title).toBe("Cursor and Notepad");
  });

  it("keeps targetless Continue for coding environment (not C-PROC-002)", () => {
    const action = resolveIntent("I need my coding environment.");
    expect(action.kind).toBe("navigate");
    if (action.kind !== "navigate") return;
    expect(action.view).toBe("resume");
  });

  it("clarifies targetless prepare phrasing without inventing apps", () => {
    const action = resolveIntent("Prepare my coding workspace.");
    expect(action.kind).toBe("unknown");
    expect(action.reply.toLowerCase()).toMatch(/which application/);
    expect(toCapabilityIntent(action)).toBeNull();
  });

  it("resolves Intent-known Kernel-unexecutable VS Code into prepare intent for Kernel preflight", () => {
    // PCW-001 Intent resolution includes VS Code; Kernel preflight must block Effects.
    const action = resolveIntent(
      "Prepare my coding workspace with Cursor and Visual Studio Code.",
    );
    expect(action.kind).toBe("prepareCodingWorkspace");
    if (action.kind !== "prepareCodingWorkspace") return;
    expect(action.encode).toContain("app:Cursor");
    expect(action.encode).toContain("app:Visual Studio Code");
    const ipc = toCapabilityIntent(action);
    expect(ipc?.operation).toBe("prepare_coding_workspace");
  });

  it("does not invent unknown targets", () => {
    const action = resolveIntent(
      "Prepare my coding workspace with Cursor and NoSuchCodingAppZZZ.",
    );
    expect(action.kind).toBe("unknown");
    expect(action.reply).toMatch(/NoSuchCodingAppZZZ/);
    expect(toCapabilityIntent(action)).toBeNull();
  });

  it("clarifies ambiguous unbound that-app target", () => {
    const action = resolveIntent(
      "Prepare my coding workspace with that app.",
    );
    expect(action.kind).toBe("unknown");
    expect(action.reply.toLowerCase()).toMatch(/which app|name each/);
  });

  it("routes single application prepare through prepare_coding_workspace (C-ACT-001 open path)", () => {
    const action = resolveIntent("Prepare my coding workspace with Notepad.");
    expect(action.kind).toBe("prepareCodingWorkspace");
    if (action.kind !== "prepareCodingWorkspace") return;
    expect(action.targets).toHaveLength(1);
    expect(action.targets[0]).toMatchObject({ kind: "app", label: "Notepad" });
    expect(toCapabilityIntent(action)?.operation).toBe(
      "prepare_coding_workspace",
    );
  });

  it("routes browser prepare through browser: encoding (C-ACT-006)", () => {
    const action = resolveIntent(
      "Prepare my coding workspace with ChatGPT.",
    );
    expect(action.kind).toBe("prepareCodingWorkspace");
    if (action.kind !== "prepareCodingWorkspace") return;
    expect(action.targets[0]?.kind).toBe("browser");
    expect(action.encode.startsWith("browser:")).toBe(true);
  });

  it("leaves existing single-open and compound-open behaviour unchanged", () => {
    expect(resolveIntent("Open Notepad.")).toMatchObject({
      kind: "appOpen",
      query: "Notepad",
    });
    expect(resolveIntent("Open Cursor and Chrome.")).toMatchObject({
      kind: "compoundOpen",
    });
  });

  it("helper match distinguishes bare vs with-targets prepare phrasing", () => {
    expect(matchPrepareCodingWorkspacePhrase("Prepare my coding workspace.")).toEqual(
      { mode: "bare" },
    );
    expect(
      matchPrepareCodingWorkspacePhrase(
        "Prepare my coding workspace with Cursor and Notepad.",
      ),
    ).toMatchObject({ mode: "with" });
    expect(
      matchPrepareCodingWorkspacePhrase("I need my coding environment."),
    ).toBeNull();
  });

  it("helper resolve returns prepare action only when every target resolves", () => {
    const ok = resolvePrepareCodingWorkspace(
      "Set up my coding workspace with Cursor and Calculator.",
    );
    expect(ok?.kind).toBe("prepareCodingWorkspace");
    expect(
      resolvePrepareCodingWorkspace(
        "Prepare my coding workspace with Cursor and TotallyUnknown999.",
      )?.kind,
    ).toBe("unknown");
  });
});
