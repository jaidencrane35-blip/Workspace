//! Workspace Cognitive Autonomy — Programme II Batch 8 (final).
//!
//! Governed decision-support / recommendation coordination layer.
//! Identifies automation opportunities while remaining strictly bounded.
//! Autonomy may suggest. Authority must still approve.
//! Never executes, self-approves, grants permissions, or mutates lifecycles.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceCognitiveAutonomyError {
    #[error("invalid autonomy status: {0}")]
    InvalidStatus(String),

    #[error("confidence/uncertainty must be 0..=100 (got {0})")]
    InvalidScore(u8),

    #[error("autonomy artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("autonomy artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("autonomy proposal must not contain executable command payloads")]
    ProposalMustNotBeExecutable,

    #[error("autonomy snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

fn clamp_score(value: u8) -> Result<u8, WorkspaceCognitiveAutonomyError> {
    if value > 100 {
        Err(WorkspaceCognitiveAutonomyError::InvalidScore(value))
    } else {
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutonomyStatus {
    Current,
    Superseded,
    Archived,
}

impl AutonomyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceCognitiveAutonomyError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceCognitiveAutonomyError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Something that could potentially be automated — suggestion only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutonomyOpportunity {
    pub opportunity_id: String,
    pub target_reference: String,
    pub description: String,
    pub expected_value: String,
    pub risk_level: String,
    pub confidence: u8,
    pub required_approval: bool,
    pub evidence: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AutonomyOpportunity {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "autonomy_opportunity:";

    pub fn suggest(
        target_reference: impl Into<String>,
        description: impl Into<String>,
        expected_value: impl Into<String>,
        risk_level: impl Into<String>,
        confidence: u8,
        evidence: Vec<String>,
    ) -> Result<Self, WorkspaceCognitiveAutonomyError> {
        Ok(Self {
            opportunity_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            target_reference: target_reference.into(),
            description: description.into(),
            expected_value: expected_value.into(),
            risk_level: risk_level.into(),
            confidence: clamp_score(confidence)?,
            required_approval: true, // always required — never self-approving
            evidence,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.required_approval
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAutonomyError> {
        clamp_score(self.confidence)?;
        if !self.is_non_actionable() {
            return Err(WorkspaceCognitiveAutonomyError::MustNotBeActionable);
        }
        Ok(())
    }
}

/// Possible workflow suggestion — no executable command payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationProposal {
    pub proposal_id: String,
    pub title: String,
    pub steps: Vec<String>,
    pub dependencies: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub approval_boundary: String,
    pub confidence: u8,
    /// Explicitly empty / descriptive only — never command payloads.
    pub executable_payload: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AutomationProposal {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "automation_proposal:";
    pub const APPROVAL_BOUNDARY: &'static str =
        "Requires explicit user/system approval via Intent → CommandPipeline → PermissionGateway";

    pub fn suggest(
        title: impl Into<String>,
        steps: Vec<String>,
        dependencies: Vec<String>,
        required_capabilities: Vec<String>,
        confidence: u8,
    ) -> Result<Self, WorkspaceCognitiveAutonomyError> {
        Ok(Self {
            proposal_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            title: title.into(),
            steps,
            dependencies,
            required_capabilities,
            approval_boundary: Self::APPROVAL_BOUNDARY.into(),
            confidence: clamp_score(confidence)?,
            executable_payload: None,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_executable(&self) -> bool {
        self.executable_payload.is_none()
            && !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAutonomyError> {
        clamp_score(self.confidence)?;
        if self.executable_payload.is_some() {
            return Err(WorkspaceCognitiveAutonomyError::ProposalMustNotBeExecutable);
        }
        if !self.is_non_executable() {
            return Err(WorkspaceCognitiveAutonomyError::MustNotBeActionable);
        }
        Ok(())
    }
}

/// Safety evidence — not enforcement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutonomySafetyAssessment {
    pub assessment_id: String,
    pub risks: Vec<String>,
    pub constraints: Vec<String>,
    pub failure_modes: Vec<String>,
    pub required_controls: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AutonomySafetyAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "autonomy_safety:";

    pub fn assess(
        risks: Vec<String>,
        constraints: Vec<String>,
        failure_modes: Vec<String>,
        required_controls: Vec<String>,
    ) -> Self {
        Self {
            assessment_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            risks,
            constraints,
            failure_modes,
            required_controls,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAutonomyError> {
        if !self.is_non_actionable() {
            return Err(WorkspaceCognitiveAutonomyError::MustNotBeActionable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutonomyRecommendation {
    pub recommendation_id: String,
    pub statement: String,
    pub related_opportunity_ids: Vec<String>,
    pub confidence: u8,
    pub requires_approval: bool,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AutonomyRecommendation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "autonomy_recommendation:";

    pub fn suggest(
        statement: impl Into<String>,
        related_opportunity_ids: Vec<String>,
        confidence: u8,
    ) -> Result<Self, WorkspaceCognitiveAutonomyError> {
        Ok(Self {
            recommendation_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            statement: statement.into(),
            related_opportunity_ids,
            confidence: clamp_score(confidence)?,
            requires_approval: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.requires_approval
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAutonomyError> {
        clamp_score(self.confidence)?;
        if !self.is_non_actionable() {
            return Err(WorkspaceCognitiveAutonomyError::MustNotBeActionable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutonomyEvidenceLink {
    pub external_ref: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAutonomyMeta {
    pub autonomy_id: String,
    pub workspace_id: String,
    pub status: AutonomyStatus,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub confidence: u8,
    pub uncertainty: u8,
    pub opportunity_count: usize,
    pub proposal_count: usize,
    pub recommendation_count: usize,
    pub authority_effect: String,
    pub terminal: bool,
    pub actionable: bool,
}

impl CognitiveAutonomyMeta {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cognitive_autonomy:";

    pub fn new(
        workspace_id: impl Into<String>,
        created_at: impl Into<String>,
        confidence: u8,
        uncertainty: u8,
        opportunity_count: usize,
        proposal_count: usize,
        recommendation_count: usize,
    ) -> Result<Self, WorkspaceCognitiveAutonomyError> {
        Ok(Self {
            autonomy_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            status: AutonomyStatus::Current,
            created_at: created_at.into(),
            superseded_at: None,
            confidence: clamp_score(confidence)?,
            uncertainty: clamp_score(uncertainty)?,
            opportunity_count,
            proposal_count,
            recommendation_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            terminal: false,
            actionable: false,
        })
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = AutonomyStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAutonomyError> {
        clamp_score(self.confidence)?;
        clamp_score(self.uncertainty)?;
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(WorkspaceCognitiveAutonomyError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAutonomyView {
    pub meta: CognitiveAutonomyMeta,
    pub opportunities: Vec<AutonomyOpportunity>,
    pub proposals: Vec<AutomationProposal>,
    pub recommendations: Vec<AutonomyRecommendation>,
    pub risk_assessments: Vec<AutonomySafetyAssessment>,
    pub approval_requirements: Vec<String>,
    pub confidence: u8,
    pub uncertainty: u8,
    pub evidence_links: Vec<AutonomyEvidenceLink>,
    pub constraints: Vec<String>,
    pub authority_effect: String,
}

impl CognitiveAutonomyView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.meta.authority_effect == CognitiveAutonomyMeta::AUTHORITY_EFFECT_NONE
            && !self.meta.actionable
            && !self.meta.terminal
            && self.opportunities.iter().all(|o| o.is_non_actionable())
            && self.proposals.iter().all(|p| p.is_non_executable())
            && self.recommendations.iter().all(|r| r.is_non_actionable())
            && self.risk_assessments.iter().all(|s| s.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAutonomyError> {
        self.meta.validate()?;
        for o in &self.opportunities {
            o.validate()?;
        }
        for p in &self.proposals {
            p.validate()?;
        }
        for r in &self.recommendations {
            r.validate()?;
        }
        for s in &self.risk_assessments {
            s.validate()?;
        }
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceCognitiveAutonomyError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAutonomyHistoryEntry {
    pub autonomy_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub confidence: u8,
    pub uncertainty: u8,
    pub opportunity_count: usize,
    pub proposal_count: usize,
    pub recommendation_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl CognitiveAutonomyHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_meta(meta: &CognitiveAutonomyMeta) -> Option<Self> {
        if !meta.status.is_terminal() {
            return None;
        }
        Some(Self {
            autonomy_id: meta.autonomy_id.clone(),
            status: meta.status.as_str().into(),
            created_at: meta.created_at.clone(),
            superseded_at: meta.superseded_at.clone(),
            confidence: meta.confidence,
            uncertainty: meta.uncertainty,
            opportunity_count: meta.opportunity_count,
            proposal_count: meta.proposal_count,
            recommendation_count: meta.recommendation_count,
            terminal: true,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        self.terminal
            && !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Dual-channel cognitive autonomy projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAutonomySnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<CognitiveAutonomyView>,
    pub history: Vec<CognitiveAutonomyHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl CognitiveAutonomySnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<CognitiveAutonomyView>,
        history: Vec<CognitiveAutonomyHistoryEntry>,
        history_count: usize,
        generated_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            generated_at: generated_at.into(),
            current,
            history,
            history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_non_commandable(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.history.iter().all(|h| h.is_non_actionable())
            && self
                .current
                .as_ref()
                .map(|c| c.is_non_executing())
                .unwrap_or(true)
    }

    pub fn summary(&self, history_limit: usize) -> CognitiveAutonomySummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        CognitiveAutonomySummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            autonomy_id: self.current.as_ref().map(|c| c.meta.autonomy_id.clone()),
            opportunity_count: self
                .current
                .as_ref()
                .map(|c| c.opportunities.len())
                .unwrap_or(0),
            proposal_count: self
                .current
                .as_ref()
                .map(|c| c.proposals.len())
                .unwrap_or(0),
            recommendation_count: self
                .current
                .as_ref()
                .map(|c| c.recommendations.len())
                .unwrap_or(0),
            confidence: self.current.as_ref().map(|c| c.confidence),
            uncertainty: self.current.as_ref().map(|c| c.uncertainty),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAutonomySummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub autonomy_id: Option<String>,
    pub opportunity_count: usize,
    pub proposal_count: usize,
    pub recommendation_count: usize,
    pub confidence: Option<u8>,
    pub uncertainty: Option<u8>,
    pub history: Vec<CognitiveAutonomyHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autonomy_snapshot_invariants_non_commandable() {
        let opp = AutonomyOpportunity::suggest(
            "learning:patterns",
            "Repeated weekly workspace cleanup detected",
            "Reduce manual repetition",
            "low",
            78,
            vec!["pattern:1".into()],
        )
        .unwrap();
        assert!(opp.required_approval);
        let meta = CognitiveAutonomyMeta::new("ws", "t0", 70, 30, 1, 0, 0).unwrap();
        let view = CognitiveAutonomyView {
            meta,
            opportunities: vec![opp],
            proposals: vec![],
            recommendations: vec![],
            risk_assessments: vec![],
            approval_requirements: vec!["All autonomy suggestions require approval".into()],
            confidence: 70,
            uncertainty: 30,
            evidence_links: vec![],
            constraints: vec!["No self-execution".into()],
            authority_effect: "none".into(),
        };
        assert!(view.is_non_executing());
        let snap = CognitiveAutonomySnapshot::assemble("ws", Some(view), vec![], 0, "t0");
        assert!(snap.is_non_commandable());
    }

    #[test]
    fn opportunity_evidence_integrity() {
        let opp = AutonomyOpportunity::suggest(
            "task:cleanup",
            "Cleanup opportunity",
            "time saved",
            "low",
            80,
            vec!["task:1".into(), "task:2".into()],
        )
        .unwrap();
        assert_eq!(opp.evidence.len(), 2);
        assert!(opp.is_non_actionable());
    }

    #[test]
    fn proposal_non_executability() {
        let p = AutomationProposal::suggest(
            "Weekly cleanup workflow",
            vec!["Review stale artefacts".into(), "Request approval".into()],
            vec!["orchestration".into()],
            vec!["work_context.write".into()],
            70,
        )
        .unwrap();
        assert!(p.is_non_executable());
        assert!(p.executable_payload.is_none());
        let mut bad = p.clone();
        bad.executable_payload = Some("launch_app".into());
        assert!(bad.validate().is_err());
    }

    #[test]
    fn safety_assessment_separation() {
        let s = AutonomySafetyAssessment::assess(
            vec!["Over-automation risk".into()],
            vec!["Approval required".into()],
            vec!["Silent execution must never occur".into()],
            vec!["CommandPipeline + PermissionGateway".into()],
        );
        assert!(s.is_non_actionable());
        assert!(s.validate().is_ok());
    }

    #[test]
    fn history_separation() {
        let mut meta = CognitiveAutonomyMeta::new("ws", "t0", 50, 40, 1, 1, 1).unwrap();
        assert!(CognitiveAutonomyHistoryEntry::from_meta(&meta).is_none());
        meta.mark_superseded("t1");
        let entry = CognitiveAutonomyHistoryEntry::from_meta(&meta).unwrap();
        assert!(entry.is_non_actionable());
        let snap =
            CognitiveAutonomySnapshot::assemble("ws", None, vec![entry.clone(), entry], 2, "t2");
        let summary = snap.summary(1);
        assert_eq!(summary.history.len(), 1);
        assert_eq!(summary.history_count, 2);
    }
}
