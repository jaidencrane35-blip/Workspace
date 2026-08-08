import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("P17.S4 Conversation working-state continuity", () => {
  const ui = readFileSync(
    path.join(root, "app/src/components/operator/OperatorRoot.tsx"),
    "utf8",
  );
  const intel = readFileSync(
    path.join(root, "app/src/lib/operator/intelligence.ts"),
    "utf8",
  );
  const app = readFileSync(path.join(root, "app/src/App.tsx"), "utf8");

  it("enters busy and working ack before Operator IPC", () => {
    expect(ui).toMatch(/Working on that/);
    expect(ui).toContain("composerBusy");
    expect(ui).toContain("holdBusy");
    const submit = ui.slice(ui.indexOf("const submitUtterance"));
    expect(submit.indexOf("setBusy(true)")).toBeLessThan(
      submit.indexOf("await handleIntent("),
    );
  });

  it("bridges App Moments busy into Conversation composer", () => {
    expect(app).toContain("toolBusy={busy}");
    expect(ui).toContain("toolBusy");
    expect(ui).toContain("disabled={composerBusy}");
  });

  it("surfaces support Creating… before export_support_bundle", () => {
    const block = intel.slice(intel.indexOf('kind === "supportBundle"'));
    expect(block.indexOf("onWorking")).toBeLessThan(
      block.indexOf("export_support_bundle"),
    );
  });

  it("keeps Soft Send and F10 review path", () => {
    expect(ui).toContain("data-voice-ready");
    expect(ui).toMatch(/never auto-submit/i);
  });
});
