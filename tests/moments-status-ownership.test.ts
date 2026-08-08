import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("P17.S5 Moments status ownership", () => {
  const resume = readFileSync(
    path.join(root, "app/src/components/ResumeContextPanel.tsx"),
    "utf8",
  );
  const app = readFileSync(path.join(root, "app/src/App.tsx"), "utf8");
  const save = readFileSync(
    path.join(root, "app/src/components/SaveContextPanel.tsx"),
    "utf8",
  );

  it("keeps Delete acknowledgement inline without ok-banner", () => {
    expect(resume).toContain("deletedAck");
    expect(resume).toMatch(/is gone/);
    expect(resume).not.toMatch(/onMessage\(`Deleted/);
    expect(resume).not.toMatch(/onMessage\("Deleted/);
  });

  it("clears stale specialized status when Moments continues", () => {
    expect(resume).toContain("clearStatusChrome");
    expect(resume).toContain("onMessage(null)");
    expect(save).toContain("onMessage(null)");
  });

  it("presents restore completion in Owner language", () => {
    expect(resume).toContain("describeRestoreOutcome");
    expect(resume).toContain("Mostly restored.");
    expect(resume).not.toMatch(/Outcome:\s*<strong>\{\s*result\.outcome\.replace/);
  });

  it("gives error banners a dismiss path", () => {
    expect(app).toContain("ws-toast__dismiss");
    expect(app).toContain("Dismiss");
    expect(app).toMatch(/onClick=\{\(\) => onError\(null\)\}/);
  });
});
