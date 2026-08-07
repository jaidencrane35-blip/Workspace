import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import {
  CAPABILITY_INTENT_COMMAND,
  composeTransportFailureMessage,
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
      query: "Notepad",
    });
    expect(isCapabilityIntentAction(resolveIntent("save this"))).toBe(false);
  });

  it("bans provider-specific IPC from the Operator façade", () => {
    expect(isBannedProviderCommand("execute_window_operation")).toBe(true);
    expect(isBannedProviderCommand(CAPABILITY_INTENT_COMMAND)).toBe(false);
  });

  it("P17.S1 — transport failures never forward raw Error.message", () => {
    const bridge = readFileSync(
      path.join(root, "app/src/lib/operator/runtimeBridge.ts"),
      "utf8",
    );
    expect(bridge).toContain("composeTransportFailureMessage");
    expect(bridge).toMatch(/message:\s*composeTransportFailureMessage\(intent\)/);
    expect(bridge).not.toMatch(/message:\s*\n?\s*(error\.message|error instanceof)/);
    expect(
      composeTransportFailureMessage({
        domain: "notifications",
        operation: "show",
      }),
    ).toMatch(/couldn’t show|couldn't show/i);
    expect(
      composeTransportFailureMessage({
        domain: "notifications",
        operation: "show",
      }).toLowerCase(),
    ).not.toContain("unknown error");
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
    // Support-package intent may use invokeIpc; capability work must stay on runtimeBridge.
    expect(intelligence).toContain("export_support_bundle");
    expect(intelligence).toContain("executeCapabilityIntent");
    expect(intelligence).not.toMatch(
      /invokeIpc[\s\S]*"(execute_window_operation|execute_application_operation|read_clipboard|write_clipboard|execute_capability_intent)"/,
    );
    expect(bridge).toContain(CAPABILITY_INTENT_COMMAND);
    // Sole invoke target is the CapabilityIntent command constant (banned names may appear in a deny-list).
    expect(bridge).toMatch(/invokeIpc[\s\S]*CAPABILITY_INTENT_COMMAND/);
    expect(bridge).not.toMatch(
      /invokeIpc\s*<[^>]*>\s*\(\s*"(execute_window_operation|execute_application_operation|read_clipboard|write_clipboard)"/,
    );
  });
});
