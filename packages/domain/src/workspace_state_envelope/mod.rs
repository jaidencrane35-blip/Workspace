//! Unified Workspace State Envelope — Programme III Batch 1.
//!
//! Read-only composition envelope over authoritative sources.
//! Answers: what does the workspace know, from which sources, at which revisions,
//! with what freshness and uncertainty?
//! Sources remain authoritative. The envelope never owns, mutates, or repairs.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceStateEnvelopeError {
    #[error("invalid envelope status: {0}")]
    InvalidStatus(String),

    #[error("invalid freshness status: {0}")]
    InvalidFreshness(String),

    #[error("invalid availability status: {0}")]
    InvalidAvailability(String),

    #[error("invalid completeness status: {0}")]
    InvalidCompleteness(String),

    #[error("invalid consistency status: {0}")]
    InvalidConsistency(String),

    #[error("envelope artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("envelope artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("envelope snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeStatus {
    Current,
    Superseded,
    Archived,
}

impl EnvelopeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceStateEnvelopeError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceStateEnvelopeError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Explicit per-source freshness — never collapse these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessStatus {
    Fresh,
    Stale,
    Unavailable,
    Unknown,
}

impl FreshnessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceStateEnvelopeError> {
        match value {
            "fresh" => Ok(Self::Fresh),
            "stale" => Ok(Self::Stale),
            "unavailable" => Ok(Self::Unavailable),
            "unknown" => Ok(Self::Unknown),
            other => Err(WorkspaceStateEnvelopeError::InvalidFreshness(other.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityStatus {
    Available,
    Unavailable,
    Unknown,
}

impl AvailabilityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceStateEnvelopeError> {
        match value {
            "available" => Ok(Self::Available),
            "unavailable" => Ok(Self::Unavailable),
            "unknown" => Ok(Self::Unknown),
            other => Err(WorkspaceStateEnvelopeError::InvalidAvailability(other.into())),
        }
    }
}

/// Envelope-level / source-level completeness — preserve uncertainty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessStatus {
    Complete,
    Partial,
    Unknown,
    Contradictory,
}

impl CompletenessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceStateEnvelopeError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            other => Err(WorkspaceStateEnvelopeError::InvalidCompleteness(other.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsistencyStatus {
    Consistent,
    Contradictory,
    Unknown,
}

impl ConsistencyStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Consistent => "consistent",
            Self::Contradictory => "contradictory",
            Self::Unknown => "unknown",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceStateEnvelopeError> {
        match value {
            "consistent" => Ok(Self::Consistent),
            "contradictory" => Ok(Self::Contradictory),
            "unknown" => Ok(Self::Unknown),
            other => Err(WorkspaceStateEnvelopeError::InvalidConsistency(other.into())),
        }
    }
}

/// One authoritative input reference — never a lifecycle payload copy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateSource {
    pub source_id: String,
    pub source_type: String,
    pub source_revision: String,
    pub observed_at: String,
    pub freshness_status: FreshnessStatus,
    pub availability_status: AvailabilityStatus,
    pub completeness_status: CompletenessStatus,
    pub note: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceStateSource {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn observe(
        source_type: impl Into<String>,
        source_id: impl Into<String>,
        source_revision: impl Into<String>,
        observed_at: impl Into<String>,
        freshness_status: FreshnessStatus,
        availability_status: AvailabilityStatus,
        completeness_status: CompletenessStatus,
        note: Option<String>,
    ) -> Self {
        Self {
            source_id: source_id.into(),
            source_type: source_type.into(),
            source_revision: source_revision.into(),
            observed_at: observed_at.into(),
            freshness_status,
            availability_status,
            completeness_status,
            note,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn unavailable(
        source_type: impl Into<String>,
        observed_at: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_type = source_type.into();
        Self::observe(
            source_type.clone(),
            format!("unavailable:{source_type}"),
            "unavailable",
            observed_at,
            FreshnessStatus::Unavailable,
            AvailabilityStatus::Unavailable,
            CompletenessStatus::Unknown,
            Some(note.into()),
        )
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceStateEnvelopeError> {
        if !self.is_non_actionable() {
            return Err(WorkspaceStateEnvelopeError::MustNotBeActionable);
        }
        Ok(())
    }
}

/// Conflict evidence — never triggers automatic repair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateConflict {
    pub conflict_id: String,
    pub source_a: String,
    pub source_b: String,
    pub description: String,
    pub severity: String,
    pub evidence: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceStateConflict {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "workspace_state_conflict:";

    pub fn record(
        source_a: impl Into<String>,
        source_b: impl Into<String>,
        description: impl Into<String>,
        severity: impl Into<String>,
        evidence: Vec<String>,
    ) -> Self {
        Self {
            conflict_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            source_a: source_a.into(),
            source_b: source_b.into(),
            description: description.into(),
            severity: severity.into(),
            evidence,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    /// Deterministic conflict id for identical source pairs + description.
    pub fn record_deterministic(
        source_a: impl Into<String>,
        source_b: impl Into<String>,
        description: impl Into<String>,
        severity: impl Into<String>,
        evidence: Vec<String>,
    ) -> Self {
        let source_a = source_a.into();
        let source_b = source_b.into();
        let description = description.into();
        let digest = format!("{source_a}|{source_b}|{description}");
        Self {
            conflict_id: format!("{}{}", Self::ID_PREFIX, stable_digest(&digest)),
            source_a,
            source_b,
            description,
            severity: severity.into(),
            evidence,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn validate(&self) -> Result<(), WorkspaceStateEnvelopeError> {
        if !self.is_non_actionable() {
            return Err(WorkspaceStateEnvelopeError::MustNotBeActionable);
        }
        Ok(())
    }
}

/// Composed runtime envelope — composition only, never a source of truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateEnvelope {
    pub state_id: String,
    pub workspace_id: String,
    pub revision: String,
    pub generated_at: String,
    pub status: EnvelopeStatus,
    pub superseded_at: Option<String>,
    pub sources: Vec<WorkspaceStateSource>,
    pub freshness: FreshnessStatus,
    pub completeness: CompletenessStatus,
    pub consistency: ConsistencyStatus,
    pub unknowns: Vec<String>,
    pub contradictions: Vec<WorkspaceStateConflict>,
    pub composition_status: String,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceStateEnvelope {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "workspace_state_envelope:";

    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        sources: Vec<WorkspaceStateSource>,
        contradictions: Vec<WorkspaceStateConflict>,
        unknowns: Vec<String>,
    ) -> Result<Self, WorkspaceStateEnvelopeError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let mut sources = sources;
        sources.sort_by(|a, b| {
            (&a.source_type, &a.source_id, &a.source_revision).cmp(&(
                &b.source_type,
                &b.source_id,
                &b.source_revision,
            ))
        });

        let revision = compute_composition_revision(&sources);
        let freshness = aggregate_freshness(&sources);
        let completeness = aggregate_completeness(&sources, &contradictions);
        let consistency = if contradictions.is_empty() {
            if sources
                .iter()
                .any(|s| s.availability_status == AvailabilityStatus::Unknown)
            {
                ConsistencyStatus::Unknown
            } else {
                ConsistencyStatus::Consistent
            }
        } else {
            ConsistencyStatus::Contradictory
        };

        let composition_status = match (&freshness, &completeness, &consistency) {
            (FreshnessStatus::Unavailable, _, _) => "unavailable_sources".into(),
            (_, CompletenessStatus::Contradictory, _)
            | (_, _, ConsistencyStatus::Contradictory) => "contradictory".into(),
            (_, CompletenessStatus::Partial, _) => "partial".into(),
            (_, CompletenessStatus::Unknown, _) => "unknown_completeness".into(),
            (FreshnessStatus::Stale, CompletenessStatus::Complete, ConsistencyStatus::Consistent) => {
                "stale_complete".into()
            }
            (FreshnessStatus::Fresh, CompletenessStatus::Complete, ConsistencyStatus::Consistent) => {
                "composed".into()
            }
            _ => "composed_with_uncertainty".into(),
        };

        let envelope = Self {
            state_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id,
            revision,
            generated_at,
            status: EnvelopeStatus::Current,
            superseded_at: None,
            sources,
            freshness,
            completeness,
            consistency,
            unknowns,
            contradictions,
            composition_status,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        envelope.validate()?;
        Ok(envelope)
    }

    /// Deterministic compose for tests: stable state_id from revision + workspace.
    pub fn compose_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        sources: Vec<WorkspaceStateSource>,
        contradictions: Vec<WorkspaceStateConflict>,
        unknowns: Vec<String>,
    ) -> Result<Self, WorkspaceStateEnvelopeError> {
        let mut envelope = Self::compose(workspace_id, generated_at, sources, contradictions, unknowns)?;
        envelope.state_id = format!(
            "{}{}",
            Self::ID_PREFIX,
            stable_digest(&format!("{}:{}", envelope.workspace_id, envelope.revision))
        );
        Ok(envelope)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = EnvelopeStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.sources.iter().all(|s| s.is_non_actionable())
            && self.contradictions.iter().all(|c| c.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), WorkspaceStateEnvelopeError> {
        for s in &self.sources {
            s.validate()?;
        }
        for c in &self.contradictions {
            c.validate()?;
        }
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(WorkspaceStateEnvelopeError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateHistoryEntry {
    pub state_id: String,
    pub revision: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub freshness: String,
    pub completeness: String,
    pub consistency: String,
    pub source_count: usize,
    pub conflict_count: usize,
    pub unknown_count: usize,
    pub composition_status: String,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl WorkspaceStateHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_envelope(envelope: &WorkspaceStateEnvelope) -> Option<Self> {
        if !envelope.status.is_terminal() {
            return None;
        }
        Some(Self {
            state_id: envelope.state_id.clone(),
            revision: envelope.revision.clone(),
            status: envelope.status.as_str().into(),
            created_at: envelope.generated_at.clone(),
            superseded_at: envelope.superseded_at.clone(),
            freshness: envelope.freshness.as_str().into(),
            completeness: envelope.completeness.as_str().into(),
            consistency: envelope.consistency.as_str().into(),
            source_count: envelope.sources.len(),
            conflict_count: envelope.contradictions.len(),
            unknown_count: envelope.unknowns.len(),
            composition_status: envelope.composition_status.clone(),
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

/// Dual-channel unified workspace state projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<WorkspaceStateEnvelope>,
    pub history: Vec<WorkspaceStateHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl WorkspaceStateSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceStateEnvelope>,
        history: Vec<WorkspaceStateHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceStateSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        WorkspaceStateSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            state_id: self.current.as_ref().map(|c| c.state_id.clone()),
            revision: self.current.as_ref().map(|c| c.revision.clone()),
            freshness: self.current.as_ref().map(|c| c.freshness.as_str().into()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            consistency: self
                .current
                .as_ref()
                .map(|c| c.consistency.as_str().into()),
            source_count: self.current.as_ref().map(|c| c.sources.len()).unwrap_or(0),
            conflict_count: self
                .current
                .as_ref()
                .map(|c| c.contradictions.len())
                .unwrap_or(0),
            unknown_count: self.current.as_ref().map(|c| c.unknowns.len()).unwrap_or(0),
            composition_status: self
                .current
                .as_ref()
                .map(|c| c.composition_status.clone()),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub state_id: Option<String>,
    pub revision: Option<String>,
    pub freshness: Option<String>,
    pub completeness: Option<String>,
    pub consistency: Option<String>,
    pub source_count: usize,
    pub conflict_count: usize,
    pub unknown_count: usize,
    pub composition_status: Option<String>,
    pub history: Vec<WorkspaceStateHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

fn stable_digest(input: &str) -> String {
    // Lightweight deterministic digest without adding crypto deps.
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

fn compute_composition_revision(sources: &[WorkspaceStateSource]) -> String {
    let mut parts: Vec<String> = sources
        .iter()
        .map(|s| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                s.source_type,
                s.source_id,
                s.source_revision,
                s.freshness_status.as_str(),
                s.availability_status.as_str(),
                s.completeness_status.as_str()
            )
        })
        .collect();
    parts.sort();
    format!("rev:{}", stable_digest(&parts.join("|")))
}

fn aggregate_freshness(sources: &[WorkspaceStateSource]) -> FreshnessStatus {
    if sources.is_empty() {
        return FreshnessStatus::Unknown;
    }
    if sources
        .iter()
        .any(|s| s.freshness_status == FreshnessStatus::Unavailable)
    {
        // Presence of unavailable is explicit — envelope freshness is not Fresh.
        if sources
            .iter()
            .all(|s| s.freshness_status == FreshnessStatus::Unavailable)
        {
            return FreshnessStatus::Unavailable;
        }
    }
    if sources
        .iter()
        .any(|s| s.freshness_status == FreshnessStatus::Unknown)
    {
        return FreshnessStatus::Unknown;
    }
    if sources
        .iter()
        .any(|s| s.freshness_status == FreshnessStatus::Stale)
    {
        return FreshnessStatus::Stale;
    }
    if sources
        .iter()
        .any(|s| s.freshness_status == FreshnessStatus::Unavailable)
    {
        return FreshnessStatus::Stale;
    }
    FreshnessStatus::Fresh
}

fn aggregate_completeness(
    sources: &[WorkspaceStateSource],
    contradictions: &[WorkspaceStateConflict],
) -> CompletenessStatus {
    if !contradictions.is_empty() {
        return CompletenessStatus::Contradictory;
    }
    if sources.is_empty() {
        return CompletenessStatus::Unknown;
    }
    if sources
        .iter()
        .any(|s| s.completeness_status == CompletenessStatus::Contradictory)
    {
        return CompletenessStatus::Contradictory;
    }
    if sources
        .iter()
        .any(|s| s.completeness_status == CompletenessStatus::Unknown)
        || sources
            .iter()
            .any(|s| s.availability_status == AvailabilityStatus::Unavailable)
    {
        return CompletenessStatus::Partial;
    }
    if sources
        .iter()
        .any(|s| s.completeness_status == CompletenessStatus::Partial)
    {
        return CompletenessStatus::Partial;
    }
    CompletenessStatus::Complete
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_source(kind: &str, rev: &str, fresh: FreshnessStatus) -> WorkspaceStateSource {
        WorkspaceStateSource::observe(
            kind,
            format!("id:{kind}"),
            rev,
            "t0",
            fresh,
            AvailabilityStatus::Available,
            CompletenessStatus::Complete,
            None,
        )
    }

    #[test]
    fn deterministic_composition_same_inputs_same_revision() {
        let sources = vec![
            sample_source("planning", "p1", FreshnessStatus::Fresh),
            sample_source("reasoning", "r1", FreshnessStatus::Fresh),
        ];
        let a = WorkspaceStateEnvelope::compose_deterministic(
            "ws",
            "t0",
            sources.clone(),
            vec![],
            vec![],
        )
        .unwrap();
        let b = WorkspaceStateEnvelope::compose_deterministic(
            "ws",
            "t0",
            sources,
            vec![],
            vec![],
        )
        .unwrap();
        assert_eq!(a.revision, b.revision);
        assert_eq!(a.state_id, b.state_id);
        assert_eq!(a.sources, b.sources);
        assert!(a.is_non_executing());
    }

    #[test]
    fn freshness_separation_never_collapses() {
        assert_ne!(FreshnessStatus::Fresh, FreshnessStatus::Stale);
        assert_ne!(FreshnessStatus::Stale, FreshnessStatus::Unavailable);
        assert_ne!(FreshnessStatus::Unavailable, FreshnessStatus::Unknown);
        let stale = sample_source("execution", "421", FreshnessStatus::Stale);
        let unavailable = WorkspaceStateSource::unavailable(
            "execution",
            "t0",
            "Execution lifecycle unavailable because source revision 421 could not be loaded.",
        );
        assert_eq!(stale.freshness_status, FreshnessStatus::Stale);
        assert_eq!(unavailable.freshness_status, FreshnessStatus::Unavailable);
        assert_ne!(stale.freshness_status, unavailable.freshness_status);
    }

    #[test]
    fn completeness_states_preserved() {
        for status in [
            CompletenessStatus::Complete,
            CompletenessStatus::Partial,
            CompletenessStatus::Unknown,
            CompletenessStatus::Contradictory,
        ] {
            assert_eq!(CompletenessStatus::parse(status.as_str()).unwrap(), status);
        }
        let conflict = WorkspaceStateConflict::record_deterministic(
            "planning",
            "reasoning",
            "Planning present while reasoning absent",
            "medium",
            vec!["planning:available".into(), "reasoning:unavailable".into()],
        );
        let envelope = WorkspaceStateEnvelope::compose(
            "ws",
            "t0",
            vec![
                sample_source("planning", "p1", FreshnessStatus::Fresh),
                WorkspaceStateSource::unavailable("reasoning", "t0", "Reasoning snapshot missing"),
            ],
            vec![conflict],
            vec!["reasoning current unknown".into()],
        )
        .unwrap();
        assert_eq!(envelope.completeness, CompletenessStatus::Contradictory);
        assert_eq!(envelope.consistency, ConsistencyStatus::Contradictory);
        assert!(!envelope.contradictions.is_empty());
    }

    #[test]
    fn contradiction_preservation_is_evidence_only() {
        let c = WorkspaceStateConflict::record_deterministic(
            "autonomy",
            "planning",
            "Autonomy present without planning baseline",
            "high",
            vec!["autonomy:available".into()],
        );
        assert!(c.is_non_actionable());
        assert!(c.validate().is_ok());
        assert!(!c.actionable);
        assert_eq!(c.authority_effect, "none");
    }

    #[test]
    fn unknown_handling_distinct_from_unavailable() {
        let unknown = WorkspaceStateSource::observe(
            "memory",
            "memory:unknown",
            "unknown",
            "t0",
            FreshnessStatus::Unknown,
            AvailabilityStatus::Unknown,
            CompletenessStatus::Unknown,
            Some("Memory availability could not be determined".into()),
        );
        let unavailable = WorkspaceStateSource::unavailable(
            "memory",
            "t0",
            "Memory source could not be loaded",
        );
        assert_eq!(unknown.freshness_status, FreshnessStatus::Unknown);
        assert_eq!(unavailable.freshness_status, FreshnessStatus::Unavailable);
        assert_ne!(unknown.availability_status, unavailable.availability_status);
    }

    #[test]
    fn history_separation_non_actionable() {
        let mut envelope = WorkspaceStateEnvelope::compose(
            "ws",
            "t0",
            vec![sample_source("planning", "p1", FreshnessStatus::Fresh)],
            vec![],
            vec![],
        )
        .unwrap();
        envelope.mark_superseded("t1");
        let hist = WorkspaceStateHistoryEntry::from_envelope(&envelope).unwrap();
        assert!(hist.is_non_actionable());
        let snap = WorkspaceStateSnapshot::assemble("ws", None, vec![hist], 1, "t1");
        assert!(snap.is_non_commandable());
        assert_eq!(snap.summary(0).history_count, 1);
    }
}
