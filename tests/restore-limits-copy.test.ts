/**
 * PP-P01B — Restore-limits copy consistency.
 *
 * Product Proof must state the same restore boundaries on Save and Resume:
 * same continuing session, still-open windows only, no silent relaunch.
 * Copy is Experience presentation only; this audit prevents drift and
 * capability overclaim.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  RESTORE_LIMITS_DOES,
  RESTORE_LIMITS_DOES_NOT,
  RESTORE_LIMITS_SUMMARY,
} from "../app/src/lib/restoreLimits";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

const copySource = fs.readFileSync(
  path.join(root, "app/src/lib/restoreLimits.ts"),
  "utf8",
);
const notice = fs.readFileSync(
  path.join(root, "app/src/components/RestoreLimitsNotice.tsx"),
  "utf8",
);
const savePanel = fs.readFileSync(
  path.join(root, "app/src/components/SaveContextPanel.tsx"),
  "utf8",
);
const resumePanel = fs.readFileSync(
  path.join(root, "app/src/components/ResumeContextPanel.tsx"),
  "utf8",
);

describe("PP-P01B restore-limits copy", () => {
  it("states same-session, still-open, and no-relaunch limits", () => {
    expect(RESTORE_LIMITS_SUMMARY).toMatch(/still open/i);
    expect(RESTORE_LIMITS_SUMMARY).toMatch(/same Windows session/i);
    expect(RESTORE_LIMITS_SUMMARY).toMatch(/does not relaunch/i);

    const doesNot = RESTORE_LIMITS_DOES_NOT.join("\n");
    expect(doesNot).toMatch(/Relaunch/i);
    expect(doesNot).toMatch(/reboot|sign-in/i);
    expect(doesNot).toMatch(/files or links/i);
    expect(doesNot).toMatch(/Guess|rewrite/i);

    const does = RESTORE_LIMITS_DOES.join("\n");
    expect(does).toMatch(/handoff/i);
    expect(does).toMatch(/still open/i);
    expect(does).toMatch(/continuing desktop session/i);
  });

  it("does not claim launch, cross-session restore, or AI explanation", () => {
    expect(copySource).not.toMatch(/will relaunch|auto(?:matic)?ally (?:re)?launch/i);
    expect(copySource).not.toMatch(/after (?:a )?reboot.*will restore/i);
    expect(copySource).not.toMatch(/AI|generated summary|infer(?:s|red)? your next/i);
    expect(notice).not.toMatch(/AI|generated summary/i);
  });

  it("surfaces the shared notice on Save review and confirmation", () => {
    expect(savePanel).toContain('from "./RestoreLimitsNotice"');
    expect(savePanel).toContain("<RestoreLimitsNotice />");
    // Review and saved confirmation each mount the notice.
    expect(savePanel.match(/<RestoreLimitsNotice\s*\/>/g)?.length).toBeGreaterThanOrEqual(
      2,
    );
  });

  it("surfaces restore limits on Resume browse, preview, and outcomes", () => {
    expect(resumePanel).toContain("RESTORE_LIMITS_SUMMARY");
    expect(resumePanel).toContain('from "./RestoreLimitsNotice"');
    expect(resumePanel).toContain("<RestoreLimitsNotice />");
    expect(resumePanel).toContain("<RestoreLimitsNotice compact />");
  });

  it("renders limits from the shared copy module, not inline restatements", () => {
    expect(notice).toContain("RESTORE_LIMITS_DOES");
    expect(notice).toContain("RESTORE_LIMITS_DOES_NOT");
    expect(notice).toContain("RESTORE_LIMITS_SUMMARY");
    expect(notice).toContain('from "../lib/restoreLimits"');
  });
});
