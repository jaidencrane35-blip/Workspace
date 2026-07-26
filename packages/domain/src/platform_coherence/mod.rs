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
}
