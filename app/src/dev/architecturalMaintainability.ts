/**
 * Sprint 74 — Architectural Maintainability.
 * Deterministic maintainability + dependency health reporting.
 * Derived only. No Runtime / Experience / persistence behaviour changes.
 */

import {
  ENGINEERING_INVARIANTS,
  type EngineeringSubsystem,
} from "./engineeringCertification";
import { buildArchitecturalComplexityReport } from "../experience/architecturalComplexity";

export type MaintainabilitySubsystem = EngineeringSubsystem;

export interface ModuleInventoryEntry {
  id: string;
  lines: number;
  exports: number;
  imports: readonly string[];
  subsystem: MaintainabilitySubsystem;
}

export interface ModuleSizeEntry {
  id: string;
  lines: number;
  subsystem: MaintainabilitySubsystem;
}

export interface MaintainabilityReport {
  schemaVersion: 1;
  reportId: "arch-maintainability-v1";
  moduleCount: number;
  dependencyFanOutTotal: number;
  dependencyDepthMax: number;
  exportedSymbolCount: number;
  duplicatedImplementation: {
    clampRoundSites: number;
    varianceSites: number;
    deprecatedAliases: number;
  };
  averageModuleSize: number;
  largestModules: ModuleSizeEntry[];
  validationCoverage: {
    validationMethodCount: number;
    invariantCount: number;
    /** Distinct validation methods / invariant count, scaled 0–100. */
    percent: number;
  };
  invariantCoverage: {
    invariantCount: number;
    scopeCount: number;
    subsystemCount: number;
    /** Subsystems owning ≥1 invariant / total EngineeringSubsystem kinds, scaled 0–100. */
    percent: number;
  };
  subsystems: Array<{
    subsystem: MaintainabilitySubsystem;
    moduleCount: number;
    lines: number;
    exports: number;
  }>;
}

export interface DependencyModuleMetric {
  id: string;
  count: number;
  subsystem: MaintainabilitySubsystem;
}

export interface BoundaryCrossing {
  from: string;
  to: string;
  kind: "experience_to_dev" | "dev_to_experience";
}

export interface DependencyHealthReport {
  schemaVersion: 1;
  reportId: "dependency-health-v1";
  circularDependencyCount: number;
  circularDependencies: string[][];
  isolatedModules: string[];
  highFanOutModules: DependencyModuleMetric[];
  highFanInModules: DependencyModuleMetric[];
  architecturalBoundaryCrossings: BoundaryCrossing[];
  subsystemOwnership: Array<{
    subsystem: MaintainabilitySubsystem;
    modules: string[];
  }>;
  edgeCount: number;
}

export interface MaintainabilityTrendReport {
  schemaVersion: 1;
  reportId: "maintainability-trend-v1";
  modulesAdded: string[];
  modulesRemoved: string[];
  dependencyIncreases: number;
  dependencyReductions: number;
  invariantCoverageDelta: number;
  validationCoverageDelta: number;
}

/** Frozen inventory of experience + dev TypeScript modules (Sprint 74 tip). */
export const MODULE_INVENTORY: readonly ModuleInventoryEntry[] = [
  {
    id: "dev/architecturalIntegrity.ts",
    lines: 662,
    exports: 26,
    imports: ["dev/devHash.ts", "dev/engineeringGovernance.ts", "dev/experienceEvidence.ts", "dev/experienceGovernance.ts", "dev/experienceImprovement.ts", "dev/experienceStore.ts", "dev/governanceStore.ts"] as const,
    subsystem: "integrity",
  },
  {
    id: "dev/architecturalMaintainability.ts",
    lines: 699,
    exports: 12,
    imports: ["dev/engineeringCertification.ts", "experience/architecturalComplexity.ts"] as const,
    subsystem: "complexity",
  },
  {
    id: "dev/devHash.ts",
    lines: 11,
    exports: 1,
    imports: [] as const,
    subsystem: "integrity",
  },
  {
    id: "dev/engineeringCertification.ts",
    lines: 595,
    exports: 16,
    imports: ["dev/architecturalIntegrity.ts", "dev/engineeringGovernance.ts", "dev/experienceEvidence.ts", "dev/experienceGovernance.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationCertification.ts", "experience/adaptationComposition.ts", "experience/adaptationPacks.ts", "experience/architecturalComplexity.ts", "experience/index.ts", "experience/workspaceAdaptation.ts", "experience/workspaceAnticipation.ts"] as const,
    subsystem: "certification",
  },
  {
    id: "dev/engineeringGovernance.ts",
    lines: 748,
    exports: 35,
    imports: ["dev/devHash.ts", "dev/experienceGovernance.ts", "dev/experienceStore.ts", "dev/governanceStore.ts"] as const,
    subsystem: "governance",
  },
  {
    id: "dev/experienceEvents.ts",
    lines: 138,
    exports: 11,
    imports: [] as const,
    subsystem: "evidence",
  },
  {
    id: "dev/experienceEvidence.ts",
    lines: 366,
    exports: 23,
    imports: ["dev/devHash.ts", "dev/experienceEvents.ts", "dev/experienceStore.ts", "dev/governanceStore.ts", "dev/traceAnalysis.ts"] as const,
    subsystem: "evidence",
  },
  {
    id: "dev/experienceGovernance.ts",
    lines: 642,
    exports: 29,
    imports: ["dev/devHash.ts", "dev/experienceEvidence.ts", "dev/experienceImprovement.ts", "dev/experienceStore.ts", "dev/governanceStore.ts"] as const,
    subsystem: "governance",
  },
  {
    id: "dev/experienceImprovement.ts",
    lines: 424,
    exports: 12,
    imports: ["dev/devHash.ts", "dev/experienceEvidence.ts"] as const,
    subsystem: "governance",
  },
  {
    id: "dev/experienceInstrumentation.ts",
    lines: 345,
    exports: 16,
    imports: ["dev/experienceEvents.ts", "dev/experienceStore.ts", "dev/experienceValidationGate.ts", "dev/frictionModel.ts", "dev/sessionReplay.ts"] as const,
    subsystem: "evidence",
  },
  {
    id: "dev/experienceStore.ts",
    lines: 107,
    exports: 11,
    imports: ["dev/experienceEvents.ts", "dev/governanceStore.ts"] as const,
    subsystem: "governance",
  },
  {
    id: "dev/experienceValidationGate.ts",
    lines: 15,
    exports: 1,
    imports: [] as const,
    subsystem: "governance",
  },
  {
    id: "dev/frictionModel.ts",
    lines: 143,
    exports: 3,
    imports: ["dev/experienceEvents.ts"] as const,
    subsystem: "evidence",
  },
  {
    id: "dev/governancePrimitives.ts",
    lines: 141,
    exports: 18,
    imports: ["dev/architecturalIntegrity.ts", "dev/devHash.ts", "dev/engineeringGovernance.ts", "dev/experienceEvidence.ts", "dev/experienceGovernance.ts", "dev/experienceImprovement.ts", "dev/experienceStore.ts", "dev/governanceStore.ts"] as const,
    subsystem: "governance",
  },
  {
    id: "dev/governanceStore.ts",
    lines: 57,
    exports: 5,
    imports: ["dev/experienceStore.ts"] as const,
    subsystem: "governance",
  },
  {
    id: "dev/index.ts",
    lines: 188,
    exports: 21,
    imports: ["dev/architecturalIntegrity.ts", "dev/engineeringCertification.ts", "dev/engineeringGovernance.ts", "dev/experienceEvents.ts", "dev/experienceEvidence.ts", "dev/experienceGovernance.ts", "dev/experienceImprovement.ts", "dev/experienceInstrumentation.ts", "dev/experienceStore.ts", "dev/experienceValidationGate.ts", "dev/frictionModel.ts", "dev/governancePrimitives.ts", "dev/sessionReplay.ts", "dev/traceAnalysis.ts"] as const,
    subsystem: "integrity",
  },
  {
    id: "dev/sessionReplay.ts",
    lines: 87,
    exports: 4,
    imports: ["dev/experienceEvents.ts", "dev/frictionModel.ts"] as const,
    subsystem: "evidence",
  },
  {
    id: "dev/traceAnalysis.ts",
    lines: 324,
    exports: 4,
    imports: ["dev/experienceEvents.ts", "dev/frictionModel.ts", "dev/sessionReplay.ts"] as const,
    subsystem: "evidence",
  },
  {
    id: "experience/adaptationCertification.ts",
    lines: 543,
    exports: 19,
    imports: ["dev/devHash.ts", "dev/experienceEvidence.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationComposition.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "certification",
  },
  {
    id: "experience/adaptationComposition.ts",
    lines: 430,
    exports: 13,
    imports: ["dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/longitudinalAdaptation.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "adaptation",
  },
  {
    id: "experience/adaptationExperiments.ts",
    lines: 758,
    exports: 23,
    imports: ["dev/architecturalIntegrity.ts", "dev/devHash.ts", "dev/engineeringGovernance.ts", "dev/experienceEvents.ts", "dev/experienceEvidence.ts", "dev/experienceGovernance.ts", "dev/experienceImprovement.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/longitudinalAdaptation.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "adaptation",
  },
  {
    id: "experience/adaptationOperations.ts",
    lines: 521,
    exports: 12,
    imports: ["dev/architecturalIntegrity.ts", "dev/experienceEvidence.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/longitudinalAdaptation.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "adaptation",
  },
  {
    id: "experience/adaptationPacks.ts",
    lines: 601,
    exports: 19,
    imports: ["dev/devHash.ts", "dev/experienceEvidence.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationCertification.ts", "experience/adaptationComposition.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "adaptation",
  },
  {
    id: "experience/adaptationValidation.ts",
    lines: 651,
    exports: 17,
    imports: ["dev/experienceEvidence.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationCertification.ts", "experience/adaptationPacks.ts", "experience/experienceMath.ts", "experience/longitudinalAdaptation.ts", "experience/productionAdaptation.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "adaptation",
  },
  {
    id: "experience/anticipationPrediction.ts",
    lines: 119,
    exports: 7,
    imports: ["dev/experienceEvents.ts", "dev/experienceEvidence.ts", "experience/experienceMath.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "presentation",
  },
  {
    id: "experience/architecturalComplexity.ts",
    lines: 221,
    exports: 9,
    imports: [] as const,
    subsystem: "complexity",
  },
  {
    id: "experience/experienceMath.ts",
    lines: 33,
    exports: 5,
    imports: [] as const,
    subsystem: "presentation",
  },
  {
    id: "experience/index.ts",
    lines: 297,
    exports: 32,
    imports: ["experience/adaptationCertification.ts", "experience/adaptationComposition.ts", "experience/adaptationExperiments.ts", "experience/adaptationOperations.ts", "experience/adaptationPacks.ts", "experience/adaptationValidation.ts", "experience/architecturalComplexity.ts", "experience/experienceMath.ts", "experience/longitudinalAdaptation.ts", "experience/productionAdaptation.ts", "experience/useResolvedPresentation.ts", "experience/workspaceAdaptation.ts", "experience/workspaceAnticipation.ts", "experience/workspaceCalibration.ts", "experience/workspaceMemoryEvolution.ts", "experience/workspacePresence.ts", "experience/workspacePresentationStability.ts"] as const,
    subsystem: "presentation",
  },
  {
    id: "experience/longitudinalAdaptation.ts",
    lines: 517,
    exports: 16,
    imports: ["dev/experienceEvidence.ts", "dev/experienceStore.ts", "experience/experienceMath.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "adaptation",
  },
  {
    id: "experience/productionAdaptation.ts",
    lines: 636,
    exports: 11,
    imports: ["dev/devHash.ts", "dev/engineeringGovernance.ts", "dev/experienceEvents.ts", "dev/experienceEvidence.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationExperiments.ts", "experience/adaptationOperations.ts", "experience/longitudinalAdaptation.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "adaptation",
  },
  {
    id: "experience/useResolvedPresentation.ts",
    lines: 41,
    exports: 1,
    imports: ["experience/workspaceAdaptation.ts", "experience/workspaceAnticipation.ts"] as const,
    subsystem: "presentation",
  },
  {
    id: "experience/workspaceAdaptation.ts",
    lines: 857,
    exports: 39,
    imports: ["dev/architecturalIntegrity.ts", "dev/devHash.ts", "dev/engineeringGovernance.ts", "dev/experienceEvidence.ts", "dev/experienceGovernance.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts"] as const,
    subsystem: "adaptation",
  },
  {
    id: "experience/workspaceAnticipation.ts",
    lines: 439,
    exports: 14,
    imports: ["dev/experienceEvidence.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationComposition.ts", "experience/anticipationPrediction.ts", "experience/experienceMath.ts", "experience/workspaceAdaptation.ts", "experience/workspaceCalibration.ts", "experience/workspaceMemoryEvolution.ts", "experience/workspacePresence.ts", "experience/workspacePresentationStability.ts"] as const,
    subsystem: "presentation",
  },
  {
    id: "experience/workspaceCalibration.ts",
    lines: 381,
    exports: 16,
    imports: ["dev/experienceEvidence.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationComposition.ts", "experience/anticipationPrediction.ts", "experience/experienceMath.ts", "experience/workspaceMemoryEvolution.ts"] as const,
    subsystem: "presentation",
  },
  {
    id: "experience/workspaceMemoryEvolution.ts",
    lines: 481,
    exports: 15,
    imports: ["dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationCertification.ts", "experience/adaptationComposition.ts", "experience/adaptationPacks.ts", "experience/experienceMath.ts", "experience/workspaceAdaptation.ts"] as const,
    subsystem: "adaptation",
  },
  {
    id: "experience/workspacePresence.ts",
    lines: 374,
    exports: 12,
    imports: ["dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationComposition.ts", "experience/experienceMath.ts", "experience/workspaceAdaptation.ts", "experience/workspaceMemoryEvolution.ts"] as const,
    subsystem: "presentation",
  },
  {
    id: "experience/workspacePresentationStability.ts",
    lines: 440,
    exports: 13,
    imports: ["dev/experienceEvidence.ts", "dev/experienceStore.ts", "dev/governancePrimitives.ts", "experience/adaptationComposition.ts", "experience/experienceMath.ts", "experience/workspaceAdaptation.ts", "experience/workspaceCalibration.ts", "experience/workspaceMemoryEvolution.ts"] as const,
    subsystem: "presentation",
  },
];

/** Sprint 73 certification baseline metrics (pre-maintainability module). */
export const MAINTAINABILITY_BASELINE = {
  moduleIds: [
    "dev/architecturalIntegrity.ts",
    "dev/devHash.ts",
    "dev/engineeringCertification.ts",
    "dev/engineeringGovernance.ts",
    "dev/experienceEvents.ts",
    "dev/experienceEvidence.ts",
    "dev/experienceGovernance.ts",
    "dev/experienceImprovement.ts",
    "dev/experienceInstrumentation.ts",
    "dev/experienceStore.ts",
    "dev/experienceValidationGate.ts",
    "dev/frictionModel.ts",
    "dev/governancePrimitives.ts",
    "dev/governanceStore.ts",
    "dev/index.ts",
    "dev/sessionReplay.ts",
    "dev/traceAnalysis.ts",
    "experience/adaptationCertification.ts",
    "experience/adaptationComposition.ts",
    "experience/adaptationExperiments.ts",
    "experience/adaptationOperations.ts",
    "experience/adaptationPacks.ts",
    "experience/adaptationValidation.ts",
    "experience/anticipationPrediction.ts",
    "experience/architecturalComplexity.ts",
    "experience/experienceMath.ts",
    "experience/index.ts",
    "experience/longitudinalAdaptation.ts",
    "experience/productionAdaptation.ts",
    "experience/useResolvedPresentation.ts",
    "experience/workspaceAdaptation.ts",
    "experience/workspaceAnticipation.ts",
    "experience/workspaceCalibration.ts",
    "experience/workspaceMemoryEvolution.ts",
    "experience/workspacePresence.ts",
    "experience/workspacePresentationStability.ts",
  ] as const,
  dependencyEdgeCount: 198,
  invariantCount: 12,
  validationMethodCount: 12,
  invariantCoveragePercent: 100,
  validationCoveragePercent: 100,
} as const;

const FAN_OUT_THRESHOLD = 8;
const FAN_IN_THRESHOLD = 10;
const LARGEST_LIMIT = 8;
const SUBSYSTEM_KINDS: MaintainabilitySubsystem[] = [
  "authority",
  "governance",
  "evidence",
  "adaptation",
  "presentation",
  "certification",
  "complexity",
  "integrity",
];

function countCycles(
  inventory: readonly ModuleInventoryEntry[],
): string[][] {
  const adj = new Map(inventory.map((m) => [m.id, m.imports]));
  const color = new Map<string, 0 | 1 | 2>();
  const stack: string[] = [];
  const found: string[][] = [];

  function dfs(u: string): void {
    color.set(u, 1);
    stack.push(u);
    for (const v of adj.get(u) ?? []) {
      if (!adj.has(v)) continue;
      if (color.get(v) === 1) {
        const i = stack.indexOf(v);
        found.push([...stack.slice(i), v]);
      } else if (color.get(v) !== 2) {
        dfs(v);
      }
    }
    stack.pop();
    color.set(u, 2);
  }

  for (const m of inventory) {
    if (!color.has(m.id)) dfs(m.id);
  }
  return found.sort((a, b) => a.join(">").localeCompare(b.join(">")));
}

function maxDependencyDepth(
  inventory: readonly ModuleInventoryEntry[],
): number {
  const adj = new Map(inventory.map((m) => [m.id, m.imports]));
  const memo = new Map<string, number>();
  const visiting = new Set<string>();

  function depth(u: string): number {
    if (memo.has(u)) return memo.get(u)!;
    if (visiting.has(u)) return 0; // cycle — depth contribution ignored
    visiting.add(u);
    let best = 0;
    for (const v of adj.get(u) ?? []) {
      if (!adj.has(v)) continue;
      best = Math.max(best, 1 + depth(v));
    }
    visiting.delete(u);
    memo.set(u, best);
    return best;
  }

  let max = 0;
  for (const m of inventory) {
    max = Math.max(max, depth(m.id));
  }
  return max;
}

function fanInCount(
  inventory: readonly ModuleInventoryEntry[],
  id: string,
): number {
  return inventory.filter((m) => m.imports.includes(id)).length;
}

/**
 * Deterministic maintainability report — no I/O, no persistence.
 */
export function buildMaintainabilityReport(
  inventory: readonly ModuleInventoryEntry[] = MODULE_INVENTORY,
): MaintainabilityReport {
  const complexity = buildArchitecturalComplexityReport();
  const moduleCount = inventory.length;
  const totalLines = inventory.reduce((s, m) => s + m.lines, 0);
  const exportedSymbolCount = inventory.reduce((s, m) => s + m.exports, 0);
  const dependencyFanOutTotal = inventory.reduce(
    (s, m) => s + m.imports.length,
    0,
  );
  const largestModules = [...inventory]
    .sort((a, b) => b.lines - a.lines || a.id.localeCompare(b.id))
    .slice(0, LARGEST_LIMIT)
    .map((m) => ({ id: m.id, lines: m.lines, subsystem: m.subsystem }));

  const validationMethods = new Set(
    ENGINEERING_INVARIANTS.map((i) => i.validationMethod),
  );
  const scopes = new Set(ENGINEERING_INVARIANTS.map((i) => i.scope));
  const invSubsystems = new Set(
    ENGINEERING_INVARIANTS.map((i) => i.owningSubsystem),
  );
  const validationCoveragePercent = Math.round(
    (validationMethods.size / Math.max(1, ENGINEERING_INVARIANTS.length)) *
      100,
  );
  const invariantCoveragePercent = Math.round(
    (invSubsystems.size / SUBSYSTEM_KINDS.length) * 100,
  );

  const bySub = new Map<
    MaintainabilitySubsystem,
    { moduleCount: number; lines: number; exports: number }
  >();
  for (const m of inventory) {
    const entry = bySub.get(m.subsystem) ?? {
      moduleCount: 0,
      lines: 0,
      exports: 0,
    };
    entry.moduleCount += 1;
    entry.lines += m.lines;
    entry.exports += m.exports;
    bySub.set(m.subsystem, entry);
  }

  return {
    schemaVersion: 1,
    reportId: "arch-maintainability-v1",
    moduleCount,
    dependencyFanOutTotal,
    dependencyDepthMax: maxDependencyDepth(inventory),
    exportedSymbolCount,
    duplicatedImplementation: {
      clampRoundSites: complexity.after.duplicatedClampRoundSites,
      varianceSites: complexity.after.duplicatedVarianceSites,
      deprecatedAliases: complexity.after.deprecatedPresentationAliases,
    },
    averageModuleSize:
      moduleCount === 0 ? 0 : Math.round(totalLines / moduleCount),
    largestModules,
    validationCoverage: {
      validationMethodCount: validationMethods.size,
      invariantCount: ENGINEERING_INVARIANTS.length,
      percent: validationCoveragePercent,
    },
    invariantCoverage: {
      invariantCount: ENGINEERING_INVARIANTS.length,
      scopeCount: scopes.size,
      subsystemCount: invSubsystems.size,
      percent: invariantCoveragePercent,
    },
    subsystems: SUBSYSTEM_KINDS.filter((s) => bySub.has(s))
      .map((subsystem) => ({
        subsystem,
        ...bySub.get(subsystem)!,
      }))
      .sort((a, b) => a.subsystem.localeCompare(b.subsystem)),
  };
}

/**
 * Deterministic dependency health — derived from module inventory only.
 */
export function buildDependencyHealthReport(
  inventory: readonly ModuleInventoryEntry[] = MODULE_INVENTORY,
): DependencyHealthReport {
  const cycles = countCycles(inventory);
  const isolatedModules = inventory
    .filter(
      (m) => m.imports.length === 0 && fanInCount(inventory, m.id) === 0,
    )
    .map((m) => m.id)
    .sort();

  const highFanOutModules = inventory
    .filter((m) => m.imports.length >= FAN_OUT_THRESHOLD)
    .map((m) => ({
      id: m.id,
      count: m.imports.length,
      subsystem: m.subsystem,
    }))
    .sort((a, b) => b.count - a.count || a.id.localeCompare(b.id));

  const highFanInModules = inventory
    .map((m) => ({
      id: m.id,
      count: fanInCount(inventory, m.id),
      subsystem: m.subsystem,
    }))
    .filter((m) => m.count >= FAN_IN_THRESHOLD)
    .sort((a, b) => b.count - a.count || a.id.localeCompare(b.id));

  const architecturalBoundaryCrossings: BoundaryCrossing[] = [];
  for (const m of inventory) {
    for (const to of m.imports) {
      const fromExp = m.id.startsWith("experience/");
      const toExp = to.startsWith("experience/");
      if (fromExp === toExp) continue;
      architecturalBoundaryCrossings.push({
        from: m.id,
        to,
        kind: fromExp ? "experience_to_dev" : "dev_to_experience",
      });
    }
  }
  architecturalBoundaryCrossings.sort((a, b) =>
    (a.from + "->" + a.to).localeCompare(b.from + "->" + b.to),
  );

  const ownership = new Map<MaintainabilitySubsystem, string[]>();
  for (const m of inventory) {
    const list = ownership.get(m.subsystem) ?? [];
    list.push(m.id);
    ownership.set(m.subsystem, list);
  }

  return {
    schemaVersion: 1,
    reportId: "dependency-health-v1",
    circularDependencyCount: cycles.length,
    circularDependencies: cycles,
    isolatedModules,
    highFanOutModules,
    highFanInModules,
    architecturalBoundaryCrossings,
    subsystemOwnership: [...ownership.entries()]
      .map(([subsystem, modules]) => ({
        subsystem,
        modules: modules.sort(),
      }))
      .sort((a, b) => a.subsystem.localeCompare(b.subsystem)),
    edgeCount: inventory.reduce((s, m) => s + m.imports.length, 0),
  };
}

/**
 * Compare current maintainability metrics against the Sprint 73 baseline.
 * Engineering metrics only — no implementation diff.
 */
export function compareMaintainabilityTrend(
  current: MaintainabilityReport = buildMaintainabilityReport(),
  health: DependencyHealthReport = buildDependencyHealthReport(),
): MaintainabilityTrendReport {
  const currentIds = new Set(MODULE_INVENTORY.map((m) => m.id));
  const baselineIds = new Set<string>(MAINTAINABILITY_BASELINE.moduleIds);

  const modulesAdded = [...currentIds]
    .filter((id) => !baselineIds.has(id))
    .sort();
  const modulesRemoved = [...baselineIds]
    .filter((id) => !currentIds.has(id))
    .sort();

  const edgeDelta =
    health.edgeCount - MAINTAINABILITY_BASELINE.dependencyEdgeCount;
  const dependencyIncreases = Math.max(0, edgeDelta);
  const dependencyReductions = Math.max(0, -edgeDelta);

  const invariantCoverageDelta =
    current.invariantCoverage.percent -
    MAINTAINABILITY_BASELINE.invariantCoveragePercent;
  const validationCoverageDelta =
    current.validationCoverage.percent -
    MAINTAINABILITY_BASELINE.validationCoveragePercent;

  return {
    schemaVersion: 1,
    reportId: "maintainability-trend-v1",
    modulesAdded,
    modulesRemoved,
    dependencyIncreases,
    dependencyReductions,
    invariantCoverageDelta,
    validationCoverageDelta,
  };
}
