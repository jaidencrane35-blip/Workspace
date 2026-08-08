/**
 * P24.S2 — Language Faculty Boundary.
 *
 * Proves the seam: Faculty → validated Meaning → GoalContract, with no
 * capability selection, IPC, or invented identity. The sole Faculty is
 * deterministic comprehend(); this is not a model.
 */
import { afterEach, describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import {
  comprehendViaFaculty,
  deterministicLanguageFaculty,
  resetLanguageFaculty,
  setLanguageFaculty,
  validateMeaningProposal,
  type LanguageFaculty,
  type MeaningProposal,
} from "../app/src/lib/languageFaculty";
import { comprehend } from "../app/src/lib/goalContract";
import { resolveIntentWithGoal } from "../app/src/lib/intentBridge";
import { resetWorkspaceContext } from "../app/src/lib/workspaceContext";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

afterEach(() => {
  resetLanguageFaculty();
  resetWorkspaceContext();
});

describe("P24.S2 Language Faculty boundary", () => {
  it("A. Queensland time is KNOW/information/time — no capability or operation", () => {
    const goal = comprehendViaFaculty("What is the time in Queensland?");
    expect(goal.outcome).toBe("KNOW");
    expect(goal.mode).toBe("information");
    expect(goal.domain).toBe("time");
    expect(JSON.stringify(goal)).not.toMatch(/C-[A-Z]+-\d+|CapabilityIntent|provider|operation/i);
    expect(goal.requestedResult).toMatch(/time/i);
  });

  it("B. 'What does minimize mean?' stays knowledge — no desktop effect meaning as action-only", () => {
    const goal = comprehendViaFaculty("What does minimize mean?");
    expect(goal.outcome).toBe("KNOW");
    expect(goal.mode).not.toBe("action");
    expect(goal.outcome).not.toBe("REACH_STATE");
  });

  it("C. 'Open Notepad.' is action-oriented meaning without selecting a capability", () => {
    const goal = comprehendViaFaculty("Open Notepad.");
    expect(goal.outcome).toBe("REACH_STATE");
    expect(goal.mode === "action" || goal.mode === "hybrid").toBe(true);
    const serialized = JSON.stringify(goal);
    expect(serialized).not.toContain("C-ACT-");
    expect(serialized).not.toContain("CapabilityIntent");
    expect(serialized).not.toMatch(/ApplicationProvider|WindowProvider/);
    // Faculty does not choose the implementation — only names the Owner's target.
    expect(goal.targets.some((t) => /notepad/i.test(t.phrase))).toBe(true);
  });

  it("D. compound count goal preserves Pictures, Screenshots, and requested count", () => {
    const goal = comprehendViaFaculty(
      "Open Pictures and count the pictures in Screenshots.",
    );
    expect(goal.compound).toBe(true);
    expect(goal.requestedResult).toMatch(/count/i);
    const blob = `${goal.subject ?? ""} ${goal.clauses.join(" ")} ${goal.targets
      .map((t) => t.phrase)
      .join(" ")}`.toLowerCase();
    expect(blob).toMatch(/pictures/);
    expect(blob).toMatch(/screenshots/);
    const serialized = JSON.stringify(goal);
    expect(serialized).not.toMatch(/explorer\.exe|C-ACT-001|C-OBS-003|CapabilityIntent/i);
    expect(serialized).not.toMatch(/"steps"|click|invokeIpc/i);
  });

  it("E. application question is machine observation — no invented application identity", () => {
    const goal = comprehendViaFaculty("What application am I using?");
    expect(goal.outcome).toBe("PERCEIVE_MACHINE");
    expect(goal.mode).toBe("observation");
    // Faculty must not invent a process/application name as a recognized target.
    for (const target of goal.targets) {
      if (target.role === "application") {
        expect(target.recognized && /cursor|chrome|notepad/i.test(target.canonical ?? "")).toBe(
          false,
        );
      }
    }
    expect(JSON.stringify(goal)).not.toMatch(/\.exe/);
  });

  it("F. 'Open that.' without context requires clarification — no invented target", () => {
    resetWorkspaceContext();
    const goal = comprehendViaFaculty("Open that.");
    const unresolved = goal.references.some((r) => !r.resolved);
    expect(unresolved || goal.clarificationNeeded || goal.uncertainties.length > 0).toBe(
      true,
    );
    for (const ref of goal.references) {
      if (!ref.resolved) {
        expect(ref.referent).toBeNull();
      }
    }
  });

  it("G. malformed Faculty output fails validation and falls back deterministically", () => {
    const bad: LanguageFaculty = {
      proposeMeaning: () =>
        ({
          utterance: "Open Notepad.",
          normalized: "open notepad",
          mode: "action",
          outcome: "REACH_STATE",
          domain: "application",
          subject: "Notepad",
          targets: [],
          references: [],
          requestedResult: null,
          compound: false,
          clauses: ["open notepad"],
          clarificationNeeded: false,
          uncertainties: [],
          evidence: [],
          // Authority leak — must be rejected.
          capabilityId: "C-ACT-001",
          steps: [{ operation: "launch" }],
        }) as unknown as MeaningProposal,
    };

    const rejected = validateMeaningProposal(bad.proposeMeaning("Open Notepad."));
    expect(rejected.ok).toBe(false);
    if (!rejected.ok) {
      expect(rejected.reason).toMatch(/authority field|capability/i);
    }

    setLanguageFaculty(bad);
    const goal = comprehendViaFaculty("Open Notepad.");
    expect(goal.outcome).toBe("REACH_STATE");
    expect(goal.evidence.some((e) => e.startsWith("faculty_rejected="))).toBe(true);
    expect(goal.evidence).toContain("faculty=deterministic_fallback");
    // Fallback matches deterministic comprehend for the same utterance.
    expect(goal.outcome).toBe(comprehend("Open Notepad.").outcome);
  });

  it("deterministic Faculty matches comprehend() exactly", () => {
    const utterance = "What windows are open?";
    expect(deterministicLanguageFaculty.proposeMeaning(utterance)).toEqual(
      comprehend(utterance),
    );
    expect(comprehendViaFaculty(utterance)).toEqual(comprehend(utterance));
  });

  it("bridge still resolves through Faculty without changing Queensland answer path", () => {
    const { goal, action } = resolveIntentWithGoal("What is the time in Queensland?");
    expect(goal.outcome).toBe("KNOW");
    expect(goal.domain).toBe("time");
    // Answer ladder / speaking path — not a desktop effect.
    expect(action.kind).not.toMatch(/win|app|browser/i);
  });

  it("module source performs no IPC and selects no capability", () => {
    const source = readFileSync(
      path.join(root, "app/src/lib/languageFaculty.ts"),
      "utf8",
    );
    // Imports and write APIs — the leak-detector string list may mention forbidden
    // tokens as needles; that is detection, not use.
    expect(source).not.toMatch(/from\s+["']@tauri-apps|from\s+["']\.\/ipc["']/);
    expect(source).not.toMatch(/from\s+["']\.\/capabilityRegistry|from\s+["']\.\/executionPlanner/);
    expect(source).not.toMatch(/from\s+["']\.\/operator\//);
    expect(source).not.toMatch(/\bcommitWorkspaceContext\s*\(|\bcreateMemory\s*\(/);
  });

  it("validator rejects provider identity and invented unresolved referents", () => {
    const withProvider = validateMeaningProposal({
      ...comprehend("Open Notepad."),
      evidence: ["use WindowProvider"],
    });
    expect(withProvider.ok).toBe(false);

    const inventedRef = validateMeaningProposal({
      ...comprehend("Open that."),
      references: [{ phrase: "that", resolved: false, referent: "Notepad" }],
    });
    expect(inventedRef.ok).toBe(false);
  });
});
