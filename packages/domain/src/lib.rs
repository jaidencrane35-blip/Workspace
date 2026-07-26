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
pub mod resource;
pub mod suggestion;
pub mod suggestion_intent;
pub mod suggestion_lifecycle;
pub mod workspace;
pub mod workspace_activity;
pub mod workspace_attention;
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

pub use action_catalog::{
    ActionCatalog, ActionCatalogEntry, ActionCatalogError, AiActionAwareness,
};
pub use action_proposal::{
    ActionProposal, ActionProposalError, ActionProposalRisk, AdaptationReviewAuditEvent,
    AdaptationReviewerIdentity, AdaptationRiskClass, BehaviourVersion, BehaviourVersionLifecycle,
    ChangeEvaluation, ChangeRollbackMetadata, ControlledChangeAuditMetadata,
    ControlledChangeSurface, GovernanceActorRefs, GovernanceDecisionEvidence,
    GovernanceDecisionHistoryEntry, GovernanceDissentRecord, GovernanceEvidenceView,
    GovernanceExpiryRules, GovernanceImpactClass, GovernanceLifecycleStage, GovernancePolicy,
    GovernanceProposalView, GovernanceRecord, GovernanceReviewDecision,
    GovernanceReviewDecisionKind, GovernanceReviewRouting, GovernanceReviewerRequirements,
    GovernanceReviewerView, GovernanceRisk, GovernanceRiskView, GovernanceTimeline,
    GovernanceTimelineEvent, GovernanceTimestamps, GovernanceWorkspace,
    OutcomeAdaptationProposal, OutcomeAdaptationReviewStatus, PublicationCompatibilityCheck,
    PublicationEnvironment, PublicationFailureHandling, PublicationMigrationRequirement,
    PublicationReadiness, PublicationReadinessState, PublicationRollbackRequirement,
    PublicationRolloutStage, PublicationSafetyContract, PublicationSafetyLifecycleState,
    PublicationValidationGate, PublishRequest, PublishRequestStatus, PublishedVersionRecord,
    RecommendationFamily, RecommendationGovernanceRecord, RecommendationIdentity,
    RecommendationLifecycle, RecommendationLifecycleState, RecommendationOutcome,
    RecommendationOutcomeQuality, RecommendationProvenance, RecommendationResolutionType,
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
    SuggestionLifecycleError, SuggestionLifecycleRecord, SuggestionLifecycleState,
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
    reconcile_execution_state, reconcile_execution_states, ExecutionReconciliation,
    ExecutionReconciliationError, ExecutionState,
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
    DecisionCandidate, DecisionContext, DecisionEngineActionResult, DecisionEngineError,
    DecisionEngineHandoff, DecisionEngineOverlay, DecisionEngineState, DecisionEngineSummary,
    DecisionExplanation, DecisionOutcome, DecisionReason, DecisionScore,
};
pub use decision_queue::{
    DecisionActionResult, DecisionCategory, DecisionHandoff, DecisionItem, DecisionLifecycleOverlay,
    DecisionPriority, DecisionQueue, DecisionQueueError, DecisionQueueSummary, DecisionSourceType,
    DecisionState,
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
    validate_recommendation_engine_workspace_id, RecommendationConfidence, RecommendationEvidence,
    RecommendationItem, RecommendationKind, RecommendationRelationship,
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
pub use workspace_task_graph::{
    would_create_cycle, TaskDependency, TaskGraph, TaskGraphError, TaskGraphSummary, TaskMetadata,
    TaskNode, TaskRelationship, TaskRelationshipKind, WorkspaceTask, WorkspaceTaskPriority,
    WorkspaceTaskStatus,
};
pub use workspace_intelligence::{
    BlockedActionSummary, IntelligenceApplicationSummary, IntelligenceHighlight,
    PendingDecisionSummary, RecentActivityItem, WorkspaceIntelligenceComparison,
    WorkspaceIntelligenceError, WorkspaceIntelligenceState, WorkspaceRecommendation,
};
pub use layout::{
    Layout, LayoutBounds, LayoutError, LayoutMetadata, LayoutNode, LayoutSnapshot, Position2D,
    Size2D, Viewport,
};
pub use projection::{
    ApplicationSummary, LayoutPlacementSummary, ProjectionError, ProjectionRelationship,
    WidgetSummary, WorkspaceSnapshot, ZoneSummary,
};
