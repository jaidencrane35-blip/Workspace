/**
 * Sprint 50 — perception-driven convergence gate.
 */
import { readFileSync, existsSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";

const root = path.resolve(__dirname, "..");
const sprint50 = path.join(
  root,
  "architecture/research/experience/screenshots/v2-sprint-50",
);
const reportPath = path.join(sprint50, "perceptual-metrics.json");
const docPath = path.join(root, "architecture/41_Perceptual_Convergence.md");

describe("perceptual convergence (Sprint 50)", () => {
  it("stores the convergence report and authority doc", () => {
    expect(existsSync(reportPath)).toBe(true);
    expect(existsSync(docPath)).toBe(true);
    expect(existsSync(path.join(sprint50, "analyze.mjs"))).toBe(true);
    expect(existsSync(path.join(sprint50, "concept/board-a.png"))).toBe(true);
    expect(existsSync(path.join(sprint50, "concept/board-b.png"))).toBe(true);
  });

  it("declares no-change when no EV≥0.05 candidate exists", () => {
    const report = JSON.parse(readFileSync(reportPath, "utf8"));
    expect(report.decision.action).toBe("no-change");
    expect(report.decision.declareOptimum).toBe("sprint-49");
    expect(report.actionable).toEqual([]);
    expect(report.parityProjection.sprint50).toBe(9.35);
    expect(report.parityProjection.delta).toBe(0);
  });

  it("rejects refused concept-board literalism", () => {
    const report = JSON.parse(readFileSync(reportPath, "utf8"));
    const rejectedIds = report.rejected.map((r: { id: string }) => r.id);
    expect(rejectedIds).toContain("D-AI-SIDEBAR");
    expect(rejectedIds).toContain("D-SYSTEM-GAUGES");
    expect(rejectedIds).toContain("D-LIVE-THUMBNAILS");
  });
});
