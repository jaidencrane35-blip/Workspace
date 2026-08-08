import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("P18.S1 Moments browse truth", () => {
  const app = readFileSync(path.join(root, "app/src/App.tsx"), "utf8");
  const active = readFileSync(
    path.join(root, "app/src/components/ActiveMoment.tsx"),
    "utf8",
  );
  const home = readFileSync(
    path.join(root, "app/src/components/HomeWorkspacePanel.tsx"),
    "utf8",
  );
  const resume = readFileSync(
    path.join(root, "app/src/components/ResumeContextPanel.tsx"),
    "utf8",
  );

  it("mounts ActiveMomentProvider on the App product path", () => {
    expect(app).toContain("ActiveMomentProvider");
    expect(active).toMatch(/moments:\s*contexts/);
    expect(active).toContain("list_saved_contexts");
  });

  it("Home and Continue share useActiveMoment moments authority", () => {
    expect(home).toContain("useActiveMoment");
    expect(home).not.toContain("invokeIpc");
    expect(resume).toContain("moments");
    expect(home).toContain('data-moments-browse="home"');
    expect(resume).toContain('data-moments-browse="continue"');
  });

  it("Continue empty state is truthful (moments list empty only)", () => {
    expect(resume).toContain("moments.length === 0");
    expect(resume).not.toContain('!primary && step === "browse"');
    expect(resume).toContain("Nothing to continue yet");
  });

  it("preserves P17 agency and status ownership hooks", () => {
    expect(resume).toContain("useAgencyKeyboard");
    expect(resume).toContain("deletedAck");
    expect(resume).toContain("describeRestoreOutcome");
  });
});
