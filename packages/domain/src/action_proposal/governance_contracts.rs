//! Sprints 151–154 — governance conditions, compatibility, integrity, archive.
//!
//! Architecture contracts only. No runtime publication, activation, or execution.

use serde::{Deserialize, Serialize};

use super::{
    ActionProposalError, BehaviourVersion, GovernanceDecisionEvidence, GovernanceLifecycleStage,
    GovernanceRecord, GovernanceReviewDecision, GovernanceReviewDecisionKind, GovernanceTimeline,
    OutcomeAdaptationProposal, PublicationEnvironment, PublicationReadiness,
    RecommendationProvenance,
};

// ---------------------------------------------------------------------------
// Sprint 151 — Governance Condition & Obligation Contract
// ---------------------------------------------------------------------------

/// Kinds of conditions / obligations attached to governance approvals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceObligationKind {
    RequiresDocumentation,
    RequiresTesting,
    RequiresCompatibilityVerification,
    RequiresRollbackPlan,
    RequiresSecondReviewer,
    ExpiresIfUnmet,
}

impl GovernanceObligationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequiresDocumentation => "requires_documentation",
            Self::RequiresTesting => "requires_testing",
            Self::RequiresCompatibilityVerification => "requires_compatibility_verification",
            Self::RequiresRollbackPlan => "requires_rollback_plan",
            Self::RequiresSecondReviewer => "requires_second_reviewer",
            Self::ExpiresIfUnmet => "expires_if_unmet",
        }
    }
}

/// Status of a declared obligation (architecture tracking only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceObligationStatus {
    Declared,
    Pending,
    Satisfied,
    Unmet,
    Expired,
}

impl GovernanceObligationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Declared => "declared",
            Self::Pending => "pending",
            Self::Satisfied => "satisfied",
            Self::Unmet => "unmet",
            Self::Expired => "expired",
        }
    }
}

/// A single condition attached to an approval — never executes work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceObligation {
    pub id: String,
    pub kind: GovernanceObligationKind,
    pub description: String,
    pub status: GovernanceObligationStatus,
    pub expires_if_unmet: bool,
}

/// Formal model of conditions attached to governance approvals (Sprint 151).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceConditionContract {
    pub id: String,
    pub decision_reference: String,
    pub obligations: Vec<GovernanceObligation>,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceConditionContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_decision(decision: &GovernanceReviewDecision) -> Result<Self, ActionProposalError> {
        if decision.decision != GovernanceReviewDecisionKind::Approve {
            return Err(ActionProposalError::GovernanceConditionBlocked(
                "conditions attach to approve decisions".into(),
            ));
        }
        let mut obligations = Vec::new();
        for (i, cond) in decision.conditions.iter().enumerate() {
            obligations.push(GovernanceObligation {
                id: format!("obligation:{}:{}", decision.id, i),
                kind: Self::infer_kind(cond),
                description: cond.clone(),
                status: GovernanceObligationStatus::Declared,
                expires_if_unmet: cond.to_lowercase().contains("expir"),
            });
        }
        // Default safety obligations when high-risk approvals carry empty condition strings.
        if obligations.is_empty() {
            for kind in [
                GovernanceObligationKind::RequiresDocumentation,
                GovernanceObligationKind::RequiresRollbackPlan,
            ] {
                obligations.push(GovernanceObligation {
                    id: format!("obligation:{}:{}", decision.id, kind.as_str()),
                    kind,
                    description: kind.as_str().into(),
                    status: GovernanceObligationStatus::Declared,
                    expires_if_unmet: kind == GovernanceObligationKind::ExpiresIfUnmet,
                });
            }
        }
        Ok(Self {
            id: format!("governance_condition:{}", decision.id),
            decision_reference: decision.id.clone(),
            obligations,
            provenance: decision.provenance_snapshot.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn infer_kind(text: &str) -> GovernanceObligationKind {
        let lower = text.to_lowercase();
        if lower.contains("second") && lower.contains("review") {
            GovernanceObligationKind::RequiresSecondReviewer
        } else if lower.contains("test") {
            GovernanceObligationKind::RequiresTesting
        } else if lower.contains("compat") {
            GovernanceObligationKind::RequiresCompatibilityVerification
        } else if lower.contains("rollback") {
            GovernanceObligationKind::RequiresRollbackPlan
        } else if lower.contains("expir") {
            GovernanceObligationKind::ExpiresIfUnmet
        } else {
            GovernanceObligationKind::RequiresDocumentation
        }
    }

    pub fn declare_obligation(
        &mut self,
        kind: GovernanceObligationKind,
        description: impl Into<String>,
        expires_if_unmet: bool,
    ) {
        let description = description.into();
        self.obligations.push(GovernanceObligation {
            id: format!("obligation:{}:{}", self.id, self.obligations.len()),
            kind,
            description,
            status: GovernanceObligationStatus::Declared,
            expires_if_unmet,
        });
    }

    pub fn mark_pending(&mut self, obligation_id: &str) -> Result<(), ActionProposalError> {
        self.set_status(obligation_id, GovernanceObligationStatus::Pending)
    }

    pub fn mark_satisfied(&mut self, obligation_id: &str) -> Result<(), ActionProposalError> {
        self.set_status(obligation_id, GovernanceObligationStatus::Satisfied)
    }

    pub fn mark_unmet(&mut self, obligation_id: &str) -> Result<(), ActionProposalError> {
        self.set_status(obligation_id, GovernanceObligationStatus::Unmet)?;
        // Expiry is a status transition only — never executes remediation.
        for o in &mut self.obligations {
            if o.id == obligation_id && o.expires_if_unmet {
                o.status = GovernanceObligationStatus::Expired;
            }
        }
        Ok(())
    }

    fn set_status(
        &mut self,
        obligation_id: &str,
        status: GovernanceObligationStatus,
    ) -> Result<(), ActionProposalError> {
        let o = self
            .obligations
            .iter_mut()
            .find(|o| o.id == obligation_id)
            .ok_or_else(|| {
                ActionProposalError::GovernanceConditionBlocked("unknown obligation".into())
            })?;
        o.status = status;
        Ok(())
    }

    pub fn unmet_or_expired(&self) -> bool {
        self.obligations.iter().any(|o| {
            matches!(
                o.status,
                GovernanceObligationStatus::Unmet | GovernanceObligationStatus::Expired
            )
        })
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_grant_authority(&self) -> bool {
        false
    }

    pub fn may_mutate_cognition(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceConditionCannotExecute)
    }

    pub fn attempt_grant_authority() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceCannotGrantAuthority)
    }

    pub fn attempt_mutate_cognition() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceRiskCannotMutateCognition)
    }
}

// ---------------------------------------------------------------------------
// Sprint 152 — Governance Compatibility & Dependency Contract
// ---------------------------------------------------------------------------

/// Dependency / compatibility requirement kinds (architecture).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDependencyKind {
    PrerequisiteContract,
    PublicationDependency,
    VersionCompatibility,
    SchemaCompatibility,
    MigrationOrdering,
    PackageCompatibility,
}

impl GovernanceDependencyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PrerequisiteContract => "prerequisite_contract",
            Self::PublicationDependency => "publication_dependency",
            Self::VersionCompatibility => "version_compatibility",
            Self::SchemaCompatibility => "schema_compatibility",
            Self::MigrationOrdering => "migration_ordering",
            Self::PackageCompatibility => "package_compatibility",
        }
    }

    pub fn pattern_source(self) -> &'static str {
        match self {
            Self::PrerequisiteContract => "Prior governance / safety contract refs",
            Self::PublicationDependency => "PublicationEnvironment / PublishRequest refs",
            Self::VersionCompatibility => "BehaviourVersion identity",
            Self::SchemaCompatibility => "WorkspaceState / Observation schema_version",
            Self::MigrationOrdering => "MigrationRunner ordered versions",
            Self::PackageCompatibility => "Workspace package / crate compatibility mindset",
        }
    }
}

/// A declared dependency or compatibility requirement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDependencyRequirement {
    pub id: String,
    pub kind: GovernanceDependencyKind,
    pub dependency_ref: String,
    pub required_version: Option<String>,
    pub declared_satisfied: bool,
    pub detail: String,
}

/// Compatibility and dependency architecture for publication preparation (Sprint 152).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceCompatibilityContract {
    pub id: String,
    pub dependencies: Vec<GovernanceDependencyRequirement>,
    pub schema_version_floor: i32,
    pub behaviour_version_ref: String,
    pub publication_environment_ref: String,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceCompatibilityContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const DEFAULT_SCHEMA_VERSION_FLOOR: i32 = 1;

    pub fn from_publication_environment(
        environment: &PublicationEnvironment,
        provenance: &RecommendationProvenance,
    ) -> Self {
        let mut dependencies = vec![
            GovernanceDependencyRequirement {
                id: "dep:behaviour_version".into(),
                kind: GovernanceDependencyKind::VersionCompatibility,
                dependency_ref: environment.rollback_scope.clone(),
                required_version: Some(BehaviourVersion::BASELINE_ID.into()),
                declared_satisfied: true,
                detail: "Behaviour version identity declared (architecture)".into(),
            },
            GovernanceDependencyRequirement {
                id: "dep:schema".into(),
                kind: GovernanceDependencyKind::SchemaCompatibility,
                dependency_ref: "schema_version".into(),
                required_version: Some(Self::DEFAULT_SCHEMA_VERSION_FLOOR.to_string()),
                declared_satisfied: true,
                detail: "schema_version floor (pattern from Observation/WorkspaceState)".into(),
            },
            GovernanceDependencyRequirement {
                id: "dep:migration_order".into(),
                kind: GovernanceDependencyKind::MigrationOrdering,
                dependency_ref: "migration_runner:ordered".into(),
                required_version: None,
                declared_satisfied: true,
                detail: "Ordered migration prepare mindset (not SQL apply)".into(),
            },
            GovernanceDependencyRequirement {
                id: "dep:publication_env".into(),
                kind: GovernanceDependencyKind::PublicationDependency,
                dependency_ref: environment.id.clone(),
                required_version: None,
                declared_satisfied: !environment.governance_record_id.is_empty(),
                detail: "Publication environment bound to governance record".into(),
            },
        ];
        for (i, req) in environment.compatibility_requirements.iter().enumerate() {
            dependencies.push(GovernanceDependencyRequirement {
                id: format!("dep:compat:{i}"),
                kind: GovernanceDependencyKind::PackageCompatibility,
                dependency_ref: req.clone(),
                required_version: None,
                declared_satisfied: true,
                detail: format!("environment compatibility: {req}"),
            });
        }
        Self {
            id: format!("governance_compatibility:{}", environment.id),
            dependencies,
            schema_version_floor: Self::DEFAULT_SCHEMA_VERSION_FLOOR,
            behaviour_version_ref: BehaviourVersion::BASELINE_ID.into(),
            publication_environment_ref: environment.id.clone(),
            provenance: provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn declare_prerequisite(
        &mut self,
        contract_ref: impl Into<String>,
        detail: impl Into<String>,
    ) {
        let contract_ref = contract_ref.into();
        self.dependencies.push(GovernanceDependencyRequirement {
            id: format!("dep:prereq:{}", self.dependencies.len()),
            kind: GovernanceDependencyKind::PrerequisiteContract,
            dependency_ref: contract_ref,
            required_version: None,
            declared_satisfied: false,
            detail: detail.into(),
        });
    }

    pub fn mark_dependency_satisfied(
        &mut self,
        dependency_id: &str,
    ) -> Result<(), ActionProposalError> {
        let dep = self
            .dependencies
            .iter_mut()
            .find(|d| d.id == dependency_id)
            .ok_or_else(|| {
                ActionProposalError::GovernanceCompatibilityBlocked("unknown dependency".into())
            })?;
        dep.declared_satisfied = true;
        Ok(())
    }

    /// Architecture verification of declared deps — does not publish or migrate.
    pub fn verify_declared(&self) -> Result<(), ActionProposalError> {
        let unsatisfied: Vec<_> = self
            .dependencies
            .iter()
            .filter(|d| !d.declared_satisfied)
            .map(|d| d.id.clone())
            .collect();
        if !unsatisfied.is_empty() {
            return Err(ActionProposalError::GovernanceCompatibilityBlocked(
                format!("unsatisfied: {}", unsatisfied.join(",")),
            ));
        }
        if self.schema_version_floor < 1 {
            return Err(ActionProposalError::GovernanceCompatibilityBlocked(
                "schema_version_floor must be >= 1".into(),
            ));
        }
        Ok(())
    }

    pub fn may_publish_runtime(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_bypass_permission_gateway(&self) -> bool {
        false
    }

    pub fn attempt_publish_runtime() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationActivationNotImplemented)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 153 — Governance Integrity Verification Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceIntegrityCheckKind {
    ProvenanceCompleteness,
    MissingEvidence,
    InvalidReviewChain,
    InvalidLifecycleOrdering,
    BrokenReference,
    TimelineConsistency,
}

impl GovernanceIntegrityCheckKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceCompleteness => "provenance_completeness",
            Self::MissingEvidence => "missing_evidence",
            Self::InvalidReviewChain => "invalid_review_chain",
            Self::InvalidLifecycleOrdering => "invalid_lifecycle_ordering",
            Self::BrokenReference => "broken_reference",
            Self::TimelineConsistency => "timeline_consistency",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceIntegrityDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

impl GovernanceIntegrityDiagnosticSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

/// Diagnostic produced by integrity verification — never auto-repairs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceIntegrityDiagnostic {
    pub kind: GovernanceIntegrityCheckKind,
    pub severity: GovernanceIntegrityDiagnosticSeverity,
    pub message: String,
    pub reference: Option<String>,
}

/// Verification model: diagnostics only; never mutates governance (Sprint 153).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceIntegrityVerification {
    pub id: String,
    pub diagnostics: Vec<GovernanceIntegrityDiagnostic>,
    pub checked_at: String,
    pub provenance_snapshot: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceIntegrityVerification {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn verify_timeline(
        timeline: &GovernanceTimeline,
        expected_provenance: &RecommendationProvenance,
        at: impl Into<String>,
    ) -> Self {
        let mut diagnostics = Vec::new();
        if timeline.provenance != *expected_provenance {
            diagnostics.push(GovernanceIntegrityDiagnostic {
                kind: GovernanceIntegrityCheckKind::ProvenanceCompleteness,
                severity: GovernanceIntegrityDiagnosticSeverity::Error,
                message: "timeline provenance does not match expected".into(),
                reference: Some(timeline.id.clone()),
            });
        }
        if expected_provenance.source_evidence.is_empty()
            && expected_provenance.reasoning_origins.is_empty()
        {
            diagnostics.push(GovernanceIntegrityDiagnostic {
                kind: GovernanceIntegrityCheckKind::ProvenanceCompleteness,
                severity: GovernanceIntegrityDiagnosticSeverity::Warning,
                message: "provenance has no source_evidence or reasoning_origins".into(),
                reference: Some(expected_provenance.recommendation_id.clone()),
            });
        }
        if let Err(err) = timeline.validate_lifecycle_integrity() {
            diagnostics.push(GovernanceIntegrityDiagnostic {
                kind: GovernanceIntegrityCheckKind::InvalidLifecycleOrdering,
                severity: GovernanceIntegrityDiagnosticSeverity::Error,
                message: err.to_string(),
                reference: Some(timeline.id.clone()),
            });
        }
        let stages = timeline.stages();
        for window in stages.windows(2) {
            if Self::is_out_of_order(window[0], window[1]) {
                diagnostics.push(GovernanceIntegrityDiagnostic {
                    kind: GovernanceIntegrityCheckKind::InvalidLifecycleOrdering,
                    severity: GovernanceIntegrityDiagnosticSeverity::Error,
                    message: format!(
                        "out-of-order stages {} -> {}",
                        window[0].as_str(),
                        window[1].as_str()
                    ),
                    reference: Some(timeline.id.clone()),
                });
            }
        }
        for event in &timeline.events {
            if let Some(eref) = &event.evidence_reference {
                if eref.is_empty() {
                    diagnostics.push(GovernanceIntegrityDiagnostic {
                        kind: GovernanceIntegrityCheckKind::BrokenReference,
                        severity: GovernanceIntegrityDiagnosticSeverity::Error,
                        message: "empty evidence_reference on timeline event".into(),
                        reference: Some(event.id.clone()),
                    });
                }
            }
        }
        if timeline.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            diagnostics.push(GovernanceIntegrityDiagnostic {
                kind: GovernanceIntegrityCheckKind::TimelineConsistency,
                severity: GovernanceIntegrityDiagnosticSeverity::Error,
                message: "timeline authority_effect must be none".into(),
                reference: Some(timeline.id.clone()),
            });
        }
        if diagnostics.is_empty() {
            diagnostics.push(GovernanceIntegrityDiagnostic {
                kind: GovernanceIntegrityCheckKind::TimelineConsistency,
                severity: GovernanceIntegrityDiagnosticSeverity::Info,
                message: "timeline integrity checks passed".into(),
                reference: Some(timeline.id.clone()),
            });
        }
        Self {
            id: format!("governance_integrity:{}", timeline.id),
            diagnostics,
            checked_at: at.into(),
            provenance_snapshot: expected_provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn verify_bundle(
        proposal: &OutcomeAdaptationProposal,
        evidence: Option<&GovernanceDecisionEvidence>,
        decisions: &[GovernanceReviewDecision],
        record: &GovernanceRecord,
        timeline: Option<&GovernanceTimeline>,
        at: impl Into<String>,
    ) -> Self {
        let at = at.into();
        let mut diagnostics = Vec::new();
        if proposal.provenance.source_evidence.is_empty()
            && proposal.provenance.reasoning_origins.is_empty()
        {
            diagnostics.push(GovernanceIntegrityDiagnostic {
                kind: GovernanceIntegrityCheckKind::ProvenanceCompleteness,
                severity: GovernanceIntegrityDiagnosticSeverity::Error,
                message: "proposal provenance incomplete".into(),
                reference: Some(proposal.id.clone()),
            });
        }
        match evidence {
            None => diagnostics.push(GovernanceIntegrityDiagnostic {
                kind: GovernanceIntegrityCheckKind::MissingEvidence,
                severity: GovernanceIntegrityDiagnosticSeverity::Error,
                message: "governance decision evidence missing".into(),
                reference: Some(record.id.clone()),
            }),
            Some(ev) => {
                if ev.supporting_evidence_references.is_empty() {
                    diagnostics.push(GovernanceIntegrityDiagnostic {
                        kind: GovernanceIntegrityCheckKind::MissingEvidence,
                        severity: GovernanceIntegrityDiagnosticSeverity::Warning,
                        message: "evidence has no supporting references".into(),
                        reference: Some(ev.id.clone()),
                    });
                }
                if record.provenance != ev.provenance_snapshot {
                    diagnostics.push(GovernanceIntegrityDiagnostic {
                        kind: GovernanceIntegrityCheckKind::ProvenanceCompleteness,
                        severity: GovernanceIntegrityDiagnosticSeverity::Error,
                        message: "record provenance diverges from evidence".into(),
                        reference: Some(record.id.clone()),
                    });
                }
            }
        }
        if decisions.is_empty() {
            diagnostics.push(GovernanceIntegrityDiagnostic {
                kind: GovernanceIntegrityCheckKind::InvalidReviewChain,
                severity: GovernanceIntegrityDiagnosticSeverity::Error,
                message: "review chain empty".into(),
                reference: Some(record.id.clone()),
            });
        }
        for d in decisions {
            if d.provenance_snapshot != proposal.provenance {
                diagnostics.push(GovernanceIntegrityDiagnostic {
                    kind: GovernanceIntegrityCheckKind::InvalidReviewChain,
                    severity: GovernanceIntegrityDiagnosticSeverity::Error,
                    message: "decision provenance diverges from proposal".into(),
                    reference: Some(d.id.clone()),
                });
            }
            if d.rationale.trim().is_empty() {
                diagnostics.push(GovernanceIntegrityDiagnostic {
                    kind: GovernanceIntegrityCheckKind::InvalidReviewChain,
                    severity: GovernanceIntegrityDiagnosticSeverity::Error,
                    message: "decision missing rationale".into(),
                    reference: Some(d.id.clone()),
                });
            }
        }
        for href in &record.review_decision_references {
            if !decisions.iter().any(|d| d.id == *href) && !href.is_empty() {
                // Reference listed but not supplied in verify call — broken ref diagnostic.
                if decisions.is_empty() {
                    diagnostics.push(GovernanceIntegrityDiagnostic {
                        kind: GovernanceIntegrityCheckKind::BrokenReference,
                        severity: GovernanceIntegrityDiagnosticSeverity::Warning,
                        message: format!("decision ref not provided for verify: {href}"),
                        reference: Some(href.clone()),
                    });
                }
            }
        }
        if let Some(timeline) = timeline {
            let nested = Self::verify_timeline(timeline, &proposal.provenance, format!("{at}:tl"));
            diagnostics.extend(nested.diagnostics);
        }
        if diagnostics.is_empty() {
            diagnostics.push(GovernanceIntegrityDiagnostic {
                kind: GovernanceIntegrityCheckKind::TimelineConsistency,
                severity: GovernanceIntegrityDiagnosticSeverity::Info,
                message: "bundle integrity checks passed".into(),
                reference: Some(record.id.clone()),
            });
        }
        Self {
            id: format!("governance_integrity:bundle:{}", record.id),
            diagnostics,
            checked_at: at,
            provenance_snapshot: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    fn is_out_of_order(a: GovernanceLifecycleStage, b: GovernanceLifecycleStage) -> bool {
        use GovernanceLifecycleStage::*;
        let rank = |s: GovernanceLifecycleStage| -> u8 {
            match s {
                ChangeProposal => 1,
                RiskClassification => 2,
                EvidenceCollection => 3,
                Review => 4,
                Decision => 5,
                RejectedVisible => 5,
                GovernanceRecordPersisted => 6,
                WorkspacePresentation => 7,
                PublicationReadinessReached => 8,
            }
        };
        // RejectedVisible may appear after Decision; otherwise ranks should not decrease.
        if matches!(b, RejectedVisible) {
            return !matches!(a, Decision | Review | RejectedVisible);
        }
        rank(b) < rank(a)
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == GovernanceIntegrityDiagnosticSeverity::Error)
    }

    pub fn may_repair_automatically(&self) -> bool {
        false
    }

    pub fn may_mutate_governance(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_repair(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceIntegrityCannotMutate)
    }

    pub fn attempt_mutate_governance(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceIntegrityCannotMutate)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 154 — Governance Archive & Historical Preservation Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceArchiveKind {
    ProposalSnapshot,
    ReviewRecord,
    PublicationRecord,
    TimelineSnapshot,
    SupersededProposal,
}

impl GovernanceArchiveKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProposalSnapshot => "proposal_snapshot",
            Self::ReviewRecord => "review_record",
            Self::PublicationRecord => "publication_record",
            Self::TimelineSnapshot => "timeline_snapshot",
            Self::SupersededProposal => "superseded_proposal",
        }
    }
}

/// Immutable historical snapshot entry (append-only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceHistoricalSnapshot {
    pub id: String,
    pub kind: GovernanceArchiveKind,
    pub archived_at: String,
    pub source_reference: String,
    /// Content digest / marker — architecture only (not a runtime blob store).
    pub payload_digest: String,
    pub provenance: RecommendationProvenance,
    pub superseded_by: Option<String>,
}

/// Long-term governance history — nothing deleted (Sprint 154).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceArchiveContract {
    pub id: String,
    pub snapshots: Vec<GovernanceHistoricalSnapshot>,
    pub append_only: bool,
    pub authority_effect: String,
}

impl GovernanceArchiveContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn new(archive_id: impl Into<String>) -> Self {
        Self {
            id: archive_id.into(),
            snapshots: Vec::new(),
            append_only: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    fn append(
        &mut self,
        kind: GovernanceArchiveKind,
        source_reference: impl Into<String>,
        payload_digest: impl Into<String>,
        provenance: RecommendationProvenance,
        archived_at: impl Into<String>,
        superseded_by: Option<String>,
    ) -> Result<&GovernanceHistoricalSnapshot, ActionProposalError> {
        if !self.append_only {
            return Err(ActionProposalError::GovernanceArchiveImmutable);
        }
        let source_reference = source_reference.into();
        let snap = GovernanceHistoricalSnapshot {
            id: format!("{}:{}:{}", self.id, kind.as_str(), self.snapshots.len()),
            kind,
            archived_at: archived_at.into(),
            source_reference,
            payload_digest: payload_digest.into(),
            provenance,
            superseded_by,
        };
        self.snapshots.push(snap);
        Ok(self.snapshots.last().unwrap())
    }

    pub fn archive_proposal(
        &mut self,
        proposal: &OutcomeAdaptationProposal,
        at: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        self.append(
            GovernanceArchiveKind::ProposalSnapshot,
            proposal.id.clone(),
            format!("digest:proposal:{}", proposal.id),
            proposal.provenance.clone(),
            at,
            None,
        )?;
        Ok(())
    }

    pub fn archive_review(
        &mut self,
        decision: &GovernanceReviewDecision,
        at: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        self.append(
            GovernanceArchiveKind::ReviewRecord,
            decision.id.clone(),
            format!("digest:review:{}", decision.id),
            decision.provenance_snapshot.clone(),
            at,
            None,
        )?;
        Ok(())
    }

    pub fn archive_publication_readiness(
        &mut self,
        readiness: &PublicationReadiness,
        at: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        self.append(
            GovernanceArchiveKind::PublicationRecord,
            readiness.id.clone(),
            format!("digest:publication:{}", readiness.id),
            readiness.provenance.clone(),
            at,
            None,
        )?;
        Ok(())
    }

    pub fn archive_timeline(
        &mut self,
        timeline: &GovernanceTimeline,
        at: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        self.append(
            GovernanceArchiveKind::TimelineSnapshot,
            timeline.id.clone(),
            format!("digest:timeline:{}:{}", timeline.id, timeline.events.len()),
            timeline.provenance.clone(),
            at,
            None,
        )?;
        Ok(())
    }

    /// Mark a proposal as superseded — prior snapshot retained; append new marker.
    pub fn mark_superseded(
        &mut self,
        prior_proposal_id: impl Into<String>,
        successor_proposal_id: impl Into<String>,
        provenance: &RecommendationProvenance,
        at: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        let prior = prior_proposal_id.into();
        let successor = successor_proposal_id.into();
        self.append(
            GovernanceArchiveKind::SupersededProposal,
            prior,
            format!("digest:superseded:{successor}"),
            provenance.clone(),
            at,
            Some(successor),
        )?;
        Ok(())
    }

    pub fn preserves_all_history(&self) -> bool {
        self.append_only && !self.snapshots.is_empty()
    }

    pub fn may_delete(&self) -> bool {
        false
    }

    pub fn may_activate_runtime(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_delete(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceArchiveImmutable)
    }

    pub fn attempt_activate_runtime() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationActivationNotImplemented)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_proposal::{
        AdaptationReviewerIdentity, GovernancePolicy, GovernanceReviewDecision,
        GovernanceReviewDecisionKind, RecommendationFamily, RecommendationProvenance,
    };
    use crate::workspace_recommendation::RecommendationEvidence;

    fn sample_provenance() -> RecommendationProvenance {
        RecommendationProvenance {
            recommendation_id: "recommendation:test".into(),
            family: RecommendationFamily::RecommendationEngine,
            source_evidence: vec![RecommendationEvidence {
                id: "ev-1".into(),
                source_model: "continuity".into(),
                source_ref: "c:1".into(),
                summary: "s".into(),
            }],
            reasoning_origins: Vec::new(),
            explanation_keys: vec!["k".into()],
            experience_trace_match_keys: Vec::new(),
            confidence: Some("medium".into()),
            priority_or_impact: Some("impact".into()),
            related_attention_id: None,
            future_capability_target: None,
        }
    }

    #[test]
    fn conditions_never_execute_or_grant() {
        let provenance = sample_provenance();
        let policy = GovernancePolicy::for_outcome_adaptation();
        let decision = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("local_user"),
            GovernanceReviewDecisionKind::Approve,
            "Approve with conditions",
            "t",
            vec![
                "requires testing".into(),
                "requires second reviewer".into(),
                "expires if unmet".into(),
            ],
            &provenance,
        )
        .unwrap();
        let mut contract = GovernanceConditionContract::from_decision(&decision).unwrap();
        assert!(contract.obligations.len() >= 3);
        assert!(contract
            .obligations
            .iter()
            .any(|o| o.kind == GovernanceObligationKind::RequiresTesting));
        assert!(contract
            .obligations
            .iter()
            .any(|o| o.kind == GovernanceObligationKind::RequiresSecondReviewer));
        let id = contract.obligations[0].id.clone();
        contract.mark_unmet(&id).unwrap();
        assert!(!contract.may_execute());
        assert!(!contract.may_grant_authority());
        assert!(!contract.may_mutate_cognition());
        assert!(GovernanceConditionContract::attempt_execute().is_err());
        assert!(GovernanceConditionContract::attempt_grant_authority().is_err());
    }

    #[test]
    fn integrity_diagnostics_do_not_repair() {
        let provenance = sample_provenance();
        let mut timeline = GovernanceTimeline {
            id: "tl:test".into(),
            events: Vec::new(),
            provenance: provenance.clone(),
            rejected_visible: false,
            dissent_immutable_refs: Vec::new(),
            authority_effect: GovernanceTimeline::AUTHORITY_EFFECT_NONE.into(),
        };
        timeline.events.push(super::super::GovernanceTimelineEvent {
            id: "e1".into(),
            stage: GovernanceLifecycleStage::ChangeProposal,
            at: "t0".into(),
            actor_reference: None,
            evidence_reference: Some(String::new()),
            decision_reference: None,
            note: None,
        });
        let report = GovernanceIntegrityVerification::verify_timeline(&timeline, &provenance, "t");
        assert!(report.has_errors());
        assert!(!report.may_repair_automatically());
        assert!(!report.may_mutate_governance());
        assert!(report.attempt_repair().is_err());
    }

    #[test]
    fn archive_is_append_only() {
        let provenance = sample_provenance();
        let mut archive = GovernanceArchiveContract::new("archive:test");
        archive
            .mark_superseded("proposal:old", "proposal:new", &provenance, "t1")
            .unwrap();
        archive
            .mark_superseded("proposal:old", "proposal:newer", &provenance, "t2")
            .unwrap();
        assert_eq!(archive.snapshots.len(), 2);
        assert!(archive.preserves_all_history());
        assert!(!archive.may_delete());
        assert!(archive.attempt_delete().is_err());
        assert!(GovernanceArchiveContract::attempt_activate_runtime().is_err());
    }
}
