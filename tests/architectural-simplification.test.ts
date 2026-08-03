/**
 * Sprint 72 — Architectural simplification.
 */
import { readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { FORBIDDEN_EVENT_KEYS } from "../app/src/dev/experienceEvents";
import { memoryStore } from "../app/src/dev/experienceStore";
import { AUTHORITY_CHAIN } from "../app/src/dev/architecturalIntegrity";
import {
  COMPLEXITY_AFTER,
  COMPLEXITY_BEFORE,
  buildArchitecturalComplexityReport,
} from "../app/src/experience/architecturalComplexity";
import { clamp01, meanOf, populationVariance, round4 } from "../app/src/experience/experienceMath";
import {
  certifyAdaptationSet,
  clearCertificationStore,
} from "../app/src/experience/adaptationCertification";
import { clearAdaptationPackStore } from "../app/src/experience/adaptationPacks";
import { clearExperimentStore } from "../app/src/experience/adaptationExperiments";
import {
  clearProductionActivationStore,
  runFirstProductionAdaptation,
} from "../app/src/experience/productionAdaptation";
import {
  deriveWorkspaceAnticipation,
  replayWorkspaceAnticipation,
  resolvePresentationWithAnticipation,
} from "../app/src/experience/workspaceAnticipation";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");
const experienceDir = path.join(root, "app/src/experience");

function teardown(store: ReturnType<typeof memoryStore>) {
  clearAdaptationPackStore(store);
  clearCertificationStore(store);
  clearProductionActivationStore(store);
  clearExperimentStore(store);
}

describe("complexity report", () => {
  it("reports measurable duplication reduction", () => {
    const report = buildArchitecturalComplexityReport();
    expect(report.schemaVersion).toBe(1);
    expect(report.before).toEqual(COMPLEXITY_BEFORE);
    expect(report.after).toEqual(COMPLEXITY_AFTER);
    expect(report.delta.duplicatedClampRoundSites).toBeLessThan(0);
    expect(report.delta.duplicatedVarianceSites).toBeLessThan(0);
    expect(report.delta.deprecatedPresentationAliases).toBeLessThan(0);
    expect(report.after.duplicatedClampRoundSites).toBe(0);
    expect(report.after.resolverStages).toBe(report.before.resolverStages);
    expect(report.simplification.newSharedModules).toContain(
      "experienceMath.ts",
    );
    expect(report.preservedContracts.length).toBeGreaterThan(0);
  });

  it("shared math is deterministic", () => {
    expect(clamp01(1.5)).toBe(1);
    expect(round4(1.23456)).toBe(1.2346);
    expect(populationVariance([1, 1, 1])).toBe(0);
    expect(meanOf([1, 2, 3])).toBe(2);
  });
});

describe("compatibility / equivalence", () => {
  it("preserves anticipation replay determinism after consolidation", () => {
    const store = memoryStore();
    const activation = runFirstProductionAdaptation(store, { now: 10 });
    expect(activation.outcome).toBe("activated");
    expect(certifyAdaptationSet(store, { now: 100 }).ok).toBe(true);

    const a = replayWorkspaceAnticipation(store);
    const b = replayWorkspaceAnticipation(store);
    expect(a.anticipation).toEqual(b.anticipation);
    expect(a.presentation).toEqual(b.presentation);
    expect(resolvePresentationWithAnticipation(store)).toEqual(
      a.presentation,
    );
    expect(deriveWorkspaceAnticipation(store)?.anticipationId).toBe(
      a.anticipation?.anticipationId,
    );

    teardown(store);
  });

  it("experience modules no longer declare local clamp01/round4/populationVariance", () => {
    const files = readdirSync(experienceDir).filter(
      (f) => f.endsWith(".ts") && f !== "experienceMath.ts",
    );
    for (const file of files) {
      const src = readFileSync(path.join(experienceDir, file), "utf8");
      expect(src).not.toMatch(/function clamp01\s*\(/);
      expect(src).not.toMatch(/function round4\s*\(/);
      expect(src).not.toMatch(/function populationVariance\s*\(/);
      expect(src).not.toMatch(/const mergePresentation\s*=/);
    }
  });
});

describe("governance", () => {
  it("extends authority chain through simplification doc", () => {
    expect(
      AUTHORITY_CHAIN.some(
        ([a, b]) =>
          a === "62_Presentation_Stability.md" &&
          b === "63_Architectural_Simplification.md",
      ),
    ).toBe(true);
  });
});

describe("privacy audit (architectural simplification)", () => {
  it("complexity module introduces no persistence or telemetry", () => {
    const src = readFileSync(
      path.join(experienceDir, "architecturalComplexity.ts"),
      "utf8",
    );
    expect(src).not.toMatch(/STORAGE_KEY\s*=/);
    expect(src).not.toMatch(/saveJsonBundle|store\.setItem/);
    expect(src).not.toMatch(/\bfetch\s*\(/);
    const raw = JSON.stringify(buildArchitecturalComplexityReport());
    for (const forbidden of FORBIDDEN_EVENT_KEYS) {
      expect(raw).not.toContain(`"${forbidden}"`);
    }
  });

  it("overlay lazy-loads complexity module", () => {
    const dash = readFileSync(
      path.join(root, "app/src/dev/ExperienceEvidenceDashboard.tsx"),
      "utf8",
    );
    expect(dash).toContain('import("../experience/architecturalComplexity")');
    expect(dash).toContain("architectural-complexity");
    expect(dash).toContain("complexity-report");
    expect(dash).toContain("complexity-duplicate-inventory");
    expect(dash).toContain("complexity-dependency-fanout");
    expect(dash).toContain("complexity-simplification-summary");
    expect(dash).toContain("complexity-before-after");
  });

  it("architecture doc records simplification evidence", () => {
    const doc = readFileSync(
      path.join(root, "architecture/63_Architectural_Simplification.md"),
      "utf8",
    );
    expect(doc).toContain("experienceMath");
    expect(doc).toContain("Removed duplication");
    expect(doc).toContain("Preserved contracts");
    expect(doc).toMatch(/Evidence only|no recommendations/i);
  });
});
