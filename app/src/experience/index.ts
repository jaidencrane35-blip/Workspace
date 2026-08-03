export {
  ADAPTATION_STORAGE_KEY,
  ADAPTATION_CHANGED_EVENT,
  activateAdaptation,
  buildAdaptationFromLineage,
  clearAdaptationStore,
  deactivateAdaptation,
  defaultAdaptationStore,
  getAdaptation,
  identityPresentation,
  listAdaptations,
  persistAdaptation,
  presentationToShellStyle,
  applyPresentationLayer,
  finalizeResolvedPresentation,
  resolvePresentationConfiguration,
  rollbackAdaptation,
  upsertValidatedAdaptation,
  validateAdaptationEvidence,
  verifyAdaptationLineage,
} from "./workspaceAdaptation";
export type {
  WorkspaceAdaptation,
  PresentationConfiguration,
  ResolvedPresentation,
  AdaptationScope,
  AdaptationRolloutState,
  AdaptationTargetComponent,
  AdaptationValidationContract,
  AdaptationLineageContext,
  AdaptationLineageError,
  AdaptationStabilityReport,
  LongitudinalAdaptationRecord,
  RollbackCriterion,
} from "./workspaceAdaptation";
export { useResolvedPresentation } from "./useResolvedPresentation";
export {
  ADAPTATION_EXPERIMENT_SPECS,
  EXPERIMENT_STORAGE_KEY,
  clearExperimentStore,
  experimentRollbackReady,
  getExperimentSummary,
  listExperimentResults,
  materializeExperimentLineage,
  runAdaptationExperiments,
  selectEligibleProposals,
  selectExperimentSpecs,
  toggleAdaptationExperiment,
} from "./adaptationExperiments";
export type {
  AdaptationExperimentResult,
  AdaptationExperimentSpec,
  ExperimentRunSummary,
  ExperimentStatus,
  RolloutDisposition,
} from "./adaptationExperiments";
export {
  LONGITUDINAL_MIN_OBSERVATIONS,
  LONGITUDINAL_STABILITY_THRESHOLD,
  LONGITUDINAL_MAX_UNRESOLVED_REGRESSIONS,
  analyzeAdaptationStability,
  appendLongitudinalObservation,
  buildLongitudinalRecord,
  getLongitudinalRecord,
  getStabilityReport,
  isRolloutReady,
  listLongitudinalRecords,
  listRolloutCandidates,
  listStabilityReports,
  promoteToRolloutCandidate,
  runLongitudinalValidation,
} from "./longitudinalAdaptation";
export type { LongitudinalTransitionError } from "./longitudinalAdaptation";
export {
  buildAdaptationCatalog,
  deriveOperationalHealth,
  getCatalogEntry,
  runBatchValidation,
} from "./adaptationOperations";
export type {
  AdaptationCatalog,
  AdaptationCatalogEntry,
  AdaptationOperationalHealth,
  BatchValidationCause,
  BatchValidationItem,
  BatchValidationOutcome,
  BatchValidationReport,
  CatalogDisposition,
} from "./adaptationOperations";
export {
  PRODUCTION_ACTIVATION_STORAGE_KEY,
  clearProductionActivationStore,
  ensureLongitudinalEligibility,
  getProductionActivationRecord,
  rollbackProductionAdaptation,
  runFirstProductionAdaptation,
  selectProductionAdaptation,
} from "./productionAdaptation";
export type {
  ProductionActivationOutcome,
  ProductionActivationRecord,
  ProductionBlockReason,
  ProductionSelection,
} from "./productionAdaptation";
export {
  analyzeAdaptationConflicts,
  composeAdaptations,
  composeAdaptationsFromStore,
  compositionPriorityRank,
  selectComposableAdaptations,
  validateComposition,
} from "./adaptationComposition";
export type {
  AdaptationConflict,
  AdaptationConflictReport,
  CompositionLineage,
  CompositionResult,
  CompositionValidationReport,
  ConflictCause,
  ResolutionStrategy,
} from "./adaptationComposition";
export {
  CERTIFICATION_STORAGE_KEY,
  certifyAdaptationSet,
  clearCertificationStore,
  compareCertifications,
  compareLatestCertifications,
  currentAdaptationSetIds,
  getCertification,
  getLatestCertification,
  hashAdaptationSet,
  listCertifications,
} from "./adaptationCertification";
export type {
  AdaptationCertification,
  CertificationComparison,
  CertificationEvidenceLineage,
  CertificationFailureReason,
  CertificationGateResult,
  CertificationRegression,
  CertificationRegressionCause,
  CertificationRegressionStatus,
} from "./adaptationCertification";
export {
  PACK_STORAGE_KEY,
  activateAdaptationPack,
  certifyAdaptationPack,
  clearAdaptationPackStore,
  composePackCandidate,
  deactivateAdaptationPack,
  getActiveAdaptationPack,
  getAdaptationPack,
  listAdaptationPacks,
  packActivationReady,
} from "./adaptationPacks";
export type {
  PackActivationError,
  PackActivationResult,
  PackCertificationFailureReason,
  PackCertificationResult,
  PackEvidenceSummary,
  PackRolloutStatus,
  PackStabilitySummary,
  WorkspaceAdaptationPack,
} from "./adaptationPacks";
export {
  buildAdaptationPerformanceReport,
  buildCertificationLongevityReport,
  buildEvidenceInventory,
  buildLongitudinalTrendReport,
  buildPackEffectivenessReport,
  buildRealWorldValidationBundle,
} from "./adaptationValidation";
export type {
  AdaptationPerformanceEntry,
  AdaptationPerformanceReport,
  CertificationLongevityEntry,
  CertificationLongevityReport,
  EvidenceInventory,
  EvidenceInventoryEntry,
  LongitudinalTrendPoint,
  LongitudinalTrendReport,
  PackEffectivenessCohort,
  PackEffectivenessReport,
  StabilityTrend,
} from "./adaptationValidation";
export {
  deriveActiveMemoryEvolution,
  evolutionCompositionFingerprint,
  getActiveMemoryEvolution,
  listMemoryEvolutions,
  presentationFromEvolution,
  replayMemoryEvolution,
  resolvePresentationFromRuntime,
  selectEvolutionMembers,
  validateMemoryEvolution,
} from "./workspaceMemoryEvolution";
export type {
  EvolutionCertificationLineage,
  EvolutionEvidenceLineage,
  EvolutionPresentationDelta,
  EvolutionValidation,
  EvolutionValidationFailure,
  WorkspaceMemoryEvolution,
} from "./workspaceMemoryEvolution";
export {
  deriveWorkspacePresence,
  getActiveWorkspacePresence,
  identityPresence,
  presenceContributorsExist,
  presentationFromPresence,
  replayWorkspacePresence,
  resolvePresentationWithPresence,
  validateWorkspacePresence,
} from "./workspacePresence";
export type {
  PresenceResolvedEnvironment,
  PresenceValidation,
  PresenceValidationFailure,
  WorkspacePresence,
} from "./workspacePresence";
