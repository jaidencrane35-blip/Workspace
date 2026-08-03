/**
 * Release gate invariants — evidence integrity and documentation consistency.
 * No demo assertions. Production evidence files must remain coherent.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { EXPERIENCE_IPC_COMMANDS } from "../app/src/demo/experienceIpcCatalog";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");
const evidenceDir = path.join(root, "architecture/evidence");
const architectureDir = path.join(root, "architecture");

function readJson(name: string): Record<string, unknown> {
  return JSON.parse(
    fs.readFileSync(path.join(evidenceDir, name), "utf8"),
  ) as Record<string, unknown>;
}

describe("release evidence inventory", () => {
  it("keeps required evidence files present and parseable", () => {
    for (const name of [
      "windows-product-proof.json",
      "multi-monitor-topology.json",
      "experience-e2e-behaviour.json",
      "process-kill-recovery.json",
    ]) {
      expect(fs.existsSync(path.join(evidenceDir, name))).toBe(true);
      expect(readJson(name)).toBeTruthy();
    }
  });

  it("records multi-monitor harness readiness without fabricating dual hardware", () => {
    const multi = readJson("multi-monitor-topology.json");
    expect(multi.harness_ready).toBe(true);
    expect(typeof multi.monitor_count).toBe("number");
    expect(multi.monitor_count as number).toBeGreaterThanOrEqual(1);
    if (multi.hardware_dual_monitor !== true) {
      expect(multi.cross_monitor_restore).toBe(
        "not_executed_insufficient_hardware",
      );
    }
  });

  it("records OS process-kill recovery as incomplete disposition", () => {
    const kill = readJson("process-kill-recovery.json");
    expect(kill.kill_ok).toBe(true);
    expect(kill.recovery_disposition).toBe("incomplete");
    expect(kill.fence_cleared).toBe(true);
  });

  it("records behavioural Experience E2E continue_again success", () => {
    const e2e = readJson("experience-e2e-behaviour.json") as {
      steps: Array<{ step: string; ok: boolean; ipc_command: string }>;
    };
    const again = e2e.steps.find((s) => s.step === "continue_again");
    expect(again?.ok).toBe(true);
    expect(again?.ipc_command).toBe("execute_resume_plan");
  });
});

describe("architecture authority numbering", () => {
  it("keeps Version 1 baseline chain documents present", () => {
    for (const name of [
      "23_Production_Integration_Status.md",
      "24_Runtime_Observation_Model.md",
      "25_Restore_Execution_Model.md",
      "26_Workspace_Runtime_State.md",
      "27_Workspace_Session_Persistence.md",
      "28_Runtime_Recovery_Model.md",
      "29_Product_Proof.md",
      "30_Release_Gate.md",
      "31_Version_1_Release_Checklist.md",
      "32_Version_1_Baseline.md",
      "33_Version_1_Final.md",
      "VERSION_1_HISTORY.md",
    ]) {
      expect(fs.existsSync(path.join(architectureDir, name))).toBe(true);
    }
  });

  it("release gate references evidence files that exist", () => {
    const gate = fs.readFileSync(
      path.join(architectureDir, "30_Release_Gate.md"),
      "utf8",
    );
    for (const file of [
      "windows-product-proof.json",
      "multi-monitor-topology.json",
      "experience-e2e-behaviour.json",
      "process-kill-recovery.json",
    ]) {
      expect(gate).toContain(file);
      expect(fs.existsSync(path.join(evidenceDir, file))).toBe(true);
    }
  });
});

describe("IPC catalog contract stability", () => {
  it("keeps frozen Experience IPC command names stable", () => {
    expect(EXPERIENCE_IPC_COMMANDS).toContain("save_workspace_context");
    expect(EXPERIENCE_IPC_COMMANDS).toContain("resolve_resume_plan");
    expect(EXPERIENCE_IPC_COMMANDS).toContain("execute_resume_plan");
    expect(EXPERIENCE_IPC_COMMANDS).toContain("list_saved_contexts");
    expect(EXPERIENCE_IPC_COMMANDS).not.toContain("get_workspace_runtime_state");
  });

  it("registers catalog commands in Tauri lib", () => {
    const tauriLib = fs.readFileSync(
      path.join(root, "app/src-tauri/src/lib.rs"),
      "utf8",
    );
    for (const command of EXPERIENCE_IPC_COMMANDS) {
      expect(tauriLib).toContain(command);
    }
  });
});
