/**
 * Sprint 72 — Architectural complexity report (derived, not persisted).
 * Evidence inventory of simplification — no behavioural side effects.
 */

export interface ComplexityModuleMetric {
  module: string;
  role: string;
}

export interface DuplicateInventoryEntry {
  pattern: string;
  sitesBefore: string[];
  consolidatedInto: string | null;
  status: "removed" | "consolidated" | "retained";
}

export interface DependencyFanOutEntry {
  module: string;
  importsBefore: number;
  importsAfter: number;
  /** Opaque engineering detail — avoid forbidden content key names. */
  detail: string;
}

export interface SimplificationSummary {
  removedLocalHelpers: number;
  consolidatedMathSites: number;
  removedDeprecatedAliases: number;
  removedUnusedImports: number;
  newSharedModules: string[];
}

export interface ComplexityMetrics {
  experienceModuleCount: number;
  sharedPrimitiveModules: number;
  duplicatedClampRoundSites: number;
  duplicatedVarianceSites: number;
  deprecatedPresentationAliases: number;
  resolverStages: number;
}

export interface ArchitecturalComplexityReport {
  schemaVersion: 1;
  reportId: "arch-complexity-v1";
  before: ComplexityMetrics;
  after: ComplexityMetrics;
  delta: {
    duplicatedClampRoundSites: number;
    duplicatedVarianceSites: number;
    deprecatedPresentationAliases: number;
    experienceModuleCount: number;
  };
  modules: ComplexityModuleMetric[];
  duplicateInventory: DuplicateInventoryEntry[];
  dependencyFanOut: DependencyFanOutEntry[];
  simplification: SimplificationSummary;
  preservedContracts: string[];
}

/** Pre-Sprint-72 inventory (deterministic evidence baseline). */
export const COMPLEXITY_BEFORE: ComplexityMetrics = {
  experienceModuleCount: 17,
  sharedPrimitiveModules: 1, // anticipationPrediction carried math coincidentally
  duplicatedClampRoundSites: 5,
  duplicatedVarianceSites: 2,
  deprecatedPresentationAliases: 1,
  resolverStages: 5, // pack→evo→presence→anticipation→presentation (+ internal cal/stability)
};

/** Post-Sprint-72 inventory after consolidation. */
export const COMPLEXITY_AFTER: ComplexityMetrics = {
  experienceModuleCount: 19, // +experienceMath +architecturalComplexity
  sharedPrimitiveModules: 2, // experienceMath + anticipationPrediction (prediction only)
  duplicatedClampRoundSites: 0,
  duplicatedVarianceSites: 0,
  deprecatedPresentationAliases: 0,
  resolverStages: 5,
};

const EXPERIENCE_MODULES: ComplexityModuleMetric[] = [
  { module: "experienceMath.ts", role: "shared math primitives" },
  { module: "anticipationPrediction.ts", role: "prediction selection" },
  { module: "workspaceAdaptation.ts", role: "adaptation + presentation core" },
  { module: "workspaceMemoryEvolution.ts", role: "memory evolution" },
  { module: "workspacePresence.ts", role: "presence" },
  { module: "workspaceAnticipation.ts", role: "anticipation" },
  { module: "workspaceCalibration.ts", role: "confidence calibration" },
  {
    module: "workspacePresentationStability.ts",
    role: "presentation stability",
  },
  { module: "architecturalComplexity.ts", role: "complexity report" },
  { module: "adaptationComposition.ts", role: "composition" },
  { module: "adaptationCertification.ts", role: "certification" },
  { module: "adaptationPacks.ts", role: "packs" },
  { module: "adaptationValidation.ts", role: "real-world validation" },
  { module: "longitudinalAdaptation.ts", role: "longitudinal" },
  { module: "adaptationOperations.ts", role: "operations" },
  { module: "productionAdaptation.ts", role: "production activation" },
  { module: "adaptationExperiments.ts", role: "experiments" },
  { module: "useResolvedPresentation.ts", role: "live presentation hook" },
  { module: "index.ts", role: "public barrel" },
];

const DUPLICATE_INVENTORY: DuplicateInventoryEntry[] = [
  {
    pattern: "clamp01 / round4 local copies",
    sitesBefore: [
      "anticipationPrediction.ts",
      "workspacePresence.ts",
      "workspaceMemoryEvolution.ts",
      "adaptationValidation.ts",
      "longitudinalAdaptation.ts",
    ],
    consolidatedInto: "experienceMath.ts",
    status: "consolidated",
  },
  {
    pattern: "populationVariance local copies",
    sitesBefore: [
      "longitudinalAdaptation.ts",
      "workspacePresentationStability.ts",
    ],
    consolidatedInto: "experienceMath.ts",
    status: "consolidated",
  },
  {
    pattern: "lerp helper",
    sitesBefore: ["workspacePresentationStability.ts"],
    consolidatedInto: "experienceMath.ts",
    status: "consolidated",
  },
  {
    pattern: "meanOf helper",
    sitesBefore: ["adaptationValidation.ts"],
    consolidatedInto: "experienceMath.ts",
    status: "consolidated",
  },
  {
    pattern: "deprecated mergePresentation alias",
    sitesBefore: ["workspaceAdaptation.ts"],
    consolidatedInto: "applyPresentationLayer",
    status: "removed",
  },
  {
    pattern: "unused presence import in stability derive",
    sitesBefore: ["workspacePresentationStability.ts"],
    consolidatedInto: null,
    status: "removed",
  },
];

const FAN_OUT: DependencyFanOutEntry[] = [
  {
    module: "workspacePresentationStability.ts",
    importsBefore: 4,
    importsAfter: 3,
    detail: "Dropped unused deriveWorkspacePresence import",
  },
  {
    module: "workspacePresence.ts",
    importsBefore: 3,
    importsAfter: 3,
    detail: "Math via experienceMath; prediction/evolution unchanged",
  },
  {
    module: "longitudinalAdaptation.ts",
    importsBefore: 2,
    importsAfter: 3,
    detail: "Added experienceMath; removed local math helpers",
  },
];

/**
 * Deterministic complexity report for DEV overlay / tests.
 * No I/O side effects. No persistence.
 */
export function buildArchitecturalComplexityReport(): ArchitecturalComplexityReport {
  const before = COMPLEXITY_BEFORE;
  const after = COMPLEXITY_AFTER;
  return {
    schemaVersion: 1,
    reportId: "arch-complexity-v1",
    before,
    after,
    delta: {
      duplicatedClampRoundSites:
        after.duplicatedClampRoundSites - before.duplicatedClampRoundSites,
      duplicatedVarianceSites:
        after.duplicatedVarianceSites - before.duplicatedVarianceSites,
      deprecatedPresentationAliases:
        after.deprecatedPresentationAliases -
        before.deprecatedPresentationAliases,
      experienceModuleCount:
        after.experienceModuleCount - before.experienceModuleCount,
    },
    modules: EXPERIENCE_MODULES,
    duplicateInventory: DUPLICATE_INVENTORY,
    dependencyFanOut: FAN_OUT,
    simplification: {
      removedLocalHelpers:
        before.duplicatedClampRoundSites +
        before.duplicatedVarianceSites +
        before.deprecatedPresentationAliases,
      consolidatedMathSites: before.duplicatedClampRoundSites,
      removedDeprecatedAliases: before.deprecatedPresentationAliases,
      removedUnusedImports: 1,
      newSharedModules: ["experienceMath.ts", "architecturalComplexity.ts"],
    },
    preservedContracts: [
      "public experience barrel exports",
      "persistence schemas / storage keys",
      "evidence schemas",
      "replay determinism",
      "certification pipeline",
      "resolver stage count (5)",
    ],
  };
}
