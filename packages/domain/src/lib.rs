//! Shared Workspace domain models — no database or UI logic.

pub mod action_catalog;
pub mod action_proposal;
pub mod actor;
pub mod ai_assistant;
pub mod ai_evaluation;
pub mod ai_memory;
pub mod ai_model;
pub mod ai_orchestration;
pub mod ai_personalization;
pub mod ai_planning;
pub mod ai_request;
pub mod analytics;
pub mod audit;
pub mod automation_contract;
pub mod automation_trigger;
pub mod decision_engine;
pub mod decision_queue;
pub mod capability;
pub mod context;
pub mod discovery;
pub mod entities;
pub mod errors;
pub mod execution_cancellation;
pub mod execution_context;
pub mod execution_guard;
pub mod execution_outcome;
pub mod execution_reconciliation;
pub mod graph;
pub mod ids;
pub mod intent;
pub mod intent_execution;
pub mod layout;
pub mod observation;
pub mod permission_approval;
pub mod platform_coherence;
pub mod projection;
pub mod projection_contract;
pub mod recovery_contract;
pub mod resource;
pub mod suggestion;
pub mod suggestion_intent;
pub mod suggestion_lifecycle;
pub mod workspace;
pub mod workspace_activity;
pub mod workspace_attention;
pub mod workspace_cognitive_model;
pub mod workspace_planning;
pub mod workspace_reasoning_memory;
pub mod workspace_cognitive_graph;
pub mod workspace_cognitive_orchestration;
pub mod workspace_learning_adaptation;
pub mod workspace_cognitive_agent_cast;
pub mod workspace_cognitive_autonomy;
pub mod workspace_state_envelope;
pub mod policy_governance;
pub mod workspace_historical_reconstruction;
pub mod workspace_temporal_intelligence;
pub mod workspace_explanation;
pub mod workspace_contextual_understanding;
pub mod workspace_knowledge_synthesis;
pub mod workspace_knowledge_integration;
pub mod workspace_insight_coordination;
pub mod workspace_composition;
pub mod workspace_purpose;
pub mod workspace_evolution;
pub mod workspace_recommendation;
pub mod workspace_operating_state;
pub mod workspace_pattern;
pub mod workspace_adaptation;
pub mod workspace_readiness;
pub mod workspace_session;
pub mod workspace_experience;
pub mod workspace_work_context;
pub mod workspace_navigation;
pub mod workspace_milestone;
pub mod workspace_working_style;
pub mod workspace_transition;
pub mod workspace_interaction;
pub mod workspace_profile;
pub mod workspace_observation;
pub mod workspace_observation_delta;
pub mod workspace_observation_event;
pub mod workspace_state;
pub mod workspace_continuity;
pub mod workspace_task_graph;
pub mod workspace_environment;
pub mod workspace_intent;
pub mod workspace_intelligence;
pub mod workspace_runtime;

pub use action_catalog::{
    ActionCatalog, ActionCatalogEntry, ActionCatalogError, AiActionAwareness,
};
pub use action_proposal::{
    ActionProposal, ActionProposalError, ActionProposalRisk, AdaptationReviewAuditEvent,
    AdaptationReviewerIdentity, AdaptationRiskClass, BehaviourVersion, BehaviourVersionLifecycle,
    ChangeEvaluation, ChangeRollbackMetadata, ControlledChangeAuditMetadata,
    ControlledChangeSurface, GOVERNANCE_AUTHORITY_EFFECT_NONE, GovernanceActorRefs,
    GovernanceAggregateRoot, GovernanceArchiveContract, GovernanceArchiveKind,
    GovernanceBoundaryOwner, GovernanceCompatibilityContract, GovernanceComplianceCheckKind,
    GovernanceComplianceContract,
    GovernanceComplianceDiagnostic, GovernanceComplianceSeverity, GovernanceConditionContract,
    GovernanceConflictEntry, GovernanceConflictKind, GovernanceConflictResolutionContract,
    GovernanceConflictState, GovernanceConsensusRule, GovernanceDashboardComplianceStatus,
    GovernanceDecisionEvidence, GovernanceDecisionHistoryEntry, GovernanceDecisionPackage,
    GovernanceDelegationAuditEvent, GovernanceDelegationContract, GovernanceDelegationRecord,
    GovernanceDelegationStatus, GovernanceDependencyKind, GovernanceDependencyRequirement,
    GovernanceDissentRecord, GovernanceEvidenceView, GovernanceExportPackage, GovernanceExpiryRules,
    GovernanceFailureCategory, GovernanceFailureRecoveryRequirement,
    GovernanceFailureRecoveryState, GovernanceFailureState, GovernanceHistoricalSnapshot,
    GovernanceImpactClass, GovernanceIntegrityCheckKind, GovernanceIntegrityDiagnostic,
    GovernanceIntegrityDiagnosticSeverity, GovernanceIntegrityVerification,
    GovernanceLifecycleStage, GovernanceMetricKind, GovernanceMetricSample,
    GovernanceMetricsContract, GovernanceNotification, GovernanceNotificationContract,
    GovernanceNotificationDeliveryStatus, GovernanceNotificationKind,
    GovernanceNotificationTrigger, governance_authority_is_none, GovernanceObligation,
    GovernanceObligationKind,
    GovernanceObligationStatus, GovernancePolicy, GovernanceProposalView,
    GovernanceReadinessDashboardProjection, GovernanceRecord, GovernanceReportContract,
    GovernanceReportKind, GovernanceReportSection, GovernanceReviewDecision,
    GovernanceReviewDecisionKind, GovernanceReviewMode, GovernanceReviewRouting,
    GovernanceReviewWorkflowContract, GovernanceReviewWorkflowStage, GovernanceReviewerAssignment,
    GovernanceReviewerAssignmentStatus, GovernanceReviewerRequirements, GovernanceReviewerView,
    GovernanceRisk, GovernanceRiskView, GovernanceTimeline, GovernanceTimelineEvent,
    GovernanceTimestamps, GovernanceWorkspace,
    OutcomeAdaptationProposal, OutcomeAdaptationReviewStatus, PublicationCompatibilityCheck,
    PublicationEnvironment, PublicationFailureHandling, PublicationMigrationRequirement,
    PublicationReadiness, PublicationReadinessState, PublicationRollbackRequirement,
    PublicationRolloutStage, PublicationSafetyContract, PublicationSafetyLifecycleState,
    PublicationValidationGate, PublishRequest, PublishRequestStatus, PublishedVersionRecord,
    RecommendationFamily, RecommendationGovernanceRecord, RecommendationIdentity,
    RecommendationLifecycle, RecommendationLifecycleOverlay, RecommendationLifecycleState,
    RecommendationOutcome, RecommendationOutcomeQuality, RecommendationProvenance,
    RecommendationResolutionType, RecommendationReviewActionResult,
    RecommendationResultKind, RecommendationUserDecision,
};
pub use actor::{
    Actor, ActorContext, ActorMetadata, ActorType, LOCAL_USER_ACTOR_ID, SYSTEM_ACTOR_ID,
};
pub use ai_assistant::{
    AiAssistantActionExplanation, AiAssistantActionPreview, AiAssistantError,
    AiAssistantPlanComparison, AiAssistantPlanPreview, AiAssistantPlanRevision,
    AiAssistantWorkflow, AiAssistantWorkflowState,
};
pub use ai_memory::{
    AiMemoryAwareness, AiMemoryError, MemoryEntry, MemoryLifecycleState, MemoryMetadata,
    MemoryType,
};
pub use ai_model::{
    AiModelError, ModelInvocationSummary, ModelProposalCandidate, ModelProviderAvailability,
    ModelProviderCapability, ModelProviderDescriptor, ModelRequest, ModelRequestKind,
    ModelResponse, ModelResponseFormat, ModelResponseStatus,
};
pub use ai_personalization::{
    AiPersonalizationAwareness, AiPersonalizationError, PersonalizedPlanComparison,
    PreferenceCategory, PreferenceSource, UserPreference, UserPreferenceProfile,
};
pub use ai_evaluation::{
    classify_authority_outcome, evaluate_plan, evaluate_submission_result, AiEvaluationError,
    AiEvaluationSummary, AiPlanEvaluationReport, AiProposalEvaluation, AiProposalOutcomeClass,
    AiProposalQualityIssue, AiProposalRelevance, AiProposalValidity,
};
pub use ai_orchestration::{
    AiOrchestratedPlan, AiOrchestratedPlanState, AiOrchestrationError, AiPlanStep, AiPlanStepState,
};
pub use ai_planning::{
    AiActionProposal, AiApplicationAwareness, AiGoal, AiPlan, AiPlanSubmissionResult,
    AiPlanningContext, AiPlanningError, AiProposalAuthorityOutcome, AiProposalSubmission,
    AiWorkspaceAwareness,
};
pub use ai_request::{AiActionRequest, AiRequestError};
pub use analytics::{AnalyticsError, CategoryActivity, WorkspaceMetrics};
pub use audit::AuditEvent;
pub use capability::{Capability, CapabilityId, CapabilityScope, CapabilitySet};
pub use permission_approval::{
    ApprovalDecisionKind, ApprovalDecisionResult, CapabilityGrant, CapabilityGrantStatus,
    GrantKind, PermissionApprovalError, PermissionApprovalRequest, PermissionApprovalStatus,
};
pub use platform_coherence::{
    vocabulary, ConceptOwnerKind, ConceptOwnership, PLATFORM_CONCEPT_OWNERS,
};
pub use context::{ContextError, WorkspaceContext};
pub use discovery::{
    AvailableIntentSummary, CapabilityDiscovery, CapabilityDiscoveryError,
};
pub use intent::{
    ActionIntentCategory, ActionIntentDefinition, ActionIntentError, ActionIntentId,
    ActionIntentMetadata, ActionIntentRegistry, ActionIntentRequest, Intent, IntentContext,
    IntentMetadata, IntentType, TargetRequirement, AI_SUGGESTION_INTENT_ID,
    SYSTEM_SHUTDOWN_INTENT_ID, SYSTEM_STARTUP_INTENT_ID, USER_REQUEST_INTENT_ID,
};
pub use observation::{
    classify_event, neutral_summary, Observation, ObservationCategory, ObservationError,
    ObservationImportance,
};
pub use resource::{Addressable, ResourceId, ResourceKind, ResourceRef};
pub use suggestion::{
    derive_suggestions, find_pending_suggestion, Suggestion, SuggestionConfidence, SuggestionError,
    SuggestionStatus, SuggestionType,
};
pub use suggestion_lifecycle::{
    classify_suggestion_lifecycle_event, extract_suggestion_id, parse_canonical_resource_ref,
    validate_suggestion_lifecycle_sequence, SuggestionLifecycleError, SuggestionLifecycleRecord,
    SuggestionLifecycleState,
};
pub use suggestion_intent::{
    map_suggestion_type_to_intent, SuggestionIntentError, SuggestionIntentRequest,
};
pub use intent_execution::{
    IntentExecutionError, IntentExecutionRequest, IntentExecutionStatus,
};
pub use execution_cancellation::{
    sanitize_reason, CancellationRequest, CancellationStatus, ExecutionCancellationError,
};
pub use execution_context::{ExecutionContextError, ExecutionContextSummary};
pub use execution_guard::{
    evaluate_execution_guard, execution_request_id_for_suggestion, ExecutionGuardError,
    ExecutionGuardResult,
};
pub use execution_outcome::{
    classify_execution_outcome_event, outcome_from_audit_event, ExecutionOutcome,
    ExecutionOutcomeError, ExecutionOutcomeStatus,
};
pub use execution_reconciliation::{
    reconcile_execution_lifecycle, reconcile_execution_state, reconcile_execution_states,
    ExecutionLifecycleActionableEntry, ExecutionLifecycleHistoryEntry, ExecutionLifecycleProjection,
    ExecutionLifecycleRecord, ExecutionReconciliation, ExecutionReconciliationError, ExecutionState,
};
pub use entities::{
    ApplicationLaunchResult, ApplicationReference, WidgetReference, Workspace, Zone,
};
pub use errors::{validate_resource_name, DomainError, Result};
pub use graph::{GraphEdge, GraphRelationship};
pub use ids::{
    ActorId, AiActionProposalId, AiAssistantWorkflowId, AiGoalId, AiOrchestratedPlanId,
    AiPlanStepId, ApplicationId, AuditEventId, AutomationContractId, AutomationIntentProposalId,
    CapabilityGrantId, IntentId, LayoutId, MemoryEntryId, ModelId, ModelProviderId,
    DecisionCandidateId, DecisionItemId, PermissionApprovalRequestId, ProjectId, TaskId,
    TriggerEventId, UserPreferenceId, WidgetId, WorkGoalId, AttentionItemId, ContinuityFacetId,
    WorkspaceActivityId, WorkspaceId, WorkspaceProfileId, WorkspaceTaskId, ZoneId,
};
pub use automation_contract::{
    AutomationContract, AutomationContractApprovalState, AutomationContractError,
    AutomationContractIntentRequest, AutomationContractScope, AutomationContractStatus,
    AutomationContractSummary, AutomationIntentDefinition, AutomationTriggerDefinition,
    AutomationTriggerKind,
};
pub use automation_trigger::{
    AutomationIntentProposal, AutomationIntentProposalStatus, AutomationIntentProposalSummary,
    AutomationTriggerError, TriggerEvaluationResult, TriggerEvent, TriggerEventType,
    TriggerRejection, TriggerRejectionSummary,
};
pub use decision_engine::{
    DecisionArtifactHistoryEntry, DecisionCandidate, DecisionCandidateEvaluationOriginContract,
    DecisionCandidateEvaluationOriginInput, DecisionCandidateEvaluationResolution,
    DecisionCandidateEvaluationResolutionInput, DecisionCandidateLifecycleIntegration,
    DecisionCandidateProgressionAcknowledgement, DecisionCandidateProgressionAcknowledgementInput,
    DecisionCandidateProgressionRequest, DecisionCandidateProgressionRequestInput,
    DecisionCandidateRanking, DecisionCandidateRankingEntry, DecisionCandidateRankingMemberInput,
    DecisionCandidateScore, DecisionCandidateScoreInput, DecisionCandidateSelection,
    DecisionCandidateSelectionInput, DecisionContext, DecisionEngineActionResult,
    DecisionEngineCandidateCreation, DecisionEngineCandidateCreationInput,
    DecisionEngineCandidateCreationRequest, DecisionEngineError, DecisionEngineHandoff,
    DecisionEngineIntakeAssessment, DecisionEngineIntakeAssessmentInput,
    DecisionEngineIntakeCandidate, DecisionEngineIntakeCandidateLifecycle,
    DecisionEngineIntakeDisposition, DecisionEngineIntakeEligibility,
    DecisionEngineIntakeEvaluation, DecisionEngineIntakePromotionBoundary,
    DecisionEngineIntakePromotionBoundaryInput, DecisionEngineIntakeReceipt, DecisionEngineOverlay,
    DecisionEngineState, DecisionEngineSummary, DecisionExplanation, DecisionOutcome,
    DecisionReason, DecisionScore,
};
pub use decision_queue::{
    DecisionActionResult, DecisionCategory, DecisionHandoff, DecisionItem, DecisionLifecycleOverlay,
    DecisionOverlayHistoryEntry, DecisionPriority, DecisionQueue, DecisionQueueError,
    DecisionQueueSummary, DecisionSourceType, DecisionState,
};
pub use workspace_activity::{
    ActivitySourceType, ActivityType, WorkspaceActivity, WorkspaceActivityError,
    WorkspaceActivityGraph, WorkspaceActivityGraphSummary,
};
pub use workspace_attention::{
    normalize_attention_reasons, AttentionCategory, AttentionConfidence, AttentionItem,
    AttentionPriority, AttentionReason, AttentionSignal, AttentionSourceType, AttentionState,
    AttentionUrgency, WorkspaceAttentionError, WorkspaceAttentionState, WorkspaceAttentionSummary,
};
pub use workspace_continuity::{
    ContinuityFacet, ContinuityFacetKind, WorkspaceContinuityError, WorkspaceContinuityState,
    WorkspaceContinuitySummary,
};
pub use workspace_composition::{
    build_composition_summary, composition_now_rfc3339, validate_composition_workspace_id,
    CompositionGap, CompositionMember, CompositionMemberKind, CompositionRelationship,
    WorkspaceCompositionError, WorkspaceCompositionState, WorkspaceCompositionSummary,
};
pub use workspace_purpose::{
    build_purpose_summary, purpose_now_rfc3339, validate_purpose_workspace_id, PurposeEvidence,
    PurposeEvidenceKind, PurposeObstacle, PurposeRelationship, WorkspacePurposeError,
    WorkspacePurposeState, WorkspacePurposeSummary,
};
pub use workspace_evolution::{
    build_evolution_summary, evolution_now_rfc3339, validate_evolution_workspace_id,
    EvolutionEvent, EvolutionInsight, EvolutionInsightKind, EvolutionRelationship,
    EvolutionSourceModel, WorkspaceEvolutionError, WorkspaceEvolutionState,
    WorkspaceEvolutionSummary,
};
pub use workspace_recommendation::{
    build_recommendation_engine_summary, recommendation_engine_now_rfc3339,
    validate_recommendation_engine_workspace_id, RecommendationConfidence,
    RecommendationDecisionBoundary, RecommendationDecisionConfirmation,
    RecommendationDecisionContext, RecommendationDecisionEngineAcceptance,
    RecommendationDecisionHandoffRequest, RecommendationDecisionIntakeAdapterPreparation,
    RecommendationDecisionIntakeCompatibility, RecommendationDecisionIntakeInspection,
    RecommendationDecisionIntakePackageSeal, RecommendationDecisionIntakeProceedDenial,
    RecommendationDecisionIntakeRequest, RecommendationDecisionPrerequisite,
    RecommendationDecisionReadiness, RecommendationEvidence,
    RecommendationExplanationView, RecommendationHistoryEntry, RecommendationItem,
    RecommendationKind, RecommendationOutcomeView, RecommendationRelationship,
    WorkspaceRecommendationEngineError, WorkspaceRecommendationEngineState,
    WorkspaceRecommendationEngineSummary,
};
pub use workspace_operating_state::{
    build_operating_state_summary, operating_state_now_rfc3339,
    validate_operating_state_workspace_id, OperatingContext, OperatingRelationship,
    OperatingSignal, OperatingSignalKind, OperatingSummary, WorkspaceOperatingState,
    WorkspaceOperatingStateError, WorkspaceOperatingStateSummary,
};
pub use workspace_pattern::{
    build_pattern_model_summary, pattern_now_rfc3339, validate_pattern_workspace_id,
    PatternConfidence, PatternEvidence, PatternKind, PatternRelationship, PatternSummary,
    WorkspacePattern, WorkspacePatternError, WorkspacePatternState, WorkspacePatternSummary,
};
pub use workspace_adaptation::{
    adaptation_now_rfc3339, build_adaptation_summary, validate_adaptation_workspace_id,
    AdaptationActionResult, AdaptationEvidence, AdaptationHandoff, AdaptationImpact,
    AdaptationKind, AdaptationProposal, AdaptationStatus, AdaptationSummary, AdaptationTarget,
    AdaptationTargetKind, WorkspaceAdaptationError, WorkspaceAdaptationState,
    WorkspaceAdaptationSummary,
};
pub use workspace_readiness::{
    build_readiness_summary, readiness_now_rfc3339, validate_readiness_workspace_id,
    ReadinessAssessment, ReadinessGap, ReadinessKind, ReadinessSignal, ReadinessStatus,
    ReadinessSummary, WorkspaceReadinessError, WorkspaceReadinessState,
    WorkspaceReadinessSummary,
};
pub use workspace_session::{
    build_session_summary, session_now_rfc3339, validate_session_workspace_id, SessionDecisionRef,
    SessionFocus, SessionHealth, SessionInterruption, SessionMember, SessionMemberKind,
    SessionMomentum, SessionReadinessView, SessionRecommendationRef, SessionRisk, SessionSummary,
    SessionTimelineItem, WorkspaceSessionComparison, WorkspaceSessionError, WorkspaceSessionState,
    WorkspaceSessionSummary,
};
pub use workspace_experience::{
    build_experience_summary, experience_now_rfc3339, validate_experience_workspace_id,
    DisplayImportance, DisplayReason, ExperienceItem, ExperienceResolverPath,
    ExperienceResolverPathKind, ExperienceSection, ExperienceSectionKind, ExperienceSummary,
    ExperienceTranslationTrace, ExperienceVisibility, WorkspaceExperienceComparison,
    WorkspaceExperienceError, WorkspaceExperienceState, WorkspaceExperienceSummary,
};
pub use workspace_work_context::{
    build_work_context_summary, validate_work_context_workspace_id, work_context_now_rfc3339,
    WorkContext, WorkContextAssociation, WorkContextConfidence, WorkContextEvidence,
    WorkContextRelationKind, WorkContextRelationship, WorkContextStatus, WorkContextType,
    WorkspaceWorkContextComparison, WorkspaceWorkContextError, WorkspaceWorkContextState,
    WorkspaceWorkContextSummary, WorkspaceWorkContextValidation,
};
pub use workspace_navigation::{
    build_navigation_summary, navigation_now_rfc3339, validate_navigation_workspace_id,
    NavigationEdge, NavigationNode, NavigationPath, NavigationPathKind, NavigationRelationKind,
    NavigationSummary, WorkspaceNavigationComparison, WorkspaceNavigationError,
    WorkspaceNavigationState, WorkspaceNavigationSummary, WorkspaceNavigationValidation,
};
pub use workspace_milestone::{
    build_milestone_summary, milestone_now_rfc3339, validate_milestone_workspace_id,
    MilestoneAssociation, MilestoneEvidence, MilestoneReadinessBand, MilestoneRelationKind,
    MilestoneRelationship, MilestoneStatus, MilestoneSummary, WorkspaceMilestone,
    WorkspaceMilestoneComparison, WorkspaceMilestoneError, WorkspaceMilestoneState,
    WorkspaceMilestoneSummary, WorkspaceMilestoneValidation,
};
pub use workspace_working_style::{
    build_working_style_summary, validate_working_style_workspace_id, working_style_now_rfc3339,
    WorkingStyleConfidence, WorkingStyleEvidence, WorkingStyleKind, WorkingStyleObservation,
    WorkingStyleOrigin, WorkingStyleSummary, WorkspaceWorkingStyleComparison,
    WorkspaceWorkingStyleError, WorkspaceWorkingStyleState, WorkspaceWorkingStyleSummary,
    WorkspaceWorkingStyleValidation,
};
pub use workspace_transition::{
    build_transition_summary, transition_now_rfc3339, validate_transition_workspace_id,
    TransitionAssociation, TransitionConfidence, TransitionEvidence, TransitionKind,
    TransitionRelationKind, TransitionRelationship, TransitionSummary, WorkspaceTransition,
    WorkspaceTransitionComparison, WorkspaceTransitionError, WorkspaceTransitionState,
    WorkspaceTransitionSummary, WorkspaceTransitionValidation,
};
pub use workspace_interaction::{
    build_interaction_summary, interaction_now_rfc3339, validate_interaction_workspace_id,
    InteractionEvidence, InteractionHandoff, InteractionItem, InteractionItemState,
    InteractionKind, InteractionPriority, InteractionSelectResult, InteractionSummary,
    WorkspaceInteractionComparison, WorkspaceInteractionError, WorkspaceInteractionState,
    WorkspaceInteractionSummary, WorkspaceInteractionValidation,
};
pub use workspace_profile::{
    build_profile_summary, profile_now_rfc3339, validate_profile_workspace_id, ProfileSummaryLines,
    WorkspaceProfile, WorkspaceProfileAlignment, WorkspaceProfileComparison,
    WorkspaceProfileDifference, WorkspaceProfileError, WorkspaceProfileEvidence,
    WorkspaceProfileMember, WorkspaceProfileMemberInput, WorkspaceProfileMemberType,
    WorkspaceProfileRelationship, WorkspaceProfileState, WorkspaceProfileStateComparison,
    WorkspaceProfileStatus, WorkspaceProfileSummary, WorkspaceProfileValidation,
};
pub use workspace_observation::{
    build_observation_status, decide_observation_refresh, empty_stub_snapshot,
    observation_age_seconds, observation_freshness, observation_meets_freshness_requirement,
    observation_now_rfc3339, observation_u32_to_i32, observation_u64_to_i32,
    observation_usize_to_i32, CaptureProvenance, CaptureRequest, CaptureRequestSource,
    ObservationCaptureErrorClass, ObservationCaptureFailure, ObservationConsumerFreshnessNeed,
    ObservationFreshnessEnsureResult,
    ObservationFreshness, ObservationFreshnessRequirement, ObservationRefreshBlockedReason,
    ObservationRefreshContext, ObservationRefreshDecision, ObservationScheduleConfig,
    ObservationSchedulerStatus, ObservationTriggerAdmissionDecision, ObservationTriggerOutcome,
    ObservationTriggerRequest, ObservationTriggerSource, ObservationWindowIdentity,
    ObservedMonitor, ObservedWindow, WindowIdentityConfidence, WorkspaceObservationError,
    WorkspaceObservationPass, WorkspaceObservationPassMetadata, WorkspaceObservationSnapshot,
    WorkspaceObservationStatus,
    OBSERVATION_FRESH_THRESHOLD_SECS, OBSERVATION_STALE_THRESHOLD_SECS,
    OBSERVATION_TRIGGER_ADMIT_WINDOW_SECS, OBSERVATION_TRIGGER_MAX_ADMITS_PER_WINDOW,
    OBSERVATION_TRIGGER_MIN_ADMIT_INTERVAL_SECS,
};
pub use workspace_observation_delta::{
    compare_observation_snapshots, ObservationFocusedWindowChange, ObservationMinimizedChange,
    ObservationMonitorAssignmentChange, ObservationWindowMove, ObservationWindowRef,
    ObservationWindowResize, WorkspaceObservationDelta,
};
pub use workspace_observation_event::{
    ObservationEvent, ObservationEventError, ObservationEventKind,
};
pub use workspace_state::{
    WorkspaceActiveApplication, WorkspaceState, WorkspaceStateMetadata, WorkspaceStateWindow,
    WORKSPACE_STATE_WINDOW_LIMIT,
};
pub use workspace_environment::{
    build_environment_summary, now_rfc3339, validate_workspace_id, EnvironmentApplication,
    EnvironmentGap, EnvironmentLayoutAssociation, EnvironmentWindow, EnvironmentWindowGroup,
    EnvironmentWindowState, WorkspaceEnvironmentError, WorkspaceEnvironmentState,
    WorkspaceEnvironmentSummary,
};
pub use workspace_intent::{
    Project, ProjectStatus, Task, TaskPriority, TaskStatus, WorkGoal, WorkGoalStatus,
    WorkflowContext, WorkspaceIntentError,
};
pub use workspace_cognitive_model::{
    CognitiveModelError, CognitiveModelState, CognitiveNode, CognitiveNodeKind,
    CognitiveNodeStatus, CognitiveRelation, CognitiveRelationKind,
};
pub use workspace_planning::{
    PlanningAlternative, PlanningAssumption, PlanningConfidence, PlanningConstraintReference,
    PlanningDependency, PlanningEvidenceReference, PlanningExplanation, PlanningGap,
    PlanningHistoryEntry, PlanningPlan, PlanningPlanStatus, PlanningProposal, PlanningRisk,
    PlanningSection, PlanningSnapshot, PlanningStep, PlanningSummary, WorkspacePlanningError,
};
pub use workspace_reasoning_memory::{
    ReasoningEvidenceReference, ReasoningHistoryEntry, ReasoningLink, ReasoningRecord,
    ReasoningRecordStatus, ReasoningSnapshot, ReasoningSummary, WorkspaceReasoningMemoryError,
};
pub use workspace_cognitive_graph::{
    dedupe_edges, dedupe_nodes, CognitiveGraphEdge, CognitiveGraphEdgeKind,
    CognitiveGraphHistoryEntry, CognitiveGraphMeta, CognitiveGraphNode, CognitiveGraphNodeKind,
    CognitiveGraphSnapshot, CognitiveGraphStatus, CognitiveGraphSummary, CognitiveGraphView,
    WorkspaceCognitiveGraphError,
};
pub use workspace_cognitive_orchestration::{
    order_dependencies, refresh_stages_from_order, OrchestrationArtefactKind,
    OrchestrationCycleDetected, OrchestrationDependency, OrchestrationEvidenceLink,
    OrchestrationHistoryEntry, OrchestrationObservation, OrchestrationOrderResult,
    OrchestrationRefreshStage, OrchestrationStatus, WorkspaceCognitiveOrchestrationError,
    WorkspaceOrchestrationMeta, WorkspaceOrchestrationSnapshot, WorkspaceOrchestrationSummary,
    WorkspaceOrchestrationView,
};
pub use workspace_learning_adaptation::{
    AdaptationCandidate, ConfidenceUpdate, LearningEvidenceLink, LearningHistoryEntry,
    LearningMeta, LearningObservation, LearningPattern, LearningSignal, LearningSnapshot,
    LearningStatus, LearningSummary, LearningView, WorkspaceLearningAdaptationError,
};
pub use workspace_cognitive_agent_cast::{
    default_cast_roles, AgentCritique, AgentPerspective, AgentSynthesis, CastEvidenceLink,
    CastStatus, CognitiveAgent, CognitiveAgentCastHistoryEntry, CognitiveAgentCastMeta,
    CognitiveAgentCastSnapshot, CognitiveAgentCastSummary, CognitiveAgentCastView,
    WorkspaceCognitiveAgentCastError,
};
pub use workspace_cognitive_autonomy::{
    AutonomyEvidenceLink, AutonomyOpportunity, AutonomyRecommendation, AutonomySafetyAssessment,
    AutonomyStatus, AutomationProposal, CognitiveAutonomyHistoryEntry, CognitiveAutonomyMeta,
    CognitiveAutonomySnapshot, CognitiveAutonomySummary, CognitiveAutonomyView,
    WorkspaceCognitiveAutonomyError,
};
pub use workspace_state_envelope::{
    AvailabilityStatus, CompletenessStatus, ConsistencyStatus, EnvelopeStatus, FreshnessStatus,
    WorkspaceStateConflict, WorkspaceStateEnvelope, WorkspaceStateEnvelopeError,
    WorkspaceStateHistoryEntry, WorkspaceStateSnapshot, WorkspaceStateSource,
    WorkspaceStateSummary,
};
pub use policy_governance::{
    aggregate_results, default_policy_catalog, evaluate_policies, policy_catalog_revision,
    GovernanceEvaluationStatus, GovernanceExplanation, GovernanceRecommendation,
    PolicyDefinition, PolicyDefinitionStatus, PolicyEvaluation, PolicyEvaluationResult,
    PolicyGovernanceError, PolicyGovernanceHistoryEntry, PolicyGovernanceMeta,
    PolicyGovernanceSnapshot, PolicyGovernanceSummary, PolicyGovernanceView, PolicyScope,
    PolicySeverity,
};
pub use workspace_historical_reconstruction::{
    EvidenceGap, HistoricalChangeExplanation, HistoricalReconstructionError,
    HistoricalReconstructionHistoryEntry, HistoricalReconstructionSnapshot,
    HistoricalReconstructionSummary, ReconstructionCompleteness, ReconstructionProvenanceLink,
    ReconstructionStatus, RevisionComparison, StateChangeEvidence, TemporalAvailability,
    TemporalSnapshot, WorkspaceHistoricalView,
};
pub use workspace_temporal_intelligence::{
    EvidenceQualityAssessment, RevisionChainSummary, TemporalAnalysisStatus, TemporalAnalysisView,
    TemporalAnalysisWindow, TemporalChangeExplanation, TemporalConflictExplanation,
    TemporalIntelligenceError, TemporalIntelligenceHistoryEntry, TemporalIntelligenceSnapshot,
    TemporalIntelligenceSummary, TemporalProvenanceLink, UnderstandingCompleteness,
};
pub use workspace_explanation::{
    EvidenceReference, ExplanationCompleteness, ExplanationConfidence, ExplanationConflict,
    ExplanationGap, ExplanationPackage, ExplanationScope, ExplanationSection, ExplanationStatus,
    WorkspaceExplanationError, WorkspaceExplanationHistoryEntry, WorkspaceExplanationSnapshot,
    WorkspaceExplanationSummary, WorkspaceSituationExplanation,
};
pub use workspace_contextual_understanding::{
    ContextFrame, ContextualCompleteness, ContextualEvidenceReference, ContextualGap,
    ContextualInsight, ContextualUnderstandingConfidence, ContextualUnderstandingError,
    ContextualUnderstandingHistoryEntry, ContextualUnderstandingProjection,
    ContextualUnderstandingStatus, ContextualUnderstandingSummary, ContextualWorkspaceSnapshot,
    SituationalTheme, WorkspaceContextExplanation,
};
pub use workspace_knowledge_synthesis::{
    KnowledgeCluster, KnowledgeCompleteness, KnowledgeConcept, KnowledgeConfidence,
    KnowledgeEvidenceReference, KnowledgeGap, KnowledgeRelationship, KnowledgeSynthesisError,
    KnowledgeSynthesisExplanation, KnowledgeSynthesisFrame, KnowledgeSynthesisHistoryEntry,
    KnowledgeSynthesisProjection, KnowledgeSynthesisStatus, KnowledgeSynthesisSummary,
    WorkspaceKnowledgeSynthesis,
};
pub use workspace_knowledge_integration::{
    IntegrationGap, KnowledgeEvidenceLink, KnowledgeIntegrationCompleteness,
    KnowledgeIntegrationError, KnowledgeIntegrationEvidenceRef, KnowledgeIntegrationExplanation,
    KnowledgeIntegrationHistoryEntry, KnowledgeIntegrationProjection, KnowledgeIntegrationResult,
    KnowledgeIntegrationStatus, KnowledgeIntegrationSummary, KnowledgeRetrievalConfidence,
    KnowledgeRetrievalFrame,
};
pub use workspace_insight_coordination::{
    CoordinationAssessment, InsightAttentionSignal, InsightCluster, InsightCoordinationCompleteness,
    InsightCoordinationError, InsightCoordinationExplanation, InsightCoordinationFrame,
    InsightCoordinationHistoryEntry, InsightCoordinationProjection, InsightCoordinationSnapshot,
    InsightCoordinationStatus, InsightCoordinationSummary, InsightEvidenceRef, InsightGap,
    InsightIntersection,
};
pub use workspace_task_graph::{
    would_create_cycle, TaskDependency, TaskGraph, TaskGraphError, TaskGraphSummary, TaskHistoryEntry,
    TaskMetadata, TaskNode, TaskRelationship, TaskRelationshipKind, WorkspaceTask,
    WorkspaceTaskPriority, WorkspaceTaskStatus,
};
pub use workspace_intelligence::{
    BlockedActionSummary, IntelligenceApplicationSummary, IntelligenceHighlight,
    PendingDecisionSummary, RecentActivityItem, WorkspaceIntelligenceComparison,
    WorkspaceIntelligenceError, WorkspaceIntelligenceState, WorkspaceRecommendation,
};
pub use workspace_runtime::{
    CognitionContextBundle, CognitionContextProjection, CognitionProjectionKind,
    GovernanceRuntimeSummary, OperatorContextProjection, OperatorRuntimeExplanation,
    OperatorRuntimeOverview, RuntimeArchitectureOwnershipEntry,
    RuntimeArchitectureOwnershipRegistry, RuntimeArchitectureOwnershipValidation,
    RuntimeArchitectureReview, RuntimeAuthorityScope, RuntimeCapabilityEntry, RuntimeCapabilityMap,
    RuntimeConsistencyCheckKind, RuntimeConsistencyDiagnostic, RuntimeConsistencySeverity,
    RuntimeConsistencyVerification, RuntimeDependencyEdge, RuntimeDependencyGraph,
    RuntimeDependencyKind, RuntimeDiagnosticArchive, RuntimeDiagnosticArchiveEntry,
    RuntimeDiagnosticArchiveKind, RuntimeDiagnosticComparison, RuntimeDiagnosticConfidence,
    RuntimeDiagnosticConsumerKind, RuntimeDiagnosticConsumptionContract,
    RuntimeDiagnosticConsumptionMode, RuntimeDiagnosticContinuityRecord, RuntimeDiagnosticDelta,
    RuntimeDiagnosticDeltaKind, RuntimeDiagnosticEvidenceBundle, RuntimeDiagnosticEvidenceKind,
    RuntimeDiagnosticEvidenceRef, RuntimeDiagnosticEvolutionReport, RuntimeDiagnosticFinding,
    RuntimeDiagnosticFindingScope, RuntimeDiagnosticForbiddenInterpretation,
    RuntimeDiagnosticHistoricalIntegrity, RuntimeDiagnosticInterpretationView,
    RuntimeDiagnosticLifecyclePhase, RuntimeDiagnosticLifecycleRecord,
    RuntimeDiagnosticLineageRecord, RuntimeDiagnosticLineageStep,
    RuntimeDiagnosticLineageValidation, RuntimeDiagnosticOwnershipBoundary,
    RuntimeDiagnosticOwnershipRole, RuntimeDiagnosticProvenance, RuntimeDiagnosticRestorationView,
    RuntimeDiagnosticRetentionPolicy, RuntimeDiagnosticSnapshot, RuntimeDiagnosticSourceKind,
    RuntimeDiagnosticSourceRef,     RuntimeDiagnosticTrustRecord, RuntimeDiagnosticCurrency,
    RuntimeDiagnosticCompatibilityContract, RuntimeDiagnosticCompatibilityReport,
    RuntimeDiagnosticContractCatalog, RuntimeDiagnosticContractCatalogEntry,
    RuntimeDiagnosticContractFamily, RuntimeDiagnosticContractIdentity,
    RuntimeDiagnosticExplanationConsistency, RuntimeDiagnosticExplanationIntegrity,
    RuntimeDiagnosticInteropContract, RuntimeDiagnosticInteropDomain,
    RuntimeDiagnosticInteropEntry, RuntimeDiagnosticLifecycleClosure,
    RuntimeDiagnosticCatalogIntegrity, RuntimeDiagnosticContractDependency,
    RuntimeDiagnosticMaturityAssessment, RuntimeDiagnosticMaturityLevel,
    RuntimeDiagnosticModuleLayering, RuntimeDiagnosticReferenceIntegrity,
    RuntimeDiagnosticSubsystemBoundary,
    RUNTIME_DIAGNOSTICS_CONTRACT_VERSION, RUNTIME_DIAGNOSTICS_SCHEMA_VERSION,
    RuntimeGovernanceIntegrationPoint,
    RuntimeGovernanceVisibilitySurface, RuntimeProjectionBoundaryEntry,
    RuntimeProjectionBoundaryLayer, RuntimeProjectionBoundaryRegistry, RuntimeSubsystemNode,
    RuntimeVisibilityScope, SubsystemHealthEntry, WorkspaceHealthLevel, WorkspaceRuntimeCoherence,
    WorkspaceRuntimeContext, WorkspaceRuntimeError, WorkspaceRuntimeHealth,
    WorkspaceRuntimeIntegrationContract, WorkspaceRuntimeOperatorView,
};
pub use layout::{
    Layout, LayoutBounds, LayoutError, LayoutMetadata, LayoutNode, LayoutSnapshot, Position2D,
    Size2D, Viewport,
};
pub use projection::{
    ApplicationSummary, LayoutPlacementSummary, ProjectionError, ProjectionRelationship,
    WidgetSummary, WorkspaceSnapshot, ZoneSummary,
};
pub use projection_contract::{history_count_is_authoritative, history_json_is_non_commandable};
pub use recovery_contract::{
    history_evidence_is_complete, in_progress_claim_is_not_terminal_evidence,
    recovered_stale_claim_is_non_retryable, recovery_diagnostic_event_type_is_non_commandable,
    recovery_diagnostic_is_evidence_only,     recovery_must_not_fabricate_actionable_graph_history,
    recovery_must_not_fabricate_actionable_history,
    recovery_must_not_fabricate_actionable_orchestration_history,
    recovery_must_not_fabricate_actionable_reasoning_history,
    recovery_must_not_fabricate_cognitive_graph,
    recovery_must_not_fabricate_actionable_learning_history,
    recovery_must_not_fabricate_actionable_agent_cast_history,
    recovery_must_not_fabricate_cognitive_agent_cast,
    recovery_must_not_fabricate_actionable_autonomy_history,
    recovery_must_not_fabricate_cognitive_autonomy,
    recovery_must_not_fabricate_actionable_workspace_state_history,
    recovery_must_not_fabricate_workspace_state_envelope,
    recovery_must_not_fabricate_actionable_policy_governance_history,
    recovery_must_not_fabricate_policy_governance,
    recovery_must_not_fabricate_actionable_historical_history,
    recovery_must_not_fabricate_historical_reconstruction,
    recovery_must_not_fabricate_actionable_temporal_history,
    recovery_must_not_fabricate_temporal_intelligence,
    recovery_must_not_fabricate_actionable_explanation_history,
    recovery_must_not_fabricate_workspace_explanation,
    recovery_must_not_fabricate_actionable_contextual_understanding_history,
    recovery_must_not_fabricate_contextual_understanding,
    recovery_must_not_fabricate_actionable_knowledge_synthesis_history,
    recovery_must_not_fabricate_knowledge_synthesis,
    recovery_must_not_fabricate_actionable_knowledge_integration_history,
    recovery_must_not_fabricate_knowledge_integration,
    recovery_must_not_fabricate_actionable_insight_coordination_history,
    recovery_must_not_fabricate_insight_coordination,
    recovery_must_not_fabricate_learning, recovery_must_not_fabricate_orchestration, recovery_must_not_fabricate_reasoning,
    recovery_must_not_invent_completed, RECOVERY_DIAGNOSTIC_ATTEMPTED,
    RECOVERY_DIAGNOSTIC_COMPLETED, RECOVERY_DIAGNOSTIC_EVENT_TYPES, RECOVERY_DIAGNOSTIC_FAILED,
    RECOVERY_SUBSYSTEM_EXECUTION_LIFECYCLE, STARTUP_IN_PROGRESS_SWEEP_LIMIT,
};
