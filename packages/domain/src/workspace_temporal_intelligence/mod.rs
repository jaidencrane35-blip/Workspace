//! Temporal Intelligence — Programme III Batch 4.
//!
//! Temporal intelligence organises and deepens historical understanding.
//! It does not predict, simulate, correct, replay, or become truth.
//! Observed sequence ≠ Cause unless causal evidence explicitly exists.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::workspace_historical_reconstruction::{
    EvidenceGap, ReconstructionCompleteness, WorkspaceHistoricalView,
};
use crate::workspace_state_envelope::WorkspaceStateEnvelope;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TemporalIntelligenceError {
    #[error("invalid temporal analysis status: {0}")]
    InvalidStatus(String),

    #[error("invalid understanding completeness: {0}")]
    InvalidCompleteness(String),

    #[error("temporal analysis artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("temporal analysis artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("temporal analysis snapshot not found")]
    SnapshotNotFound,

    #[error("temporal analysis incomplete — required evidence unavailable")]
    AnalysisIncomplete,

    #[error("temporal analysis failed — corrupt or unusable evidence")]
    AnalysisFailed,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemporalAnalysisStatus {
    Current,
    Superseded,
    Archived,
}

impl TemporalAnalysisStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, TemporalIntelligenceError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(TemporalIntelligenceError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Explicit understanding completeness — never collapse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl UnderstandingCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, TemporalIntelligenceError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(TemporalIntelligenceError::InvalidCompleteness(other.into())),
        }
    }

    pub fn from_reconstruction(c: ReconstructionCompleteness) -> Self {
        match c {
            ReconstructionCompleteness::Complete => Self::Complete,
            ReconstructionCompleteness::Partial => Self::Partial,
            ReconstructionCompleteness::Unknown => Self::Unknown,
            ReconstructionCompleteness::Contradictory => Self::Contradictory,
            ReconstructionCompleteness::Unavailable => Self::Unavailable,
        }
    }
}

/// Scoped temporal / revision window — constrains reading; does not invent coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalAnalysisWindow {
    pub from_revision: Option<String>,
    pub to_revision: Option<String>,
    pub from_observed_at: Option<String>,
    pub to_observed_at: Option<String>,
    pub max_chain_length: usize,
    pub theme: Option<String>,
}

impl TemporalAnalysisWindow {
    pub const DEFAULT_MAX_CHAIN_LENGTH: usize = 64;

    pub fn unbounded() -> Self {
        Self {
            from_revision: None,
            to_revision: None,
            from_observed_at: None,
            to_observed_at: None,
            max_chain_length: Self::DEFAULT_MAX_CHAIN_LENGTH,
            theme: None,
        }
    }

    pub fn between_revisions(
        from_revision: impl Into<String>,
        to_revision: impl Into<String>,
    ) -> Self {
        Self {
            from_revision: Some(from_revision.into()),
            to_revision: Some(to_revision.into()),
            from_observed_at: None,
            to_observed_at: None,
            max_chain_length: Self::DEFAULT_MAX_CHAIN_LENGTH,
            theme: None,
        }
    }
}

/// Long-chain understanding over ordered revisions — projection only, not memory SoT.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionChainSummary {
    pub chain_id: String,
    pub ordered_refs: Vec<String>,
    pub observed_transitions: Vec<String>,
    pub unchanged_spans: Vec<String>,
    pub gap_spans: Vec<String>,
    pub completeness: UnderstandingCompleteness,
    pub authority_effect: String,
    pub actionable: bool,
}

impl RevisionChainSummary {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "revision_chain:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Evidence-backed conflict surface — explain disagreement, never resolve it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalConflictExplanation {
    pub conflict_id: String,
    pub subject_refs: Vec<String>,
    pub description: String,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl TemporalConflictExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "temporal_conflict:";

    pub fn observed(
        subject_refs: Vec<String>,
        description: impl Into<String>,
        evidence_refs: Vec<String>,
        uncertainty: Vec<String>,
    ) -> Self {
        let description = description.into();
        let digest = stable_digest(&format!(
            "{}|{}|{}",
            subject_refs.join(","),
            description,
            evidence_refs.join(",")
        ));
        Self {
            conflict_id: format!("{}{}", Self::ID_PREFIX, digest),
            subject_refs,
            description,
            evidence_refs,
            uncertainty,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Diagnostic evidence-quality assessment — never a correctness / authority score.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceQualityAssessment {
    pub assessment_id: String,
    pub completeness: String,
    pub provenance_coverage: u8,
    pub source_availability: String,
    pub contradiction_density: u8,
    pub uncertainty: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceQualityAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "evidence_quality:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalProvenanceLink {
    pub external_ref: String,
    pub kind: String,
}

/// Read-only temporal analysis / understanding projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalAnalysisView {
    pub analysis_id: String,
    pub workspace_id: String,
    pub window: TemporalAnalysisWindow,
    pub generated_at: String,
    pub status: TemporalAnalysisStatus,
    pub superseded_at: Option<String>,
    pub chain_summary: RevisionChainSummary,
    pub conflict_explanations: Vec<TemporalConflictExplanation>,
    pub evidence_quality: EvidenceQualityAssessment,
    pub completeness: UnderstandingCompleteness,
    pub gaps: Vec<EvidenceGap>,
    pub provenance_links: Vec<TemporalProvenanceLink>,
    pub narrative: String,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl TemporalAnalysisView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "temporal_analysis:";

    /// Analyse durable reconstruction (+ optional envelopes) within a window.
    /// Uses sequence language only — never unsupported causal claims.
    pub fn analyse(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        window: TemporalAnalysisWindow,
        reconstruction: Option<&WorkspaceHistoricalView>,
        envelopes: &[WorkspaceStateEnvelope],
    ) -> Result<Self, TemporalIntelligenceError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let max_len = window.max_chain_length.max(1);

        let Some(recon) = reconstruction else {
            return Ok(Self::unavailable(
                workspace_id,
                generated_at,
                window,
                "No durable historical reconstruction available — Missing evidence ≠ No change",
            ));
        };

        let mut ordered_envs = filter_envelopes(envelopes, &window);
        ordered_envs.sort_by(|a, b| a.generated_at.cmp(&b.generated_at));
        if ordered_envs.len() > max_len {
            ordered_envs.truncate(max_len);
        }

        let mut gaps = recon.gaps.clone();
        let mut provenance_links = vec![TemporalProvenanceLink {
            external_ref: recon.reconstruction_id.clone(),
            kind: "historical_reconstruction".into(),
        }];
        for env in &ordered_envs {
            provenance_links.push(TemporalProvenanceLink {
                external_ref: env.state_id.clone(),
                kind: "workspace_state_envelope".into(),
            });
        }

        let chain_summary = build_chain_summary(recon, &ordered_envs, &window, &mut gaps);
        let conflict_explanations = build_conflict_explanations(recon, &ordered_envs);
        let evidence_quality =
            build_evidence_quality(recon, &ordered_envs, &gaps, &conflict_explanations);
        let completeness = derive_completeness(recon, &gaps, &conflict_explanations);

        let narrative = build_sequence_narrative(recon, &chain_summary, &conflict_explanations);

        let view = Self {
            analysis_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id,
            window,
            generated_at,
            status: TemporalAnalysisStatus::Current,
            superseded_at: None,
            chain_summary,
            conflict_explanations,
            evidence_quality,
            completeness,
            gaps,
            provenance_links,
            narrative,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        view.validate()?;
        Ok(view)
    }

    pub fn analyse_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        window: TemporalAnalysisWindow,
        reconstruction: Option<&WorkspaceHistoricalView>,
        envelopes: &[WorkspaceStateEnvelope],
    ) -> Result<Self, TemporalIntelligenceError> {
        let mut view = Self::analyse(workspace_id, generated_at, window, reconstruction, envelopes)?;
        let digest = stable_digest(&format!(
            "{}|{:?}|{:?}|{}|{}|{}",
            view.workspace_id,
            view.window.from_revision,
            view.window.to_revision,
            view.chain_summary.chain_id,
            view.conflict_explanations
                .iter()
                .map(|c| c.conflict_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            view.completeness.as_str()
        ));
        view.analysis_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(view)
    }

    fn unavailable(
        workspace_id: String,
        generated_at: String,
        window: TemporalAnalysisWindow,
        reason: &str,
    ) -> Self {
        let gaps = vec![EvidenceGap::record(
            "historical_reconstruction",
            reason,
            "high",
        )];
        let chain_id = format!(
            "{}{}",
            RevisionChainSummary::ID_PREFIX,
            stable_digest("unavailable")
        );
        Self {
            analysis_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id,
            window,
            generated_at,
            status: TemporalAnalysisStatus::Current,
            superseded_at: None,
            chain_summary: RevisionChainSummary {
                chain_id,
                ordered_refs: vec![],
                observed_transitions: vec![],
                unchanged_spans: vec![],
                gap_spans: vec!["unavailable:reconstruction".into()],
                completeness: UnderstandingCompleteness::Unavailable,
                authority_effect: RevisionChainSummary::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            },
            conflict_explanations: vec![],
            evidence_quality: EvidenceQualityAssessment {
                assessment_id: format!(
                    "{}{}",
                    EvidenceQualityAssessment::ID_PREFIX,
                    stable_digest("unavailable")
                ),
                completeness: "unavailable".into(),
                provenance_coverage: 0,
                source_availability: "unavailable".into(),
                contradiction_density: 0,
                uncertainty: vec![reason.into()],
                limitations: vec![
                    "Evidence quality is diagnostic only — not a correctness or authority score"
                        .into(),
                    reason.into(),
                ],
                authority_effect: EvidenceQualityAssessment::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            },
            completeness: UnderstandingCompleteness::Unavailable,
            gaps,
            provenance_links: vec![],
            narrative: format!(
                "Temporal analysis unavailable: {reason}. Observed sequence language only; no causes inferred."
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        }
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = TemporalAnalysisStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.chain_summary.is_non_actionable()
            && self
                .conflict_explanations
                .iter()
                .all(|c| c.is_non_actionable())
            && self.evidence_quality.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), TemporalIntelligenceError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(TemporalIntelligenceError::AuthorityEffectMustBeNone);
        }
        if narrative_has_forbidden_causality(&self.narrative) {
            return Err(TemporalIntelligenceError::MustNotBeActionable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalIntelligenceHistoryEntry {
    pub analysis_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub from_revision: Option<String>,
    pub to_revision: Option<String>,
    pub completeness: String,
    pub conflict_count: usize,
    pub gap_count: usize,
    pub chain_ref_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl TemporalIntelligenceHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_view(view: &TemporalAnalysisView) -> Option<Self> {
        if !view.status.is_terminal() {
            return None;
        }
        Some(Self {
            analysis_id: view.analysis_id.clone(),
            status: view.status.as_str().into(),
            created_at: view.generated_at.clone(),
            superseded_at: view.superseded_at.clone(),
            from_revision: view.window.from_revision.clone(),
            to_revision: view.window.to_revision.clone(),
            completeness: view.completeness.as_str().into(),
            conflict_count: view.conflict_explanations.len(),
            gap_count: view.gaps.len(),
            chain_ref_count: view.chain_summary.ordered_refs.len(),
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
pub struct TemporalIntelligenceSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<TemporalAnalysisView>,
    pub history: Vec<TemporalIntelligenceHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl TemporalIntelligenceSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<TemporalAnalysisView>,
        history: Vec<TemporalIntelligenceHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> TemporalIntelligenceSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        TemporalIntelligenceSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            analysis_id: self.current.as_ref().map(|c| c.analysis_id.clone()),
            from_revision: self
                .current
                .as_ref()
                .and_then(|c| c.window.from_revision.clone()),
            to_revision: self
                .current
                .as_ref()
                .and_then(|c| c.window.to_revision.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            conflict_count: self
                .current
                .as_ref()
                .map(|c| c.conflict_explanations.len())
                .unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalIntelligenceSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub analysis_id: Option<String>,
    pub from_revision: Option<String>,
    pub to_revision: Option<String>,
    pub completeness: Option<String>,
    pub conflict_count: usize,
    pub gap_count: usize,
    pub history: Vec<TemporalIntelligenceHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Explanation surface for ExplainTemporalChange — evidence-backed sequence only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalChangeExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub from_revision: Option<String>,
    pub to_revision: Option<String>,
    pub completeness: Option<String>,
    pub observed_sequences: Vec<String>,
    pub conflict_descriptions: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub gaps: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl TemporalChangeExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_view(view: &TemporalAnalysisView) -> Self {
        Self {
            explanation_id: format!("temporal_explanation:{}", view.analysis_id),
            workspace_id: view.workspace_id.clone(),
            from_revision: view.window.from_revision.clone(),
            to_revision: view.window.to_revision.clone(),
            completeness: Some(view.completeness.as_str().into()),
            observed_sequences: view.chain_summary.observed_transitions.clone(),
            conflict_descriptions: view
                .conflict_explanations
                .iter()
                .map(|c| c.description.clone())
                .collect(),
            evidence_refs: view
                .provenance_links
                .iter()
                .map(|p| p.external_ref.clone())
                .collect(),
            gaps: view.gaps.iter().map(|g| g.description.clone()).collect(),
            uncertainty: view.evidence_quality.uncertainty.clone(),
            narrative: view.narrative.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

fn filter_envelopes(
    envelopes: &[WorkspaceStateEnvelope],
    window: &TemporalAnalysisWindow,
) -> Vec<WorkspaceStateEnvelope> {
    envelopes
        .iter()
        .filter(|e| {
            if let Some(from) = &window.from_observed_at {
                if e.generated_at.as_str() < from.as_str() {
                    return false;
                }
            }
            if let Some(to) = &window.to_observed_at {
                if e.generated_at.as_str() > to.as_str() {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}

fn build_chain_summary(
    recon: &WorkspaceHistoricalView,
    envelopes: &[WorkspaceStateEnvelope],
    window: &TemporalAnalysisWindow,
    gaps: &mut Vec<EvidenceGap>,
) -> RevisionChainSummary {
    let mut ordered_refs = Vec::new();
    let mut observed_transitions = Vec::new();
    let mut unchanged_spans = Vec::new();
    let mut gap_spans = Vec::new();

    if envelopes.is_empty() {
        // Fall back to reconstruction revision bounds as refs only.
        if let Some(from) = &recon.from_revision {
            ordered_refs.push(format!("revision:{from}"));
        }
        if let Some(to) = &recon.to_revision {
            let to_ref = format!("revision:{to}");
            if ordered_refs.last() != Some(&to_ref) {
                ordered_refs.push(to_ref);
            }
        }
        for snap in &recon.temporal_snapshots {
            ordered_refs.push(format!("snapshot:{}", snap.snapshot_ref));
        }
        ordered_refs.sort();
        ordered_refs.dedup();
    } else {
        for env in envelopes {
            ordered_refs.push(format!("envelope:{}@{}", env.state_id, env.revision));
        }
    }

    // Apply revision window soft filter on envelope-derived chain when both bounds set.
    if let (Some(from), Some(to)) = (&window.from_revision, &window.to_revision) {
        if !envelopes.iter().any(|e| e.revision == *from)
            || !envelopes.iter().any(|e| e.revision == *to)
        {
            gap_spans.push(format!(
                "window:{from}→{to}: one or both bound revisions absent from durable envelopes"
            ));
            gaps.push(EvidenceGap::record(
                "temporal_window",
                format!(
                    "Requested window {from}→{to} is not fully covered by durable envelope evidence"
                ),
                "medium",
            ));
        }
    }

    for change in &recon.timeline {
        // Sequence language only — "was followed by", never "caused".
        observed_transitions.push(format!(
            "Revision {} was followed by revision {}: {}",
            change.from_ref, change.to_ref, change.description
        ));
    }

    if recon.timeline.is_empty() && envelopes.len() >= 2 {
        unchanged_spans.push(
            "Multiple envelope revisions present with no evidenced field-level transitions in reconstruction timeline"
                .into(),
        );
    }
    if recon.timeline.is_empty() && envelopes.len() < 2 {
        gap_spans.push(
            "Insufficient revision pairs to claim change or continuity — Missing evidence ≠ No change"
                .into(),
        );
    }

    for gap in recon.gaps.iter() {
        gap_spans.push(format!("{}:{}", gap.channel, gap.description));
    }

    let completeness = UnderstandingCompleteness::from_reconstruction(recon.completeness);
    let digest = stable_digest(&format!(
        "{}|{}|{}",
        ordered_refs.join(","),
        observed_transitions.join(";"),
        gap_spans.join(";")
    ));

    RevisionChainSummary {
        chain_id: format!("{}{}", RevisionChainSummary::ID_PREFIX, digest),
        ordered_refs,
        observed_transitions,
        unchanged_spans,
        gap_spans,
        completeness,
        authority_effect: RevisionChainSummary::AUTHORITY_EFFECT_NONE.into(),
        actionable: false,
    }
}

fn build_conflict_explanations(
    recon: &WorkspaceHistoricalView,
    envelopes: &[WorkspaceStateEnvelope],
) -> Vec<TemporalConflictExplanation> {
    let mut out = Vec::new();

    for gap in &recon.gaps {
        if gap.channel == "envelope_consistency" || gap.severity == "high" {
            out.push(TemporalConflictExplanation::observed(
                vec![gap.gap_id.clone()],
                format!(
                    "Evidence channels disagree or are incomplete: {}. Conflict is preserved — not resolved.",
                    gap.description
                ),
                vec![gap.gap_id.clone(), recon.reconstruction_id.clone()],
                vec![
                    "Disagreement is recorded; no side is preferred as truth".into(),
                ],
            ));
        }
    }

    for env in envelopes {
        for conflict in &env.contradictions {
            out.push(TemporalConflictExplanation::observed(
                vec![
                    conflict.source_a.clone(),
                    conflict.source_b.clone(),
                ],
                format!(
                    "Source {} reported conflict with {}: {} (severity={}). Explain disagreement only — do not declare a winner.",
                    conflict.source_a,
                    conflict.source_b,
                    conflict.description,
                    conflict.severity
                ),
                vec![env.state_id.clone(), conflict.conflict_id.clone()],
                vec![
                    "Which source (if either) is correct remains unknown without further evidence"
                        .into(),
                ],
            ));
        }
    }

    out.sort_by(|a, b| a.conflict_id.cmp(&b.conflict_id));
    out.dedup_by(|a, b| a.conflict_id == b.conflict_id);
    out
}

fn build_evidence_quality(
    recon: &WorkspaceHistoricalView,
    envelopes: &[WorkspaceStateEnvelope],
    gaps: &[EvidenceGap],
    conflicts: &[TemporalConflictExplanation],
) -> EvidenceQualityAssessment {
    let provenance_coverage = if envelopes.is_empty() {
        if recon.provenance_links.is_empty() {
            0
        } else {
            40
        }
    } else {
        ((envelopes.len().min(10) as u8) * 10).min(100)
    };
    let contradiction_density = ((conflicts.len().min(10) as u8) * 10).min(100);
    let source_availability = if envelopes.is_empty() {
        "unavailable"
    } else if gaps.iter().any(|g| g.severity == "high") {
        "partial"
    } else {
        "available"
    };
    let mut uncertainty = gaps
        .iter()
        .map(|g| format!("{}: {}", g.channel, g.description))
        .collect::<Vec<_>>();
    uncertainty.push(
        "Evidence quality is diagnostic only — not a correctness or authority score".into(),
    );
    let limitations = vec![
        "Evidence Quality ≠ correctness".into(),
        "Missing evidence ≠ no change".into(),
        "Unknown cause ≠ generated explanation".into(),
        "Observed sequence ≠ cause".into(),
    ];
    let digest = stable_digest(&format!(
        "{}|{}|{}|{}",
        recon.completeness.as_str(),
        provenance_coverage,
        contradiction_density,
        source_availability
    ));
    EvidenceQualityAssessment {
        assessment_id: format!("{}{}", EvidenceQualityAssessment::ID_PREFIX, digest),
        completeness: recon.completeness.as_str().into(),
        provenance_coverage,
        source_availability: source_availability.into(),
        contradiction_density,
        uncertainty,
        limitations,
        authority_effect: EvidenceQualityAssessment::AUTHORITY_EFFECT_NONE.into(),
        actionable: false,
    }
}

fn derive_completeness(
    recon: &WorkspaceHistoricalView,
    gaps: &[EvidenceGap],
    conflicts: &[TemporalConflictExplanation],
) -> UnderstandingCompleteness {
    if !conflicts.is_empty()
        || recon.completeness == ReconstructionCompleteness::Contradictory
    {
        UnderstandingCompleteness::Contradictory
    } else if recon.completeness == ReconstructionCompleteness::Unavailable {
        UnderstandingCompleteness::Unavailable
    } else if recon.completeness == ReconstructionCompleteness::Unknown
        || gaps.iter().any(|g| g.severity == "high")
    {
        UnderstandingCompleteness::Unknown
    } else if !gaps.is_empty()
        || recon.completeness == ReconstructionCompleteness::Partial
    {
        UnderstandingCompleteness::Partial
    } else {
        UnderstandingCompleteness::Complete
    }
}

fn build_sequence_narrative(
    recon: &WorkspaceHistoricalView,
    chain: &RevisionChainSummary,
    conflicts: &[TemporalConflictExplanation],
) -> String {
    let mut parts = Vec::new();
    parts.push(format!(
        "Temporal analysis over reconstruction {} (completeness={}).",
        recon.reconstruction_id,
        chain.completeness.as_str()
    ));
    if chain.ordered_refs.is_empty() {
        parts.push(
            "No ordered revision refs were available in the requested window.".into(),
        );
    } else {
        parts.push(format!(
            "Ordered evidence refs ({}): {}.",
            chain.ordered_refs.len(),
            chain.ordered_refs.join(" → ")
        ));
    }
    if chain.observed_transitions.is_empty() {
        parts.push(
            "No evidenced transitions were present — Missing evidence ≠ No change.".into(),
        );
    } else {
        parts.push(format!(
            "Observed sequences (not causes): {}.",
            chain.observed_transitions.join("; ")
        ));
    }
    if !conflicts.is_empty() {
        parts.push(format!(
            "{} conflict explanation(s) preserved without resolution.",
            conflicts.len()
        ));
    }
    parts.push(
        "Language constraint: use observed sequence (A was followed by B); do not claim A produced B without causal evidence."
            .into(),
    );
    parts.join(" ")
}

fn narrative_has_forbidden_causality(narrative: &str) -> bool {
    let lower = narrative.to_lowercase();
    // Reject unsupported causal phrasing. Allow negation/meta discussion only when
    // clearly framed; simple "caused" assertions are forbidden.
    let forbidden = [
        "because the user",
        "because the system",
        "therefore the system decided",
        "a caused b",
        "revision a caused",
        "this caused",
        "which caused",
    ];
    forbidden.iter().any(|p| lower.contains(p))
        || (lower.contains(" caused ")
            && !lower.contains("without causal")
            && !lower.contains("do not claim")
            && !lower.contains("not claim"))
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
    use crate::workspace_historical_reconstruction::WorkspaceHistoricalView;
    use crate::workspace_state_envelope::{
        AvailabilityStatus, CompletenessStatus, FreshnessStatus, WorkspaceStateConflict,
        WorkspaceStateSource,
    };

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

    fn envelope(
        id: &str,
        rev: &str,
        at: &str,
        sources: Vec<WorkspaceStateSource>,
        contradictions: Vec<WorkspaceStateConflict>,
    ) -> WorkspaceStateEnvelope {
        let mut e =
            WorkspaceStateEnvelope::compose_deterministic("ws", at, sources, contradictions, vec![])
                .unwrap();
        e.state_id = id.into();
        e.revision = rev.into();
        e
    }

    #[test]
    fn deterministic_analysis_identical_inputs() {
        let a = envelope("env:1", "rev:a", "t0", vec![src("planning", "p1")], vec![]);
        let b = envelope("env:2", "rev:b", "t1", vec![src("planning", "p2")], vec![]);
        let recon =
            WorkspaceHistoricalView::reconstruct_deterministic("ws", "t2", &[a.clone(), b.clone()])
                .unwrap();
        let window = TemporalAnalysisWindow::unbounded();
        let left = TemporalAnalysisView::analyse_deterministic(
            "ws",
            "t3",
            window.clone(),
            Some(&recon),
            &[a.clone(), b.clone()],
        )
        .unwrap();
        let right = TemporalAnalysisView::analyse_deterministic(
            "ws",
            "t3",
            window,
            Some(&recon),
            &[a, b],
        )
        .unwrap();
        assert_eq!(left.analysis_id, right.analysis_id);
        assert_eq!(left.chain_summary, right.chain_summary);
        assert!(left.is_non_executing());
        assert!(!narrative_has_forbidden_causality(&left.narrative));
    }

    #[test]
    fn missing_reconstruction_is_unavailable() {
        let view = TemporalAnalysisView::analyse(
            "ws",
            "t0",
            TemporalAnalysisWindow::unbounded(),
            None,
            &[],
        )
        .unwrap();
        assert_eq!(view.completeness, UnderstandingCompleteness::Unavailable);
        assert!(view.chain_summary.observed_transitions.is_empty());
        assert!(!view.gaps.is_empty());
    }

    #[test]
    fn sequence_language_not_causal() {
        let a = envelope("env:1", "rev:a", "t0", vec![src("planning", "p1")], vec![]);
        let b = envelope("env:2", "rev:b", "t1", vec![src("planning", "p2")], vec![]);
        let recon =
            WorkspaceHistoricalView::reconstruct_deterministic("ws", "t2", &[a.clone(), b.clone()])
                .unwrap();
        let view = TemporalAnalysisView::analyse(
            "ws",
            "t3",
            TemporalAnalysisWindow::unbounded(),
            Some(&recon),
            &[a, b],
        )
        .unwrap();
        assert!(view.narrative.contains("was followed by") || view.narrative.contains("Observed sequences") || view.chain_summary.observed_transitions.iter().any(|t| t.contains("was followed by")));
        assert!(!view.narrative.to_lowercase().contains(" caused "));
    }

    #[test]
    fn conflicts_explained_not_resolved() {
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
            vec![conflict],
        );
        let recon =
            WorkspaceHistoricalView::reconstruct_deterministic("ws", "t1", &[a.clone()]).unwrap();
        let view = TemporalAnalysisView::analyse(
            "ws",
            "t2",
            TemporalAnalysisWindow::unbounded(),
            Some(&recon),
            &[a],
        )
        .unwrap();
        assert!(!view.conflict_explanations.is_empty());
        assert!(view
            .conflict_explanations
            .iter()
            .all(|c| c.description.contains("not") || c.description.contains("Conflict") || c.uncertainty.iter().any(|u| u.contains("unknown"))));
        assert!(!view
            .conflict_explanations
            .iter()
            .any(|c| c.description.to_lowercase().contains("was wrong")));
        assert_eq!(
            view.completeness,
            UnderstandingCompleteness::Contradictory
        );
    }

    #[test]
    fn evidence_quality_is_diagnostic_not_authority() {
        let recon = WorkspaceHistoricalView::reconstruct("ws", "t0", &[]).unwrap();
        let view = TemporalAnalysisView::analyse(
            "ws",
            "t1",
            TemporalAnalysisWindow::unbounded(),
            Some(&recon),
            &[],
        )
        .unwrap();
        assert!(view
            .evidence_quality
            .limitations
            .iter()
            .any(|l| l.contains("≠ correctness") || l.contains("not a correctness")));
        assert_eq!(view.evidence_quality.authority_effect, "none");
        assert!(!view.evidence_quality.actionable);
    }

    #[test]
    fn history_separation_non_actionable() {
        let recon = WorkspaceHistoricalView::reconstruct("ws", "t0", &[]).unwrap();
        let mut view = TemporalAnalysisView::analyse(
            "ws",
            "t1",
            TemporalAnalysisWindow::unbounded(),
            Some(&recon),
            &[],
        )
        .unwrap();
        view.mark_superseded("t2");
        let hist = TemporalIntelligenceHistoryEntry::from_view(&view).unwrap();
        assert!(hist.is_non_actionable());
        let snap = TemporalIntelligenceSnapshot::assemble("ws", None, vec![hist], 1, "t3");
        assert!(snap.is_non_commandable());
    }
}
