//! Historical Workspace Reconstruction — Programme III Batch 3.
//!
//! Temporal understanding: explain how workspace evidence changed over time.
//! Reconstruction explains change. It does not become the source of truth.
//! No event sourcing, replay authority, lifecycle replacement, or invented history.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_state_envelope::{
    ConsistencyStatus, FreshnessStatus, WorkspaceStateEnvelope, WorkspaceStateSource,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HistoricalReconstructionError {
    #[error("invalid reconstruction status: {0}")]
    InvalidStatus(String),

    #[error("invalid completeness: {0}")]
    InvalidCompleteness(String),

    #[error("invalid availability: {0}")]
    InvalidAvailability(String),

    #[error("reconstruction artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("reconstruction artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("reconstruction snapshot not found")]
    SnapshotNotFound,

    #[error("reconstruction incomplete — required evidence unavailable")]
    ReconstructionIncomplete,

    #[error("reconstruction failed — corrupt or unusable evidence")]
    ReconstructionFailed,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconstructionStatus {
    Current,
    Superseded,
    Archived,
}

impl ReconstructionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, HistoricalReconstructionError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(HistoricalReconstructionError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Explicit reconstruction completeness — never collapse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconstructionCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl ReconstructionCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, HistoricalReconstructionError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(HistoricalReconstructionError::InvalidCompleteness(other.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemporalAvailability {
    Available,
    Unavailable,
    Unknown,
}

impl TemporalAvailability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }

    pub fn parse(value: &str) -> Result<Self, HistoricalReconstructionError> {
        match value {
            "available" => Ok(Self::Available),
            "unavailable" => Ok(Self::Unavailable),
            "unknown" => Ok(Self::Unknown),
            other => Err(HistoricalReconstructionError::InvalidAvailability(other.into())),
        }
    }
}

/// Point-in-time reference — not inferred state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalSnapshot {
    pub snapshot_ref: String,
    pub revision: String,
    pub observed_at: String,
    pub source_kind: String,
    pub availability: TemporalAvailability,
    pub authority_effect: String,
    pub actionable: bool,
}

impl TemporalSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_envelope(envelope: &WorkspaceStateEnvelope) -> Self {
        Self {
            snapshot_ref: envelope.state_id.clone(),
            revision: envelope.revision.clone(),
            observed_at: envelope.generated_at.clone(),
            source_kind: "workspace_state_envelope".into(),
            availability: TemporalAvailability::Available,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn unavailable(kind: impl Into<String>, note_ref: impl Into<String>) -> Self {
        Self {
            snapshot_ref: note_ref.into(),
            revision: "unavailable".into(),
            observed_at: String::new(),
            source_kind: kind.into(),
            availability: TemporalAvailability::Unavailable,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Observed difference evidence — never unsupported causal claims.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateChangeEvidence {
    pub change_id: String,
    pub from_ref: String,
    pub to_ref: String,
    pub description: String,
    pub evidence_refs: Vec<String>,
    pub confidence: u8,
    pub authority_effect: String,
    pub actionable: bool,
}

impl StateChangeEvidence {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "state_change:";

    pub fn observed(
        from_ref: impl Into<String>,
        to_ref: impl Into<String>,
        description: impl Into<String>,
        evidence_refs: Vec<String>,
        confidence: u8,
    ) -> Self {
        let from_ref = from_ref.into();
        let to_ref = to_ref.into();
        let description = description.into();
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}",
            from_ref,
            to_ref,
            description,
            evidence_refs.join(",")
        ));
        Self {
            change_id: format!("{}{}", Self::ID_PREFIX, digest),
            from_ref,
            to_ref,
            description,
            evidence_refs,
            confidence: confidence.min(100),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceGap {
    pub gap_id: String,
    pub channel: String,
    pub description: String,
    pub severity: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_gap:";

    pub fn record(
        channel: impl Into<String>,
        description: impl Into<String>,
        severity: impl Into<String>,
    ) -> Self {
        let channel = channel.into();
        let description = description.into();
        let digest = stable_digest(&format!("{channel}|{description}"));
        Self {
            gap_id: format!("{}{}", Self::ID_PREFIX, digest),
            channel,
            description,
            severity: severity.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionComparison {
    pub comparison_id: String,
    pub left_revision: String,
    pub right_revision: String,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub changed: Vec<String>,
    pub unchanged_count: usize,
    pub result_status: ReconstructionCompleteness,
    pub authority_effect: String,
    pub actionable: bool,
}

impl RevisionComparison {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "revision_comparison:";

    pub fn compare_envelopes(
        left: &WorkspaceStateEnvelope,
        right: &WorkspaceStateEnvelope,
    ) -> Self {
        let left_map = source_index(&left.sources);
        let right_map = source_index(&right.sources);

        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut changed = Vec::new();
        let mut unchanged_count = 0usize;

        for (key, right_src) in &right_map {
            match left_map.get(key) {
                None => added.push(format!(
                    "{}:{}@{}",
                    right_src.source_type, right_src.source_id, right_src.source_revision
                )),
                Some(left_src) => {
                    if source_fingerprint(left_src) != source_fingerprint(right_src) {
                        changed.push(format!(
                            "{}:{} {}→{}",
                            right_src.source_type,
                            right_src.source_id,
                            left_src.source_revision,
                            right_src.source_revision
                        ));
                    } else {
                        unchanged_count += 1;
                    }
                }
            }
        }
        for (key, left_src) in &left_map {
            if !right_map.contains_key(key) {
                removed.push(format!(
                    "{}:{}@{}",
                    left_src.source_type, left_src.source_id, left_src.source_revision
                ));
            }
        }

        added.sort();
        removed.sort();
        changed.sort();

        let mut result_status = ReconstructionCompleteness::Complete;
        if left.consistency == ConsistencyStatus::Contradictory
            || right.consistency == ConsistencyStatus::Contradictory
        {
            result_status = ReconstructionCompleteness::Contradictory;
        } else if left.freshness == FreshnessStatus::Unknown
            || right.freshness == FreshnessStatus::Unknown
        {
            result_status = ReconstructionCompleteness::Unknown;
        } else if left.freshness == FreshnessStatus::Unavailable
            || right.freshness == FreshnessStatus::Unavailable
            || !left.unknowns.is_empty()
            || !right.unknowns.is_empty()
        {
            result_status = ReconstructionCompleteness::Partial;
        }

        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}",
            left.revision,
            right.revision,
            added.join(","),
            removed.join(","),
            changed.join(",")
        ));

        Self {
            comparison_id: format!("{}{}", Self::ID_PREFIX, digest),
            left_revision: left.revision.clone(),
            right_revision: right.revision.clone(),
            added,
            removed,
            changed,
            unchanged_count,
            result_status,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn unavailable(
        left_revision: impl Into<String>,
        right_revision: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        let left_revision = left_revision.into();
        let right_revision = right_revision.into();
        let reason = reason.into();
        let digest = stable_digest(&format!("{left_revision}|{right_revision}|{reason}"));
        Self {
            comparison_id: format!("{}{}", Self::ID_PREFIX, digest),
            left_revision,
            right_revision,
            added: vec![],
            removed: vec![],
            changed: vec![reason],
            unchanged_count: 0,
            result_status: ReconstructionCompleteness::Unavailable,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Provenance link to durable evidence used in reconstruction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconstructionProvenanceLink {
    pub external_ref: String,
    pub kind: String,
}

/// Read-only historical interpretation envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceHistoricalView {
    pub reconstruction_id: String,
    pub workspace_id: String,
    pub from_revision: Option<String>,
    pub to_revision: Option<String>,
    pub generated_at: String,
    pub status: ReconstructionStatus,
    pub superseded_at: Option<String>,
    pub temporal_snapshots: Vec<TemporalSnapshot>,
    pub timeline: Vec<StateChangeEvidence>,
    pub comparisons: Vec<RevisionComparison>,
    pub completeness: ReconstructionCompleteness,
    pub gaps: Vec<EvidenceGap>,
    pub provenance_links: Vec<ReconstructionProvenanceLink>,
    pub explanation: String,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceHistoricalView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "historical_reconstruction:";

    pub fn reconstruct(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        envelopes: &[WorkspaceStateEnvelope],
    ) -> Result<Self, HistoricalReconstructionError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let mut gaps = Vec::new();
        let mut timeline = Vec::new();
        let mut comparisons = Vec::new();
        let mut temporal_snapshots = Vec::new();
        let mut provenance_links = Vec::new();

        // Newest first expected; normalize to oldest→newest for comparison.
        let mut ordered = envelopes.to_vec();
        ordered.sort_by(|a, b| a.generated_at.cmp(&b.generated_at));

        if ordered.is_empty() {
            gaps.push(EvidenceGap::record(
                "workspace_state_envelope",
                "No durable workspace state envelopes available — No evidence found ≠ Nothing happened",
                "high",
            ));
            return Ok(Self {
                reconstruction_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
                workspace_id,
                from_revision: None,
                to_revision: None,
                generated_at,
                status: ReconstructionStatus::Current,
                superseded_at: None,
                temporal_snapshots: vec![TemporalSnapshot::unavailable(
                    "workspace_state_envelope",
                    "unavailable:envelope",
                )],
                timeline: vec![],
                comparisons: vec![],
                completeness: ReconstructionCompleteness::Unavailable,
                gaps,
                provenance_links: vec![],
                explanation:
                    "Reconstruction unavailable: no durable envelope evidence could be loaded."
                        .into(),
                authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
                terminal: false,
            });
        }

        for env in &ordered {
            temporal_snapshots.push(TemporalSnapshot::from_envelope(env));
            provenance_links.push(ReconstructionProvenanceLink {
                external_ref: env.state_id.clone(),
                kind: "workspace_state_envelope".into(),
            });
            if env.consistency == ConsistencyStatus::Contradictory {
                gaps.push(EvidenceGap::record(
                    "envelope_consistency",
                    format!(
                        "Envelope {} reports contradictory source evidence",
                        env.revision
                    ),
                    "medium",
                ));
            }
            if !env.unknowns.is_empty() {
                gaps.push(EvidenceGap::record(
                    "envelope_unknowns",
                    format!(
                        "Envelope {} has {} unknown channel(s)",
                        env.revision,
                        env.unknowns.len()
                    ),
                    "low",
                ));
            }
        }

        if ordered.len() == 1 {
            gaps.push(EvidenceGap::record(
                "revision_pair",
                "Only one durable envelope present — cannot compare two revisions",
                "medium",
            ));
            let only = &ordered[0];
            let completeness = if !gaps.is_empty() {
                ReconstructionCompleteness::Partial
            } else {
                ReconstructionCompleteness::Partial
            };
            return Ok(Self {
                reconstruction_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
                workspace_id,
                from_revision: Some(only.revision.clone()),
                to_revision: Some(only.revision.clone()),
                generated_at,
                status: ReconstructionStatus::Current,
                superseded_at: None,
                temporal_snapshots,
                timeline: vec![],
                comparisons: vec![],
                completeness,
                gaps,
                provenance_links,
                explanation: format!(
                    "Partial reconstruction: single envelope revision {} observed; no prior revision available for comparison.",
                    only.revision
                ),
                authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
                terminal: false,
            });
        }

        // Compare consecutive pairs oldest→newest.
        for window in ordered.windows(2) {
            let left = &window[0];
            let right = &window[1];
            let comparison = RevisionComparison::compare_envelopes(left, right);
            for item in &comparison.added {
                timeline.push(StateChangeEvidence::observed(
                    &left.revision,
                    &right.revision,
                    format!("Source appeared in later revision: {item}"),
                    vec![left.state_id.clone(), right.state_id.clone()],
                    80,
                ));
            }
            for item in &comparison.removed {
                timeline.push(StateChangeEvidence::observed(
                    &left.revision,
                    &right.revision,
                    format!("Source absent in later revision: {item}"),
                    vec![left.state_id.clone(), right.state_id.clone()],
                    80,
                ));
            }
            for item in &comparison.changed {
                timeline.push(StateChangeEvidence::observed(
                    &left.revision,
                    &right.revision,
                    format!("Source revision changed: {item}"),
                    vec![left.state_id.clone(), right.state_id.clone()],
                    85,
                ));
            }
            if left.freshness != right.freshness {
                timeline.push(StateChangeEvidence::observed(
                    &left.revision,
                    &right.revision,
                    format!(
                        "Envelope freshness changed: {} → {}",
                        left.freshness.as_str(),
                        right.freshness.as_str()
                    ),
                    vec![left.state_id.clone(), right.state_id.clone()],
                    70,
                ));
            }
            if left.completeness != right.completeness {
                timeline.push(StateChangeEvidence::observed(
                    &left.revision,
                    &right.revision,
                    format!(
                        "Envelope completeness changed: {} → {}",
                        left.completeness.as_str(),
                        right.completeness.as_str()
                    ),
                    vec![left.state_id.clone(), right.state_id.clone()],
                    70,
                ));
            }
            comparisons.push(comparison);
        }

        timeline.sort_by(|a, b| a.change_id.cmp(&b.change_id));

        let completeness = if gaps.iter().any(|g| g.channel == "envelope_consistency")
            || comparisons
                .iter()
                .any(|c| c.result_status == ReconstructionCompleteness::Contradictory)
        {
            ReconstructionCompleteness::Contradictory
        } else if comparisons
            .iter()
            .any(|c| c.result_status == ReconstructionCompleteness::Unknown)
            || gaps.iter().any(|g| g.severity == "high")
        {
            ReconstructionCompleteness::Unknown
        } else if !gaps.is_empty()
            || comparisons
                .iter()
                .any(|c| c.result_status == ReconstructionCompleteness::Partial)
        {
            ReconstructionCompleteness::Partial
        } else {
            ReconstructionCompleteness::Complete
        };

        let from_revision = ordered.first().map(|e| e.revision.clone());
        let to_revision = ordered.last().map(|e| e.revision.clone());
        let explanation = format!(
            "Reconstruction of {} envelope revision(s) from {:?} to {:?}: {} observed change(s), completeness={}.",
            ordered.len(),
            from_revision,
            to_revision,
            timeline.len(),
            completeness.as_str()
        );

        let view = Self {
            reconstruction_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id,
            from_revision,
            to_revision,
            generated_at,
            status: ReconstructionStatus::Current,
            superseded_at: None,
            temporal_snapshots,
            timeline,
            comparisons,
            completeness,
            gaps,
            provenance_links,
            explanation,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        view.validate()?;
        Ok(view)
    }

    /// Deterministic reconstruction id for tests (same inputs ⇒ same id).
    pub fn reconstruct_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        envelopes: &[WorkspaceStateEnvelope],
    ) -> Result<Self, HistoricalReconstructionError> {
        let mut view = Self::reconstruct(workspace_id, generated_at, envelopes)?;
        let digest = stable_digest(&format!(
            "{}|{:?}|{:?}|{}|{}",
            view.workspace_id,
            view.from_revision,
            view.to_revision,
            view.timeline
                .iter()
                .map(|t| t.change_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            view.completeness.as_str()
        ));
        view.reconstruction_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(view)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = ReconstructionStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.temporal_snapshots.iter().all(|s| s.is_non_actionable())
            && self.timeline.iter().all(|t| t.is_non_actionable())
            && self.comparisons.iter().all(|c| c.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), HistoricalReconstructionError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(HistoricalReconstructionError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalReconstructionHistoryEntry {
    pub reconstruction_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub from_revision: Option<String>,
    pub to_revision: Option<String>,
    pub completeness: String,
    pub change_count: usize,
    pub gap_count: usize,
    pub comparison_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl HistoricalReconstructionHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_view(view: &WorkspaceHistoricalView) -> Option<Self> {
        if !view.status.is_terminal() {
            return None;
        }
        Some(Self {
            reconstruction_id: view.reconstruction_id.clone(),
            status: view.status.as_str().into(),
            created_at: view.generated_at.clone(),
            superseded_at: view.superseded_at.clone(),
            from_revision: view.from_revision.clone(),
            to_revision: view.to_revision.clone(),
            completeness: view.completeness.as_str().into(),
            change_count: view.timeline.len(),
            gap_count: view.gaps.len(),
            comparison_count: view.comparisons.len(),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalReconstructionSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<WorkspaceHistoricalView>,
    pub history: Vec<HistoricalReconstructionHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl HistoricalReconstructionSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceHistoricalView>,
        history: Vec<HistoricalReconstructionHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> HistoricalReconstructionSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        HistoricalReconstructionSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            reconstruction_id: self
                .current
                .as_ref()
                .map(|c| c.reconstruction_id.clone()),
            from_revision: self.current.as_ref().and_then(|c| c.from_revision.clone()),
            to_revision: self.current.as_ref().and_then(|c| c.to_revision.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            change_count: self.current.as_ref().map(|c| c.timeline.len()).unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalReconstructionSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub reconstruction_id: Option<String>,
    pub from_revision: Option<String>,
    pub to_revision: Option<String>,
    pub completeness: Option<String>,
    pub change_count: usize,
    pub gap_count: usize,
    pub history: Vec<HistoricalReconstructionHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Explanation surface for ExplainHistoricalChange.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalChangeExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub from_revision: Option<String>,
    pub to_revision: Option<String>,
    pub completeness: Option<String>,
    pub observed_changes: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub gaps: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl HistoricalChangeExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_view(view: &WorkspaceHistoricalView) -> Self {
        Self {
            explanation_id: format!("historical_explanation:{}", view.reconstruction_id),
            workspace_id: view.workspace_id.clone(),
            from_revision: view.from_revision.clone(),
            to_revision: view.to_revision.clone(),
            completeness: Some(view.completeness.as_str().into()),
            observed_changes: view.timeline.iter().map(|t| t.description.clone()).collect(),
            evidence_refs: view
                .timeline
                .iter()
                .flat_map(|t| t.evidence_refs.clone())
                .collect(),
            gaps: view.gaps.iter().map(|g| g.description.clone()).collect(),
            uncertainty: view
                .gaps
                .iter()
                .map(|g| format!("{}: {}", g.channel, g.description))
                .collect(),
            narrative: view.explanation.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

fn source_index(
    sources: &[WorkspaceStateSource],
) -> std::collections::BTreeMap<String, WorkspaceStateSource> {
    let mut map = std::collections::BTreeMap::new();
    for s in sources {
        map.insert(format!("{}:{}", s.source_type, s.source_id), s.clone());
    }
    map
}

fn source_fingerprint(source: &WorkspaceStateSource) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        source.source_revision,
        source.freshness_status.as_str(),
        source.availability_status.as_str(),
        source.completeness_status.as_str(),
        source.note.clone().unwrap_or_default()
    )
}

fn stable_digest(input: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_state_envelope::{
        AvailabilityStatus, CompletenessStatus, ConsistencyStatus, FreshnessStatus,
        WorkspaceStateConflict, WorkspaceStateSource,
    };

    fn envelope(
        id: &str,
        rev: &str,
        at: &str,
        sources: Vec<WorkspaceStateSource>,
        contradictions: Vec<WorkspaceStateConflict>,
        unknowns: Vec<String>,
    ) -> WorkspaceStateEnvelope {
        let mut e = WorkspaceStateEnvelope::compose_deterministic("ws", at, sources, contradictions, unknowns)
            .unwrap();
        e.state_id = id.into();
        e.revision = rev.into();
        e
    }

    fn src(kind: &str, rev: &str) -> WorkspaceStateSource {
        WorkspaceStateSource::observe(
            kind,
            format!("id:{kind}"),
            rev,
            "t0",
            FreshnessStatus::Fresh,
            AvailabilityStatus::Available,
            CompletenessStatus::Complete,
            None,
        )
    }

    #[test]
    fn deterministic_reconstruction_identical_inputs() {
        let a = envelope(
            "env:1",
            "rev:a",
            "t0",
            vec![src("planning", "p1")],
            vec![],
            vec![],
        );
        let b = envelope(
            "env:2",
            "rev:b",
            "t1",
            vec![src("planning", "p2")],
            vec![],
            vec![],
        );
        let left = WorkspaceHistoricalView::reconstruct_deterministic("ws", "t2", &[a.clone(), b.clone()])
            .unwrap();
        let right = WorkspaceHistoricalView::reconstruct_deterministic("ws", "t2", &[a, b]).unwrap();
        assert_eq!(left.reconstruction_id, right.reconstruction_id);
        assert_eq!(left.timeline, right.timeline);
        assert_eq!(left.comparisons, right.comparisons);
        assert!(left.is_non_executing());
    }

    #[test]
    fn missing_evidence_is_unavailable_not_invented() {
        let view = WorkspaceHistoricalView::reconstruct("ws", "t0", &[]).unwrap();
        assert_eq!(view.completeness, ReconstructionCompleteness::Unavailable);
        assert!(view.timeline.is_empty());
        assert!(!view.gaps.is_empty());
        assert!(view.explanation.contains("unavailable"));
    }

    #[test]
    fn single_envelope_is_partial_not_fabricated_transition() {
        let a = envelope(
            "env:1",
            "rev:a",
            "t0",
            vec![src("planning", "p1")],
            vec![],
            vec![],
        );
        let view = WorkspaceHistoricalView::reconstruct("ws", "t1", &[a]).unwrap();
        assert_eq!(view.completeness, ReconstructionCompleteness::Partial);
        assert!(view.timeline.is_empty());
        assert!(view.gaps.iter().any(|g| g.channel == "revision_pair"));
    }

    #[test]
    fn contradictory_evidence_preserved() {
        let conflict = WorkspaceStateConflict::record_deterministic(
            "planning",
            "reasoning",
            "conflict",
            "high",
            vec!["e".into()],
        );
        let a = envelope(
            "env:1",
            "rev:a",
            "t0",
            vec![src("planning", "p1")],
            vec![conflict.clone()],
            vec![],
        );
        let b = envelope(
            "env:2",
            "rev:b",
            "t1",
            vec![src("planning", "p1")],
            vec![conflict],
            vec![],
        );
        // Force contradictory consistency via compose path — compose sets contradictory when conflicts present
        assert_eq!(a.consistency, ConsistencyStatus::Contradictory);
        let view = WorkspaceHistoricalView::reconstruct("ws", "t2", &[a, b]).unwrap();
        assert_eq!(view.completeness, ReconstructionCompleteness::Contradictory);
        assert!(view.gaps.iter().any(|g| g.channel == "envelope_consistency"));
    }

    #[test]
    fn no_fabricated_transitions_when_unchanged() {
        let a = envelope(
            "env:1",
            "rev:a",
            "t0",
            vec![src("planning", "p1")],
            vec![],
            vec![],
        );
        let mut b = a.clone();
        b.state_id = "env:2".into();
        b.generated_at = "t1".into();
        // Same revision fingerprint content
        let view = WorkspaceHistoricalView::reconstruct("ws", "t2", &[a, b]).unwrap();
        assert!(view
            .timeline
            .iter()
            .all(|t| !t.description.contains("invented")));
        assert!(view.comparisons.iter().all(|c| c.added.is_empty() && c.removed.is_empty() && c.changed.is_empty()));
    }

    #[test]
    fn history_separation_non_actionable() {
        let a = envelope(
            "env:1",
            "rev:a",
            "t0",
            vec![src("planning", "p1")],
            vec![],
            vec![],
        );
        let mut view = WorkspaceHistoricalView::reconstruct("ws", "t1", &[a]).unwrap();
        view.mark_superseded("t2");
        let hist = HistoricalReconstructionHistoryEntry::from_view(&view).unwrap();
        assert!(hist.is_non_actionable());
        let snap = HistoricalReconstructionSnapshot::assemble("ws", None, vec![hist], 1, "t2");
        assert!(snap.is_non_commandable());
    }
}
