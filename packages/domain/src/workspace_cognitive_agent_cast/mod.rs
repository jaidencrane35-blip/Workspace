//! Workspace Cognitive Agent Cast — Programme II Batch 7.
//!
//! Governed representation of specialised cognitive roles that contribute
//! analysis, critique, synthesis, and perspectives. Evidence / coordination
//! only — agents are role representations, not actors or authorities.
//! Never execute, own lifecycles, or self-authorise.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceCognitiveAgentCastError {
    #[error("invalid cast status: {0}")]
    InvalidStatus(String),

    #[error("confidence/uncertainty must be 0..=100 (got {0})")]
    InvalidScore(u8),

    #[error("agent cast artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("agent cast artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("agent name must not be empty")]
    EmptyName,

    #[error("agent cast snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

fn clamp_score(value: u8) -> Result<u8, WorkspaceCognitiveAgentCastError> {
    if value > 100 {
        Err(WorkspaceCognitiveAgentCastError::InvalidScore(value))
    } else {
        Ok(value)
    }
}

fn normalize_name(name: impl Into<String>) -> Result<String, WorkspaceCognitiveAgentCastError> {
    let trimmed = name.into().trim().to_string();
    if trimmed.is_empty() {
        Err(WorkspaceCognitiveAgentCastError::EmptyName)
    } else {
        Ok(trimmed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CastStatus {
    Current,
    Superseded,
    Archived,
}

impl CastStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceCognitiveAgentCastError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceCognitiveAgentCastError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Cognitive role representation — never an operating authority or actor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAgent {
    pub agent_id: String,
    pub name: String,
    pub role: String,
    pub purpose: String,
    pub specialisation: String,
    pub constraints: Vec<String>,
    pub confidence_profile: String,
    pub created_at: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CognitiveAgent {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cognitive_agent:";

    pub fn define(
        name: impl Into<String>,
        role: impl Into<String>,
        purpose: impl Into<String>,
        specialisation: impl Into<String>,
        constraints: Vec<String>,
        confidence_profile: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Result<Self, WorkspaceCognitiveAgentCastError> {
        Ok(Self {
            agent_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            name: normalize_name(name)?,
            role: normalize_name(role)?,
            purpose: purpose.into(),
            specialisation: specialisation.into(),
            constraints,
            confidence_profile: confidence_profile.into(),
            created_at: created_at.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAgentCastError> {
        if !self.is_non_actionable() {
            return Err(WorkspaceCognitiveAgentCastError::MustNotBeActionable);
        }
        Ok(())
    }
}

/// Perspective contribution — observational only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentPerspective {
    pub perspective_id: String,
    pub agent_id: String,
    pub observation: String,
    pub supporting_evidence: Vec<String>,
    pub confidence: u8,
    pub uncertainty: u8,
    pub created_at: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AgentPerspective {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "agent_perspective:";

    pub fn new(
        agent_id: impl Into<String>,
        observation: impl Into<String>,
        supporting_evidence: Vec<String>,
        confidence: u8,
        uncertainty: u8,
        created_at: impl Into<String>,
    ) -> Result<Self, WorkspaceCognitiveAgentCastError> {
        Ok(Self {
            perspective_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            agent_id: agent_id.into(),
            observation: observation.into(),
            supporting_evidence,
            confidence: clamp_score(confidence)?,
            uncertainty: clamp_score(uncertainty)?,
            created_at: created_at.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAgentCastError> {
        clamp_score(self.confidence)?;
        clamp_score(self.uncertainty)?;
        if !self.is_non_actionable() {
            return Err(WorkspaceCognitiveAgentCastError::MustNotBeActionable);
        }
        Ok(())
    }
}

/// Critique — informational only; cannot reject or block systems.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCritique {
    pub critique_id: String,
    pub agent_id: String,
    pub target_reference: String,
    pub concerns: Vec<String>,
    pub risk_level: String,
    pub evidence: Vec<String>,
    pub confidence: u8,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AgentCritique {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "agent_critique:";

    pub fn new(
        agent_id: impl Into<String>,
        target_reference: impl Into<String>,
        concerns: Vec<String>,
        risk_level: impl Into<String>,
        evidence: Vec<String>,
        confidence: u8,
    ) -> Result<Self, WorkspaceCognitiveAgentCastError> {
        Ok(Self {
            critique_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            agent_id: agent_id.into(),
            target_reference: target_reference.into(),
            concerns,
            risk_level: risk_level.into(),
            evidence,
            confidence: clamp_score(confidence)?,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAgentCastError> {
        clamp_score(self.confidence)?;
        if !self.is_non_actionable() {
            return Err(WorkspaceCognitiveAgentCastError::MustNotBeActionable);
        }
        Ok(())
    }
}

/// Synthesis summary — not a decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSynthesis {
    pub synthesis_id: String,
    pub included_perspectives: Vec<String>,
    pub agreement_points: Vec<String>,
    pub disagreement_points: Vec<String>,
    pub open_questions: Vec<String>,
    pub confidence: u8,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AgentSynthesis {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "agent_synthesis:";

    pub fn new(
        included_perspectives: Vec<String>,
        agreement_points: Vec<String>,
        disagreement_points: Vec<String>,
        open_questions: Vec<String>,
        confidence: u8,
    ) -> Result<Self, WorkspaceCognitiveAgentCastError> {
        Ok(Self {
            synthesis_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            included_perspectives,
            agreement_points,
            disagreement_points,
            open_questions,
            confidence: clamp_score(confidence)?,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAgentCastError> {
        clamp_score(self.confidence)?;
        if !self.is_non_actionable() {
            return Err(WorkspaceCognitiveAgentCastError::MustNotBeActionable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CastEvidenceLink {
    pub external_ref: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAgentCastMeta {
    pub cast_id: String,
    pub workspace_id: String,
    pub status: CastStatus,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub confidence: u8,
    pub uncertainty: u8,
    pub agent_count: usize,
    pub perspective_count: usize,
    pub critique_count: usize,
    pub synthesis_count: usize,
    pub authority_effect: String,
    pub terminal: bool,
    pub actionable: bool,
}

impl CognitiveAgentCastMeta {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cognitive_cast:";

    pub fn new(
        workspace_id: impl Into<String>,
        created_at: impl Into<String>,
        confidence: u8,
        uncertainty: u8,
        agent_count: usize,
        perspective_count: usize,
        critique_count: usize,
        synthesis_count: usize,
    ) -> Result<Self, WorkspaceCognitiveAgentCastError> {
        Ok(Self {
            cast_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            status: CastStatus::Current,
            created_at: created_at.into(),
            superseded_at: None,
            confidence: clamp_score(confidence)?,
            uncertainty: clamp_score(uncertainty)?,
            agent_count,
            perspective_count,
            critique_count,
            synthesis_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            terminal: false,
            actionable: false,
        })
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = CastStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAgentCastError> {
        clamp_score(self.confidence)?;
        clamp_score(self.uncertainty)?;
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(WorkspaceCognitiveAgentCastError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAgentCastView {
    pub meta: CognitiveAgentCastMeta,
    pub agents: Vec<CognitiveAgent>,
    pub roles: Vec<String>,
    pub perspectives: Vec<AgentPerspective>,
    pub critiques: Vec<AgentCritique>,
    pub syntheses: Vec<AgentSynthesis>,
    pub confidence: u8,
    pub uncertainty: u8,
    pub evidence_links: Vec<CastEvidenceLink>,
    pub source_references: Vec<CastEvidenceLink>,
    pub authority_effect: String,
}

impl CognitiveAgentCastView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.meta.authority_effect == CognitiveAgentCastMeta::AUTHORITY_EFFECT_NONE
            && !self.meta.actionable
            && !self.meta.terminal
            && self.agents.iter().all(|a| a.is_non_actionable())
            && self.perspectives.iter().all(|p| p.is_non_actionable())
            && self.critiques.iter().all(|c| c.is_non_actionable())
            && self.syntheses.iter().all(|s| s.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveAgentCastError> {
        self.meta.validate()?;
        for a in &self.agents {
            a.validate()?;
        }
        for p in &self.perspectives {
            p.validate()?;
        }
        for c in &self.critiques {
            c.validate()?;
        }
        for s in &self.syntheses {
            s.validate()?;
        }
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceCognitiveAgentCastError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAgentCastHistoryEntry {
    pub cast_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub confidence: u8,
    pub uncertainty: u8,
    pub agent_count: usize,
    pub perspective_count: usize,
    pub critique_count: usize,
    pub synthesis_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl CognitiveAgentCastHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_meta(meta: &CognitiveAgentCastMeta) -> Option<Self> {
        if !meta.status.is_terminal() {
            return None;
        }
        Some(Self {
            cast_id: meta.cast_id.clone(),
            status: meta.status.as_str().into(),
            created_at: meta.created_at.clone(),
            superseded_at: meta.superseded_at.clone(),
            confidence: meta.confidence,
            uncertainty: meta.uncertainty,
            agent_count: meta.agent_count,
            perspective_count: meta.perspective_count,
            critique_count: meta.critique_count,
            synthesis_count: meta.synthesis_count,
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

/// Dual-channel cognitive agent cast projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveAgentCastSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<CognitiveAgentCastView>,
    pub history: Vec<CognitiveAgentCastHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl CognitiveAgentCastSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<CognitiveAgentCastView>,
        history: Vec<CognitiveAgentCastHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> CognitiveAgentCastSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        CognitiveAgentCastSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            cast_id: self.current.as_ref().map(|c| c.meta.cast_id.clone()),
            agent_count: self.current.as_ref().map(|c| c.agents.len()).unwrap_or(0),
            perspective_count: self
                .current
                .as_ref()
                .map(|c| c.perspectives.len())
                .unwrap_or(0),
            critique_count: self
                .current
                .as_ref()
                .map(|c| c.critiques.len())
                .unwrap_or(0),
            synthesis_count: self
                .current
                .as_ref()
                .map(|c| c.syntheses.len())
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
pub struct CognitiveAgentCastSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub cast_id: Option<String>,
    pub agent_count: usize,
    pub perspective_count: usize,
    pub critique_count: usize,
    pub synthesis_count: usize,
    pub confidence: Option<u8>,
    pub uncertainty: Option<u8>,
    pub history: Vec<CognitiveAgentCastHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Default specialised cognitive roles (perspectives, not permissions).
pub fn default_cast_roles() -> &'static [(&'static str, &'static str, &'static str, &'static str, &'static str)] {
    &[
        (
            "Planner Critic",
            "planner_critic",
            "Challenge planning assumptions and sequencing gaps",
            "planning_critique",
            "cautious",
        ),
        (
            "Risk Analyst",
            "risk_analyst",
            "Surface risks and uncertainty from evidence",
            "risk_analysis",
            "conservative",
        ),
        (
            "Systems Architect",
            "systems_architect",
            "Assess structural coherence across cognitive artefacts",
            "systems_structure",
            "balanced",
        ),
        (
            "User Advocate",
            "user_advocate",
            "Represent end-user clarity and friction concerns",
            "user_experience",
            "empathetic",
        ),
        (
            "Evidence Reviewer",
            "evidence_reviewer",
            "Check evidence completeness and broken references",
            "evidence_integrity",
            "skeptical",
        ),
        (
            "Efficiency Analyst",
            "efficiency_analyst",
            "Observe cost, delay, and efficiency patterns",
            "efficiency",
            "pragmatic",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_snapshot_invariants_non_commandable() {
        let agent = CognitiveAgent::define(
            "Evidence Reviewer",
            "evidence_reviewer",
            "Check evidence",
            "evidence_integrity",
            vec!["cannot execute".into()],
            "skeptical",
            "t0",
        )
        .unwrap();
        let perspective = AgentPerspective::new(
            agent.agent_id.clone(),
            "Evidence links appear incomplete",
            vec!["graph:1".into()],
            70,
            40,
            "t0",
        )
        .unwrap();
        let meta = CognitiveAgentCastMeta::new("ws", "t0", 65, 35, 1, 1, 0, 0).unwrap();
        let view = CognitiveAgentCastView {
            meta,
            agents: vec![agent],
            roles: vec!["evidence_reviewer".into()],
            perspectives: vec![perspective],
            critiques: vec![],
            syntheses: vec![],
            confidence: 65,
            uncertainty: 35,
            evidence_links: vec![],
            source_references: vec![],
            authority_effect: "none".into(),
        };
        assert!(view.is_non_executing());
        let snap = CognitiveAgentCastSnapshot::assemble("ws", Some(view), vec![], 0, "t0");
        assert!(snap.is_non_commandable());
    }

    #[test]
    fn role_separation_keeps_distinct_agents() {
        let roles = default_cast_roles();
        assert_eq!(roles.len(), 6);
        let names: Vec<_> = roles.iter().map(|r| r.0).collect();
        let mut uniq = names.clone();
        uniq.sort();
        uniq.dedup();
        assert_eq!(names.len(), uniq.len());
    }

    #[test]
    fn perspective_evidence_integrity() {
        let p = AgentPerspective::new("agent:1", "obs", vec!["ref:1".into()], 80, 20, "t0").unwrap();
        assert!(p.is_non_actionable());
        assert_eq!(p.supporting_evidence, vec!["ref:1".to_string()]);
        assert!(p.validate().is_ok());
    }

    #[test]
    fn critique_non_actionability() {
        let c = AgentCritique::new(
            "agent:1",
            "planning:1",
            vec!["Optimistic estimates".into()],
            "medium",
            vec!["task:1".into()],
            75,
        )
        .unwrap();
        assert!(c.is_non_actionable());
        let mut bad = c.clone();
        bad.actionable = true;
        assert!(bad.validate().is_err());
    }

    #[test]
    fn synthesis_identity_is_summary_not_decision() {
        let s = AgentSynthesis::new(
            vec!["perspective:1".into()],
            vec!["Evidence incomplete".into()],
            vec!["Risk level disputed".into()],
            vec!["Should planning refresh precede reasoning?".into()],
            60,
        )
        .unwrap();
        assert!(s.is_non_actionable());
        assert!(!s.synthesis_id.is_empty());
        assert!(s.agreement_points.len() + s.disagreement_points.len() > 0);
    }

    #[test]
    fn history_separation_and_summary_count() {
        let mut meta = CognitiveAgentCastMeta::new("ws", "t0", 50, 40, 2, 2, 1, 1).unwrap();
        assert!(CognitiveAgentCastHistoryEntry::from_meta(&meta).is_none());
        meta.mark_superseded("t1");
        let entry = CognitiveAgentCastHistoryEntry::from_meta(&meta).unwrap();
        assert!(entry.is_non_actionable());
        let snap =
            CognitiveAgentCastSnapshot::assemble("ws", None, vec![entry.clone(), entry], 2, "t2");
        let summary = snap.summary(1);
        assert_eq!(summary.history.len(), 1);
        assert_eq!(summary.history_count, 2);
    }
}
