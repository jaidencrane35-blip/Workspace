//! Review-ops aggregate — workflow, conflict, package, compliance, dashboard (155–159).
//!
//! Architecture contracts only. No runtime publication, activation, or execution.
//! Part of the consolidated governance subsystem (Sprints 165–169).

use serde::{Deserialize, Serialize};

use super::super::{
    ActionProposalError, AdaptationReviewerIdentity, GovernanceDecisionEvidence,
    GovernanceObligationStatus, GovernanceReviewDecision, GovernanceReviewDecisionKind,
    GovernanceRisk, OutcomeAdaptationProposal, PublicationReadiness, PublicationReadinessState,
    PublicationSafetyContract, PublicationSafetyLifecycleState, RecommendationProvenance,
};
use super::{
    GovernanceArchiveContract, GovernanceCompatibilityContract, GovernanceConditionContract,
    GovernanceIntegrityVerification, GOVERNANCE_AUTHORITY_EFFECT_NONE,
};

// ---------------------------------------------------------------------------
// Sprint 155 — Governance Review Workflow Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceReviewMode {
    Ordered,
    Parallel,
    Optional,
}

impl GovernanceReviewMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ordered => "ordered",
            Self::Parallel => "parallel",
            Self::Optional => "optional",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceReviewWorkflowStage {
    Unassigned,
    Assigned,
    InQueue,
    InReview,
    Escalated,
    Completed,
}

impl GovernanceReviewWorkflowStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unassigned => "unassigned",
            Self::Assigned => "assigned",
            Self::InQueue => "in_queue",
            Self::InReview => "in_review",
            Self::Escalated => "escalated",
            Self::Completed => "completed",
        }
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::Unassigned, Self::Assigned)
                | (Self::Assigned, Self::InQueue)
                | (Self::InQueue, Self::InReview)
                | (Self::InReview, Self::Escalated)
                | (Self::InReview, Self::Completed)
                | (Self::Escalated, Self::InReview)
                | (Self::Escalated, Self::Completed)
                | (Self::Assigned, Self::InReview)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceReviewerAssignmentStatus {
    Pending,
    Active,
    Completed,
    Reassigned,
    Escalated,
}

impl GovernanceReviewerAssignmentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Reassigned => "reassigned",
            Self::Escalated => "escalated",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReviewerAssignment {
    pub id: String,
    pub reviewer: AdaptationReviewerIdentity,
    pub order: u32,
    pub optional: bool,
    pub status: GovernanceReviewerAssignmentStatus,
    pub reassigned_from: Option<String>,
}

/// Complete review workflow architecture — metadata only (Sprint 155).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReviewWorkflowContract {
    pub id: String,
    pub proposal_reference: String,
    pub mode: GovernanceReviewMode,
    pub stage: GovernanceReviewWorkflowStage,
    pub stage_history: Vec<GovernanceReviewWorkflowStage>,
    pub assignments: Vec<GovernanceReviewerAssignment>,
    pub pending_queue: Vec<String>,
    pub escalation_notes: Vec<String>,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceReviewWorkflowContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn begin(
        proposal: &OutcomeAdaptationProposal,
        mode: GovernanceReviewMode,
    ) -> Self {
        Self {
            id: format!("governance_review_workflow:{}", proposal.id),
            proposal_reference: proposal.id.clone(),
            mode,
            stage: GovernanceReviewWorkflowStage::Unassigned,
            stage_history: vec![GovernanceReviewWorkflowStage::Unassigned],
            assignments: Vec::new(),
            pending_queue: Vec::new(),
            escalation_notes: Vec::new(),
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    fn transition(
        &mut self,
        to: GovernanceReviewWorkflowStage,
    ) -> Result<(), ActionProposalError> {
        if !self.stage.allows_transition(to) {
            return Err(ActionProposalError::InvalidLifecycleTransition {
                from: self.stage.as_str().into(),
                to: to.as_str().into(),
            });
        }
        self.stage = to;
        self.stage_history.push(to);
        Ok(())
    }

    pub fn assign_reviewer(
        &mut self,
        reviewer: AdaptationReviewerIdentity,
        order: u32,
        optional: bool,
    ) -> Result<String, ActionProposalError> {
        if self.stage == GovernanceReviewWorkflowStage::Completed {
            return Err(ActionProposalError::GovernanceReviewWorkflowBlocked(
                "workflow already completed".into(),
            ));
        }
        let id = format!("assignment:{}:{}", self.id, self.assignments.len());
        self.assignments.push(GovernanceReviewerAssignment {
            id: id.clone(),
            reviewer,
            order,
            optional,
            status: GovernanceReviewerAssignmentStatus::Pending,
            reassigned_from: None,
        });
        if self.stage == GovernanceReviewWorkflowStage::Unassigned {
            self.transition(GovernanceReviewWorkflowStage::Assigned)?;
        }
        Ok(id)
    }

    pub fn enqueue_pending(&mut self) -> Result<(), ActionProposalError> {
        if self.assignments.is_empty() {
            return Err(ActionProposalError::GovernanceReviewWorkflowBlocked(
                "no reviewers assigned".into(),
            ));
        }
        self.pending_queue = match self.mode {
            GovernanceReviewMode::Ordered => {
                let mut ordered = self.assignments.clone();
                ordered.sort_by_key(|a| a.order);
                ordered
                    .into_iter()
                    .filter(|a| {
                        a.status == GovernanceReviewerAssignmentStatus::Pending && !a.optional
                            || a.status == GovernanceReviewerAssignmentStatus::Pending
                    })
                    .map(|a| a.id)
                    .collect()
            }
            GovernanceReviewMode::Parallel => self
                .assignments
                .iter()
                .filter(|a| a.status == GovernanceReviewerAssignmentStatus::Pending)
                .map(|a| a.id.clone())
                .collect(),
            GovernanceReviewMode::Optional => self
                .assignments
                .iter()
                .filter(|a| a.status == GovernanceReviewerAssignmentStatus::Pending)
                .map(|a| a.id.clone())
                .collect(),
        };
        if self.stage == GovernanceReviewWorkflowStage::Assigned {
            self.transition(GovernanceReviewWorkflowStage::InQueue)?;
        }
        Ok(())
    }

    pub fn start_review(&mut self, assignment_id: &str) -> Result<(), ActionProposalError> {
        if self.mode == GovernanceReviewMode::Ordered {
            let min_pending = self
                .assignments
                .iter()
                .filter(|a| {
                    matches!(
                        a.status,
                        GovernanceReviewerAssignmentStatus::Pending
                            | GovernanceReviewerAssignmentStatus::Active
                    ) && !a.optional
                })
                .map(|a| a.order)
                .min();
            let target = self.assignments.iter().find(|a| a.id == assignment_id);
            if let (Some(min), Some(assignment)) = (min_pending, target) {
                if assignment.order > min
                    && assignment.status == GovernanceReviewerAssignmentStatus::Pending
                {
                    return Err(ActionProposalError::GovernanceReviewWorkflowBlocked(
                        "ordered review requires earlier reviewers first".into(),
                    ));
                }
            }
        }
        let assignment = self
            .assignments
            .iter_mut()
            .find(|a| a.id == assignment_id)
            .ok_or_else(|| {
                ActionProposalError::GovernanceReviewWorkflowBlocked(
                    "unknown assignment".into(),
                )
            })?;
        assignment.status = GovernanceReviewerAssignmentStatus::Active;
        self.pending_queue.retain(|id| id != assignment_id);
        if matches!(
            self.stage,
            GovernanceReviewWorkflowStage::InQueue | GovernanceReviewWorkflowStage::Assigned
        ) {
            self.transition(GovernanceReviewWorkflowStage::InReview)?;
        } else if self.stage == GovernanceReviewWorkflowStage::Escalated {
            self.transition(GovernanceReviewWorkflowStage::InReview)?;
        }
        Ok(())
    }

    pub fn reassign(
        &mut self,
        from_assignment_id: &str,
        to_reviewer: AdaptationReviewerIdentity,
    ) -> Result<String, ActionProposalError> {
        let order;
        let optional;
        {
            let from = self
                .assignments
                .iter_mut()
                .find(|a| a.id == from_assignment_id)
                .ok_or_else(|| {
                    ActionProposalError::GovernanceReviewWorkflowBlocked(
                        "unknown assignment for reassignment".into(),
                    )
                })?;
            order = from.order;
            optional = from.optional;
            from.status = GovernanceReviewerAssignmentStatus::Reassigned;
        }
        let id = format!("assignment:{}:{}", self.id, self.assignments.len());
        self.assignments.push(GovernanceReviewerAssignment {
            id: id.clone(),
            reviewer: to_reviewer,
            order,
            optional,
            status: GovernanceReviewerAssignmentStatus::Pending,
            reassigned_from: Some(from_assignment_id.into()),
        });
        if !self.pending_queue.contains(&id) {
            self.pending_queue.push(id.clone());
        }
        Ok(id)
    }

    pub fn escalate(&mut self, note: impl Into<String>) -> Result<(), ActionProposalError> {
        self.escalation_notes.push(note.into());
        for a in &mut self.assignments {
            if a.status == GovernanceReviewerAssignmentStatus::Active {
                a.status = GovernanceReviewerAssignmentStatus::Escalated;
            }
        }
        if self.stage == GovernanceReviewWorkflowStage::InReview {
            self.transition(GovernanceReviewWorkflowStage::Escalated)?;
        }
        Ok(())
    }

    pub fn complete_assignment(
        &mut self,
        assignment_id: &str,
    ) -> Result<(), ActionProposalError> {
        let assignment = self
            .assignments
            .iter_mut()
            .find(|a| a.id == assignment_id)
            .ok_or_else(|| {
                ActionProposalError::GovernanceReviewWorkflowBlocked(
                    "unknown assignment".into(),
                )
            })?;
        assignment.status = GovernanceReviewerAssignmentStatus::Completed;
        self.pending_queue.retain(|id| id != assignment_id);
        Ok(())
    }

    pub fn complete_workflow(&mut self) -> Result<(), ActionProposalError> {
        let required_done = self.assignments.iter().filter(|a| !a.optional).all(|a| {
            matches!(
                a.status,
                GovernanceReviewerAssignmentStatus::Completed
                    | GovernanceReviewerAssignmentStatus::Reassigned
            )
        });
        if !required_done && self.mode != GovernanceReviewMode::Optional {
            return Err(ActionProposalError::GovernanceReviewWorkflowBlocked(
                "required reviewers not complete".into(),
            ));
        }
        if self.mode == GovernanceReviewMode::Optional
            && !self.assignments.iter().any(|a| {
                a.status == GovernanceReviewerAssignmentStatus::Completed
            })
            && !self.assignments.is_empty()
        {
            // Optional mode may complete with zero completions if none started —
            // still allow explicit completion for empty-optional architecture paths.
        }
        self.transition(GovernanceReviewWorkflowStage::Completed)?;
        Ok(())
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_publish(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceReviewWorkflowBlocked(
            "workflow cannot execute".into(),
        ))
    }

    pub fn attempt_publish() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationActivationNotImplemented)
    }
}

// ---------------------------------------------------------------------------
// Sprint 156 — Governance Conflict Resolution Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceConflictKind {
    ConflictingDecisions,
    ConflictingConditions,
    DissentRecorded,
    ArbitrationRequired,
}

impl GovernanceConflictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ConflictingDecisions => "conflicting_decisions",
            Self::ConflictingConditions => "conflicting_conditions",
            Self::DissentRecorded => "dissent_recorded",
            Self::ArbitrationRequired => "arbitration_required",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceConflictState {
    Detected,
    UnderArbitration,
    ConsensusReached,
    Unresolved,
}

impl GovernanceConflictState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Detected => "detected",
            Self::UnderArbitration => "under_arbitration",
            Self::ConsensusReached => "consensus_reached",
            Self::Unresolved => "unresolved",
        }
    }

    pub fn allows_transition(self, to: Self) -> bool {
        matches!(
            (self, to),
            (Self::Detected, Self::UnderArbitration)
                | (Self::UnderArbitration, Self::ConsensusReached)
                | (Self::UnderArbitration, Self::Unresolved)
                | (Self::Detected, Self::Unresolved)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceConsensusRule {
    Unanimous,
    Majority,
    QuorumWithArbiter,
}

impl GovernanceConsensusRule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unanimous => "unanimous",
            Self::Majority => "majority",
            Self::QuorumWithArbiter => "quorum_with_arbiter",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceConflictEntry {
    pub id: String,
    pub kind: GovernanceConflictKind,
    pub decision_references: Vec<String>,
    pub condition_notes: Vec<String>,
    pub dissent_references: Vec<String>,
    pub detail: String,
}

/// Conflict resolution architecture — cannot publish/execute/rewrite history (Sprint 156).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceConflictResolutionContract {
    pub id: String,
    pub proposal_reference: String,
    pub conflicts: Vec<GovernanceConflictEntry>,
    pub state: GovernanceConflictState,
    pub state_history: Vec<GovernanceConflictState>,
    pub consensus_rule: GovernanceConsensusRule,
    pub arbiter_reference: Option<String>,
    pub provenance: RecommendationProvenance,
    pub history_immutable: bool,
    pub authority_effect: String,
}

impl GovernanceConflictResolutionContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn from_decisions(
        proposal: &OutcomeAdaptationProposal,
        decisions: &[GovernanceReviewDecision],
        consensus_rule: GovernanceConsensusRule,
    ) -> Self {
        let mut conflicts = Vec::new();
        let approves: Vec<_> = decisions
            .iter()
            .filter(|d| d.decision == GovernanceReviewDecisionKind::Approve)
            .collect();
        let rejects: Vec<_> = decisions
            .iter()
            .filter(|d| d.decision == GovernanceReviewDecisionKind::Reject)
            .collect();
        if !approves.is_empty() && !rejects.is_empty() {
            conflicts.push(GovernanceConflictEntry {
                id: format!("conflict:decisions:{}", proposal.id),
                kind: GovernanceConflictKind::ConflictingDecisions,
                decision_references: decisions.iter().map(|d| d.id.clone()).collect(),
                condition_notes: Vec::new(),
                dissent_references: Vec::new(),
                detail: "approve and reject decisions present".into(),
            });
        }
        // Conflicting conditions across approve decisions.
        let mut condition_sets: Vec<Vec<String>> = approves
            .iter()
            .map(|d| {
                let mut c = d.conditions.clone();
                c.sort();
                c
            })
            .collect();
        condition_sets.dedup();
        if condition_sets.len() > 1 {
            conflicts.push(GovernanceConflictEntry {
                id: format!("conflict:conditions:{}", proposal.id),
                kind: GovernanceConflictKind::ConflictingConditions,
                decision_references: approves.iter().map(|d| d.id.clone()).collect(),
                condition_notes: condition_sets.into_iter().flatten().collect(),
                dissent_references: Vec::new(),
                detail: "approvals carry differing condition sets".into(),
            });
        }
        let state = if conflicts.is_empty() {
            GovernanceConflictState::ConsensusReached
        } else {
            GovernanceConflictState::Detected
        };
        Self {
            id: format!("governance_conflict:{}", proposal.id),
            proposal_reference: proposal.id.clone(),
            conflicts,
            state,
            state_history: vec![state],
            consensus_rule,
            arbiter_reference: None,
            provenance: proposal.provenance.clone(),
            history_immutable: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn record_dissent(
        &mut self,
        dissent_id: impl Into<String>,
        detail: impl Into<String>,
    ) {
        let dissent_id = dissent_id.into();
        self.conflicts.push(GovernanceConflictEntry {
            id: format!("conflict:dissent:{}", self.conflicts.len()),
            kind: GovernanceConflictKind::DissentRecorded,
            decision_references: Vec::new(),
            condition_notes: Vec::new(),
            dissent_references: vec![dissent_id],
            detail: detail.into(),
        });
        if self.state == GovernanceConflictState::ConsensusReached {
            self.state = GovernanceConflictState::Detected;
            self.state_history.push(GovernanceConflictState::Detected);
        }
    }

    fn transition(&mut self, to: GovernanceConflictState) -> Result<(), ActionProposalError> {
        if !self.state.allows_transition(to) {
            return Err(ActionProposalError::InvalidLifecycleTransition {
                from: self.state.as_str().into(),
                to: to.as_str().into(),
            });
        }
        self.state = to;
        self.state_history.push(to);
        Ok(())
    }

    pub fn require_arbitration(
        &mut self,
        arbiter: impl Into<String>,
    ) -> Result<(), ActionProposalError> {
        self.arbiter_reference = Some(arbiter.into());
        self.conflicts.push(GovernanceConflictEntry {
            id: format!("conflict:arbitration:{}", self.conflicts.len()),
            kind: GovernanceConflictKind::ArbitrationRequired,
            decision_references: Vec::new(),
            condition_notes: Vec::new(),
            dissent_references: Vec::new(),
            detail: "arbitration required".into(),
        });
        if self.state == GovernanceConflictState::Detected {
            self.transition(GovernanceConflictState::UnderArbitration)?;
        }
        Ok(())
    }

    pub fn mark_consensus(&mut self) -> Result<(), ActionProposalError> {
        if self.state == GovernanceConflictState::Detected {
            self.transition(GovernanceConflictState::UnderArbitration)?;
        }
        self.transition(GovernanceConflictState::ConsensusReached)
    }

    pub fn mark_unresolved(&mut self) -> Result<(), ActionProposalError> {
        self.transition(GovernanceConflictState::Unresolved)
    }

    pub fn is_blocking_publication(&self) -> bool {
        matches!(
            self.state,
            GovernanceConflictState::Detected
                | GovernanceConflictState::UnderArbitration
                | GovernanceConflictState::Unresolved
        ) || !self.conflicts.is_empty()
            && self.state != GovernanceConflictState::ConsensusReached
    }

    pub fn may_publish(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_modify_history(&self) -> bool {
        false
    }

    pub fn attempt_publish() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceConflictResolutionBlocked(
            "conflict resolution cannot publish".into(),
        ))
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceConflictResolutionBlocked(
            "conflict resolution cannot execute".into(),
        ))
    }

    pub fn attempt_modify_history(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceConflictResolutionBlocked(
            "conflict resolution cannot modify history".into(),
        ))
    }
}

// ---------------------------------------------------------------------------
// Sprint 157 — Governance Decision Package Contract
// ---------------------------------------------------------------------------

/// Canonical immutable review artifact (Sprint 157).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDecisionPackage {
    pub id: String,
    pub proposal_reference: String,
    pub evidence_reference: Option<String>,
    pub condition_contract_reference: Option<String>,
    pub obligation_ids: Vec<String>,
    pub reviewer_actor_ids: Vec<String>,
    pub risk_reference: Option<String>,
    pub compatibility_contract_reference: Option<String>,
    pub integrity_verification_reference: Option<String>,
    pub archive_references: Vec<String>,
    pub publication_readiness_reference: Option<String>,
    pub workflow_reference: Option<String>,
    pub conflict_resolution_reference: Option<String>,
    pub sealed: bool,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceDecisionPackage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn assemble(
        proposal: &OutcomeAdaptationProposal,
        evidence: Option<&GovernanceDecisionEvidence>,
        risk: Option<&GovernanceRisk>,
        conditions: Option<&GovernanceConditionContract>,
        compatibility: Option<&GovernanceCompatibilityContract>,
        integrity: Option<&GovernanceIntegrityVerification>,
        archive: Option<&GovernanceArchiveContract>,
        readiness: Option<&PublicationReadiness>,
        workflow: Option<&GovernanceReviewWorkflowContract>,
        conflict: Option<&GovernanceConflictResolutionContract>,
        decisions: &[GovernanceReviewDecision],
    ) -> Self {
        let obligation_ids = conditions
            .map(|c| c.obligations.iter().map(|o| o.id.clone()).collect())
            .unwrap_or_default();
        let mut reviewer_actor_ids: Vec<_> = decisions
            .iter()
            .map(|d| d.reviewer.actor_id.clone())
            .collect();
        reviewer_actor_ids.sort();
        reviewer_actor_ids.dedup();
        let archive_references = archive
            .map(|a| a.snapshots.iter().map(|s| s.id.clone()).collect())
            .unwrap_or_default();
        Self {
            id: format!("governance_decision_package:{}", proposal.id),
            proposal_reference: proposal.id.clone(),
            evidence_reference: evidence.map(|e| e.id.clone()),
            condition_contract_reference: conditions.map(|c| c.id.clone()),
            obligation_ids,
            reviewer_actor_ids,
            risk_reference: risk.map(|r| r.id.clone()),
            compatibility_contract_reference: compatibility.map(|c| c.id.clone()),
            integrity_verification_reference: integrity.map(|i| i.id.clone()),
            archive_references,
            publication_readiness_reference: readiness.map(|r| r.id.clone()),
            workflow_reference: workflow.map(|w| w.id.clone()),
            conflict_resolution_reference: conflict.map(|c| c.id.clone()),
            sealed: false,
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn seal(&mut self) -> Result<(), ActionProposalError> {
        if self.sealed {
            return Err(ActionProposalError::GovernanceDecisionPackageImmutable);
        }
        self.sealed = true;
        Ok(())
    }

    pub fn attempt_mutate_after_seal(&self) -> Result<(), ActionProposalError> {
        if self.sealed {
            Err(ActionProposalError::GovernanceDecisionPackageImmutable)
        } else {
            Ok(())
        }
    }

    pub fn may_activate(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_activate() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationActivationNotImplemented)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 158 — Governance Compliance Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComplianceCheckKind {
    RequiredEvidenceExists,
    ReviewerCountSatisfied,
    RequiredObligationsCompleted,
    CompatibilityVerified,
    IntegrityVerified,
    PublicationSafetySatisfied,
}

impl GovernanceComplianceCheckKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequiredEvidenceExists => "required_evidence_exists",
            Self::ReviewerCountSatisfied => "reviewer_count_satisfied",
            Self::RequiredObligationsCompleted => "required_obligations_completed",
            Self::CompatibilityVerified => "compatibility_verified",
            Self::IntegrityVerified => "integrity_verified",
            Self::PublicationSafetySatisfied => "publication_safety_satisfied",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceComplianceSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComplianceDiagnostic {
    pub kind: GovernanceComplianceCheckKind,
    pub severity: GovernanceComplianceSeverity,
    pub message: String,
    pub reference: Option<String>,
}

/// Compliance verification — diagnostics only (Sprint 158).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceComplianceContract {
    pub id: String,
    pub package_reference: String,
    pub diagnostics: Vec<GovernanceComplianceDiagnostic>,
    pub checked_at: String,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceComplianceContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn verify(
        package: &GovernanceDecisionPackage,
        evidence: Option<&GovernanceDecisionEvidence>,
        conditions: Option<&GovernanceConditionContract>,
        decisions: &[GovernanceReviewDecision],
        min_reviewers: u32,
        compatibility: Option<&GovernanceCompatibilityContract>,
        integrity: Option<&GovernanceIntegrityVerification>,
        safety: Option<&PublicationSafetyContract>,
        at: impl Into<String>,
    ) -> Self {
        let mut diagnostics = Vec::new();
        match evidence {
            Some(ev) if !ev.supporting_evidence_references.is_empty() => {
                diagnostics.push(GovernanceComplianceDiagnostic {
                    kind: GovernanceComplianceCheckKind::RequiredEvidenceExists,
                    severity: GovernanceComplianceSeverity::Info,
                    message: "required evidence present".into(),
                    reference: Some(ev.id.clone()),
                });
            }
            _ => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::RequiredEvidenceExists,
                severity: GovernanceComplianceSeverity::Error,
                message: "required evidence missing".into(),
                reference: package.evidence_reference.clone(),
            }),
        }
        let reviewer_count = package.reviewer_actor_ids.len() as u32;
        if reviewer_count >= min_reviewers && !decisions.is_empty() {
            diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::ReviewerCountSatisfied,
                severity: GovernanceComplianceSeverity::Info,
                message: format!("reviewer count {reviewer_count} >= {min_reviewers}"),
                reference: Some(package.id.clone()),
            });
        } else {
            diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::ReviewerCountSatisfied,
                severity: GovernanceComplianceSeverity::Error,
                message: format!("reviewer count {reviewer_count} < {min_reviewers}"),
                reference: Some(package.id.clone()),
            });
        }
        match conditions {
            Some(c)
                if c.obligations.iter().all(|o| {
                    matches!(
                        o.status,
                        GovernanceObligationStatus::Satisfied | GovernanceObligationStatus::Declared
                    ) && !matches!(
                        o.status,
                        GovernanceObligationStatus::Unmet | GovernanceObligationStatus::Expired
                    )
                }) && !c.unmet_or_expired() =>
            {
                // Declared-only is warning; all satisfied is info.
                let all_satisfied = c
                    .obligations
                    .iter()
                    .all(|o| o.status == GovernanceObligationStatus::Satisfied);
                diagnostics.push(GovernanceComplianceDiagnostic {
                    kind: GovernanceComplianceCheckKind::RequiredObligationsCompleted,
                    severity: if all_satisfied {
                        GovernanceComplianceSeverity::Info
                    } else {
                        GovernanceComplianceSeverity::Warning
                    },
                    message: if all_satisfied {
                        "obligations completed".into()
                    } else {
                        "obligations declared but not all satisfied".into()
                    },
                    reference: Some(c.id.clone()),
                });
            }
            Some(c) if c.unmet_or_expired() => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::RequiredObligationsCompleted,
                severity: GovernanceComplianceSeverity::Error,
                message: "obligations unmet or expired".into(),
                reference: Some(c.id.clone()),
            }),
            _ => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::RequiredObligationsCompleted,
                severity: GovernanceComplianceSeverity::Warning,
                message: "no condition contract supplied".into(),
                reference: package.condition_contract_reference.clone(),
            }),
        }
        match compatibility {
            Some(c) if c.verify_declared().is_ok() => {
                diagnostics.push(GovernanceComplianceDiagnostic {
                    kind: GovernanceComplianceCheckKind::CompatibilityVerified,
                    severity: GovernanceComplianceSeverity::Info,
                    message: "compatibility declared verified".into(),
                    reference: Some(c.id.clone()),
                });
            }
            Some(c) => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::CompatibilityVerified,
                severity: GovernanceComplianceSeverity::Error,
                message: "compatibility verification failed".into(),
                reference: Some(c.id.clone()),
            }),
            None => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::CompatibilityVerified,
                severity: GovernanceComplianceSeverity::Warning,
                message: "compatibility contract not supplied".into(),
                reference: None,
            }),
        }
        match integrity {
            Some(i) if !i.has_errors() => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::IntegrityVerified,
                severity: GovernanceComplianceSeverity::Info,
                message: "integrity verification has no errors".into(),
                reference: Some(i.id.clone()),
            }),
            Some(i) => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::IntegrityVerified,
                severity: GovernanceComplianceSeverity::Error,
                message: "integrity verification reported errors".into(),
                reference: Some(i.id.clone()),
            }),
            None => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::IntegrityVerified,
                severity: GovernanceComplianceSeverity::Warning,
                message: "integrity verification not supplied".into(),
                reference: None,
            }),
        }
        match safety {
            Some(s)
                if s.lifecycle_state == PublicationSafetyLifecycleState::ReleaseApproved
                    || s.lifecycle_state
                        == PublicationSafetyLifecycleState::RollbackPrepared
                    || s.lifecycle_state
                        == PublicationSafetyLifecycleState::MigrationPrepared
                    || s.lifecycle_state == PublicationSafetyLifecycleState::Validation =>
            {
                let ok = s.lifecycle_state != PublicationSafetyLifecycleState::ValidationFailed
                    && s.validation_gates.iter().all(|g| !g.required || g.passed);
                diagnostics.push(GovernanceComplianceDiagnostic {
                    kind: GovernanceComplianceCheckKind::PublicationSafetySatisfied,
                    severity: if ok {
                        GovernanceComplianceSeverity::Info
                    } else {
                        GovernanceComplianceSeverity::Error
                    },
                    message: format!("publication safety state {}", s.lifecycle_state.as_str()),
                    reference: Some(s.id.clone()),
                });
            }
            Some(s)
                if s.lifecycle_state == PublicationSafetyLifecycleState::ValidationFailed =>
            {
                diagnostics.push(GovernanceComplianceDiagnostic {
                    kind: GovernanceComplianceCheckKind::PublicationSafetySatisfied,
                    severity: GovernanceComplianceSeverity::Error,
                    message: "publication safety validation failed".into(),
                    reference: Some(s.id.clone()),
                });
            }
            Some(s) => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::PublicationSafetySatisfied,
                severity: GovernanceComplianceSeverity::Warning,
                message: format!(
                    "publication safety not yet release-approved ({})",
                    s.lifecycle_state.as_str()
                ),
                reference: Some(s.id.clone()),
            }),
            None => diagnostics.push(GovernanceComplianceDiagnostic {
                kind: GovernanceComplianceCheckKind::PublicationSafetySatisfied,
                severity: GovernanceComplianceSeverity::Warning,
                message: "publication safety not supplied".into(),
                reference: None,
            }),
        }
        Self {
            id: format!("governance_compliance:{}", package.id),
            package_reference: package.id.clone(),
            diagnostics,
            checked_at: at.into(),
            provenance: package.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == GovernanceComplianceSeverity::Error)
    }

    pub fn may_repair_automatically(&self) -> bool {
        false
    }

    pub fn may_grant_authority(&self) -> bool {
        false
    }

    pub fn attempt_repair(&self) -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceComplianceCannotMutate)
    }

    pub fn attempt_grant_authority() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceComplianceCannotMutate)
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 159 — Governance Readiness Dashboard Contract
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDashboardComplianceStatus {
    Unknown,
    Passing,
    Warnings,
    Failing,
}

impl GovernanceDashboardComplianceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Passing => "passing",
            Self::Warnings => "warnings",
            Self::Failing => "failing",
        }
    }
}

/// Domain projection for a future governance workspace UI (Sprint 159) — not UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceReadinessDashboardProjection {
    pub id: String,
    pub proposal_reference: String,
    pub review_progress_completed: u32,
    pub review_progress_total: u32,
    pub review_workflow_stage: Option<String>,
    pub outstanding_obligation_ids: Vec<String>,
    pub unresolved_conflict_ids: Vec<String>,
    pub compliance_status: GovernanceDashboardComplianceStatus,
    pub publication_readiness_state: Option<String>,
    pub archived_snapshot_count: u32,
    pub package_reference: Option<String>,
    pub provenance: RecommendationProvenance,
    pub authority_effect: String,
}

impl GovernanceReadinessDashboardProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = GOVERNANCE_AUTHORITY_EFFECT_NONE;

    pub fn project(
        proposal: &OutcomeAdaptationProposal,
        workflow: Option<&GovernanceReviewWorkflowContract>,
        conditions: Option<&GovernanceConditionContract>,
        conflict: Option<&GovernanceConflictResolutionContract>,
        compliance: Option<&GovernanceComplianceContract>,
        readiness: Option<&PublicationReadiness>,
        archive: Option<&GovernanceArchiveContract>,
        package: Option<&GovernanceDecisionPackage>,
    ) -> Self {
        let (completed, total, stage) = match workflow {
            Some(w) => (
                w.assignments
                    .iter()
                    .filter(|a| a.status == GovernanceReviewerAssignmentStatus::Completed)
                    .count() as u32,
                w.assignments.len() as u32,
                Some(w.stage.as_str().to_string()),
            ),
            None => (0, 0, None),
        };
        let outstanding_obligation_ids = conditions
            .map(|c| {
                c.obligations
                    .iter()
                    .filter(|o| {
                        !matches!(o.status, GovernanceObligationStatus::Satisfied)
                    })
                    .map(|o| o.id.clone())
                    .collect()
            })
            .unwrap_or_default();
        let unresolved_conflict_ids = conflict
            .map(|c| {
                if c.is_blocking_publication() || c.state == GovernanceConflictState::Unresolved
                {
                    c.conflicts.iter().map(|e| e.id.clone()).collect()
                } else {
                    Vec::new()
                }
            })
            .unwrap_or_default();
        let compliance_status = match compliance {
            None => GovernanceDashboardComplianceStatus::Unknown,
            Some(c) if c.has_errors() => GovernanceDashboardComplianceStatus::Failing,
            Some(c)
                if c.diagnostics
                    .iter()
                    .any(|d| d.severity == GovernanceComplianceSeverity::Warning) =>
            {
                GovernanceDashboardComplianceStatus::Warnings
            }
            Some(_) => GovernanceDashboardComplianceStatus::Passing,
        };
        Self {
            id: format!("governance_dashboard:{}", proposal.id),
            proposal_reference: proposal.id.clone(),
            review_progress_completed: completed,
            review_progress_total: total,
            review_workflow_stage: stage,
            outstanding_obligation_ids,
            unresolved_conflict_ids,
            compliance_status,
            publication_readiness_state: readiness.map(|r| r.state.as_str().to_string()),
            archived_snapshot_count: archive.map(|a| a.snapshots.len() as u32).unwrap_or(0),
            package_reference: package.map(|p| p.id.clone()),
            provenance: proposal.provenance.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_publication_ready_projection(&self) -> bool {
        self.publication_readiness_state.as_deref()
            == Some(PublicationReadinessState::ReadyForPublication.as_str())
            && self.compliance_status == GovernanceDashboardComplianceStatus::Passing
            && self.unresolved_conflict_ids.is_empty()
            && self.outstanding_obligation_ids.is_empty()
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn may_publish(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::GovernanceDashboardCannotExecute)
    }

    pub fn attempt_publish() -> Result<(), ActionProposalError> {
        Err(ActionProposalError::PublicationActivationNotImplemented)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_proposal::{
        AdaptationReviewerIdentity, GovernancePolicy, GovernanceReviewDecision,
        GovernanceReviewDecisionKind, OutcomeAdaptationReviewStatus, RecommendationFamily,
        RecommendationProvenance,
    };
    use crate::workspace_recommendation::RecommendationEvidence;

    fn sample_provenance() -> RecommendationProvenance {
        RecommendationProvenance {
            recommendation_id: "recommendation:wf".into(),
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
            id: "proposal:wf".into(),
            source_outcome_id: "outcome:wf".into(),
            source_recommendation_id: "recommendation:wf".into(),
            provenance: sample_provenance(),
            affected_area: "presentation".into(),
            proposed_change: "Keep DisplayReason primary".into(),
            expected_effect: "Clearer rationale".into(),
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
    fn workflow_supports_parallel_assignment_without_execution() {
        let proposal = stub_proposal();
        let mut wf =
            GovernanceReviewWorkflowContract::begin(&proposal, GovernanceReviewMode::Parallel);
        let a1 = wf
            .assign_reviewer(AdaptationReviewerIdentity::local_user("r1"), 1, false)
            .unwrap();
        let a2 = wf
            .assign_reviewer(AdaptationReviewerIdentity::local_user("r2"), 1, false)
            .unwrap();
        wf.enqueue_pending().unwrap();
        wf.start_review(&a1).unwrap();
        wf.start_review(&a2).unwrap();
        assert!(!wf.may_execute());
        assert!(GovernanceReviewWorkflowContract::attempt_execute().is_err());
    }

    #[test]
    fn conflict_cannot_publish_or_rewrite_history() {
        let proposal = stub_proposal();
        let policy = GovernancePolicy::for_outcome_adaptation();
        let approve = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("r1"),
            GovernanceReviewDecisionKind::Approve,
            "ok",
            "t1",
            vec!["requires testing".into()],
            &proposal.provenance,
        )
        .unwrap();
        let reject = GovernanceReviewDecision::new(
            &policy,
            AdaptationReviewerIdentity::local_user("r2"),
            GovernanceReviewDecisionKind::Reject,
            "no",
            "t2",
            vec![],
            &proposal.provenance,
        )
        .unwrap();
        let mut conflict = GovernanceConflictResolutionContract::from_decisions(
            &proposal,
            &[approve, reject],
            GovernanceConsensusRule::Majority,
        );
        assert!(conflict
            .conflicts
            .iter()
            .any(|c| c.kind == GovernanceConflictKind::ConflictingDecisions));
        assert!(conflict.history_immutable);
        assert!(!conflict.may_modify_history());
        assert!(conflict.attempt_modify_history().is_err());
        assert!(GovernanceConflictResolutionContract::attempt_publish().is_err());
        conflict.require_arbitration("arbiter-1").unwrap();
        conflict.mark_unresolved().unwrap();
        assert_eq!(conflict.state, GovernanceConflictState::Unresolved);
    }

    #[test]
    fn sealed_package_is_immutable_and_dashboard_cannot_execute() {
        let proposal = stub_proposal();
        let mut package = GovernanceDecisionPackage::assemble(
            &proposal,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            &[],
        );
        package.seal().unwrap();
        assert!(package.attempt_mutate_after_seal().is_err());
        let dash = GovernanceReadinessDashboardProjection::project(
            &proposal,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(&package),
        );
        assert!(!dash.may_execute());
        assert!(GovernanceReadinessDashboardProjection::attempt_execute().is_err());
    }
}
