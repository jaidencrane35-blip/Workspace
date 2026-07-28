//! Workspace Reasoning Memory — Programme II Batch 3.
//!
//! Durable reasoning evidence: hypothesis, assumptions, alternatives,
//! confidence/uncertainty evolution, reflection, lessons, rationale.
//! Never owns planning, decisions, recommendations, execution, tasks, or Intent.
//! Reference-only composition. Permanently non-executing.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceReasoningMemoryError {
    #[error("title must not be empty")]
    EmptyTitle,

    #[error("invalid reasoning status: {0}")]
    InvalidStatus(String),

    #[error("confidence/uncertainty must be 0..=100 (got {0})")]
    InvalidScore(u8),

    #[error("reasoning artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("reasoning record not found")]
    RecordNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

fn clamp_score(value: u8) -> Result<u8, WorkspaceReasoningMemoryError> {
    if value > 100 {
        Err(WorkspaceReasoningMemoryError::InvalidScore(value))
    } else {
        Ok(value)
    }
}

fn normalize_title(title: impl Into<String>) -> Result<String, WorkspaceReasoningMemoryError> {
    let trimmed = title.into().trim().to_string();
    if trimmed.is_empty() {
        Err(WorkspaceReasoningMemoryError::EmptyTitle)
    } else {
        Ok(trimmed)
    }
}

/// Informational status only — no execution semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningRecordStatus {
    Draft,
    Current,
    Superseded,
    Archived,
}

impl ReasoningRecordStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceReasoningMemoryError> {
        match value {
            "draft" => Ok(Self::Draft),
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceReasoningMemoryError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }

    pub fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Reference to a foreign durable identity — never duplicates payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningEvidenceReference {
    pub external_ref: String,
    pub kind: String,
}

/// Typed link to another governed surface (planning, intent, task, …).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningLink {
    pub external_ref: String,
    pub kind: String,
    pub note: String,
}

/// Durable reasoning evidence record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningRecord {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub status: ReasoningRecordStatus,
    pub hypothesis: String,
    pub assumptions: Vec<String>,
    pub alternatives: Vec<String>,
    pub rejected_alternatives: Vec<String>,
    pub evidence_refs: Vec<ReasoningEvidenceReference>,
    pub links: Vec<ReasoningLink>,
    pub confidence: u8,
    pub uncertainty: u8,
    /// Prior confidence values retained for evolution (append-only trail).
    pub confidence_evolution: Vec<u8>,
    /// Prior uncertainty values retained for evolution (append-only trail).
    pub uncertainty_evolution: Vec<u8>,
    pub reflection: String,
    pub lessons: Vec<String>,
    pub rationale: String,
    pub created_at: String,
    pub updated_at: String,
    pub superseded_at: Option<String>,
    pub authority_effect: String,
}

impl ReasoningRecord {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "reasoning:";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workspace_id: impl Into<String>,
        title: impl Into<String>,
        hypothesis: impl Into<String>,
        assumptions: Vec<String>,
        alternatives: Vec<String>,
        rejected_alternatives: Vec<String>,
        evidence_refs: Vec<ReasoningEvidenceReference>,
        links: Vec<ReasoningLink>,
        confidence: u8,
        uncertainty: u8,
        reflection: impl Into<String>,
        lessons: Vec<String>,
        rationale: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Result<Self, WorkspaceReasoningMemoryError> {
        let title = normalize_title(title)?;
        let confidence = clamp_score(confidence)?;
        let uncertainty = clamp_score(uncertainty)?;
        let created_at = created_at.into();
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            title,
            status: ReasoningRecordStatus::Current,
            hypothesis: hypothesis.into().trim().to_string(),
            assumptions,
            alternatives,
            rejected_alternatives,
            evidence_refs,
            links,
            confidence,
            uncertainty,
            confidence_evolution: vec![confidence],
            uncertainty_evolution: vec![uncertainty],
            reflection: reflection.into(),
            lessons,
            rationale: rationale.into(),
            created_at: created_at.clone(),
            updated_at: created_at,
            superseded_at: None,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn validate(&self) -> Result<(), WorkspaceReasoningMemoryError> {
        normalize_title(&self.title)?;
        clamp_score(self.confidence)?;
        clamp_score(self.uncertainty)?;
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceReasoningMemoryError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = ReasoningRecordStatus::Superseded;
        let at = at.into();
        self.superseded_at = Some(at.clone());
        self.updated_at = at;
    }

    /// Append confidence/uncertainty observation (evolution trail).
    pub fn record_score_evolution(
        &mut self,
        confidence: u8,
        uncertainty: u8,
        updated_at: impl Into<String>,
    ) -> Result<(), WorkspaceReasoningMemoryError> {
        let confidence = clamp_score(confidence)?;
        let uncertainty = clamp_score(uncertainty)?;
        self.confidence = confidence;
        self.uncertainty = uncertainty;
        self.confidence_evolution.push(confidence);
        self.uncertainty_evolution.push(uncertainty);
        self.updated_at = updated_at.into();
        Ok(())
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Terminal reasoning evidence — never actionable / never executable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningHistoryEntry {
    pub record_id: String,
    pub title: String,
    pub status: String,
    pub confidence: u8,
    pub uncertainty: u8,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub reflection_excerpt: String,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl ReasoningHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_record(record: &ReasoningRecord) -> Option<Self> {
        if !record.status.is_terminal() {
            return None;
        }
        let excerpt: String = record.reflection.chars().take(200).collect();
        Some(Self {
            record_id: record.id.clone(),
            title: record.title.clone(),
            status: record.status.as_str().into(),
            confidence: record.confidence,
            uncertainty: record.uncertainty,
            created_at: record.created_at.clone(),
            superseded_at: record.superseded_at.clone(),
            reflection_excerpt: excerpt,
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

/// Dual-channel reasoning projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<ReasoningRecord>,
    pub history: Vec<ReasoningHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl ReasoningSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<ReasoningRecord>,
        history: Vec<ReasoningHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> ReasoningSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        ReasoningSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            current_id: self.current.as_ref().map(|c| c.id.clone()),
            current_title: self.current.as_ref().map(|c| c.title.clone()),
            confidence: self.current.as_ref().map(|c| c.confidence),
            uncertainty: self.current.as_ref().map(|c| c.uncertainty),
            reflection_excerpt: self
                .current
                .as_ref()
                .map(|c| c.reflection.chars().take(200).collect())
                .unwrap_or_default(),
            lesson_count: self
                .current
                .as_ref()
                .map(|c| c.lessons.len())
                .unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Compact projection summary for UI / IPC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub current_id: Option<String>,
    pub current_title: Option<String>,
    pub confidence: Option<u8>,
    pub uncertainty: Option<u8>,
    pub reflection_excerpt: String,
    pub lesson_count: usize,
    pub history: Vec<ReasoningHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_from_current_is_none() {
        let record = ReasoningRecord::new(
            "ws",
            "Why focus first",
            "Focus drives sequencing",
            vec!["Intent is current".into()],
            vec!["Defer tasks".into()],
            vec!["Execute immediately".into()],
            vec![],
            vec![],
            70,
            30,
            "Prior plans over-rotated on tasks",
            vec!["Anchor on cognitive focus".into()],
            "Focus is the primary sequencing signal",
            "t0",
        )
        .unwrap();
        assert!(ReasoningHistoryEntry::from_record(&record).is_none());
    }

    #[test]
    fn superseded_is_non_actionable_history() {
        let mut record = ReasoningRecord::new(
            "ws",
            "Why focus first",
            "h",
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            70,
            30,
            "reflection",
            vec![],
            "rationale",
            "t0",
        )
        .unwrap();
        record.mark_superseded("t1");
        let entry = ReasoningHistoryEntry::from_record(&record).unwrap();
        assert!(entry.is_non_actionable());
        assert!(!entry.actionable);
    }

    #[test]
    fn snapshot_rejects_commandability() {
        let record = ReasoningRecord::new(
            "ws",
            "Rationale",
            "h",
            vec![],
            vec![],
            vec![],
            vec![ReasoningEvidenceReference {
                external_ref: "planning_plan:1".into(),
                kind: "planning".into(),
            }],
            vec![],
            80,
            20,
            "reflection",
            vec!["lesson".into()],
            "because",
            "t0",
        )
        .unwrap();
        let snap = ReasoningSnapshot::assemble("ws", Some(record), vec![], 0, "t0");
        assert!(snap.is_non_commandable());
        assert!(crate::projection_contract::history_count_is_authoritative(
            snap.history.len(),
            snap.history_count
        ));
    }

    #[test]
    fn confidence_and_uncertainty_evolution_append_only() {
        let mut record = ReasoningRecord::new(
            "ws",
            "Evolve",
            "h",
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            60,
            40,
            "r",
            vec![],
            "why",
            "t0",
        )
        .unwrap();
        record.record_score_evolution(70, 35, "t1").unwrap();
        record.record_score_evolution(75, 30, "t2").unwrap();
        assert_eq!(record.confidence_evolution, vec![60, 70, 75]);
        assert_eq!(record.uncertainty_evolution, vec![40, 35, 30]);
        assert_eq!(record.confidence, 75);
        assert_eq!(record.uncertainty, 30);
    }

    #[test]
    fn summary_preserves_history_count_under_window() {
        let mut record = ReasoningRecord::new(
            "ws",
            "Old",
            "h",
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            50,
            50,
            "reflection text",
            vec![],
            "why",
            "t0",
        )
        .unwrap();
        record.mark_superseded("t1");
        let entry = ReasoningHistoryEntry::from_record(&record).unwrap();
        let snap = ReasoningSnapshot::assemble("ws", None, vec![entry.clone(), entry], 2, "t2");
        let summary = snap.summary(1);
        assert_eq!(summary.history.len(), 1);
        assert_eq!(summary.history_count, 2);
        assert!(!summary.has_current);
    }

    #[test]
    fn reference_integrity_keeps_external_refs_only() {
        let record = ReasoningRecord::new(
            "ws",
            "Refs",
            "h",
            vec![],
            vec![],
            vec![],
            vec![
                ReasoningEvidenceReference {
                    external_ref: "work_goal:abc".into(),
                    kind: "intent".into(),
                },
                ReasoningEvidenceReference {
                    external_ref: "task:xyz".into(),
                    kind: "task".into(),
                },
            ],
            vec![ReasoningLink {
                external_ref: "planning_plan:1".into(),
                kind: "planning".into(),
                note: "Derived from active plan".into(),
            }],
            50,
            50,
            "r",
            vec![],
            "why",
            "t0",
        )
        .unwrap();
        assert!(record
            .evidence_refs
            .iter()
            .all(|e| e.external_ref.contains(':')));
        assert!(record.links.iter().all(|l| l.external_ref.contains(':')));
    }

    #[test]
    fn superseded_retention_keeps_terminal_identity() {
        let mut a = ReasoningRecord::new(
            "ws", "A", "h", vec![], vec![], vec![], vec![], vec![], 50, 50, "r", vec![], "why", "t0",
        )
        .unwrap();
        let id = a.id.clone();
        a.mark_superseded("t1");
        let entry = ReasoningHistoryEntry::from_record(&a).unwrap();
        assert_eq!(entry.record_id, id);
        assert_eq!(entry.status, "superseded");
        assert!(a.status.is_terminal());
    }
}
