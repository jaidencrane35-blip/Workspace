//! Workspace Learning & Adaptation — Programme II Batch 6.
//!
//! Meta-evidence layer downstream of Orchestration. Observes outcomes and
//! produces patterns, confidence evolution, and adaptation suggestions.
//! Never owns decisions, execution, planning truth, or lifecycles.
//! Adaptation candidates are permanently non-actionable suggestions.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceLearningAdaptationError {
    #[error("invalid learning status: {0}")]
    InvalidStatus(String),

    #[error("confidence/uncertainty must be 0..=100 (got {0})")]
    InvalidScore(u8),

    #[error("learning artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("adaptation candidates must not be actionable")]
    AdaptationMustNotBeActionable,

    #[error("learning snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

fn clamp_score(value: u8) -> Result<u8, WorkspaceLearningAdaptationError> {
    if value > 100 {
        Err(WorkspaceLearningAdaptationError::InvalidScore(value))
    } else {
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningStatus {
    Current,
    Superseded,
    Archived,
}

impl LearningStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceLearningAdaptationError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceLearningAdaptationError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Observable outcome / evidence finding (never mutates other systems).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningObservation {
    pub kind: String,
    pub statement: String,
    pub source_domain: String,
    pub evidence_refs: Vec<String>,
}

/// Detected pattern — observational only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningPattern {
    pub kind: String,
    pub label: String,
    pub statement: String,
    pub occurrence_count: u32,
    pub evidence_refs: Vec<String>,
    pub authority_effect: String,
}

impl LearningPattern {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const KIND_PATTERN_OBSERVED: &'static str = "pattern_observed";

    pub fn observed(
        label: impl Into<String>,
        statement: impl Into<String>,
        occurrence_count: u32,
        evidence_refs: Vec<String>,
    ) -> Self {
        Self {
            kind: Self::KIND_PATTERN_OBSERVED.into(),
            label: label.into(),
            statement: statement.into(),
            occurrence_count,
            evidence_refs,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Success or failure signal derived from observed outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningSignal {
    pub kind: String,
    pub polarity: String,
    pub statement: String,
    pub evidence_refs: Vec<String>,
}

impl LearningSignal {
    pub fn success(kind: impl Into<String>, statement: impl Into<String>, refs: Vec<String>) -> Self {
        Self {
            kind: kind.into(),
            polarity: "success".into(),
            statement: statement.into(),
            evidence_refs: refs,
        }
    }

    pub fn failure(kind: impl Into<String>, statement: impl Into<String>, refs: Vec<String>) -> Self {
        Self {
            kind: kind.into(),
            polarity: "failure".into(),
            statement: statement.into(),
            evidence_refs: refs,
        }
    }
}

/// Confidence evolution trail — records change, does not own foreign confidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfidenceUpdate {
    pub subject_ref: String,
    pub subject_kind: String,
    pub confidence_before: u8,
    pub evidence_received: String,
    pub confidence_after: u8,
    pub authority_effect: String,
}

impl ConfidenceUpdate {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn new(
        subject_ref: impl Into<String>,
        subject_kind: impl Into<String>,
        confidence_before: u8,
        evidence_received: impl Into<String>,
        confidence_after: u8,
    ) -> Result<Self, WorkspaceLearningAdaptationError> {
        Ok(Self {
            subject_ref: subject_ref.into(),
            subject_kind: subject_kind.into(),
            confidence_before: clamp_score(confidence_before)?,
            evidence_received: evidence_received.into(),
            confidence_after: clamp_score(confidence_after)?,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

/// Suggestion-only adaptation candidate — never auto-applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptationCandidate {
    pub id: String,
    pub title: String,
    pub suggestion: String,
    pub evidence_summary: String,
    pub supporting_evidence_refs: Vec<String>,
    pub confidence: u8,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl AdaptationCandidate {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "adaptation_candidate:";

    pub fn suggest(
        title: impl Into<String>,
        suggestion: impl Into<String>,
        evidence_summary: impl Into<String>,
        supporting_evidence_refs: Vec<String>,
        confidence: u8,
    ) -> Result<Self, WorkspaceLearningAdaptationError> {
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            title: title.into(),
            suggestion: suggestion.into(),
            evidence_summary: evidence_summary.into(),
            supporting_evidence_refs,
            confidence: clamp_score(confidence)?,
            terminal: false,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceLearningAdaptationError> {
        clamp_score(self.confidence)?;
        if self.actionable || self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceLearningAdaptationError::AdaptationMustNotBeActionable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningEvidenceLink {
    pub external_ref: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningMeta {
    pub learning_id: String,
    pub workspace_id: String,
    pub status: LearningStatus,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub uncertainty: u8,
    pub observation_count: usize,
    pub pattern_count: usize,
    pub adaptation_count: usize,
    pub authority_effect: String,
    pub terminal: bool,
    pub actionable: bool,
}

impl LearningMeta {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "learning:";

    pub fn new(
        workspace_id: impl Into<String>,
        created_at: impl Into<String>,
        uncertainty: u8,
        observation_count: usize,
        pattern_count: usize,
        adaptation_count: usize,
    ) -> Result<Self, WorkspaceLearningAdaptationError> {
        Ok(Self {
            learning_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            status: LearningStatus::Current,
            created_at: created_at.into(),
            superseded_at: None,
            uncertainty: clamp_score(uncertainty)?,
            observation_count,
            pattern_count,
            adaptation_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            terminal: false,
            actionable: false,
        })
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = LearningStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn validate(&self) -> Result<(), WorkspaceLearningAdaptationError> {
        clamp_score(self.uncertainty)?;
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(WorkspaceLearningAdaptationError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningView {
    pub meta: LearningMeta,
    pub observations: Vec<LearningObservation>,
    pub patterns: Vec<LearningPattern>,
    pub success_signals: Vec<LearningSignal>,
    pub failure_signals: Vec<LearningSignal>,
    pub confidence_updates: Vec<ConfidenceUpdate>,
    pub adaptation_candidates: Vec<AdaptationCandidate>,
    pub evidence_links: Vec<LearningEvidenceLink>,
    pub uncertainty: u8,
    pub authority_effect: String,
}

impl LearningView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.meta.authority_effect == LearningMeta::AUTHORITY_EFFECT_NONE
            && !self.meta.actionable
            && !self.meta.terminal
            && self.adaptation_candidates.iter().all(|c| c.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), WorkspaceLearningAdaptationError> {
        self.meta.validate()?;
        for c in &self.adaptation_candidates {
            c.validate()?;
        }
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceLearningAdaptationError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningHistoryEntry {
    pub learning_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub uncertainty: u8,
    pub observation_count: usize,
    pub pattern_count: usize,
    pub adaptation_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl LearningHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_meta(meta: &LearningMeta) -> Option<Self> {
        if !meta.status.is_terminal() {
            return None;
        }
        Some(Self {
            learning_id: meta.learning_id.clone(),
            status: meta.status.as_str().into(),
            created_at: meta.created_at.clone(),
            superseded_at: meta.superseded_at.clone(),
            uncertainty: meta.uncertainty,
            observation_count: meta.observation_count,
            pattern_count: meta.pattern_count,
            adaptation_count: meta.adaptation_count,
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

/// Dual-channel learning projection (`LearningSnapshot` per Batch 6 contract).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<LearningView>,
    pub history: Vec<LearningHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl LearningSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<LearningView>,
        history: Vec<LearningHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> LearningSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        LearningSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            learning_id: self
                .current
                .as_ref()
                .map(|c| c.meta.learning_id.clone()),
            observation_count: self
                .current
                .as_ref()
                .map(|c| c.observations.len())
                .unwrap_or(0),
            pattern_count: self
                .current
                .as_ref()
                .map(|c| c.patterns.len())
                .unwrap_or(0),
            adaptation_count: self
                .current
                .as_ref()
                .map(|c| c.adaptation_candidates.len())
                .unwrap_or(0),
            uncertainty: self.current.as_ref().map(|c| c.uncertainty),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub learning_id: Option<String>,
    pub observation_count: usize,
    pub pattern_count: usize,
    pub adaptation_count: usize,
    pub uncertainty: Option<u8>,
    pub history: Vec<LearningHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learning_snapshot_invariants_non_commandable() {
        let meta = LearningMeta::new("ws", "t0", 35, 1, 0, 0).unwrap();
        let view = LearningView {
            meta,
            observations: vec![LearningObservation {
                kind: "duration_estimation_error".into(),
                statement: "planned vs actual divergence observed".into(),
                source_domain: "execution".into(),
                evidence_refs: vec!["execution:1".into()],
            }],
            patterns: vec![],
            success_signals: vec![],
            failure_signals: vec![],
            confidence_updates: vec![],
            adaptation_candidates: vec![],
            evidence_links: vec![],
            uncertainty: 35,
            authority_effect: "none".into(),
        };
        assert!(view.is_non_executing());
        let snap = LearningSnapshot::assemble("ws", Some(view), vec![], 0, "t0");
        assert!(snap.is_non_commandable());
    }

    #[test]
    fn pattern_evidence_is_observational() {
        let pattern = LearningPattern::observed(
            "repeated_delays",
            "Similar delays observed across tasks",
            3,
            vec!["task:1".into(), "task:2".into()],
        );
        assert_eq!(pattern.kind, LearningPattern::KIND_PATTERN_OBSERVED);
        assert_eq!(pattern.authority_effect, "none");
        assert_eq!(pattern.occurrence_count, 3);
    }

    #[test]
    fn confidence_evolution_records_before_and_after() {
        let update = ConfidenceUpdate::new(
            "reasoning:1",
            "reasoning",
            70,
            "task completion evidence",
            55,
        )
        .unwrap();
        assert_eq!(update.confidence_before, 70);
        assert_eq!(update.confidence_after, 55);
        assert_eq!(update.authority_effect, "none");
    }

    #[test]
    fn history_separation_from_current() {
        let meta = LearningMeta::new("ws", "t0", 40, 0, 0, 0).unwrap();
        assert!(LearningHistoryEntry::from_meta(&meta).is_none());
        let mut superseded = meta;
        superseded.mark_superseded("t1");
        let entry = LearningHistoryEntry::from_meta(&superseded).unwrap();
        assert!(entry.is_non_actionable());
    }

    #[test]
    fn adaptation_candidate_non_actionability() {
        let candidate = AdaptationCandidate::suggest(
            "Optimistic planning estimates",
            "Planning estimates frequently optimistic",
            "7 completed tasks",
            vec!["task:a".into()],
            82,
        )
        .unwrap();
        assert!(candidate.is_non_actionable());
        assert!(candidate.validate().is_ok());
        let mut bad = candidate.clone();
        bad.actionable = true;
        assert!(bad.validate().is_err());
    }

    #[test]
    fn summary_preserves_history_count() {
        let mut meta = LearningMeta::new("ws", "t0", 50, 2, 1, 1).unwrap();
        meta.mark_superseded("t1");
        let entry = LearningHistoryEntry::from_meta(&meta).unwrap();
        let snap =
            LearningSnapshot::assemble("ws", None, vec![entry.clone(), entry], 2, "t2");
        let summary = snap.summary(1);
        assert_eq!(summary.history.len(), 1);
        assert_eq!(summary.history_count, 2);
        assert!(!summary.has_current);
    }
}
