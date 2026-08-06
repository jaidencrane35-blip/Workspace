import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { resolveIntent } from "../app/src/lib/intentBridge";
import {
  isCapabilityIntent,
  isProviderCommand,
  planFromIntent,
  sanitizeUserText,
} from "../app/src/lib/operator";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("Operator Intelligence Foundation", () => {
  it("plans capability intents without Conversation owning IPC", () => {
    const win = resolveIntent("What windows are open?");
    expect(isCapabilityIntent(win)).toBe(true);
    expect(planFromIntent(win)).toMatchObject({
      steps: [{ domain: "window", operation: "enumerate" }],
    });

    const open = resolveIntent("open notepad");
    expect(planFromIntent(open)).toMatchObject({
      compositionId: "app.open_or_focus",
      steps: [{ domain: "application", operation: "find" }],
    });
  });

  it("returns shell directives for non-capability intents", () => {
    const save = resolveIntent("save this");
    expect(isCapabilityIntent(save)).toBe(false);
    expect(planFromIntent(save)).toBeNull();
  });

  it("strips provider jargon from user-facing text", () => {
    expect(
      sanitizeUserText("Reading via Capability Runtime and Window Provider."),
    ).not.toMatch(/capability runtime|window provider/i);
  });

  it("classifies provider IPC commands for authority checks", () => {
    expect(isProviderCommand("execute_window_operation")).toBe(true);
    expect(isProviderCommand("get_workspace")).toBe(false);
  });

  it("keeps OperatorRoot free of provider IPC (source law)", () => {
    const source = readFileSync(
      path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
      "utf8",
    );
    expect(source).toContain("handleOperatorUtterance");
    expect(source).not.toContain("execute_window_operation");
    expect(source).not.toContain("execute_application_operation");
    expect(source).not.toContain("read_clipboard");
  });
});
