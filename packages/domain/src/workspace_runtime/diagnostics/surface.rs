//! Runtime diagnostics — observational only.
//!
//! No execution, automation, repair, recommendations, or authority.

use serde::{Deserialize, Serialize};

use crate::action_proposal::GOVERNANCE_AUTHORITY_EFFECT_NONE as AUTH_NONE;
use crate::workspace_runtime::WorkspaceRuntimeError;

use super::{
    RuntimeConsistencySeverity, RuntimeDiagnosticArchive, RuntimeDiagnosticArchiveEntry,
    RuntimeDiagnosticDeltaKind, RuntimeDiagnosticEvidenceBundle, RuntimeDiagnosticEvolutionReport,
};

// ---------------------------------------------------------------------------
// Runtime Diagnostic Consumption & Interpretation (post–integrity audit)
// ---------------------------------------------------------------------------
//
// Evidence/evolution/archives exist, but consumers lacked a contract forbidding
// command/recommendation/authority interpretations, and findings lacked
// structured confidence/scope/limitations. Restoration is read-only rehydration.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticConsumerKind {
    OperatorProjection,
    RuntimeOverview,
    ArchitectureReview,
    ArchiveInspect,
}

impl RuntimeDiagnosticConsumerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OperatorProjection => "operator_projection",
            Self::RuntimeOverview => "runtime_overview",
            Self::ArchitectureReview => "architecture_review",
            Self::ArchiveInspect => "archive_inspect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticConsumptionMode {
    Observe,
    Explain,
    ArchiveInspect,
}

impl RuntimeDiagnosticConsumptionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Explain => "explain",
            Self::ArchiveInspect => "archive_inspect",
        }
    }

    pub fn is_allowed(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticForbiddenInterpretation {
    Command,
    Recommendation,
    Authority,
    Approval,
    ExperienceTranslation,
    GovernanceDecision,
}

impl RuntimeDiagnosticForbiddenInterpretation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Command => "command",
            Self::Recommendation => "recommendation",
            Self::Authority => "authority",
            Self::Approval => "approval",
            Self::ExperienceTranslation => "experience_translation",
            Self::GovernanceDecision => "governance_decision",
        }
    }
}

/// How diagnostic outputs may be consumed — observational only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticConsumptionContract {
    pub id: String,
    pub allowed_consumers: Vec<RuntimeDiagnosticConsumerKind>,
    pub allowed_modes: Vec<RuntimeDiagnosticConsumptionMode>,
    pub forbidden_interpretations: Vec<RuntimeDiagnosticForbiddenInterpretation>,
    pub may_drive_experience: bool,
    pub may_drive_governance: bool,
    pub may_enter_command_pipeline: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticConsumptionContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        Self {
            id: "runtime_diagnostic_consumption:canonical".into(),
            allowed_consumers: vec![
                RuntimeDiagnosticConsumerKind::OperatorProjection,
                RuntimeDiagnosticConsumerKind::RuntimeOverview,
                RuntimeDiagnosticConsumerKind::ArchitectureReview,
                RuntimeDiagnosticConsumerKind::ArchiveInspect,
            ],
            allowed_modes: vec![
                RuntimeDiagnosticConsumptionMode::Observe,
                RuntimeDiagnosticConsumptionMode::Explain,
                RuntimeDiagnosticConsumptionMode::ArchiveInspect,
            ],
            forbidden_interpretations: vec![
                RuntimeDiagnosticForbiddenInterpretation::Command,
                RuntimeDiagnosticForbiddenInterpretation::Recommendation,
                RuntimeDiagnosticForbiddenInterpretation::Authority,
                RuntimeDiagnosticForbiddenInterpretation::Approval,
                RuntimeDiagnosticForbiddenInterpretation::ExperienceTranslation,
                RuntimeDiagnosticForbiddenInterpretation::GovernanceDecision,
            ],
            may_drive_experience: false,
            may_drive_governance: false,
            may_enter_command_pipeline: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn allows_consumer(&self, consumer: RuntimeDiagnosticConsumerKind) -> bool {
        self.allowed_consumers.contains(&consumer)
    }

    pub fn forbids(&self, interpretation: RuntimeDiagnosticForbiddenInterpretation) -> bool {
        self.forbidden_interpretations.contains(&interpretation)
    }

    pub fn is_safe(&self) -> bool {
        !self.may_drive_experience
            && !self.may_drive_governance
            && !self.may_enter_command_pipeline
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.forbids(RuntimeDiagnosticForbiddenInterpretation::Command)
            && self.forbids(RuntimeDiagnosticForbiddenInterpretation::Authority)
    }

    pub fn attempt_as_command(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticConsumptionForbidden)
    }

    pub fn attempt_as_recommendation(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticConsumptionForbidden)
    }

    pub fn attempt_enter_command_pipeline(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticConsumptionForbidden)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticConfidence {
    High,
    Medium,
    Low,
    Unknown,
}

impl RuntimeDiagnosticConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticFindingScope {
    Workspace,
    Subsystem,
    AuthorityBoundary,
    Projection,
    Lifecycle,
}

impl RuntimeDiagnosticFindingScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Subsystem => "subsystem",
            Self::AuthorityBoundary => "authority_boundary",
            Self::Projection => "projection",
            Self::Lifecycle => "lifecycle",
        }
    }
}

/// Structured observational finding — not a recommendation or command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticFinding {
    pub id: String,
    pub severity: RuntimeConsistencySeverity,
    pub confidence: RuntimeDiagnosticConfidence,
    pub scope: RuntimeDiagnosticFindingScope,
    pub source: String,
    pub detail: String,
    pub limitations: Vec<String>,
    pub is_recommendation: bool,
    pub is_command: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticFinding {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn observational(
        id: impl Into<String>,
        severity: RuntimeConsistencySeverity,
        confidence: RuntimeDiagnosticConfidence,
        scope: RuntimeDiagnosticFindingScope,
        source: impl Into<String>,
        detail: impl Into<String>,
        limitations: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            severity,
            confidence,
            scope,
            source: source.into(),
            detail: detail.into(),
            limitations,
            is_recommendation: false,
            is_command: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_actionable(&self) -> bool {
        false
    }
}

/// Operator-safe interpretation view derived from evolution — observational only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticInterpretationView {
    pub id: String,
    pub workspace_id: String,
    pub evolution_report_id: String,
    pub findings: Vec<RuntimeDiagnosticFinding>,
    pub consumption: RuntimeDiagnosticConsumptionContract,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticInterpretationView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn from_evolution(evolution: &RuntimeDiagnosticEvolutionReport) -> Self {
        let consumption = RuntimeDiagnosticConsumptionContract::canonical();
        let mut findings = Vec::new();
        if let Some(cmp) = &evolution.comparison {
            for (i, d) in cmp.deltas.iter().enumerate() {
                let (confidence, scope, limitations) = match d.kind {
                    RuntimeDiagnosticDeltaKind::GatewayAuthoritySignal => (
                        RuntimeDiagnosticConfidence::High,
                        RuntimeDiagnosticFindingScope::AuthorityBoundary,
                        vec![
                            "observational_signal_only".into(),
                            "does_not_change_permission_gateway".into(),
                        ],
                    ),
                    RuntimeDiagnosticDeltaKind::HealthOverall => (
                        RuntimeDiagnosticConfidence::Medium,
                        RuntimeDiagnosticFindingScope::Workspace,
                        vec!["health_is_observational_not_prescriptive".into()],
                    ),
                    RuntimeDiagnosticDeltaKind::DependencyTopology => (
                        RuntimeDiagnosticConfidence::High,
                        RuntimeDiagnosticFindingScope::Subsystem,
                        vec!["topology_labels_only".into()],
                    ),
                    RuntimeDiagnosticDeltaKind::GovernanceSummary => (
                        RuntimeDiagnosticConfidence::Medium,
                        RuntimeDiagnosticFindingScope::Projection,
                        vec![
                            "governance_summary_not_authoritative".into(),
                            "not_a_governance_decision".into(),
                        ],
                    ),
                    RuntimeDiagnosticDeltaKind::None => (
                        RuntimeDiagnosticConfidence::High,
                        RuntimeDiagnosticFindingScope::Workspace,
                        vec!["no_delta_does_not_imply_health".into()],
                    ),
                    _ => (
                        RuntimeDiagnosticConfidence::Medium,
                        RuntimeDiagnosticFindingScope::Subsystem,
                        vec!["diagnostic_interpretation_only".into()],
                    ),
                };
                findings.push(RuntimeDiagnosticFinding::observational(
                    format!("finding:{}:{}", evolution.id, i),
                    d.severity,
                    confidence,
                    scope,
                    format!("comparison:{}", cmp.id),
                    d.detail.clone(),
                    limitations,
                ));
            }
        } else {
            findings.push(RuntimeDiagnosticFinding::observational(
                format!("finding:{}:initial", evolution.id),
                RuntimeConsistencySeverity::Info,
                RuntimeDiagnosticConfidence::Medium,
                RuntimeDiagnosticFindingScope::Lifecycle,
                evolution.id.clone(),
                "initial_diagnostic_snapshot",
                vec![
                    "no_prior_snapshot".into(),
                    "not_a_recommendation".into(),
                ],
            ));
        }
        let limitations = vec![
            "findings_are_observational".into(),
            "not_commands".into(),
            "not_recommendations".into(),
            "not_experience_translation".into(),
            "not_governance_authority".into(),
            "permission_gateway_sole_execution".into(),
        ];
        Self {
            id: format!("runtime_diagnostic_interpretation:{}", evolution.workspace_id),
            workspace_id: evolution.workspace_id.clone(),
            evolution_report_id: evolution.id.clone(),
            findings,
            consumption,
            limitations,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn all_findings_non_actionable(&self) -> bool {
        self.findings.iter().all(|f| {
            !f.is_actionable() && !f.is_command && !f.is_recommendation
        }) && self.consumption.is_safe()
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_as_recommendation(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticConsumptionForbidden)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeProjectionBoundaryLayer {
    RuntimeDiagnostics,
    OperatorProjection,
    Experience,
    GovernanceProjection,
    AuditHistory,
    WorkContinuity,
}

impl RuntimeProjectionBoundaryLayer {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeDiagnostics => "runtime_diagnostics",
            Self::OperatorProjection => "operator_projection",
            Self::Experience => "experience",
            Self::GovernanceProjection => "governance_projection",
            Self::AuditHistory => "audit_history",
            Self::WorkContinuity => "work_continuity",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeProjectionBoundaryEntry {
    pub layer: RuntimeProjectionBoundaryLayer,
    pub may_consume_diagnostics: bool,
    pub may_emit_commands: bool,
    pub owns_scoring: bool,
    pub notes: String,
}

/// Projection boundary registry — keeps diagnostics distinct from Experience/Governance/Audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeProjectionBoundaryRegistry {
    pub entries: Vec<RuntimeProjectionBoundaryEntry>,
    pub authority_effect: String,
}

impl RuntimeProjectionBoundaryRegistry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        let entry = |layer: RuntimeProjectionBoundaryLayer,
                     may_consume_diagnostics: bool,
                     may_emit_commands: bool,
                     owns_scoring: bool,
                     notes: &str| RuntimeProjectionBoundaryEntry {
            layer,
            may_consume_diagnostics,
            may_emit_commands,
            owns_scoring,
            notes: notes.into(),
        };
        Self {
            entries: vec![
                entry(
                    RuntimeProjectionBoundaryLayer::RuntimeDiagnostics,
                    true,
                    false,
                    false,
                    "produces observational findings only",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::OperatorProjection,
                    true,
                    false,
                    false,
                    "may explain diagnostics; never executes",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::Experience,
                    false,
                    false,
                    false,
                    "experience translation ownership unchanged; diagnostics must not drive it",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::GovernanceProjection,
                    false,
                    false,
                    false,
                    "governance summaries visible elsewhere; diagnostics do not decide",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::AuditHistory,
                    false,
                    false,
                    false,
                    "AuditService command trail ≠ diagnostic archive",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::WorkContinuity,
                    false,
                    false,
                    false,
                    "work resume facets ≠ diagnostic continuity",
                ),
            ],
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn boundaries_respected(&self) -> bool {
        self.entries.iter().all(|e| !e.may_emit_commands)
            && self
                .entries
                .iter()
                .find(|e| e.layer == RuntimeProjectionBoundaryLayer::Experience)
                .is_some_and(|e| !e.may_consume_diagnostics && !e.owns_scoring)
            && self
                .entries
                .iter()
                .find(|e| e.layer == RuntimeProjectionBoundaryLayer::GovernanceProjection)
                .is_some_and(|e| !e.may_consume_diagnostics)
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Read-only rehydration of archived diagnostic evidence — never mutates archive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticRestorationView {
    pub id: String,
    pub workspace_id: String,
    pub archive_entry_id: String,
    pub evidence_bundle_id: Option<String>,
    pub restored_refs: Vec<String>,
    pub restored_at: String,
    pub may_mutate: bool,
    pub may_delete: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticRestorationView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn rehydrate(
        archive: &RuntimeDiagnosticArchive,
        entry: &RuntimeDiagnosticArchiveEntry,
        evidence: Option<&RuntimeDiagnosticEvidenceBundle>,
        at: impl Into<String>,
    ) -> Self {
        let mut restored_refs = vec![entry.source_reference.clone(), entry.digest.clone()];
        if let Some(eid) = &entry.evidence_bundle_id {
            restored_refs.push(eid.clone());
        }
        if let Some(ev) = evidence {
            for r in &ev.refs {
                restored_refs.push(r.artifact_id.clone());
            }
        }
        Self {
            id: format!(
                "runtime_diagnostic_restoration:{}:{}",
                archive.workspace_id, entry.id
            ),
            workspace_id: archive.workspace_id.clone(),
            archive_entry_id: entry.id.clone(),
            evidence_bundle_id: entry.evidence_bundle_id.clone(),
            restored_refs,
            restored_at: at.into(),
            may_mutate: false,
            may_delete: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn attempt_mutate(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticArchiveImmutable)
    }

    pub fn attempt_delete(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticArchiveImmutable)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

