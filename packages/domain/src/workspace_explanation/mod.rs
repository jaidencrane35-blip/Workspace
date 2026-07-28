//! Workspace Explanation Layer — Programme III Batch 5.
//!
//! Explain evidence. Do not become the authority that changes reality.
//! Consumer of state / policy / reconstruction / temporal surfaces — never a controller.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::policy_governance::PolicyGovernanceSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceExplanationError {
    #[error("invalid explanation status: {0}")]
    InvalidStatus(String),

    #[error("invalid explanation completeness: {0}")]
    InvalidCompleteness(String),

    #[error("explanation artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("explanation artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("explanation snapshot not found")]
    SnapshotNotFound,

    #[error("explanation incomplete — required evidence unavailable")]
    ExplanationIncomplete,

    #[error("explanation failed — corrupt or unusable evidence")]
    ExplanationFailed,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplanationStatus {
    Current,
    Superseded,
    Archived,
}

impl ExplanationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceExplanationError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceExplanationError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplanationCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl ExplanationCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceExplanationError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(WorkspaceExplanationError::InvalidCompleteness(other.into())),
        }
    }
}

/// Requested composition bounds — constrains reading; does not invent coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplanationScope {
    pub include_state: bool,
    pub include_policy: bool,
    pub include_reconstruction: bool,
    pub include_temporal: bool,
    pub theme: Option<String>,
    pub max_sections: usize,
}

impl ExplanationScope {
    pub const DEFAULT_MAX_SECTIONS: usize = 16;

    pub fn all_surfaces() -> Self {
        Self {
            include_state: true,
            include_policy: true,
            include_reconstruction: true,
            include_temporal: true,
            theme: None,
            max_sections: Self::DEFAULT_MAX_SECTIONS,
        }
    }
}

/// Provenance reference to an existing durable identity — never duplicated lifecycle payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceReference {
    pub external_ref: String,
    pub kind: String,
    pub authority_effect: String,
    pub actionable: bool,
}

impl EvidenceReference {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn link(kind: impl Into<String>, external_ref: impl Into<String>) -> Self {
        Self {
            external_ref: external_ref.into(),
            kind: kind.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplanationGap {
    pub gap_id: String,
    pub surface: String,
    pub description: String,
    pub severity: String,
    pub evidence_refs: Vec<EvidenceReference>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ExplanationGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "explanation_gap:";

    pub fn record(
        surface: impl Into<String>,
        description: impl Into<String>,
        severity: impl Into<String>,
        evidence_refs: Vec<EvidenceReference>,
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

/// Conflict explanation — preserve disagreement; never resolve.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplanationConflict {
    pub conflict_id: String,
    pub surface: String,
    pub description: String,
    pub evidence_refs: Vec<EvidenceReference>,
    pub uncertainty: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ExplanationConflict {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "explanation_conflict:";

    pub fn observed(
        surface: impl Into<String>,
        description: impl Into<String>,
        evidence_refs: Vec<EvidenceReference>,
        uncertainty: Vec<String>,
    ) -> Self {
        let surface = surface.into();
        let description = description.into();
        let digest = stable_digest(&format!(
            "{surface}|{description}|{}",
            evidence_refs
                .iter()
                .map(|r| r.external_ref.clone())
                .collect::<Vec<_>>()
                .join(",")
        ));
        Self {
            conflict_id: format!("{}{}", Self::ID_PREFIX, digest),
            surface,
            description,
            evidence_refs,
            uncertainty,
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

/// Diagnostic confidence about explanation coverage — never decision authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplanationConfidence {
    pub assessment_id: String,
    pub coverage: u8,
    pub available_surfaces: usize,
    pub requested_surfaces: usize,
    pub gap_count: usize,
    pub conflict_count: usize,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ExplanationConfidence {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "explanation_confidence:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplanationSection {
    pub section_id: String,
    pub surface: String,
    pub title: String,
    pub body: String,
    pub evidence_refs: Vec<EvidenceReference>,
    pub completeness: ExplanationCompleteness,
    pub authority_effect: String,
    pub actionable: bool,
}

impl ExplanationSection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "explanation_section:";

    pub fn compose(
        surface: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        evidence_refs: Vec<EvidenceReference>,
        completeness: ExplanationCompleteness,
    ) -> Self {
        let surface = surface.into();
        let title = title.into();
        let body = body.into();
        let digest = stable_digest(&format!("{surface}|{title}|{body}"));
        Self {
            section_id: format!("{}{}", Self::ID_PREFIX, digest),
            surface,
            title,
            body,
            evidence_refs,
            completeness,
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

/// Human-readable evidence synthesis package — consumer projection, not controller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplanationPackage {
    pub explanation_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: ExplanationStatus,
    pub superseded_at: Option<String>,
    pub scope: ExplanationScope,
    pub sections: Vec<ExplanationSection>,
    pub gaps: Vec<ExplanationGap>,
    pub conflicts: Vec<ExplanationConflict>,
    pub confidence: ExplanationConfidence,
    pub completeness: ExplanationCompleteness,
    pub provenance_links: Vec<EvidenceReference>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl ExplanationPackage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "workspace_explanation:";

    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        scope: ExplanationScope,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
    ) -> Result<Self, WorkspaceExplanationError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let max_sections = scope.max_sections.max(1);

        let mut sections = Vec::new();
        let mut gaps = Vec::new();
        let mut conflicts = Vec::new();
        let mut provenance_links = Vec::new();
        let mut requested = 0usize;
        let mut available = 0usize;

        if scope.include_state {
            requested += 1;
            match state.and_then(|s| s.current.as_ref()) {
                Some(envelope) => {
                    available += 1;
                    let refs = vec![EvidenceReference::link(
                        "workspace_state_envelope",
                        envelope.state_id.clone(),
                    )];
                    provenance_links.extend(refs.clone());
                    let completeness = if envelope.consistency.as_str() == "contradictory" {
                        ExplanationCompleteness::Contradictory
                    } else if !envelope.unknowns.is_empty()
                        || envelope.completeness.as_str() == "partial"
                        || envelope.freshness.as_str() == "stale"
                        || envelope.freshness.as_str() == "unknown"
                    {
                        ExplanationCompleteness::Partial
                    } else {
                        ExplanationCompleteness::Complete
                    };
                    if envelope.consistency.as_str() == "contradictory" {
                        for c in &envelope.contradictions {
                            conflicts.push(ExplanationConflict::observed(
                                "state",
                                format!(
                                    "Source {} reported conflict with {}: {} (severity={}). Conflict preserved — not resolved.",
                                    c.source_a, c.source_b, c.description, c.severity
                                ),
                                vec![EvidenceReference::link(
                                    "workspace_state_conflict",
                                    c.conflict_id.clone(),
                                )],
                                vec![
                                    "Which source (if either) is correct remains unknown".into(),
                                ],
                            ));
                        }
                    }
                    sections.push(ExplanationSection::compose(
                        "state",
                        "Unified Workspace State",
                        format!(
                            "Current envelope revision {} — freshness={}, completeness={}, consistency={}, sources={}, unknowns={}. State is composed evidence, not explanation authority.",
                            envelope.revision,
                            envelope.freshness.as_str(),
                            envelope.completeness.as_str(),
                            envelope.consistency.as_str(),
                            envelope.sources.len(),
                            envelope.unknowns.len()
                        ),
                        refs,
                        completeness,
                    ));
                }
                None => {
                    gaps.push(ExplanationGap::record(
                        "state",
                        "No durable workspace state envelope available — Missing evidence ≠ No change",
                        "high",
                        vec![],
                    ));
                }
            }
        }

        if scope.include_policy {
            requested += 1;
            match policy.and_then(|p| p.current.as_ref()) {
                Some(view) => {
                    available += 1;
                    let refs = vec![EvidenceReference::link(
                        "policy_governance",
                        view.meta.evaluation_set_id.clone(),
                    )];
                    provenance_links.extend(refs.clone());
                    let agg = view.meta.aggregate_result.as_str();
                    let completeness = if agg == "unknown" {
                        ExplanationCompleteness::Unknown
                    } else if agg == "violation" || agg == "requires_review" {
                        ExplanationCompleteness::Partial
                    } else {
                        ExplanationCompleteness::Complete
                    };
                    sections.push(ExplanationSection::compose(
                        "policy",
                        "Policy Governance",
                        format!(
                            "Governance evaluation {} — aggregate_result={}, evaluations={}, context_revision={:?}. Policy explains authority; Gateway remains final. Unknown must not become Compliant.",
                            view.meta.evaluation_set_id,
                            agg,
                            view.evaluations.len(),
                            view.context_revision
                        ),
                        refs,
                        completeness,
                    ));
                    if agg == "unknown" {
                        gaps.push(ExplanationGap::record(
                            "policy",
                            "Policy aggregate result is Unknown — never assume Compliant",
                            "high",
                            vec![EvidenceReference::link(
                                "policy_governance",
                                view.meta.evaluation_set_id.clone(),
                            )],
                        ));
                    }
                }
                None => {
                    gaps.push(ExplanationGap::record(
                        "policy",
                        "No durable policy governance evaluation available — never assume Compliant",
                        "high",
                        vec![],
                    ));
                }
            }
        }

        if scope.include_reconstruction {
            requested += 1;
            match reconstruction.and_then(|r| r.current.as_ref()) {
                Some(view) => {
                    available += 1;
                    let refs = vec![EvidenceReference::link(
                        "historical_reconstruction",
                        view.reconstruction_id.clone(),
                    )];
                    provenance_links.extend(refs.clone());
                    let completeness = match view.completeness.as_str() {
                        "complete" => ExplanationCompleteness::Complete,
                        "partial" => ExplanationCompleteness::Partial,
                        "unknown" => ExplanationCompleteness::Unknown,
                        "contradictory" => ExplanationCompleteness::Contradictory,
                        _ => ExplanationCompleteness::Unavailable,
                    };
                    if completeness == ExplanationCompleteness::Contradictory {
                        conflicts.push(ExplanationConflict::observed(
                            "reconstruction",
                            "Historical reconstruction reports contradictory evidence. Disagreement preserved — no winner selected.",
                            refs.clone(),
                            vec!["Reconstruction conflict is observational only".into()],
                        ));
                    }
                    sections.push(ExplanationSection::compose(
                        "reconstruction",
                        "Historical Reconstruction",
                        format!(
                            "Reconstruction {} from {:?} to {:?} — completeness={}, changes={}, gaps={}. Reconstruction explains change; it is not source of truth.",
                            view.reconstruction_id,
                            view.from_revision,
                            view.to_revision,
                            view.completeness.as_str(),
                            view.timeline.len(),
                            view.gaps.len()
                        ),
                        refs,
                        completeness,
                    ));
                    for g in &view.gaps {
                        gaps.push(ExplanationGap::record(
                            "reconstruction",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![EvidenceReference::link("evidence_gap", g.gap_id.clone())],
                        ));
                    }
                }
                None => {
                    gaps.push(ExplanationGap::record(
                        "reconstruction",
                        "No durable historical reconstruction available — Missing evidence ≠ No change",
                        "high",
                        vec![],
                    ));
                }
            }
        }

        if scope.include_temporal {
            requested += 1;
            match temporal.and_then(|t| t.current.as_ref()) {
                Some(view) => {
                    available += 1;
                    let refs = vec![EvidenceReference::link(
                        "temporal_analysis",
                        view.analysis_id.clone(),
                    )];
                    provenance_links.extend(refs.clone());
                    let completeness = match view.completeness.as_str() {
                        "complete" => ExplanationCompleteness::Complete,
                        "partial" => ExplanationCompleteness::Partial,
                        "unknown" => ExplanationCompleteness::Unknown,
                        "contradictory" => ExplanationCompleteness::Contradictory,
                        _ => ExplanationCompleteness::Unavailable,
                    };
                    for c in &view.conflict_explanations {
                        conflicts.push(ExplanationConflict::observed(
                            "temporal",
                            c.description.clone(),
                            vec![EvidenceReference::link(
                                "temporal_conflict",
                                c.conflict_id.clone(),
                            )],
                            c.uncertainty.clone(),
                        ));
                    }
                    sections.push(ExplanationSection::compose(
                        "temporal",
                        "Temporal Intelligence",
                        format!(
                            "Temporal analysis {} — completeness={}, chain_refs={}, conflicts={}. Observed sequence ≠ cause. Evidence quality is diagnostic only.",
                            view.analysis_id,
                            view.completeness.as_str(),
                            view.chain_summary.ordered_refs.len(),
                            view.conflict_explanations.len()
                        ),
                        refs,
                        completeness,
                    ));
                }
                None => {
                    gaps.push(ExplanationGap::record(
                        "temporal",
                        "No durable temporal analysis available — Missing evidence ≠ No change",
                        "medium",
                        vec![],
                    ));
                }
            }
        }

        if let Some(theme) = &scope.theme {
            if theme == "conflicts-only" {
                sections.retain(|s| {
                    s.completeness == ExplanationCompleteness::Contradictory
                        || conflicts.iter().any(|c| c.surface == s.surface)
                });
            } else if theme == "gaps-only" {
                sections.clear();
            }
        }

        sections.truncate(max_sections);
        sections.sort_by(|a, b| a.section_id.cmp(&b.section_id));
        conflicts.sort_by(|a, b| a.conflict_id.cmp(&b.conflict_id));
        gaps.sort_by(|a, b| a.gap_id.cmp(&b.gap_id));

        let completeness = derive_completeness(requested, available, &gaps, &conflicts, &sections);
        let confidence = build_confidence(requested, available, &gaps, &conflicts);
        let limitations = vec![
            "Explanation Layer synthesises evidence — it does not change reality".into(),
            "Explanation confidence is diagnostic only — not decision authority".into(),
            "Observed sequence ≠ cause without explicit causal evidence".into(),
            "Conflicts are explained, not resolved".into(),
            "Unknown / missing upstream evidence remains unknown".into(),
        ];
        let narrative = build_narrative(&sections, &gaps, &conflicts, completeness);

        let package = Self {
            explanation_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id,
            generated_at,
            status: ExplanationStatus::Current,
            superseded_at: None,
            scope,
            sections,
            gaps,
            conflicts,
            confidence,
            completeness,
            provenance_links,
            narrative,
            limitations,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        package.validate()?;
        Ok(package)
    }

    pub fn compose_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        scope: ExplanationScope,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
    ) -> Result<Self, WorkspaceExplanationError> {
        let mut package = Self::compose(
            workspace_id,
            generated_at,
            scope,
            state,
            policy,
            reconstruction,
            temporal,
        )?;
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}",
            package.workspace_id,
            package
                .sections
                .iter()
                .map(|s| s.section_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            package
                .gaps
                .iter()
                .map(|g| g.gap_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            package
                .conflicts
                .iter()
                .map(|c| c.conflict_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            package.completeness.as_str()
        ));
        package.explanation_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(package)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = ExplanationStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.sections.iter().all(|s| s.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.conflicts.iter().all(|c| c.is_non_actionable())
            && self.confidence.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), WorkspaceExplanationError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(WorkspaceExplanationError::AuthorityEffectMustBeNone);
        }
        if narrative_has_forbidden_causality(&self.narrative) {
            return Err(WorkspaceExplanationError::MustNotBeActionable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceExplanationHistoryEntry {
    pub explanation_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub section_count: usize,
    pub gap_count: usize,
    pub conflict_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl WorkspaceExplanationHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_package(package: &ExplanationPackage) -> Option<Self> {
        if !package.status.is_terminal() {
            return None;
        }
        Some(Self {
            explanation_id: package.explanation_id.clone(),
            status: package.status.as_str().into(),
            created_at: package.generated_at.clone(),
            superseded_at: package.superseded_at.clone(),
            completeness: package.completeness.as_str().into(),
            section_count: package.sections.len(),
            gap_count: package.gaps.len(),
            conflict_count: package.conflicts.len(),
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
pub struct WorkspaceExplanationSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<ExplanationPackage>,
    pub history: Vec<WorkspaceExplanationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl WorkspaceExplanationSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<ExplanationPackage>,
        history: Vec<WorkspaceExplanationHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceExplanationSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        WorkspaceExplanationSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            explanation_id: self.current.as_ref().map(|c| c.explanation_id.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            section_count: self.current.as_ref().map(|c| c.sections.len()).unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            conflict_count: self
                .current
                .as_ref()
                .map(|c| c.conflicts.len())
                .unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceExplanationSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub explanation_id: Option<String>,
    pub completeness: Option<String>,
    pub section_count: usize,
    pub gap_count: usize,
    pub conflict_count: usize,
    pub history: Vec<WorkspaceExplanationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Explanation surface for ExplainWorkspaceSituation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSituationExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub section_summaries: Vec<String>,
    pub gaps: Vec<String>,
    pub conflicts: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceSituationExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_package(package: &ExplanationPackage) -> Self {
        Self {
            explanation_id: format!("situation_explanation:{}", package.explanation_id),
            workspace_id: package.workspace_id.clone(),
            completeness: Some(package.completeness.as_str().into()),
            section_summaries: package
                .sections
                .iter()
                .map(|s| format!("{}: {}", s.surface, s.title))
                .collect(),
            gaps: package.gaps.iter().map(|g| g.description.clone()).collect(),
            conflicts: package
                .conflicts
                .iter()
                .map(|c| c.description.clone())
                .collect(),
            evidence_refs: package
                .provenance_links
                .iter()
                .map(|p| p.external_ref.clone())
                .collect(),
            uncertainty: package
                .gaps
                .iter()
                .map(|g| format!("{}: {}", g.surface, g.description))
                .collect(),
            narrative: package.narrative.clone(),
            limitations: package.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

fn derive_completeness(
    requested: usize,
    available: usize,
    gaps: &[ExplanationGap],
    conflicts: &[ExplanationConflict],
    sections: &[ExplanationSection],
) -> ExplanationCompleteness {
    if requested == 0 {
        return ExplanationCompleteness::Unavailable;
    }
    if available == 0 {
        return ExplanationCompleteness::Unavailable;
    }
    if !conflicts.is_empty()
        || sections
            .iter()
            .any(|s| s.completeness == ExplanationCompleteness::Contradictory)
    {
        return ExplanationCompleteness::Contradictory;
    }
    if gaps.iter().any(|g| g.severity == "high")
        || sections
            .iter()
            .any(|s| s.completeness == ExplanationCompleteness::Unknown)
    {
        return ExplanationCompleteness::Unknown;
    }
    if available < requested || !gaps.is_empty() {
        return ExplanationCompleteness::Partial;
    }
    ExplanationCompleteness::Complete
}

fn build_confidence(
    requested: usize,
    available: usize,
    gaps: &[ExplanationGap],
    conflicts: &[ExplanationConflict],
) -> ExplanationConfidence {
    let coverage = if requested == 0 {
        0
    } else {
        ((available * 100) / requested).min(100) as u8
    };
    let digest = stable_digest(&format!(
        "{requested}|{available}|{}|{}",
        gaps.len(),
        conflicts.len()
    ));
    ExplanationConfidence {
        assessment_id: format!("{}{}", ExplanationConfidence::ID_PREFIX, digest),
        coverage,
        available_surfaces: available,
        requested_surfaces: requested,
        gap_count: gaps.len(),
        conflict_count: conflicts.len(),
        limitations: vec![
            "ExplanationConfidence is diagnostic coverage only — not decision authority".into(),
            "High coverage ≠ safe to act".into(),
        ],
        authority_effect: ExplanationConfidence::AUTHORITY_EFFECT_NONE.into(),
        actionable: false,
    }
}

fn build_narrative(
    sections: &[ExplanationSection],
    gaps: &[ExplanationGap],
    conflicts: &[ExplanationConflict],
    completeness: ExplanationCompleteness,
) -> String {
    let mut parts = Vec::new();
    parts.push(format!(
        "Workspace explanation package (completeness={}).",
        completeness.as_str()
    ));
    if sections.is_empty() {
        parts.push("No evidence sections could be composed from requested surfaces.".into());
    } else {
        parts.push(format!(
            "Sections present: {}.",
            sections
                .iter()
                .map(|s| s.surface.clone())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        for s in sections {
            parts.push(format!("{} — {}", s.title, s.body));
        }
    }
    if !gaps.is_empty() {
        parts.push(format!(
            "{} evidence gap(s) recorded — Missing evidence ≠ No change.",
            gaps.len()
        ));
    }
    if !conflicts.is_empty() {
        parts.push(format!(
            "{} conflict(s) preserved without resolution.",
            conflicts.len()
        ));
    }
    parts.push(
        "Language constraint: use observed evidence and sequence; do not claim unsupported causes."
            .into(),
    );
    parts.join(" ")
}

fn narrative_has_forbidden_causality(narrative: &str) -> bool {
    let lower = narrative.to_lowercase();
    let forbidden = [
        "because the user",
        "because the system",
        "therefore the system decided",
        "a caused b",
        "revision a caused",
        "this caused",
        "which caused",
        "was wrong",
        "safe to act",
        "policy would have approved",
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
    fn missing_upstreams_yield_unavailable_or_partial_with_gaps() {
        let package = ExplanationPackage::compose(
            "ws",
            "t0",
            ExplanationScope::all_surfaces(),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(package.completeness, ExplanationCompleteness::Unavailable);
        assert!(package.sections.is_empty());
        assert_eq!(package.gaps.len(), 4);
        assert!(package.is_non_executing());
    }

    #[test]
    fn deterministic_composition_identical_inputs() {
        let empty_state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let left = ExplanationPackage::compose_deterministic(
            "ws",
            "t1",
            ExplanationScope::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
        )
        .unwrap();
        let right = ExplanationPackage::compose_deterministic(
            "ws",
            "t1",
            ExplanationScope::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(left.explanation_id, right.explanation_id);
        assert_eq!(left.gaps, right.gaps);
        assert_eq!(left.sections, right.sections);
    }

    #[test]
    fn confidence_is_diagnostic_not_authority() {
        let package = ExplanationPackage::compose(
            "ws",
            "t0",
            ExplanationScope::all_surfaces(),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(package.confidence.authority_effect, "none");
        assert!(!package.confidence.actionable);
        assert!(package
            .confidence
            .limitations
            .iter()
            .any(|l| l.contains("not decision authority")));
    }

    #[test]
    fn history_separation_non_actionable() {
        let mut package = ExplanationPackage::compose(
            "ws",
            "t0",
            ExplanationScope::all_surfaces(),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        package.mark_superseded("t1");
        let hist = WorkspaceExplanationHistoryEntry::from_package(&package).unwrap();
        assert!(hist.is_non_actionable());
        let snap = WorkspaceExplanationSnapshot::assemble("ws", None, vec![hist], 1, "t2");
        assert!(snap.is_non_commandable());
    }

    #[test]
    fn narrative_rejects_unsupported_causality_patterns() {
        assert!(narrative_has_forbidden_causality(
            "Revision A caused Revision B"
        ));
        assert!(narrative_has_forbidden_causality(
            "Policy would have approved"
        ));
        assert!(!narrative_has_forbidden_causality(
            "Revision A was followed by Revision B"
        ));
    }
}
