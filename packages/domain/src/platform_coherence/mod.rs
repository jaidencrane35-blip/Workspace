//! Canonical concept ownership registry (Phase 4 Batch 9.5).
//!
//! Documents which subsystem owns each Workspace concept. Aggregators
//! (Decision Queue, Activity Graph, Intelligence) consume — they do not own.

/// Authoritative owner kind for a Workspace platform concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptOwnerKind {
    /// Persisted in SQLite by a dedicated service/repository.
    DurableStore,
    /// Aggregator / overlay only — never a payload source of truth.
    Aggregator,
    /// Derived from audit / session stores; not a second durable entity table.
    Derived,
}

/// One row in the platform ownership map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConceptOwnership {
    pub concept: &'static str,
    pub owner: &'static str,
    pub kind: ConceptOwnerKind,
}

/// Exactly-one-owner map for major Workspace concepts.
pub const PLATFORM_CONCEPT_OWNERS: &[ConceptOwnership] = &[
    ConceptOwnership {
        concept: "workspace",
        owner: "WorkspaceService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "project",
        owner: "WorkspaceIntentService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "task",
        owner: "WorkspaceIntentService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "work_goal",
        owner: "WorkspaceIntentService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_node",
        owner: "WorkspaceCognitiveModelService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_relation",
        owner: "WorkspaceCognitiveModelService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_model",
        owner: "WorkspaceCognitiveModelService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "planning_plan",
        owner: "WorkspacePlanningService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "planning_snapshot",
        owner: "WorkspacePlanningService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "planning_proposal",
        owner: "WorkspacePlanningService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "reasoning_record",
        owner: "WorkspaceReasoningMemoryService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "reasoning_memory",
        owner: "WorkspaceReasoningMemoryService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_graph",
        owner: "WorkspaceCognitiveGraphService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_graph_snapshot",
        owner: "WorkspaceCognitiveGraphService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_orchestration",
        owner: "WorkspaceCognitiveOrchestrationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "orchestration_snapshot",
        owner: "WorkspaceCognitiveOrchestrationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "learning_adaptation",
        owner: "WorkspaceLearningAdaptationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "learning_snapshot",
        owner: "WorkspaceLearningAdaptationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_agent_cast",
        owner: "WorkspaceCognitiveAgentCastService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_agent_cast_snapshot",
        owner: "WorkspaceCognitiveAgentCastService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_autonomy",
        owner: "WorkspaceCognitiveAutonomyService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cognitive_autonomy_snapshot",
        owner: "WorkspaceCognitiveAutonomyService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "workspace_state_envelope",
        owner: "WorkspaceStateCompositionService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "workspace_state_snapshot",
        owner: "WorkspaceStateCompositionService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "policy_governance",
        owner: "PolicyGovernanceService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "policy_governance_snapshot",
        owner: "PolicyGovernanceService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "historical_reconstruction",
        owner: "WorkspaceHistoricalReconstructionService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "historical_reconstruction_snapshot",
        owner: "WorkspaceHistoricalReconstructionService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "temporal_intelligence",
        owner: "WorkspaceTemporalIntelligenceService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "temporal_intelligence_snapshot",
        owner: "WorkspaceTemporalIntelligenceService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "workspace_explanation",
        owner: "WorkspaceExplanationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "workspace_explanation_snapshot",
        owner: "WorkspaceExplanationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "contextual_understanding",
        owner: "WorkspaceContextualUnderstandingService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "contextual_understanding_snapshot",
        owner: "WorkspaceContextualUnderstandingService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "knowledge_synthesis",
        owner: "WorkspaceKnowledgeSynthesisService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "knowledge_synthesis_snapshot",
        owner: "WorkspaceKnowledgeSynthesisService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "knowledge_integration",
        owner: "WorkspaceKnowledgeIntegrationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "knowledge_integration_snapshot",
        owner: "WorkspaceKnowledgeIntegrationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "insight_coordination",
        owner: "WorkspaceInsightCoordinationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "insight_coordination_snapshot",
        owner: "WorkspaceInsightCoordinationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cross_workspace_intelligence",
        owner: "WorkspaceCrossIntelligenceService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "cross_workspace_intelligence_snapshot",
        owner: "WorkspaceCrossIntelligenceService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "decision_support",
        owner: "WorkspaceDecisionSupportService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "decision_support_snapshot",
        owner: "WorkspaceDecisionSupportService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "intelligence_hub",
        owner: "WorkspaceIntelligenceHubService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "intelligence_hub_snapshot",
        owner: "WorkspaceIntelligenceHubService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "semantic_query",
        owner: "WorkspaceSemanticQueryService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "semantic_query_snapshot",
        owner: "WorkspaceSemanticQueryService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_navigation",
        owner: "WorkspaceEvidenceNavigationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_navigation_snapshot",
        owner: "WorkspaceEvidenceNavigationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_trace",
        owner: "WorkspaceEvidenceTraceService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_trace_snapshot",
        owner: "WorkspaceEvidenceTraceService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_coverage",
        owner: "WorkspaceEvidenceCoverageService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_coverage_snapshot",
        owner: "WorkspaceEvidenceCoverageService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_consistency",
        owner: "WorkspaceEvidenceConsistencyService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_consistency_snapshot",
        owner: "WorkspaceEvidenceConsistencyService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_dependency",
        owner: "WorkspaceEvidenceDependencyService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_dependency_snapshot",
        owner: "WorkspaceEvidenceDependencyService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_freshness",
        owner: "WorkspaceEvidenceFreshnessService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_freshness_snapshot",
        owner: "WorkspaceEvidenceFreshnessService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_completeness",
        owner: "WorkspaceEvidenceCompletenessService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "evidence_completeness_snapshot",
        owner: "WorkspaceEvidenceCompletenessService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "automation_contract",
        owner: "AutomationContractService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "trigger_event",
        owner: "TriggerEvaluatorService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "intent_proposal",
        owner: "TriggerEvaluatorService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "decision_item",
        owner: "DecisionQueueService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "activity",
        owner: "WorkspaceActivityGraphService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "continuity",
        owner: "WorkspaceContinuityService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "attention",
        owner: "WorkspaceAttentionService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "decision_candidate",
        owner: "DecisionEngineService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "workspace_task",
        owner: "TaskGraphService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "desktop_observation_pass",
        owner: "WorkspaceObservationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "observed_window",
        owner: "WorkspaceObservationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "observed_monitor",
        owner: "WorkspaceObservationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "window_identity_registry",
        owner: "WorkspaceObservationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "environment",
        owner: "WorkspaceEnvironmentService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "composition",
        owner: "WorkspaceCompositionService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "purpose",
        owner: "WorkspacePurposeService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "evolution",
        owner: "WorkspaceEvolutionService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "recommendation_candidate",
        owner: "WorkspaceRecommendationEngineService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "operating_state",
        owner: "WorkspaceOperatingStateService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "pattern",
        owner: "WorkspacePatternService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "adaptation_proposal",
        owner: "WorkspaceAdaptationService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "readiness",
        owner: "WorkspaceReadinessService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "session",
        owner: "WorkspaceSessionService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "experience",
        owner: "WorkspaceExperienceService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "work_context",
        owner: "WorkspaceWorkContextService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "navigation",
        owner: "WorkspaceNavigationService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "milestones",
        owner: "WorkspaceMilestoneService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "working_style",
        owner: "WorkspaceWorkingStyleService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "transition",
        owner: "WorkspaceTransitionService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "interaction",
        owner: "WorkspaceInteractionService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "workspace_profile",
        owner: "WorkspaceProfileService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "permission_approval",
        owner: "PermissionApprovalService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "execution_outcome",
        owner: "ExecutionOutcomeService",
        kind: ConceptOwnerKind::Derived,
    },
    ConceptOwnership {
        concept: "memory",
        owner: "AiMemoryService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "preference",
        owner: "AiPersonalizationService",
        kind: ConceptOwnerKind::DurableStore,
    },
    ConceptOwnership {
        concept: "workspace_runtime_context",
        owner: "WorkspaceRuntimeService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "workspace_runtime_health",
        owner: "WorkspaceRuntimeService",
        kind: ConceptOwnerKind::Aggregator,
    },
    ConceptOwnership {
        concept: "governance_runtime_summary",
        owner: "Governance (visible summaries only)",
        kind: ConceptOwnerKind::Derived,
    },
    ConceptOwnership {
        concept: "workspace_runtime_operator_view",
        owner: "WorkspaceRuntimeService",
        kind: ConceptOwnerKind::Aggregator,
    },
];

/// Canonical user-facing vocabulary tokens (Batch 9.5).
pub mod vocabulary {
    pub const WORK_GOAL: &str = "Work Goal";
    pub const ASSISTANT_GOAL: &str = "Assistant Goal";
    pub const TASK: &str = "Task";
    pub const ACTION: &str = "Action";
    pub const AUTOMATION_CONTRACT: &str = "Automation Contract";
    pub const INTENT_PROPOSAL: &str = "Intent Proposal";
    pub const ACTION_PROPOSAL: &str = "Action Proposal";
    pub const DECISION: &str = "Decision";
    pub const PERMISSION_APPROVAL: &str = "Permission Approval";
    pub const CONTRACT_APPROVAL: &str = "Contract Approval";
    pub const RECOMMENDATION: &str = "Recommendation";
    pub const INTENT: &str = "Intent";
    pub const ACTIVITY: &str = "Activity";
    pub const CONTINUITY: &str = "Continuity";
    pub const CURRENT_FOCUS: &str = "Current Focus";
    pub const INTERRUPTED_WORK: &str = "Interrupted Work";
    pub const RESUMABLE_WORK: &str = "Resumable Work";
    pub const ATTENTION: &str = "Attention";
    pub const DECISION_ENGINE: &str = "Decision Engine";
    pub const DECISION_CANDIDATE: &str = "Decision Candidate";
    pub const TASK_GRAPH: &str = "Task Graph";
    pub const WORKSPACE_TASK: &str = "Workspace Task";
    pub const DESKTOP_OBSERVATION: &str = "Desktop Observation";
    pub const OBSERVED_WINDOW: &str = "Observed Window";
    pub const OBSERVED_MONITOR: &str = "Observed Monitor";
    pub const WINDOW_IDENTITY: &str = "Window Identity";
    pub const ENVIRONMENT: &str = "Environment";
    pub const ENVIRONMENT_WINDOW: &str = "Environment Window";
    pub const COMPOSITION: &str = "Composition";
    pub const WORKING_ENVIRONMENT: &str = "Working Environment";
    pub const PURPOSE: &str = "Purpose";
    pub const WORKSPACE_PURPOSE: &str = "Workspace Purpose";
    pub const EVOLUTION: &str = "Evolution";
    pub const WORKSPACE_EVOLUTION: &str = "Workspace Evolution";
    pub const RECOMMENDATION_ENGINE: &str = "Recommendation Engine";
    pub const RECOMMENDATION_CANDIDATE: &str = "Recommendation Candidate";
    pub const OPERATING_STATE: &str = "Operating State";
    pub const WORKSPACE_OPERATING_STATE: &str = "Workspace Operating State";
    pub const PATTERN: &str = "Pattern";
    pub const WORKSPACE_PATTERN: &str = "Workspace Pattern";
    pub const ADAPTATION: &str = "Adaptation";
    pub const ADAPTATION_PROPOSAL: &str = "Adaptation Proposal";
    pub const READINESS: &str = "Readiness";
    pub const WORKSPACE_READINESS: &str = "Workspace Readiness";
    pub const SESSION: &str = "Session";
    pub const WORKSPACE_SESSION: &str = "Workspace Session";
    pub const EXPERIENCE: &str = "Experience";
    pub const WORKSPACE_EXPERIENCE: &str = "Workspace Experience";
    pub const WORK_CONTEXT: &str = "Work Context";
    pub const WORKSPACE_WORK_CONTEXT: &str = "Workspace Work Context";
    pub const NAVIGATION: &str = "Navigation";
    pub const WORKSPACE_NAVIGATION: &str = "Workspace Navigation";
    pub const MILESTONE: &str = "Milestone";
    pub const WORKSPACE_MILESTONES: &str = "Workspace Milestones";
    pub const WORKING_STYLE: &str = "Working Style";
    pub const WORKSPACE_WORKING_STYLE: &str = "Workspace Working Style";
    pub const TRANSITION: &str = "Transition";
    pub const WORKSPACE_TRANSITION: &str = "Workspace Transition";
    pub const INTERACTION: &str = "Interaction";
    pub const WORKSPACE_INTERACTION: &str = "Workspace Interaction";
    pub const WORKSPACE_PROFILE: &str = "Workspace Profile";
    pub const ENVIRONMENT_PROFILE: &str = "Environment Profile";
}
