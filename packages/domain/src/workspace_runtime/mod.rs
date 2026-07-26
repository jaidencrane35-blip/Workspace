//! Workspace runtime integration — Sprints 170–181 + diagnostic continuity.
//!
//! Read-only contracts that make governance and cognition context visible to the
//! runtime without granting authority or changing scoring / WorkspaceState ownership.
//!
//! Naming note: Sprint 19 [`crate::context::WorkspaceContext`] remains the
//! execution/capability read composition. This module’s
//! [`WorkspaceRuntimeContext`] is the cognition + governance operating context.
//!
//! Sprints 176–181 (`diagnostics`) add dependency graph, capability map, snapshots,
//! consistency verification, and operator overview — all observational.
//! Diagnostic provenance/continuity records link snapshots over time without
//! mutating sources or overlapping the work Continuity Engine.
//! Diagnostic evolution adds structured comparison, lifecycle phases, and
//! validation/interpretation — still observational only.
//! Diagnostic evidence integrity and append-only retention archive sealed
//! history without deletion, mutation, or authority.
//! Diagnostic consumption contracts define how surfaces may observe findings
//! without treating them as commands, recommendations, or authority.
//! Diagnostic trust/compatibility identities and lineage records let consumers
//! know producer version and currency — informational only; no migration apply.
//! Diagnostic closure adds terminal lifecycle validation, static contract catalog,
//! cross-domain interoperability, and explanation integrity.
//! Diagnostic maturity assesses catalog/reference/explanation readiness as
//! meta-diagnostics only — never prescriptive or executable.
//!
//! Diagnostics module layout (`diagnostics/`):
//! `foundation` → `history` → `surface` → `meta` (flat `pub use` for API stability).

mod diagnostics;
pub use diagnostics::*;

use serde::{Deserialize, Serialize};

use crate::action_proposal::{
    GOVERNANCE_AUTHORITY_EFFECT_NONE, GovernanceAggregateRoot, PublicationReadinessState,
};
use crate::decision_engine::DecisionEngineSummary;
use crate::workspace_attention::WorkspaceAttentionSummary;
use crate::workspace_environment::WorkspaceEnvironmentSummary;
use crate::workspace_experience::WorkspaceExperienceSummary;
use crate::workspace_intelligence::WorkspaceIntelligenceState;
use crate::workspace_recommendation::WorkspaceRecommendationEngineSummary;
use crate::workspace_state::WorkspaceState;

// ---------------------------------------------------------------------------
// Sprint 170 — Workspace Runtime Integration Contract
// ---------------------------------------------------------------------------

/// Where governance information may be observed in the runtime (read-only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeGovernanceVisibilitySurface {
    OperatorProjection,
    CognitionContext,
    RuntimeHealth,
    IntelligenceOverlay,
    SessionDiagnostics,
}

impl RuntimeGovernanceVisibilitySurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OperatorProjection => "operator_projection",
            Self::CognitionContext => "cognition_context",
            Self::RuntimeHealth => "runtime_health",
            Self::IntelligenceOverlay => "intelligence_overlay",
            Self::SessionDiagnostics => "session_diagnostics",
        }
    }
}

/// One audited integration point: subsystem → governance visibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeGovernanceIntegrationPoint {
    pub subsystem: String,
    pub surface: RuntimeGovernanceVisibilitySurface,
    pub governance_aggregate: String,
    pub observable: bool,
    pub authoritative: bool,
    pub detail: String,
}

/// Read-only integration contract — governance visible, never authoritative (Sprint 170).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRuntimeIntegrationContract {
    pub id: String,
    pub points: Vec<RuntimeGovernanceIntegrationPoint>,
    pub authority_effect: String,
}

impl WorkspaceRuntimeIntegrationContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    /// Audit of runtime subsystems that may observe governance summaries.
    pub fn audit_default(workspace_id: impl Into<String>) -> Self {
        let workspace_id = workspace_id.into();
        let mk = |subsystem: &str,
                  surface: RuntimeGovernanceVisibilitySurface,
                  aggregate: GovernanceAggregateRoot,
                  detail: &str| RuntimeGovernanceIntegrationPoint {
            subsystem: subsystem.into(),
            surface,
            governance_aggregate: aggregate.as_str().into(),
            observable: true,
            authoritative: false,
            detail: detail.into(),
        };
        Self {
            id: format!("runtime_integration:{workspace_id}"),
            points: vec![
                mk(
                    "WorkspaceState",
                    RuntimeGovernanceVisibilitySurface::RuntimeHealth,
                    GovernanceAggregateRoot::PublicationPrep,
                    "State freshness observable; governance never mutates WorkspaceState",
                ),
                mk(
                    "WorkspaceEnvironment",
                    RuntimeGovernanceVisibilitySurface::CognitionContext,
                    GovernanceAggregateRoot::PublicationPrep,
                    "Environment facts visible beside governance readiness labels",
                ),
                mk(
                    "WorkspaceAttention",
                    RuntimeGovernanceVisibilitySurface::CognitionContext,
                    GovernanceAggregateRoot::ReviewOps,
                    "Attention consumes context only; scoring unchanged",
                ),
                mk(
                    "WorkspaceIntelligence",
                    RuntimeGovernanceVisibilitySurface::IntelligenceOverlay,
                    GovernanceAggregateRoot::Observability,
                    "Intelligence may display governance summaries; never owns decisions",
                ),
                mk(
                    "DecisionEngine",
                    RuntimeGovernanceVisibilitySurface::CognitionContext,
                    GovernanceAggregateRoot::PolicyRisk,
                    "Decision projections see context; scoring unchanged",
                ),
                mk(
                    "RecommendationEngine",
                    RuntimeGovernanceVisibilitySurface::CognitionContext,
                    GovernanceAggregateRoot::Ledger,
                    "Recommendations see provenance/governance labels only",
                ),
                mk(
                    "WorkspaceExperience",
                    RuntimeGovernanceVisibilitySurface::CognitionContext,
                    GovernanceAggregateRoot::Observability,
                    "Experience owns translation; governance summaries are display inputs",
                ),
                mk(
                    "OperatorConsole",
                    RuntimeGovernanceVisibilitySurface::OperatorProjection,
                    GovernanceAggregateRoot::ReviewOps,
                    "Operator projections show review/compliance/readiness; no UI authority",
                ),
                mk(
                    "PermissionGateway",
                    RuntimeGovernanceVisibilitySurface::SessionDiagnostics,
                    GovernanceAggregateRoot::Ledger,
                    "Gateway remains sole execution authority; governance not merged",
                ),
            ],
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn all_non_authoritative(&self) -> bool {
        self.points.iter().all(|p| !p.authoritative && p.observable)
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_grant_authority(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum WorkspaceRuntimeError {
    #[error("workspace runtime contracts cannot execute or grant authority")]
    CannotExecute,

    #[error("workspace runtime context is read-only")]
    ReadOnly,

    #[error("workspace runtime health is observational only")]
    HealthNotPrescriptive,

    #[error("workspace runtime consistency verification cannot repair or mutate")]
    ConsistencyCannotMutate,

    #[error("runtime diagnostic history cannot be rewritten or healed")]
    DiagnosticHistoryImmutable,

    #[error("runtime diagnostic evolution cannot mutate, repair, or execute")]
    DiagnosticEvolutionReadOnly,

    #[error("runtime diagnostic archive is append-only; deletion and mutation forbidden")]
    DiagnosticArchiveImmutable,

    #[error("runtime diagnostic evidence is observational and cannot become a decision")]
    DiagnosticEvidenceNotAuthoritative,

    #[error("runtime diagnostic consumption forbids commands, recommendations, or authority")]
    DiagnosticConsumptionForbidden,

    #[error("runtime diagnostic compatibility is identity-only; migration apply forbidden")]
    DiagnosticCompatibilityReadOnly,

    #[error("runtime diagnostic closure forbids mutation, loading, or foreign ownership")]
    DiagnosticClosureReadOnly,

    #[error("runtime diagnostic maturity assessment is meta-diagnostic only")]
    DiagnosticMaturityReadOnly,
}

// ---------------------------------------------------------------------------
// Sprint 171 — Workspace Runtime Context (canonical cognition operating context)
// ---------------------------------------------------------------------------

/// Lightweight governance summary embedded in runtime context (never authoritative).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRuntimeSummary {
    pub package_reference: Option<String>,
    pub review_workflow_stage: Option<String>,
    pub compliance_status: Option<String>,
    pub publication_readiness_state: Option<String>,
    pub unresolved_conflict_count: u32,
    pub outstanding_obligation_count: u32,
    pub publication_blocked: bool,
    pub authority_effect: String,
}

impl GovernanceRuntimeSummary {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn empty() -> Self {
        Self {
            package_reference: None,
            review_workflow_stage: None,
            compliance_status: None,
            publication_readiness_state: None,
            unresolved_conflict_count: 0,
            outstanding_obligation_count: 0,
            publication_blocked: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn from_labels(
        package_reference: Option<String>,
        review_workflow_stage: Option<String>,
        compliance_status: Option<String>,
        publication_readiness_state: Option<PublicationReadinessState>,
        unresolved_conflict_count: u32,
        outstanding_obligation_count: u32,
    ) -> Self {
        Self {
            package_reference,
            review_workflow_stage,
            compliance_status,
            publication_readiness_state: publication_readiness_state.map(|s| s.as_str().into()),
            unresolved_conflict_count,
            outstanding_obligation_count,
            publication_blocked: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Canonical operating context available to cognition (Sprint 171).
///
/// Distinct from Sprint 19 [`crate::context::WorkspaceContext`] (execution/capability
/// composition). This type is read-only and owns no execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRuntimeContext {
    pub id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub workspace_state: Option<WorkspaceState>,
    pub environment: Option<WorkspaceEnvironmentSummary>,
    pub attention: Option<WorkspaceAttentionSummary>,
    pub intelligence: Option<WorkspaceIntelligenceState>,
    pub experience: Option<WorkspaceExperienceSummary>,
    pub governance: GovernanceRuntimeSummary,
    pub authority_effect: String,
}

impl WorkspaceRuntimeContext {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn assemble(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        workspace_state: Option<WorkspaceState>,
        environment: Option<WorkspaceEnvironmentSummary>,
        attention: Option<WorkspaceAttentionSummary>,
        intelligence: Option<WorkspaceIntelligenceState>,
        experience: Option<WorkspaceExperienceSummary>,
        governance: GovernanceRuntimeSummary,
    ) -> Self {
        let workspace_id = workspace_id.into();
        Self {
            id: format!("workspace_runtime_context:{workspace_id}"),
            workspace_id,
            generated_at: generated_at.into(),
            workspace_state,
            environment,
            attention,
            intelligence,
            experience,
            governance,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn owns_workspace_state(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }

    pub fn attempt_mutate() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::ReadOnly)
    }
}

// ---------------------------------------------------------------------------
// Sprint 172 — Cognition Context Projections
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitionProjectionKind {
    Attention,
    Intelligence,
    Decision,
    Recommendation,
}

impl CognitionProjectionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Attention => "attention",
            Self::Intelligence => "intelligence",
            Self::Decision => "decision",
            Self::Recommendation => "recommendation",
        }
    }
}

/// Context-only projection for a cognition consumer (Sprint 172).
/// Exposes context; does not change scoring or reasoning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitionContextProjection {
    pub kind: CognitionProjectionKind,
    pub workspace_id: String,
    pub has_workspace_state: bool,
    pub has_environment: bool,
    pub has_attention: bool,
    pub has_intelligence: bool,
    pub has_experience: bool,
    pub governance_publication_blocked: bool,
    pub governance_compliance_status: Option<String>,
    pub note: String,
    pub authority_effect: String,
}

impl CognitionContextProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn from_runtime_context(
        kind: CognitionProjectionKind,
        ctx: &WorkspaceRuntimeContext,
    ) -> Self {
        let note = match kind {
            CognitionProjectionKind::Attention => {
                "Attention projection: context only; scoring unchanged"
            }
            CognitionProjectionKind::Intelligence => {
                "Intelligence projection: context only; no authority"
            }
            CognitionProjectionKind::Decision => {
                "Decision projection: context only; scoring unchanged"
            }
            CognitionProjectionKind::Recommendation => {
                "Recommendation projection: context only; no execution"
            }
        };
        Self {
            kind,
            workspace_id: ctx.workspace_id.clone(),
            has_workspace_state: ctx.workspace_state.is_some(),
            has_environment: ctx.environment.is_some(),
            has_attention: ctx.attention.is_some(),
            has_intelligence: ctx.intelligence.is_some(),
            has_experience: ctx.experience.is_some(),
            governance_publication_blocked: ctx.governance.publication_blocked,
            governance_compliance_status: ctx.governance.compliance_status.clone(),
            note: note.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn project_all(ctx: &WorkspaceRuntimeContext) -> Vec<Self> {
        [
            CognitionProjectionKind::Attention,
            CognitionProjectionKind::Intelligence,
            CognitionProjectionKind::Decision,
            CognitionProjectionKind::Recommendation,
        ]
        .into_iter()
        .map(|k| Self::from_runtime_context(k, ctx))
        .collect()
    }

    pub fn may_change_scoring(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

/// Optional typed slices for consumers that already hold engine summaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitionContextBundle {
    pub attention_context: CognitionContextProjection,
    pub intelligence_context: CognitionContextProjection,
    pub decision_context: CognitionContextProjection,
    pub recommendation_context: CognitionContextProjection,
    pub decision_engine_summary: Option<DecisionEngineSummary>,
    pub recommendation_engine_summary: Option<WorkspaceRecommendationEngineSummary>,
    pub authority_effect: String,
}

impl CognitionContextBundle {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn from_runtime_context(
        ctx: &WorkspaceRuntimeContext,
        decision_engine_summary: Option<DecisionEngineSummary>,
        recommendation_engine_summary: Option<WorkspaceRecommendationEngineSummary>,
    ) -> Self {
        Self {
            attention_context: CognitionContextProjection::from_runtime_context(
                CognitionProjectionKind::Attention,
                ctx,
            ),
            intelligence_context: CognitionContextProjection::from_runtime_context(
                CognitionProjectionKind::Intelligence,
                ctx,
            ),
            decision_context: CognitionContextProjection::from_runtime_context(
                CognitionProjectionKind::Decision,
                ctx,
            ),
            recommendation_context: CognitionContextProjection::from_runtime_context(
                CognitionProjectionKind::Recommendation,
                ctx,
            ),
            decision_engine_summary,
            recommendation_engine_summary,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Sprint 173 — Operator Context Projection
// ---------------------------------------------------------------------------

/// Operator-facing domain projection (Sprint 173) — not UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorContextProjection {
    pub id: String,
    pub workspace_id: String,
    pub runtime_health_label: String,
    pub cognition_health_label: String,
    pub governance_health_label: String,
    pub observation_freshness_label: String,
    pub publication_readiness_label: String,
    pub publication_blocked: bool,
    pub review_status_label: String,
    pub generated_at: String,
    pub authority_effect: String,
}

impl OperatorContextProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn from_runtime_context(
        ctx: &WorkspaceRuntimeContext,
        health: &WorkspaceRuntimeHealth,
    ) -> Self {
        Self {
            id: format!("operator_context:{}", ctx.workspace_id),
            workspace_id: ctx.workspace_id.clone(),
            runtime_health_label: health.overall.as_str().into(),
            cognition_health_label: health.cognition_readiness.as_str().into(),
            governance_health_label: health.governance_readiness.as_str().into(),
            observation_freshness_label: health.observation_freshness.as_str().into(),
            publication_readiness_label: ctx
                .governance
                .publication_readiness_state
                .clone()
                .unwrap_or_else(|| "blocked".into()),
            publication_blocked: true,
            review_status_label: ctx
                .governance
                .review_workflow_stage
                .clone()
                .unwrap_or_else(|| "none".into()),
            generated_at: ctx.generated_at.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 174 — Workspace Runtime Health Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceHealthLevel {
    Unknown,
    Healthy,
    Degraded,
    Stale,
    Blocked,
}

impl WorkspaceHealthLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubsystemHealthEntry {
    pub subsystem: String,
    pub level: WorkspaceHealthLevel,
    pub detail: String,
}

/// Unified observational health aggregate (Sprint 174).
///
/// Distinct from kernel lifecycle `WorkspaceHealth` and from Readiness Model
/// preparedness. Observational only — never prescriptive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRuntimeHealth {
    pub id: String,
    pub workspace_id: String,
    pub overall: WorkspaceHealthLevel,
    pub subsystems: Vec<SubsystemHealthEntry>,
    pub degraded_services: Vec<String>,
    pub stale_observations: bool,
    pub observation_freshness: WorkspaceHealthLevel,
    pub governance_readiness: WorkspaceHealthLevel,
    pub cognition_readiness: WorkspaceHealthLevel,
    pub experience_readiness: WorkspaceHealthLevel,
    pub publication_blocked: bool,
    pub authority_effect: String,
}

impl WorkspaceRuntimeHealth {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn observe(ctx: &WorkspaceRuntimeContext) -> Self {
        let mut subsystems = Vec::new();
        let mut degraded = Vec::new();

        let state_level = if ctx.workspace_state.is_some() {
            WorkspaceHealthLevel::Healthy
        } else {
            degraded.push("WorkspaceState".into());
            WorkspaceHealthLevel::Unknown
        };
        subsystems.push(SubsystemHealthEntry {
            subsystem: "WorkspaceState".into(),
            level: state_level,
            detail: "canonical runtime projection presence".into(),
        });

        let env_level = if ctx.environment.is_some() {
            WorkspaceHealthLevel::Healthy
        } else {
            degraded.push("Environment".into());
            WorkspaceHealthLevel::Degraded
        };
        subsystems.push(SubsystemHealthEntry {
            subsystem: "Environment".into(),
            level: env_level,
            detail: "environment summary presence".into(),
        });

        let attention_level = if ctx.attention.is_some() {
            WorkspaceHealthLevel::Healthy
        } else {
            WorkspaceHealthLevel::Unknown
        };
        subsystems.push(SubsystemHealthEntry {
            subsystem: "Attention".into(),
            level: attention_level,
            detail: "attention summary presence; scoring not evaluated here".into(),
        });

        let cognition_readiness = if ctx.intelligence.is_some() && ctx.attention.is_some() {
            WorkspaceHealthLevel::Healthy
        } else if ctx.intelligence.is_some() || ctx.attention.is_some() {
            WorkspaceHealthLevel::Degraded
        } else {
            WorkspaceHealthLevel::Unknown
        };

        let experience_readiness = if ctx.experience.is_some() {
            WorkspaceHealthLevel::Healthy
        } else {
            WorkspaceHealthLevel::Unknown
        };

        let governance_readiness = if ctx.governance.unresolved_conflict_count > 0 {
            WorkspaceHealthLevel::Blocked
        } else if ctx.governance.outstanding_obligation_count > 0 {
            WorkspaceHealthLevel::Degraded
        } else if ctx.governance.package_reference.is_some() {
            WorkspaceHealthLevel::Healthy
        } else {
            WorkspaceHealthLevel::Unknown
        };

        let stale_observations = ctx.workspace_state.as_ref().is_none_or(|s| {
            s.metadata.window_count == 0 && s.metadata.observation_pass_id.is_none()
        });
        let observation_freshness = if stale_observations {
            WorkspaceHealthLevel::Stale
        } else if ctx.workspace_state.is_some() {
            WorkspaceHealthLevel::Healthy
        } else {
            WorkspaceHealthLevel::Unknown
        };

        let overall = [
            state_level,
            env_level,
            cognition_readiness,
            experience_readiness,
            governance_readiness,
            observation_freshness,
        ]
        .into_iter()
        .fold(WorkspaceHealthLevel::Healthy, |acc, level| {
            rank_health(acc, level)
        });

        Self {
            id: format!("workspace_runtime_health:{}", ctx.workspace_id),
            workspace_id: ctx.workspace_id.clone(),
            overall,
            subsystems,
            degraded_services: degraded,
            stale_observations,
            observation_freshness,
            governance_readiness,
            cognition_readiness,
            experience_readiness,
            publication_blocked: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_prescribe(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_prescribe(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::HealthNotPrescriptive)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

fn rank_health(a: WorkspaceHealthLevel, b: WorkspaceHealthLevel) -> WorkspaceHealthLevel {
    use WorkspaceHealthLevel::*;
    let rank = |l: WorkspaceHealthLevel| match l {
        Healthy => 0,
        Unknown => 1,
        Degraded => 2,
        Stale => 3,
        Blocked => 4,
    };
    if rank(b) > rank(a) {
        b
    } else {
        a
    }
}

// ---------------------------------------------------------------------------
// Sprint 175 — Workspace Runtime Coherence
// ---------------------------------------------------------------------------

/// Structural coherence of the runtime chain (Sprint 175).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRuntimeCoherence {
    pub id: String,
    pub chain: Vec<String>,
    pub coherent: bool,
    pub notes: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceRuntimeCoherence {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub const CANONICAL_CHAIN: &'static [&'static str] = &[
        "WorkspaceState",
        "Environment",
        "Attention",
        "Intelligence",
        "Decision",
        "Experience",
        "GovernanceProjections",
        "OperatorProjections",
    ];

    pub fn review(
        ctx: &WorkspaceRuntimeContext,
        integration: &WorkspaceRuntimeIntegrationContract,
        health: &WorkspaceRuntimeHealth,
        operator: &OperatorContextProjection,
    ) -> Self {
        let mut notes: Vec<String> = Vec::new();
        if !integration.all_non_authoritative() {
            notes.push("integration contract has authoritative points".into());
        }
        if ctx.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || health.authority_effect != Self::AUTHORITY_EFFECT_NONE
            || operator.authority_effect != Self::AUTHORITY_EFFECT_NONE
        {
            notes.push("authority_effect must remain none".into());
        }
        if !operator.publication_blocked || !health.publication_blocked {
            notes.push("publication must remain blocked in projections".into());
        }
        if ctx.owns_workspace_state() {
            notes.push("runtime context must not own WorkspaceState".into());
        }
        notes.push(
            "chain: WorkspaceState→Environment→Attention→Intelligence→Decision→Experience→Governance→Operator"
                .into(),
        );
        let failure = notes.iter().any(|n| {
            n.contains("authoritative")
                || n.contains("authority_effect must")
                || n.contains("publication must")
                || n.contains("must not own")
        });
        Self {
            id: format!("workspace_runtime_coherence:{}", ctx.workspace_id),
            chain: Self::CANONICAL_CHAIN.iter().map(|s| (*s).into()).collect(),
            coherent: !failure,
            notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 182–185 — Operator runtime projection (live wiring DTO)
// ---------------------------------------------------------------------------

/// Operator-facing runtime projection assembled from live foundations.
///
/// Composition only — never owns WorkspaceState, scoring, Experience translation,
/// governance authority, or Gateway execution. Distinct from diagnostic meta-contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRuntimeOperatorView {
    pub workspace_id: String,
    pub generated_at: String,
    pub runtime_context_id: String,
    pub health: WorkspaceRuntimeHealth,
    pub operator_context: OperatorContextProjection,
    pub overview: OperatorRuntimeOverview,
    pub coherence_ok: bool,
    pub architecture_review_passed: bool,
    pub consistency_has_errors: bool,
    pub diagnostic_snapshot_id: String,
    pub publication_blocked: bool,
    pub authority_effect: String,
}

impl WorkspaceRuntimeOperatorView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn compose(
        ctx: &WorkspaceRuntimeContext,
        health: WorkspaceRuntimeHealth,
        operator_context: OperatorContextProjection,
        overview: OperatorRuntimeOverview,
        coherence: &WorkspaceRuntimeCoherence,
        review: &RuntimeArchitectureReview,
        verification: &RuntimeConsistencyVerification,
        snapshot_id: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: ctx.workspace_id.clone(),
            generated_at: ctx.generated_at.clone(),
            runtime_context_id: ctx.id.clone(),
            health,
            operator_context,
            overview,
            coherence_ok: coherence.coherent,
            architecture_review_passed: review.passed(),
            consistency_has_errors: verification.has_errors(),
            diagnostic_snapshot_id: snapshot_id.into(),
            publication_blocked: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_change_scoring(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_is_visible_not_authoritative() {
        let c = WorkspaceRuntimeIntegrationContract::audit_default("ws-1");
        assert!(c.all_non_authoritative());
        assert!(WorkspaceRuntimeIntegrationContract::attempt_execute().is_err());
    }

    #[test]
    fn runtime_context_and_health_are_read_only() {
        let ctx = WorkspaceRuntimeContext::assemble(
            "ws-1",
            "t0",
            None,
            None,
            None,
            None,
            None,
            GovernanceRuntimeSummary::empty(),
        );
        assert!(!ctx.owns_workspace_state());
        assert!(WorkspaceRuntimeContext::attempt_mutate().is_err());
        let health = WorkspaceRuntimeHealth::observe(&ctx);
        assert!(health.publication_blocked);
        assert!(health.attempt_prescribe().is_err());
        let op = OperatorContextProjection::from_runtime_context(&ctx, &health);
        assert!(op.publication_blocked);
        let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-1");
        let coherence =
            WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &op);
        assert!(coherence.coherent);
    }

    #[test]
    fn cognition_projections_do_not_change_scoring() {
        let ctx = WorkspaceRuntimeContext::assemble(
            "ws-1",
            "t0",
            None,
            None,
            None,
            None,
            None,
            GovernanceRuntimeSummary::empty(),
        );
        for p in CognitionContextProjection::project_all(&ctx) {
            assert!(!p.may_change_scoring());
            assert!(!p.may_execute());
        }
    }
}
