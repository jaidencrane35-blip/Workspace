export {
  clearInstrumentation,
  computeFrictionFromSession,
  disposeExperienceInstrumentation,
  frictionForSession,
  getActiveSession,
  initExperienceInstrumentation,
  isInstrumentationActive,
  listStoredSessions,
  replayActiveSession,
  replayStoredSession,
  trackCommand,
  trackContinueSuccess,
  trackFlowStart,
  trackNavigate,
  trackSaveSuccess,
} from "./experienceInstrumentation";
export { isExperienceValidationEnabled } from "./experienceValidationGate";
export { FORBIDDEN_EVENT_KEYS } from "./experienceEvents";
export { computeFrictionScore } from "./frictionModel";
export { replaySession, frictionEquals } from "./sessionReplay";
export {
  STORAGE_KEY as EXPERIENCE_VALIDATION_STORAGE_KEY,
  memoryStore,
} from "./experienceStore";
export { analyzeTraces } from "./traceAnalysis";
export {
  EVIDENCE_STORAGE_KEY,
  COMPARABLE_METRICS,
  aggregateToEvidence,
  buildEvidenceFromSessions,
  compareEvidence,
  clearEvidenceStore,
  defaultEvidenceStore,
  getEvidenceBaseline,
  getReplayInvocationCount,
  listEvidenceSnapshots,
  persistEvidenceSnapshot,
  recordReplayInvocation,
  setEvidenceBaseline,
  verdictForMetric,
} from "./experienceEvidence";
export type {
  ExperienceEvidence,
  ExperienceEvidenceMetrics,
  EvidenceComparison,
  MetricComparison,
  MetricVerdict,
} from "./experienceEvidence";
export type { TraceAggregate } from "./traceAnalysis";
export {
  OPPORTUNITY_RULES,
  LONGITUDINAL_SIGNIFICANCE,
  detectOpportunities,
  evolveBaselines,
  evolutionVerdictFor,
  opportunitiesHaveReplayLinkage,
} from "./experienceImprovement";
export type {
  ExperienceOpportunity,
  BaselineEvolution,
  MetricEvolution,
  EvolutionVerdict,
  OpportunitySeverity,
  OpportunityWorkflow,
} from "./experienceImprovement";
export {
  GOVERNANCE_STORAGE_KEY,
  buildProposalFromOpportunities,
  evaluateValidationContract,
  persistProposal,
  transitionProposal,
  listProposals,
  getProposal,
  listProposalHistory,
  listAllHistory,
  assertHistoryImmutable,
  clearGovernanceStore,
  defaultGovernanceStore,
  isTransitionAllowed,
} from "./experienceGovernance";
export type {
  ExperienceChangeProposal,
  ProposalLifecycle,
  ProposalValidationContract,
  GovernanceHistoryEntry,
  GovernanceReason,
  ValidationStatus,
  TransitionError,
  ImplementationScope,
  AffectedComponent,
} from "./experienceGovernance";
export {
  ENGINEERING_STORAGE_KEY,
  ARCHITECTURE_AUTHORITY_DOCS,
  buildEngineeringRecordFromProposals,
  verifyEngineeringConsistency,
  buildReleaseTraceability,
  persistEngineeringRecord,
  transitionEngineeringRecord,
  listEngineeringRecords,
  getEngineeringRecord,
  listEngineeringHistory,
  listAllEngineeringHistory,
  assertEngineeringHistoryImmutable,
  assertNoOrphanReleasedRecords,
  clearEngineeringStore,
  defaultEngineeringStore,
  isEngineeringTransitionAllowed,
  isArchitectureAuthorityDoc,
} from "./engineeringGovernance";
export type {
  EngineeringChangeRecord,
  EngineeringLifecycle,
  EngineeringValidationEvidence,
  EngineeringHistoryEntry,
  ReleaseTraceability,
  ConsistencyResult,
  ConsistencyError,
  ArchitectureAuthorityDoc,
  ReleaseImpact,
} from "./engineeringGovernance";
export {
  ARCHITECTURE_INTEGRITY_STORAGE_KEY,
  AUTHORITY_CHAIN,
  AUTHORITY_ROOT,
  buildArchitectureGraph,
  validateArchitectureIntegrity,
  hashArchitectureGraph,
  createArchitectureSnapshot,
  persistArchitectureSnapshot,
  listArchitectureSnapshots,
  assertSnapshotsImmutable,
  listAuthoritySuccessors,
  listDependencies,
  clearArchitectureIntegrityStore,
  defaultArchitectureIntegrityStore,
} from "./architecturalIntegrity";
export type {
  ArchitectureGraph,
  ArchitectureNode,
  ArchitectureEdge,
  ArchitectureNodeKind,
  ArchitectureEdgeType,
  ArchitectureGraphSource,
  ArchitectureSnapshot,
  IntegrityResult,
  IntegrityViolation,
  IntegrityViolationCode,
} from "./architecturalIntegrity";
