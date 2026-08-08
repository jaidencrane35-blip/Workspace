import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

describe("P18.S2 Moments tool session continuity", () => {
  const app = readFileSync(path.join(root, "app/src/App.tsx"), "utf8");
  const session = readFileSync(
    path.join(root, "app/src/components/MomentsToolSession.tsx"),
    "utf8",
  );
  const save = readFileSync(
    path.join(root, "app/src/components/SaveContextPanel.tsx"),
    "utf8",
  );
  const resume = readFileSync(
    path.join(root, "app/src/components/ResumeContextPanel.tsx"),
    "utf8",
  );
  const active = readFileSync(
    path.join(root, "app/src/components/ActiveMoment.tsx"),
    "utf8",
  );

  it("mounts a continuity holder above Moments tool panels", () => {
    expect(session).toContain("MomentsToolSessionProvider");
    expect(app).toContain("MomentsToolSessionProvider");
    expect(resume).toContain("useMomentsToolSession");
    expect(save).toContain("useMomentsToolSession");
  });

  it("never leaves Save as an empty attach shell", () => {
    expect(save).not.toMatch(
      /expandHost \? createPortal\(writeForm, expandHost\) : null/,
    );
    expect(save).toMatch(/data-moments-save=\{expandHost \? "portaled" : "inline"\}|data-moments-save="inline"/);
  });

  it("preserves Moments focus and pin across Home/Continue/Save", () => {
    expect(app).toContain("MOMENTS_VIEWS");
    expect(active).toMatch(/view === "home".*view === "resume".*view === "save"|momentsViews/);
  });

  it("preserves P18.S1 browse truth and P17 agency", () => {
    expect(resume).toContain("moments.length === 0");
    expect(resume).toContain("useAgencyKeyboard");
    expect(app).toContain("ActiveMomentProvider");
  });
});
