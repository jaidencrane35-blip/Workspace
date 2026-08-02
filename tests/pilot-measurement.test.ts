/**
 * PP-P01E — Consented local pilot measurement kit.
 *
 * Measurements require explicit consent, stay local, and must not ambiently
 * observe the desktop or upload data.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");

const panel = fs.readFileSync(
  path.join(root, "app/src/components/PilotMeasurementPanel.tsx"),
  "utf8",
);
const domain = fs.readFileSync(
  path.join(root, "packages/domain/src/pilot_measurement/mod.rs"),
  "utf8",
);
const service = fs.readFileSync(
  path.join(root, "packages/kernel/src/services/pilot_measurement.rs"),
  "utf8",
);
const app = fs.readFileSync(path.join(root, "app/src/App.tsx"), "utf8");
const chrome = fs.readFileSync(path.join(root, "app/src/lib/pilotChrome.ts"), "utf8");

describe("PP-P01E consented pilot measurement", () => {
  it("requires explicit consent before recording measurements", () => {
    expect(panel).toContain("I consent to local pilot measurement");
    expect(panel).toContain("grant_pilot_consent");
    expect(panel).toContain("approvedScope: scope.id");
    expect(service).toContain("require_active_consent");
    expect(domain).toContain("ConsentRequired");
  });

  it("covers baseline, leave→resume, correction, and week-four interview", () => {
    expect(panel).toContain("record_pilot_baseline");
    expect(panel).toContain("record_pilot_leave_resume");
    expect(panel).toContain("correctionNeeded");
    expect(panel).toContain("record_pilot_interview");
    expect(panel).toContain('phase === "baseline"');
    expect(panel).toContain("week_four");
    expect(domain).toContain("leave_resume");
    expect(domain).toContain("habit_days");
  });

  it("keeps measurement local and non-ambient", () => {
    expect(domain).toContain("No background watching");
    expect(domain).toContain("Nothing is uploaded");
    expect(panel).not.toMatch(/fetch\(|XMLHttpRequest|navigator\.sendBeacon|telemetry/i);
    expect(service).not.toMatch(/reqwest|http::|upload|telemetry/i);
    expect(panel).not.toMatch(/setInterval|MutationObserver/);
    expect(panel).toContain("Nothing is recorded until you consent");
  });

  it("distinguishes pilot evaluation data from product saved contexts", () => {
    expect(domain).toContain("separate from your saved");
    expect(panel).toContain("are not saved");
    expect(panel).toContain("withdraw_pilot_consent");
  });

  it("exposes Pilot as a participant surface without restoring engine tabs", () => {
    expect(chrome).toContain('"pilot"');
    expect(app).toContain("PilotMeasurementPanel");
    expect(app).toContain(">\n            Pilot\n          </button>");
    expect(app).not.toContain("OperatorConsole");
    expect(app).not.toContain("CanvasShell");
  });
});
