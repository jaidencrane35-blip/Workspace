//! Sprints 160–164 — notifications, delegation, metrics, reporting, export.
//!
//! Architecture contracts only. No runtime publication, activation, or execution.

use serde::{Deserialize, Serialize};

use super::{
    ActionProposalError, AdaptationReviewerIdentity, GovernanceArchiveContract,
    GovernanceCompatibilityContract, GovernanceComplianceContract, GovernanceDecisionEvidence,
    GovernanceDecisionPackage, GovernanceRisk, OutcomeAdaptationProposal, PublicationReadiness,
    RecommendationProvenance,
};

// ---------------------------------------------------------------------------
// Sprint 160 — Governance Notification Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceNotificationKind {
    ReviewAssigned,
    ReviewReminder,
    DecisionRecorded,
    ConflictDetected,
    ObligationOutstanding,
    ComplianceFailed,
    ReadyForPublication,
    ExpiryWarning,
}

impl GovernanceNotificationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReviewAssigned => "review_assigned",
            Self::ReviewReminder => "review_reminder",
            Self::DecisionRecorded => "decision_recorded",
            Self::ConflictDetected => "conflict_detected",
            Self::ObligationOutstanding => "obligation_outstanding",
            Self::ComplianceFailed => "compliance_failed",
            Self::ReadyForPublication => "ready_for_publication",
            Self::ExpiryWarning => "expiry_warning",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceNotificationTrigger {
    WorkflowAssigned,
    ReminderDue,
    DecisionSealed,
    ConflictOpened,
    ObligationUnmet,
    ComplianceError,
    ReadinessReached,
    TtlApproaching,
}

impl GovernanceNotificationTrigger {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorkflowAssigned => "workflow_assigned",
            Self::ReminderDue => "reminder_due",
            Self::DecisionSealed => "decision_sealed",
            Self::ConflictOpened => "conflict_opened",
            Self::ObligationUnmet => "obligation_unmet",
            Self::ComplianceError => "compliance_error",
            Self::ReadinessReached => "readiness_reached",
            Self::TtlApproaching => "ttl_approaching",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceNotificationDeliveryStatus {
    Pending,
    Delivered,
    Acknowledged,
    Expired,
    Failed,
}

impl GovernanceNotificationDeliveryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Delivered => "delivered",
            Self::Acknowledged => "acknowledged",
            Self::Expired => "expired",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceNotification {
    pub id: String,
    pub kind: GovernanceNotificationKind,
    pub trigger: GovernanceNotificationTrigger,
    pub recipients: Vec<String>,
    pub delivery_status: GovernanceNotificationDeliveryStatus,
    pub acknowledged_by: Vec<String>,
    pub reminder_at: Option<String>,
    pub expires_at: Option<String>,
    pub subject_reference: String,
    pub detail: String,
}

/// Informational governance notifications — never execute or grant authority (Sprint 160).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceNotificationContract {
    pub id: String,
    pub notifications: Vec<GovernanceNotification>,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceNotificationContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn new(proposal: &OutcomeAdaptationProposal) -> Self {
        Self {
            id: format!("governance_notifications:{}", proposal.id),
            notifications: Vec::new(),
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn emit(
        &mut self,
        kind: GovernanceNotificationKind,
        trigger: GovernanceNotificationTrigger,
        recipients: Vec<String>,
        subject_reference: impl Into<String>,
        detail: impl Into<String>,
        reminder_at: Option<String>,
        expires_at: Option<String>,
    ) -> String {
        let id = format!("notification:{}:{}", self.id, self.notifications.len());
        self.notifications.push(GovernanceNotification {
            id: id.clone(),
            kind,
            trigger,
            recipients,
            delivery_status: GovernanceNotificationDeliveryStatus::Pending,
            acknowledged_by: Vec::new(),
            reminder_at,
            expires_at,
            subject_reference: subject_reference.into(),
            detail: detail.into(),
        });
        id
    }

    pub fn mark_delivered(&mut self, notification_id: &str) -> Result<(), ActionProposalError> {
        let n = self.find_mut(notification_id)?;
        if n.delivery_status == GovernanceNotificationDeliveryStatus::Expired {
            return Err(ActionProposalError::GovernanceNotificationCannotExecute);
        }
        n.delivery_status = GovernanceNotificationDeliveryStatus::Delivered;
        Ok(())
    }

    pub fn acknowledge(
        &mut self,
        notification_id: &str,
        actor_id: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        let actor_id = actor_id.into();
        let n = self.find_mut(notification_id)?;
        if !n.recipients.contains(&actor_id) {
            return Err(ActionProposalError::GovernanceNotificationCannotExecute);
        }
        if !n.acknowledged_by.contains(&actor_id) {
            n.acknowledged_by.push(actor_id);
        }
        n.delivery_status = GovernanceNotificationDeliveryStatus::Acknowledged;
        Ok(())
    }

    pub fn schedule_reminder(
        &mut self,
        notification_id: &str,
        reminder_at: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        let n = self.find_mut(notification_id)?;
        n.reminder_at = Some(reminder_at.into());
        Ok(())
    }

    pub fn expire(&mut self, notification_id: &str) -> Result<(), ActionProposalError> {
        let n = self.find_mut(notification_id)?;
        n.delivery_status = GovernanceNotificationDeliveryStatus::Expired;
        Ok(())
    }

    fn find_mut(
        &mut self,
        notification_id: &str,
    ) -> Result<&mut GovernanceNotification, ActionProposalError> {
        self.notifications
            .iter_mut()
            .find(|n| n.id == notification_id)
            .ok_or(ActionProposalError::GovernanceNotificationCannotExecute)
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_grant_authority(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceNotificationCannotExecute)
    }

    pub fn attempt_grant_authority() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceNotificationCannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 161 — Governance Delegation Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDelegationStatus {
    Active,
    Revoked,
    Expired,
}

impl GovernanceDelegationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDelegationAuditEvent {
    pub at: String,
    pub action: String,
    pub actor_id: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDelegationRecord {
    pub id: String,
    pub from_reviewer: AdaptationReviewerIdentity,
    pub to_reviewer: AdaptationReviewerIdentity,
    pub parent_delegation_id: Option<String>,
    pub max_chain_depth: u32,
    pub temporary: bool,
    pub expires_at: Option<String>,
    pub status: GovernanceDelegationStatus,
    pub scope: String,
}

/// Review delegation only — never transfers execution authority (Sprint 161).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDelegationContract {
    pub id: String,
    pub proposal_reference: String,
    pub delegations: Vec<GovernanceDelegationRecord>,
    pub audit_events: Vec<GovernanceDelegationAuditEvent>,
    pub max_chain_depth: u32,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceDelegationContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const DEFAULT_MAX_CHAIN_DEPTH: u32 = 3;
    pub const SCOPE_REVIEW_ONLY: &'static str = "governance_review_only";

    pub fn new(proposal: &OutcomeAdaptationProposal) -> Self {
        Self {
            id: format!("governance_delegation:{}", proposal.id),
            proposal_reference: proposal.id.clone(),
            delegations: Vec::new(),
            audit_events: Vec::new(),
            max_chain_depth: Self::DEFAULT_MAX_CHAIN_DEPTH,
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    fn chain_depth(&self, parent_id: Option<&str>) -> u32 {
        let mut depth = 0;
        let mut current = parent_id.map(|s| s.to_string());
        while let Some(id) = current {
            depth += 1;
            current = self
                .delegations
                .iter()
                .find(|d| d.id == id)
                .and_then(|d| d.parent_delegation_id.clone());
        }
        depth
    }

    pub fn delegate(
        &mut self,
        from: AdaptationReviewerIdentity,
        to: AdaptationReviewerIdentity,
        parent_delegation_id: Option<String>,
        temporary: bool,
        expires_at: Option<String>,
        at: impl Into<String>,
    ) -> Result<String, ActionProposalError> {
        if from.actor_id == to.actor_id {
            return Err(ActionProposalError::GovernanceDelegationBlocked(
                "cannot delegate to self".into(),
            ));
        }
        let depth = self.chain_depth(parent_delegation_id.as_deref()) + 1;
        if depth > self.max_chain_depth {
            return Err(ActionProposalError::GovernanceDelegationBlocked(
                "delegation chain depth exceeded".into(),
            ));
        }
        let id = format!("delegation:{}:{}", self.id, self.delegations.len());
        self.delegations.push(GovernanceDelegationRecord {
            id: id.clone(),
            from_reviewer: from.clone(),
            to_reviewer: to.clone(),
            parent_delegation_id,
            max_chain_depth: self.max_chain_depth,
            temporary,
            expires_at,
            status: GovernanceDelegationStatus::Active,
            scope: Self::SCOPE_REVIEW_ONLY.into(),
        });
        self.audit_events.push(GovernanceDelegationAuditEvent {
            at: at.into(),
            action: "delegated".into(),
            actor_id: from.actor_id,
            detail: format!("to {}", to.actor_id),
        });
        Ok(id)
    }

    pub fn revoke(
        &mut self,
        delegation_id: &str,
        by_actor_id: impl Into<String>,
        at: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        let d = self
            .delegations
            .iter_mut()
            .find(|d| d.id == delegation_id)
            .ok_or_else(|| {
                ActionProposalError::GovernanceDelegationBlocked("unknown delegation".into())
            })?;
        if d.status != GovernanceDelegationStatus::Active {
            return Err(ActionProposalError::GovernanceDelegationBlocked(
                "delegation not active".into(),
            ));
        }
        d.status = GovernanceDelegationStatus::Revoked;
        self.audit_events.push(GovernanceDelegationAuditEvent {
            at: at.into(),
            action: "revoked".into(),
            actor_id: by_actor_id.into(),
            detail: delegation_id.into(),
        });
        Ok(())
    }

    pub fn expire(
        &mut self,
        delegation_id: &str,
        at: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        let d = self
            .delegations
            .iter_mut()
            .find(|d| d.id == delegation_id)
            .ok_or_else(|| {
                ActionProposalError::GovernanceDelegationBlocked("unknown delegation".into())
            })?;
        d.status = GovernanceDelegationStatus::Expired;
        self.audit_events.push(GovernanceDelegationAuditEvent {
            at: at.into(),
            action: "expired".into(),
            actor_id: "system:delegation".into(),
            detail: delegation_id.into(),
        });
        Ok(())
    }

    pub fn active_delegatees(&self) -> Vec<&AdaptationReviewerIdentity> {
        self.delegations
            .iter()
            .filter(|d| d.status == GovernanceDelegationStatus::Active)
            .map(|d| &d.to_reviewer)
            .collect()
    }

    pub fn may_transfer_execution_authority(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_transfer_execution() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceDelegationBlocked(
            "delegation cannot transfer execution authority".into(),
        ))
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 162 — Governance Metrics Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceMetricKind {
    ReviewDuration,
    ApprovalLatency,
    ConflictFrequency,
    ObligationCompletionRate,
    ArchiveGrowth,
    PublicationReadinessTrend,
}

impl GovernanceMetricKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReviewDuration => "review_duration",
            Self::ApprovalLatency => "approval_latency",
            Self::ConflictFrequency => "conflict_frequency",
            Self::ObligationCompletionRate => "obligation_completion_rate",
            Self::ArchiveGrowth => "archive_growth",
            Self::PublicationReadinessTrend => "publication_readiness_trend",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceMetricSample {
    pub kind: GovernanceMetricKind,
    pub value_milli: i64,
    pub unit: String,
    pub observed_at: String,
    pub subject_reference: Option<String>,
}

/// Immutable observational metrics — no optimisation / adaptation (Sprint 162).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceMetricsContract {
    pub id: String,
    pub samples: Vec<GovernanceMetricSample>,
    pub append_only: bool,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceMetricsContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn new(proposal: &OutcomeAdaptationProposal) -> Self {
        Self {
            id: format!("governance_metrics:{}", proposal.id),
            samples: Vec::new(),
            append_only: true,
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn record(
        &mut self,
        kind: GovernanceMetricKind,
        value_milli: i64,
        unit: impl Into<String>,
        observed_at: impl Into<String>,
        subject_reference: Option<String>,
    ) -> Result<(), ActionProposalError> {
        if !self.append_only {
            return Err(ActionProposalError::GovernanceMetricsCannotMutate);
        }
        self.samples.push(GovernanceMetricSample {
            kind,
            value_milli,
            unit: unit.into(),
            observed_at: observed_at.into(),
            subject_reference,
        });
        Ok(())
    }

    pub fn samples_of(&self, kind: GovernanceMetricKind) -> Vec<&GovernanceMetricSample> {
        self.samples.iter().filter(|s| s.kind == kind).collect()
    }

    pub fn may_optimise(&self) -> bool {
        false
    }

    pub fn may_adapt_automatically(&self) -> bool {
        false
    }

    pub fn attempt_optimise(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceMetricsCannotMutate)
    }

    pub fn attempt_adapt(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceMetricsCannotMutate)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 163 — Governance Reporting Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceReportKind {
    GovernanceSummary,
    ComplianceSummary,
    ReviewHistory,
    PublicationReadiness,
    UnresolvedRisk,
    Archival,
}

impl GovernanceReportKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GovernanceSummary => "governance_summary",
            Self::ComplianceSummary => "compliance_summary",
            Self::ReviewHistory => "review_history",
            Self::PublicationReadiness => "publication_readiness",
            Self::UnresolvedRisk => "unresolved_risk",
            Self::Archival => "archival",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReportSection {
    pub title: String,
    pub lines: Vec<String>,
}

/// Report projection models — not UI (Sprint 163).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReportContract {
    pub id: String,
    pub kind: GovernanceReportKind,
    pub generated_at: String,
    pub sections: Vec<GovernanceReportSection>,
    pub package_reference: Option<String>,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceReportContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn governance_summary(
        proposal: &OutcomeAdaptationProposal,
        package: Option<&GovernanceDecisionPackage>,
        at: impl Into<String>,
    ) -> Self {
        let mut lines = vec![
            format!("proposal:{}", proposal.id),
            format!("review_status:{}", proposal.review_status.as_str()),
        ];
        if let Some(p) = package {
            lines.push(format!("package:{}", p.id));
            lines.push(format!("sealed:{}", p.sealed));
            lines.push(format!("reviewers:{}", p.reviewer_actor_ids.len()));
        }
        Self {
            id: format!("governance_report:summary:{}", proposal.id),
            kind: GovernanceReportKind::GovernanceSummary,
            generated_at: at.into(),
            sections: vec![GovernanceReportSection {
                title: "Governance summary".into(),
                lines,
            }],
            package_reference: package.map(|p| p.id.clone()),
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn compliance_summary(
        proposal: &OutcomeAdaptationProposal,
        compliance: &GovernanceComplianceContract,
        at: impl Into<String>,
    ) -> Self {
        let errors = compliance
            .diagnostics
            .iter()
            .filter(|d| {
                matches!(
                    d.severity,
                    super::GovernanceComplianceSeverity::Error
                )
            })
            .count();
        let warnings = compliance
            .diagnostics
            .iter()
            .filter(|d| {
                matches!(
                    d.severity,
                    super::GovernanceComplianceSeverity::Warning
                )
            })
            .count();
        Self {
            id: format!("governance_report:compliance:{}", compliance.id),
            kind: GovernanceReportKind::ComplianceSummary,
            generated_at: at.into(),
            sections: vec![GovernanceReportSection {
                title: "Compliance summary".into(),
                lines: vec![
                    format!("package:{}", compliance.package_reference),
                    format!("errors:{errors}"),
                    format!("warnings:{warnings}"),
                    format!("has_errors:{}", compliance.has_errors()),
                ],
            }],
            package_reference: Some(compliance.package_reference.clone()),
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn review_history(
        proposal: &OutcomeAdaptationProposal,
        decision_ids: &[String],
        at: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("governance_report:review_history:{}", proposal.id),
            kind: GovernanceReportKind::ReviewHistory,
            generated_at: at.into(),
            sections: vec![GovernanceReportSection {
                title: "Review history".into(),
                lines: decision_ids.to_vec(),
            }],
            package_reference: None,
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn publication_readiness_report(
        proposal: &OutcomeAdaptationProposal,
        readiness: &PublicationReadiness,
        at: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("governance_report:readiness:{}", readiness.id),
            kind: GovernanceReportKind::PublicationReadiness,
            generated_at: at.into(),
            sections: vec![GovernanceReportSection {
                title: "Publication readiness".into(),
                lines: vec![
                    format!("state:{}", readiness.state.as_str()),
                    format!("history_len:{}", readiness.history.len()),
                    format!("evidence:{}", readiness.evidence_reference),
                ],
            }],
            package_reference: None,
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn unresolved_risk_report(
        proposal: &OutcomeAdaptationProposal,
        risk: &GovernanceRisk,
        unresolved_notes: Vec<String>,
        at: impl Into<String>,
    ) -> Self {
        let mut lines = vec![
            format!("risk:{}", risk.id),
            format!("level:{}", risk.risk_level.as_str()),
        ];
        lines.extend(unresolved_notes);
        Self {
            id: format!("governance_report:risk:{}", risk.id),
            kind: GovernanceReportKind::UnresolvedRisk,
            generated_at: at.into(),
            sections: vec![GovernanceReportSection {
                title: "Unresolved risk".into(),
                lines,
            }],
            package_reference: None,
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn archival_report(
        proposal: &OutcomeAdaptationProposal,
        archive: &GovernanceArchiveContract,
        at: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("governance_report:archive:{}", archive.id),
            kind: GovernanceReportKind::Archival,
            generated_at: at.into(),
            sections: vec![GovernanceReportSection {
                title: "Archival".into(),
                lines: vec![
                    format!("snapshots:{}", archive.snapshots.len()),
                    format!("append_only:{}", archive.append_only),
                ],
            }],
            package_reference: None,
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceReportCannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 164 — Governance Export Contract
// ---------------------------------------------------------------------------

/// Read-only immutable export package — no import/sync/publish (Sprint 164).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceExportPackage {
    pub id: String,
    pub exported_at: String,
    pub decision_package_reference: Option<String>,
    pub provenance_bundle: RecommendationProvenance,
    pub evidence_references: Vec<String>,
    pub archive_references: Vec<String>,
    pub compatibility_metadata: Vec<String>,
    pub version_metadata: String,
    /// Placeholder only — architecture marker, not a runtime hash compute path.
    pub integrity_hash_placeholder: String,
    pub read_only: bool,
    pub authority_effect: String,
}

impl GovernanceExportPackage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const VERSION_METADATA: &'static str = "governance_export:v1";

    pub fn from_artifacts(
        proposal: &OutcomeAdaptationProposal,
        package: Option<&GovernanceDecisionPackage>,
        evidence: Option<&GovernanceDecisionEvidence>,
        archive: Option<&GovernanceArchiveContract>,
        compatibility: Option<&GovernanceCompatibilityContract>,
        at: impl Into<String>,
    ) -> Self {
        let evidence_references = evidence
            .map(|e| {
                let mut refs = e.supporting_evidence_references.clone();
                refs.insert(0, e.id.clone());
                refs
            })
            .unwrap_or_default();
        let archive_references = archive
            .map(|a| a.snapshots.iter().map(|s| s.id.clone()).collect())
            .unwrap_or_default();
        let compatibility_metadata = compatibility
            .map(|c| {
                c.dependencies
                    .iter()
                    .map(|d| format!("{}:{}", d.kind.as_str(), d.dependency_ref))
                    .collect()
            })
            .unwrap_or_default();
        let package_id = package.map(|p| p.id.clone()).unwrap_or_else(|| "none".into());
        let integrity_hash_placeholder =
            format!("integrity_placeholder:sha256:pending:{package_id}");
        Self {
            id: format!("governance_export:{}", proposal.id),
            exported_at: at.into(),
            decision_package_reference: package.map(|p| p.id.clone()),
            provenance_bundle: proposal.provenance.clone(),
            evidence_references,
            archive_references,
            compatibility_metadata,
            version_metadata: Self::VERSION_METADATA.into(),
            integrity_hash_placeholder,
            read_only: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_import(&self) -> bool {
        false
    }

    pub fn may_synchronise(&self) -> bool {
        false
    }

    pub fn may_publish(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_import(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceExportForbidden(
            "import forbidden".into(),
        ))
    }

    pub fn attempt_synchronise(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceExportForbidden(
            "synchronisation forbidden".into(),
        ))
    }

    pub fn attempt_publish(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceExportForbidden(
            "publication forbidden".into(),
        ))
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_proposal::{
        OutcomeAdaptationReviewStatus, RecommendationFamily, RecommendationProvenance,
    };
    use crate::workspace_recommendation::RecommendationEvidence;

    fn sample_provenance() -> RecommendationProvenance {
        RecommendationProvenance {
            recommendation_id: "recommendation:ops".into(),
            family: RecommendationFamily::RecommendationEngine,
            source_evidence: vec![RecommendationEvidence {
                id: "ev".into(),
                source_model: "continuity".into(),
                source_ref: "c:1".into(),
                summary: "s".into(),
            }],
            reasoning_origins: Vec::new(),
            explanation_keys: vec!["k".into()],
            experience_trace_match_keys: Vec::new(),
            confidence: Some("medium".into()),
            priority_or_impact: Some("i".into()),
            related_attention_id: None,
            future_capability_target: None,
        }
    }

    fn stub_proposal() -> OutcomeAdaptationProposal {
        OutcomeAdaptationProposal {
            id: "proposal:ops".into(),
            source_outcome_id: "outcome:ops".into(),
            source_recommendation_id: "recommendation:ops".into(),
            provenance: sample_provenance(),
            affected_area: "presentation".into(),
            proposed_change: "change".into(),
            expected_effect: "effect".into(),
            confidence: Some("medium".into()),
            review_required: true,
            review_status: OutcomeAdaptationReviewStatus::AwaitingReview,
            proposed_by_actor_id: OutcomeAdaptationProposal::PROPOSER_ACTOR_ID.into(),
            proposed_by_actor_type: OutcomeAdaptationProposal::PROPOSER_ACTOR_TYPE.into(),
            reviewer: None,
            decided_at: None,
            expires_at: None,
            rejection_reason: None,
            audit_events: Vec::new(),
            experience_trace_match_keys: Vec::new(),
            authority_effect: OutcomeAdaptationProposal::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn notifications_are_informational_only() {
        let proposal = stub_proposal();
        let mut n = GovernanceNotificationContract::new(&proposal);
        let id = n.emit(
            GovernanceNotificationKind::ReviewAssigned,
            GovernanceNotificationTrigger::WorkflowAssigned,
            vec!["r1".into()],
            "assignment:1",
            "Please review",
            Some("t+1d".into()),
            Some("t+7d".into()),
        );
        n.mark_delivered(&id).unwrap();
        n.acknowledge(&id, "r1").unwrap();
        assert!(!n.may_execute());
        assert!(GovernanceNotificationContract::attempt_execute().is_err());
    }

    #[test]
    fn delegation_respects_chain_limits_without_execution_transfer() {
        let proposal = stub_proposal();
        let mut d = GovernanceDelegationContract::new(&proposal);
        d.max_chain_depth = 2;
        let d1 = d
            .delegate(
                AdaptationReviewerIdentity::local_user("a"),
                AdaptationReviewerIdentity::local_user("b"),
                None,
                true,
                Some("t+1d".into()),
                "t0",
            )
            .unwrap();
        let d2 = d
            .delegate(
                AdaptationReviewerIdentity::local_user("b"),
                AdaptationReviewerIdentity::local_user("c"),
                Some(d1),
                true,
                None,
                "t1",
            )
            .unwrap();
        assert!(d
            .delegate(
                AdaptationReviewerIdentity::local_user("c"),
                AdaptationReviewerIdentity::local_user("e"),
                Some(d2),
                false,
                None,
                "t2",
            )
            .is_err());
        assert!(!d.may_transfer_execution_authority());
        assert!(GovernanceDelegationContract::attempt_transfer_execution().is_err());
    }

    #[test]
    fn metrics_reports_exports_are_read_only() {
        let proposal = stub_proposal();
        let mut m = GovernanceMetricsContract::new(&proposal);
        m.record(
            GovernanceMetricKind::ReviewDuration,
            120_000,
            "ms",
            "t0",
            None,
        )
        .unwrap();
        assert!(!m.may_optimise());
        assert!(m.attempt_adapt().is_err());
        let report = GovernanceReportContract::governance_summary(&proposal, None, "t1");
        assert!(!report.may_execute());
        let export = GovernanceExportPackage::from_artifacts(
            &proposal, None, None, None, None, "t2",
        );
        assert!(export.read_only);
        assert!(export.attempt_import().is_err());
        assert!(export.attempt_synchronise().is_err());
        assert!(export.attempt_publish().is_err());
    }
}
