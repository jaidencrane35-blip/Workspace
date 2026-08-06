import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import {
  CAPABILITY_INTENT_COMMAND,
  isBannedProviderCommand,
  isCapabilityIntentAction,
  toCapabilityIntent,
} from "../app/src/lib/operator";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("P12 Finalization — Kernel Operator authority", () => {
  it("maps intents to CapabilityIntent without orchestration", () => {
    expect(toCapabilityIntent(resolveIntent("What windows are open?"))).toEqual(
      {
        domain: "window",
        operation: "enumerate",
      },
    );
    expect(toCapabilityIntent(resolveIntent("open notepad"))).toEqual({
      domain: "application",
      operation: "open",
      query: "notepad",
    });
    expect(isCapabilityIntentAction(resolveIntent("save this"))).toBe(false);
  });

  it("bans provider-specific IPC from the Operator façade", () => {
    expect(isBannedProviderCommand("execute_window_operation")).toBe(true);
    expect(isBannedProviderCommand(CAPABILITY_INTENT_COMMAND)).toBe(false);
  });

  it("keeps OperatorRoot and façade free of provider invoke paths", () => {
    const rootUi = readFileSync(
      path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
      "utf8",
    );
    const intelligence = readFileSync(
      path.join(root, "app/src/lib/operator/intelligence.ts"),
      "utf8",
    );
    const bridge = readFileSync(
      path.join(root, "app/src/lib/operator/runtimeBridge.ts"),
      "utf8",
    );

    expect(rootUi).toContain("handleOperatorUtterance");
    expect(rootUi).not.toContain("invokeIpc");
    expect(intelligence).not.toContain("invokeIpc");
    expect(bridge).toContain(CAPABILITY_INTENT_COMMAND);
    // Sole invoke target is the CapabilityIntent command constant (banned names may appear in a deny-list).
    expect(bridge).toMatch(/invokeIpc[\s\S]*CAPABILITY_INTENT_COMMAND/);
    expect(bridge).not.toMatch(
      /invokeIpc\s*<[^>]*>\s*\(\s*"(execute_window_operation|execute_application_operation|read_clipboard|write_clipboard)"/,
    );
  });
});
