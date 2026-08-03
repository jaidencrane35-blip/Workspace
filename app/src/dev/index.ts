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
