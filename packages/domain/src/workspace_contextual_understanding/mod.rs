//! Contextual Workspace Understanding — Programme III Batch 6.
//!
//! Transform durable evidence into situational context.
//! Organises situational meaning — does not decide, predict, simulate, or change reality.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::policy_governance::PolicyGovernanceSnapshot;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ContextualUnderstandingError {
    #[error("invalid contextual understanding status: {0}")]
    InvalidStatus(String),

    #[error("invalid understanding completeness: {0}")]
    InvalidCompleteness(String),

    #[error("contextual understanding artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("contextual understanding artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("contextual understanding snapshot not found")]
    SnapshotNotFound,

    #[error("contextual understanding incomplete — required evidence unavailable")]
    UnderstandingIncomplete,

    #[error("contextual understanding failed — corrupt or unusable evidence")]
    UnderstandingFailed,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextualUnderstandingStatus {
    Current,
    Superseded,
    Archived,
}

impl ContextualUnderstandingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ContextualUnderstandingError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(ContextualUnderstandingError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextualCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl ContextualCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ContextualUnderstandingError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(ContextualUnderstandingError::InvalidCompleteness(other.into())),
        }
    }
}

/// Frame for situational understanding — constrains reading; does not invent coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextFrame {
    pub focus: Option<String>,
    pub include_state: bool,
    pub include_policy: bool,
    pub include_reconstruction: bool,
    pub include_temporal: bool,
    pub include_explanation: bool,
    pub max_themes: usize,
}

impl ContextFrame {
    pub const DEFAULT_MAX_THEMES: usize = 16;

    pub fn all_surfaces() -> Self {
        Self {
            focus: None,
            include_state: true,
            include_policy: true,
            include_reconstruction: true,
            include_temporal: true,
            include_explanation: true,
            max_themes: Self::DEFAULT_MAX_THEMES,
        }
    }
}

/// Provenance anchor — statement → evidence → revision → origin domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualEvidenceReference {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ContextualEvidenceReference {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn link(
        origin_domain: impl Into<String>,
        external_ref: impl Into<String>,
        source_revision: Option<String>,
    ) -> Self {
        Self {
            external_ref: external_ref.into(),
            origin_domain: origin_domain.into(),
            source_revision,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualGap {
    pub gap_id: String,
    pub surface: String,
    pub description: String,
    pub severity: String,
    pub evidence_refs: Vec<ContextualEvidenceReference>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ContextualGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "contextual_gap:";

    pub fn record(
        surface: impl Into<String>,
        description: impl Into<String>,
        severity: impl Into<String>,
        evidence_refs: Vec<ContextualEvidenceReference>,
    ) -> Self {
        let surface = surface.into();
        let description = description.into();
        let digest = stable_digest(&format!("{surface}|{description}"));
        Self {
            gap_id: format!("{}{}", Self::ID_PREFIX, digest),
            surface,
            description,
            severity: severity.into(),
            evidence_refs,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_refs.iter().all(|r| r.is_non_actionable())
    }
}

/// Evidence-backed interpretation — descriptive only, never a recommendation/command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualInsight {
    pub insight_id: String,
    pub kind: String,
    pub body: String,
    pub evidence_refs: Vec<ContextualEvidenceReference>,
    pub explanation_lineage: Vec<String>,
    pub confidence: u8,
    pub uncertainty: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ContextualInsight {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "contextual_insight:";

    pub fn observe(
        kind: impl Into<String>,
        body: impl Into<String>,
        evidence_refs: Vec<ContextualEvidenceReference>,
        explanation_lineage: Vec<String>,
        confidence: u8,
        uncertainty: Vec<String>,
        limitations: Vec<String>,
    ) -> Self {
        let kind = kind.into();
        let body = body.into();
        let digest = stable_digest(&format!("{kind}|{body}"));
        Self {
            insight_id: format!("{}{}", Self::ID_PREFIX, digest),
            kind,
            body,
            evidence_refs,
            explanation_lineage,
            confidence: confidence.min(100),
            uncertainty,
            limitations,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_refs.iter().all(|r| r.is_non_actionable())
    }
}

/// Observed workspace meaning — descriptive, never "do X / execute Y / approve Z".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SituationalTheme {
    pub theme_id: String,
    pub kind: String,
    pub title: String,
    pub body: String,
    pub insights: Vec<ContextualInsight>,
    pub evidence_refs: Vec<ContextualEvidenceReference>,
    pub completeness: ContextualCompleteness,
    pub authority_effect: String,
    pub actionable: bool,
}

impl SituationalTheme {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "situational_theme:";

    pub fn compose(
        kind: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        insights: Vec<ContextualInsight>,
        evidence_refs: Vec<ContextualEvidenceReference>,
        completeness: ContextualCompleteness,
    ) -> Self {
        let kind = kind.into();
        let title = title.into();
        let body = body.into();
        let digest = stable_digest(&format!("{kind}|{title}|{body}"));
        Self {
            theme_id: format!("{}{}", Self::ID_PREFIX, digest),
            kind,
            title,
            body,
            insights,
            evidence_refs,
            completeness,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.insights.iter().all(|i| i.is_non_actionable())
            && self.evidence_refs.iter().all(|r| r.is_non_actionable())
    }
}

/// Diagnostic understanding confidence — ≠ truth confidence / decision authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualUnderstandingConfidence {
    pub assessment_id: String,
    pub coverage: u8,
    pub available_surfaces: usize,
    pub requested_surfaces: usize,
    pub gap_count: usize,
    pub conflict_count: usize,
    pub uncertainty: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ContextualUnderstandingConfidence {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "contextual_confidence:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Situational understanding artefact — consumer projection, not controller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualWorkspaceSnapshot {
    pub understanding_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: ContextualUnderstandingStatus,
    pub superseded_at: Option<String>,
    pub frame: ContextFrame,
    pub source_revisions: Vec<String>,
    pub themes: Vec<SituationalTheme>,
    pub gaps: Vec<ContextualGap>,
    pub confidence: ContextualUnderstandingConfidence,
    pub completeness: ContextualCompleteness,
    pub provenance_links: Vec<ContextualEvidenceReference>,
    pub situation_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl ContextualWorkspaceSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "contextual_understanding:";

    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: ContextFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
    ) -> Result<Self, ContextualUnderstandingError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let max_themes = frame.max_themes.max(1);

        let mut themes = Vec::new();
        let mut gaps = Vec::new();
        let mut provenance_links = Vec::new();
        let mut source_revisions = Vec::new();
        let mut conflict_count = 0usize;
        let mut requested = 0usize;
        let mut available = 0usize;

        if frame.include_state {
            requested += 1;
            match state.and_then(|s| s.current.as_ref()) {
                Some(envelope) => {
                    available += 1;
                    source_revisions.push(format!("state:{}", envelope.revision));
                    let refs = vec![ContextualEvidenceReference::link(
                        "workspace_state_envelope",
                        envelope.state_id.clone(),
                        Some(envelope.revision.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    let completeness = if envelope.consistency.as_str() == "contradictory" {
                        conflict_count += envelope.contradictions.len().max(1);
                        ContextualCompleteness::Contradictory
                    } else if !envelope.unknowns.is_empty()
                        || envelope.completeness.as_str() == "partial"
                        || envelope.freshness.as_str() == "stale"
                        || envelope.freshness.as_str() == "unknown"
                        || envelope.freshness.as_str() == "unavailable"
                    {
                        ContextualCompleteness::Partial
                    } else {
                        ContextualCompleteness::Complete
                    };
                    let mut insights = vec![ContextualInsight::observe(
                        "current_state_posture",
                        format!(
                            "Current envelope revision {} reports freshness={}, completeness={}, consistency={}, sources={}. Descriptive posture only — not a health verdict or action.",
                            envelope.revision,
                            envelope.freshness.as_str(),
                            envelope.completeness.as_str(),
                            envelope.consistency.as_str(),
                            envelope.sources.len()
                        ),
                        refs.clone(),
                        vec![format!(
                            "statement→envelope:{}→revision:{}→domain:workspace_state_envelope",
                            envelope.state_id, envelope.revision
                        )],
                        70,
                        envelope.unknowns.clone(),
                        vec![
                            "State posture is observational — not a recommendation".into(),
                            "Understanding confidence ≠ truth confidence".into(),
                        ],
                    )];
                    if envelope.sources.len() >= 3 {
                        insights.push(ContextualInsight::observe(
                            "concentrated_activity",
                            format!(
                                "Envelope lists {} source channels — concentrated activity signal observed in composed evidence.",
                                envelope.sources.len()
                            ),
                            refs.clone(),
                            vec![format!(
                                "statement→envelope:{}→revision:{}→domain:workspace_state_envelope",
                                envelope.state_id, envelope.revision
                            )],
                            55,
                            vec!["Activity concentration is descriptive, not a priority order".into()],
                            vec!["Does not imply execute / focus / approve".into()],
                        ));
                    }
                    if completeness == ContextualCompleteness::Contradictory {
                        insights.push(ContextualInsight::observe(
                            "conflicting_signals",
                            "Envelope consistency is contradictory — conflicting signals preserved, not resolved.",
                            refs.clone(),
                            vec![format!(
                                "statement→envelope:{}→revision:{}→domain:workspace_state_envelope",
                                envelope.state_id, envelope.revision
                            )],
                            40,
                            vec!["Which source (if either) is correct remains unknown".into()],
                            vec!["Conflict framing is not conflict resolution".into()],
                        ));
                    }
                    themes.push(SituationalTheme::compose(
                        "current_state",
                        "Current state posture",
                        format!(
                            "Situational theme from unified state envelope {}. Descriptive only.",
                            envelope.revision
                        ),
                        insights,
                        refs,
                        completeness,
                    ));
                }
                None => {
                    gaps.push(ContextualGap::record(
                        "state",
                        "No durable workspace state envelope available — Missing evidence ≠ empty-but-confident context",
                        "high",
                        vec![],
                    ));
                }
            }
        }

        if frame.include_policy {
            requested += 1;
            match policy.and_then(|p| p.current.as_ref()) {
                Some(view) => {
                    available += 1;
                    let rev = view
                        .context_revision
                        .clone()
                        .unwrap_or_else(|| view.meta.evaluation_set_id.clone());
                    source_revisions.push(format!("policy:{rev}"));
                    let refs = vec![ContextualEvidenceReference::link(
                        "policy_governance",
                        view.meta.evaluation_set_id.clone(),
                        Some(rev.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    let agg = view.meta.aggregate_result.as_str();
                    let completeness = if agg == "unknown" {
                        ContextualCompleteness::Unknown
                    } else if agg == "violation" || agg == "requires_review" {
                        ContextualCompleteness::Partial
                    } else {
                        ContextualCompleteness::Complete
                    };
                    if agg == "unknown" {
                        gaps.push(ContextualGap::record(
                            "policy",
                            "Policy aggregate is Unknown — never assume Compliant or invent approval",
                            "high",
                            refs.clone(),
                        ));
                    }
                    let insight = ContextualInsight::observe(
                        "governance_posture",
                        format!(
                            "Governance evaluation {} reports aggregate_result={}. Policy explains authority; Gateway remains final.",
                            view.meta.evaluation_set_id, agg
                        ),
                        refs.clone(),
                        vec![format!(
                            "statement→policy:{}→revision:{}→domain:policy_governance",
                            view.meta.evaluation_set_id, rev
                        )],
                        if agg == "unknown" { 30 } else { 65 },
                        if agg == "unknown" {
                            vec!["Unknown policy context must not become approval".into()]
                        } else {
                            vec![]
                        },
                        vec![
                            "Governance posture is not a permission decision".into(),
                            "Does not approve, deny, or grant capabilities".into(),
                        ],
                    );
                    themes.push(SituationalTheme::compose(
                        "governance_posture",
                        "Governance posture",
                        "Situational theme from policy evaluation evidence. Descriptive only.",
                        vec![insight],
                        refs,
                        completeness,
                    ));
                }
                None => {
                    gaps.push(ContextualGap::record(
                        "policy",
                        "No durable policy evaluation available — Unavailable, not assumed current",
                        "high",
                        vec![],
                    ));
                }
            }
        }

        if frame.include_reconstruction {
            requested += 1;
            match reconstruction.and_then(|r| r.current.as_ref()) {
                Some(view) => {
                    available += 1;
                    let rev = view
                        .to_revision
                        .clone()
                        .unwrap_or_else(|| view.reconstruction_id.clone());
                    source_revisions.push(format!("reconstruction:{rev}"));
                    let refs = vec![ContextualEvidenceReference::link(
                        "historical_reconstruction",
                        view.reconstruction_id.clone(),
                        Some(rev.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    let completeness = match view.completeness.as_str() {
                        "complete" => ContextualCompleteness::Complete,
                        "partial" => ContextualCompleteness::Partial,
                        "unknown" => ContextualCompleteness::Unknown,
                        "contradictory" => {
                            conflict_count += 1;
                            ContextualCompleteness::Contradictory
                        }
                        _ => ContextualCompleteness::Unavailable,
                    };
                    for g in &view.gaps {
                        gaps.push(ContextualGap::record(
                            "reconstruction",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![ContextualEvidenceReference::link(
                                "evidence_gap",
                                g.gap_id.clone(),
                                None,
                            )],
                        ));
                    }
                    let mut insights = vec![ContextualInsight::observe(
                        "historical_continuity",
                        format!(
                            "Reconstruction {} from {:?} to {:?} — completeness={}, changes={}. Continuity framing only.",
                            view.reconstruction_id,
                            view.from_revision,
                            view.to_revision,
                            view.completeness.as_str(),
                            view.timeline.len()
                        ),
                        refs.clone(),
                        vec![format!(
                            "statement→reconstruction:{}→revision:{}→domain:historical_reconstruction",
                            view.reconstruction_id, rev
                        )],
                        60,
                        vec![],
                        vec![
                            "Historical continuity is not a plan".into(),
                            "Observed sequence ≠ cause".into(),
                        ],
                    )];
                    if !view.gaps.is_empty() {
                        insights.push(ContextualInsight::observe(
                            "unresolved_dependency_cluster",
                            format!(
                                "Reconstruction records {} evidence gap(s) — unresolved dependency / missing-evidence cluster observed.",
                                view.gaps.len()
                            ),
                            refs.clone(),
                            vec![format!(
                                "statement→reconstruction:{}→revision:{}→domain:historical_reconstruction",
                                view.reconstruction_id, rev
                            )],
                            45,
                            view.gaps.iter().map(|g| g.description.clone()).collect(),
                            vec!["Gaps preserve missing information — never invent missing truth".into()],
                        ));
                    }
                    themes.push(SituationalTheme::compose(
                        "historical_continuity",
                        "Historical continuity",
                        "Situational theme from historical reconstruction evidence. Descriptive only.",
                        insights,
                        refs,
                        completeness,
                    ));
                }
                None => {
                    gaps.push(ContextualGap::record(
                        "reconstruction",
                        "No durable historical reconstruction available — Missing evidence ≠ No change",
                        "medium",
                        vec![],
                    ));
                }
            }
        }

        if frame.include_temporal {
            requested += 1;
            match temporal.and_then(|t| t.current.as_ref()) {
                Some(view) => {
                    available += 1;
                    let rev = view
                        .window
                        .to_revision
                        .clone()
                        .or_else(|| view.window.from_revision.clone())
                        .unwrap_or_else(|| view.analysis_id.clone());
                    source_revisions.push(format!("temporal:{rev}"));
                    let refs = vec![ContextualEvidenceReference::link(
                        "temporal_analysis",
                        view.analysis_id.clone(),
                        Some(rev.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    let completeness = match view.completeness.as_str() {
                        "complete" => ContextualCompleteness::Complete,
                        "partial" => ContextualCompleteness::Partial,
                        "unknown" => ContextualCompleteness::Unknown,
                        "contradictory" => {
                            conflict_count += view.conflict_explanations.len().max(1);
                            ContextualCompleteness::Contradictory
                        }
                        _ => ContextualCompleteness::Unavailable,
                    };
                    let insight = ContextualInsight::observe(
                        "temporal_pressure",
                        format!(
                            "Temporal analysis {} — completeness={}, chain_refs={}, conflicts={}. Temporal organisation only.",
                            view.analysis_id,
                            view.completeness.as_str(),
                            view.chain_summary.ordered_refs.len(),
                            view.conflict_explanations.len()
                        ),
                        refs.clone(),
                        vec![format!(
                            "statement→temporal:{}→revision:{}→domain:temporal_analysis",
                            view.analysis_id, rev
                        )],
                        55,
                        view.conflict_explanations
                            .iter()
                            .map(|c| c.description.clone())
                            .collect(),
                        vec![
                            "Temporal pressure is descriptive — not a forecast".into(),
                            "Does not predict or simulate futures".into(),
                        ],
                    );
                    themes.push(SituationalTheme::compose(
                        "temporal_pressure",
                        "Temporal pressure signals",
                        "Situational theme from temporal intelligence evidence. Descriptive only.",
                        vec![insight],
                        refs,
                        completeness,
                    ));
                }
                None => {
                    gaps.push(ContextualGap::record(
                        "temporal",
                        "No durable temporal analysis available — Unavailable, not assumed current",
                        "medium",
                        vec![],
                    ));
                }
            }
        }

        if frame.include_explanation {
            requested += 1;
            match explanation.and_then(|e| e.current.as_ref()) {
                Some(package) => {
                    available += 1;
                    source_revisions.push(format!("explanation:{}", package.explanation_id));
                    let refs = vec![ContextualEvidenceReference::link(
                        "workspace_explanation",
                        package.explanation_id.clone(),
                        Some(package.explanation_id.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    let completeness = match package.completeness.as_str() {
                        "complete" => ContextualCompleteness::Complete,
                        "partial" => ContextualCompleteness::Partial,
                        "unknown" => ContextualCompleteness::Unknown,
                        "contradictory" => {
                            conflict_count += package.conflicts.len().max(1);
                            ContextualCompleteness::Contradictory
                        }
                        _ => ContextualCompleteness::Unavailable,
                    };
                    for g in &package.gaps {
                        gaps.push(ContextualGap::record(
                            "explanation",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![ContextualEvidenceReference::link(
                                "explanation_gap",
                                g.gap_id.clone(),
                                None,
                            )],
                        ));
                    }
                    let insight = ContextualInsight::observe(
                        "explanation_synthesis",
                        format!(
                            "Explanation package {} — completeness={}, sections={}, gaps={}, conflicts={}. Synthesis input only.",
                            package.explanation_id,
                            package.completeness.as_str(),
                            package.sections.len(),
                            package.gaps.len(),
                            package.conflicts.len()
                        ),
                        refs.clone(),
                        vec![format!(
                            "statement→explanation:{}→domain:workspace_explanation",
                            package.explanation_id
                        )],
                        package.confidence.coverage,
                        package
                            .conflicts
                            .iter()
                            .map(|c| c.description.clone())
                            .collect(),
                        vec![
                            "Explanation synthesis is not a decision package".into(),
                            "Does not convert insights into recommendations".into(),
                        ],
                    );
                    themes.push(SituationalTheme::compose(
                        "explanation_synthesis",
                        "Explanation synthesis",
                        "Situational theme from explanation-layer evidence. Descriptive only.",
                        vec![insight],
                        refs,
                        completeness,
                    ));
                }
                None => {
                    gaps.push(ContextualGap::record(
                        "explanation",
                        "No durable explanation package available — Missing evidence ≠ invented situation",
                        "medium",
                        vec![],
                    ));
                }
            }
        }

        if let Some(focus) = &frame.focus {
            themes.retain(|t| t.kind.contains(focus) || t.title.to_lowercase().contains(focus));
        }

        themes.truncate(max_themes);
        themes.sort_by(|a, b| a.theme_id.cmp(&b.theme_id));
        gaps.sort_by(|a, b| a.gap_id.cmp(&b.gap_id));
        source_revisions.sort();
        source_revisions.dedup();

        let completeness = derive_completeness(requested, available, &gaps, conflict_count, &themes);
        let confidence =
            build_confidence(requested, available, &gaps, conflict_count, &source_revisions);
        let limitations = vec![
            "Contextual understanding organises situational meaning — it does not change reality"
                .into(),
            "Understanding confidence ≠ truth confidence".into(),
            "Does not decide, plan, predict, simulate, or correct".into(),
            "Conflicts are preserved, not resolved".into(),
            "Prior understanding cache ≠ current truth without revision binding".into(),
        ];
        let situation_summary = build_situation_summary(&themes, &gaps, completeness);
        let narrative = build_narrative(&themes, &gaps, &situation_summary, completeness);

        let snapshot = Self {
            understanding_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id,
            generated_at,
            status: ContextualUnderstandingStatus::Current,
            superseded_at: None,
            frame,
            source_revisions,
            themes,
            gaps,
            confidence,
            completeness,
            provenance_links,
            situation_summary,
            narrative,
            limitations,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        snapshot.validate()?;
        Ok(snapshot)
    }

    pub fn compose_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: ContextFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
    ) -> Result<Self, ContextualUnderstandingError> {
        let mut snap = Self::compose(
            workspace_id,
            generated_at,
            frame,
            state,
            policy,
            reconstruction,
            temporal,
            explanation,
        )?;
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}",
            snap.workspace_id,
            snap.source_revisions.join(","),
            snap.themes
                .iter()
                .map(|t| t.theme_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.gaps
                .iter()
                .map(|g| g.gap_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.completeness.as_str()
        ));
        snap.understanding_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(snap)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = ContextualUnderstandingStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.themes.iter().all(|t| t.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.confidence.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), ContextualUnderstandingError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(ContextualUnderstandingError::AuthorityEffectMustBeNone);
        }
        if narrative_has_forbidden_patterns(&self.narrative)
            || narrative_has_forbidden_patterns(&self.situation_summary)
            || self
                .themes
                .iter()
                .any(|t| narrative_has_forbidden_patterns(&t.body))
        {
            return Err(ContextualUnderstandingError::MustNotBeActionable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualUnderstandingHistoryEntry {
    pub understanding_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub theme_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl ContextualUnderstandingHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &ContextualWorkspaceSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            understanding_id: snap.understanding_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            theme_count: snap.themes.len(),
            gap_count: snap.gaps.len(),
            source_revision_count: snap.source_revisions.len(),
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
pub struct ContextualUnderstandingProjection {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<ContextualWorkspaceSnapshot>,
    pub history: Vec<ContextualUnderstandingHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl ContextualUnderstandingProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<ContextualWorkspaceSnapshot>,
        history: Vec<ContextualUnderstandingHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> ContextualUnderstandingSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        ContextualUnderstandingSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            understanding_id: self.current.as_ref().map(|c| c.understanding_id.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            theme_count: self.current.as_ref().map(|c| c.themes.len()).unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            source_revision_count: self
                .current
                .as_ref()
                .map(|c| c.source_revisions.len())
                .unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualUnderstandingSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub understanding_id: Option<String>,
    pub completeness: Option<String>,
    pub theme_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub history: Vec<ContextualUnderstandingHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Explanation surface for ExplainWorkspaceContext.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceContextExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub situation_summary: Option<String>,
    pub theme_summaries: Vec<String>,
    pub gaps: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceContextExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &ContextualWorkspaceSnapshot) -> Self {
        Self {
            explanation_id: format!("context_explanation:{}", snap.understanding_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            situation_summary: Some(snap.situation_summary.clone()),
            theme_summaries: snap
                .themes
                .iter()
                .map(|t| format!("{}: {}", t.kind, t.title))
                .collect(),
            gaps: snap.gaps.iter().map(|g| g.description.clone()).collect(),
            evidence_refs: snap
                .provenance_links
                .iter()
                .map(|p| p.external_ref.clone())
                .collect(),
            uncertainty: snap.confidence.uncertainty.clone(),
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

fn derive_completeness(
    requested: usize,
    available: usize,
    gaps: &[ContextualGap],
    conflict_count: usize,
    themes: &[SituationalTheme],
) -> ContextualCompleteness {
    if requested == 0 || available == 0 {
        return ContextualCompleteness::Unavailable;
    }
    if conflict_count > 0
        || themes
            .iter()
            .any(|t| t.completeness == ContextualCompleteness::Contradictory)
    {
        return ContextualCompleteness::Contradictory;
    }
    if gaps.iter().any(|g| g.severity == "high")
        || themes
            .iter()
            .any(|t| t.completeness == ContextualCompleteness::Unknown)
    {
        return ContextualCompleteness::Unknown;
    }
    if available < requested || !gaps.is_empty() {
        return ContextualCompleteness::Partial;
    }
    ContextualCompleteness::Complete
}

fn build_confidence(
    requested: usize,
    available: usize,
    gaps: &[ContextualGap],
    conflict_count: usize,
    source_revisions: &[String],
) -> ContextualUnderstandingConfidence {
    let coverage = if requested == 0 {
        0
    } else {
        ((available * 100) / requested).min(100) as u8
    };
    let digest = stable_digest(&format!(
        "{requested}|{available}|{}|{}|{}",
        gaps.len(),
        conflict_count,
        source_revisions.join(",")
    ));
    ContextualUnderstandingConfidence {
        assessment_id: format!("{}{}", ContextualUnderstandingConfidence::ID_PREFIX, digest),
        coverage,
        available_surfaces: available,
        requested_surfaces: requested,
        gap_count: gaps.len(),
        conflict_count,
        uncertainty: gaps
            .iter()
            .map(|g| format!("{}: {}", g.surface, g.description))
            .collect(),
        limitations: vec![
            "Understanding confidence is diagnostic coverage only — not truth confidence".into(),
            "High coverage ≠ safe to act / decide / approve".into(),
        ],
        authority_effect: ContextualUnderstandingConfidence::AUTHORITY_EFFECT_NONE.into(),
        actionable: false,
    }
}

fn build_situation_summary(
    themes: &[SituationalTheme],
    gaps: &[ContextualGap],
    completeness: ContextualCompleteness,
) -> String {
    format!(
        "Situational understanding completeness={}; themes={}; gaps={}. Themes are descriptive observations only.",
        completeness.as_str(),
        themes
            .iter()
            .map(|t| t.kind.clone())
            .collect::<Vec<_>>()
            .join(","),
        gaps.len()
    )
}

fn build_narrative(
    themes: &[SituationalTheme],
    gaps: &[ContextualGap],
    situation_summary: &str,
    completeness: ContextualCompleteness,
) -> String {
    let mut parts = Vec::new();
    parts.push(format!(
        "Contextual workspace understanding (completeness={}).",
        completeness.as_str()
    ));
    parts.push(situation_summary.to_string());
    if themes.is_empty() {
        parts.push("No situational themes could be composed from requested surfaces.".into());
    } else {
        for t in themes {
            parts.push(format!("{} — {}", t.title, t.body));
        }
    }
    if !gaps.is_empty() {
        parts.push(format!(
            "{} contextual gap(s) preserved — Missing evidence ≠ invented certainty.",
            gaps.len()
        ));
    }
    parts.push(
        "Language constraint: descriptive situational framing only; no do/execute/approve commands."
            .into(),
    );
    parts.join(" ")
}

fn narrative_has_forbidden_patterns(text: &str) -> bool {
    let lower = text.to_lowercase();
    let forbidden = [
        "do x",
        "execute y",
        "approve z",
        "you should execute",
        "you should approve",
        "create task",
        "mutate intent",
        "safe to act",
        "policy would have approved",
        "because the user",
        "because the system",
        "revision a caused",
        "this caused",
        "was wrong",
        "recommend that you",
        "please execute",
    ];
    forbidden.iter().any(|p| lower.contains(p))
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

    #[test]
    fn missing_upstreams_are_unavailable_with_gaps() {
        let snap = ContextualWorkspaceSnapshot::compose(
            "ws",
            "t0",
            ContextFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(snap.completeness, ContextualCompleteness::Unavailable);
        assert!(snap.themes.is_empty());
        assert_eq!(snap.gaps.len(), 5);
        assert!(snap.is_non_executing());
    }

    #[test]
    fn deterministic_composition_identical_inputs() {
        let empty_state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let left = ContextualWorkspaceSnapshot::compose_deterministic(
            "ws",
            "t1",
            ContextFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let right = ContextualWorkspaceSnapshot::compose_deterministic(
            "ws",
            "t1",
            ContextFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(left.understanding_id, right.understanding_id);
        assert_eq!(left.themes, right.themes);
        assert_eq!(left.gaps, right.gaps);
    }

    #[test]
    fn confidence_is_diagnostic_not_truth() {
        let snap = ContextualWorkspaceSnapshot::compose(
            "ws",
            "t0",
            ContextFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(snap.confidence.authority_effect, "none");
        assert!(!snap.confidence.actionable);
        assert!(snap
            .confidence
            .limitations
            .iter()
            .any(|l| l.contains("not truth")));
    }

    #[test]
    fn themes_remain_descriptive_not_commands() {
        assert!(narrative_has_forbidden_patterns("Please execute the plan"));
        assert!(narrative_has_forbidden_patterns("You should approve Z"));
        assert!(!narrative_has_forbidden_patterns(
            "Concentrated activity signal observed in composed evidence"
        ));
    }

    #[test]
    fn history_separation_non_actionable() {
        let mut snap = ContextualWorkspaceSnapshot::compose(
            "ws",
            "t0",
            ContextFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        snap.mark_superseded("t1");
        let hist = ContextualUnderstandingHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(hist.is_non_actionable());
        let proj = ContextualUnderstandingProjection::assemble("ws", None, vec![hist], 1, "t2");
        assert!(proj.is_non_commandable());
    }
}
