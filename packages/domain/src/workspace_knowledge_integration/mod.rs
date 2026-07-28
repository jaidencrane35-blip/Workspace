//! Workspace Knowledge Integration — Programme III Batch 8.
//!
//! Integrate evidence. Do not become the authority.
//! Retrieval relevance ≠ factual authority.
//! integration ≠ authority; retrieval ≠ truth; relevance ≠ correctness; confidence ≠ permission.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::policy_governance::PolicyGovernanceSnapshot;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_knowledge_synthesis::KnowledgeSynthesisProjection;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum KnowledgeIntegrationError {
    #[error("invalid knowledge integration status: {0}")]
    InvalidStatus(String),

    #[error("invalid knowledge integration completeness: {0}")]
    InvalidCompleteness(String),

    #[error("invalid knowledge evidence link kind: {0}")]
    InvalidLinkKind(String),

    #[error("knowledge integration artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("knowledge integration artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("knowledge evidence links require at least one evidence reference")]
    HitRequiresEvidence,

    #[error("knowledge integration snapshot not found")]
    SnapshotNotFound,

    #[error("knowledge integration incomplete — required evidence unavailable")]
    IntegrationIncomplete,

    #[error("knowledge integration failed — corrupt or unusable evidence")]
    IntegrationFailed,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeIntegrationStatus {
    Current,
    Superseded,
    Archived,
}

impl KnowledgeIntegrationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, KnowledgeIntegrationError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(KnowledgeIntegrationError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeIntegrationCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl KnowledgeIntegrationCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, KnowledgeIntegrationError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(KnowledgeIntegrationError::InvalidCompleteness(other.into())),
        }
    }
}

/// Frame for knowledge retrieval / integration — constrains reading; does not invent coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeRetrievalFrame {
    pub focus: Option<String>,
    pub include_state: bool,
    pub include_policy: bool,
    pub include_reconstruction: bool,
    pub include_temporal: bool,
    pub include_explanation: bool,
    pub include_contextual: bool,
    pub include_knowledge_synthesis: bool,
    pub max_hits: usize,
}

impl KnowledgeRetrievalFrame {
    pub const DEFAULT_MAX_HITS: usize = 48;

    pub fn all_surfaces() -> Self {
        Self {
            focus: None,
            include_state: true,
            include_policy: true,
            include_reconstruction: true,
            include_temporal: true,
            include_explanation: true,
            include_contextual: true,
            include_knowledge_synthesis: true,
            max_hits: Self::DEFAULT_MAX_HITS,
        }
    }
}

/// Provenance anchor — integration statement → evidence → revision → origin domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeIntegrationEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeIntegrationEvidenceRef {
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
pub struct IntegrationGap {
    pub gap_id: String,
    pub surface: String,
    pub description: String,
    pub severity: String,
    pub evidence_refs: Vec<KnowledgeIntegrationEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl IntegrationGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "integration_gap:";

    pub fn record(
        surface: impl Into<String>,
        description: impl Into<String>,
        severity: impl Into<String>,
        evidence_refs: Vec<KnowledgeIntegrationEvidenceRef>,
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

/// Primary hit / link unit — integration result → source artefact → revision.
/// Hits use descriptive surface kinds; cross-layer joins use meaning-only ALLOWED_KINDS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeEvidenceLink {
    pub link_id: String,
    pub kind: String,
    pub label: String,
    pub body: String,
    pub source_artefact_ref: String,
    pub related_ref: Option<String>,
    pub source_revision: Option<String>,
    pub origin_domain: String,
    pub evidence_refs: Vec<KnowledgeIntegrationEvidenceRef>,
    pub provenance: Vec<String>,
    pub uncertainty: Vec<String>,
    pub confidence: u8,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeEvidenceLink {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "knowledge_evidence_link:";

    pub const ALLOWED_KINDS: &'static [&'static str] = &[
        "references",
        "relates_to",
        "derived_from",
        "supported_by",
    ];

    pub const HIT_KINDS: &'static [&'static str] = &[
        "hit_state",
        "hit_policy",
        "hit_reconstruction",
        "hit_temporal",
        "hit_explanation",
        "hit_contextual",
        "hit_knowledge_concept",
        "hit_knowledge_cluster",
        "hit_theme",
    ];

    pub const FORBIDDEN_KINDS: &'static [&'static str] = &[
        "causes",
        "requires",
        "should_execute",
        "authorises",
        "requires_action",
        "triggers",
        "leads_to_action",
        "approve",
    ];

    pub fn derive_hit(
        kind: impl Into<String>,
        label: impl Into<String>,
        body: impl Into<String>,
        source_artefact_ref: impl Into<String>,
        source_revision: Option<String>,
        origin_domain: impl Into<String>,
        evidence_refs: Vec<KnowledgeIntegrationEvidenceRef>,
        provenance: Vec<String>,
        uncertainty: Vec<String>,
        confidence: u8,
    ) -> Result<Self, KnowledgeIntegrationError> {
        if evidence_refs.is_empty() {
            return Err(KnowledgeIntegrationError::HitRequiresEvidence);
        }
        let kind = kind.into();
        validate_hit_kind(&kind)?;
        let label = label.into();
        let body = body.into();
        let source_artefact_ref = source_artefact_ref.into();
        let origin_domain = origin_domain.into();
        let digest = stable_digest(&format!(
            "{kind}|{label}|{source_artefact_ref}|{}",
            source_revision.as_deref().unwrap_or("")
        ));
        Ok(Self {
            link_id: format!("{}{}", Self::ID_PREFIX, digest),
            kind,
            label,
            body,
            source_artefact_ref,
            related_ref: None,
            source_revision,
            origin_domain,
            evidence_refs,
            provenance,
            uncertainty,
            confidence: confidence.min(100),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn connect(
        kind: impl Into<String>,
        label: impl Into<String>,
        body: impl Into<String>,
        source_artefact_ref: impl Into<String>,
        related_ref: impl Into<String>,
        origin_domain: impl Into<String>,
        evidence_refs: Vec<KnowledgeIntegrationEvidenceRef>,
        provenance: Vec<String>,
        uncertainty: Vec<String>,
    ) -> Result<Self, KnowledgeIntegrationError> {
        if evidence_refs.is_empty() {
            return Err(KnowledgeIntegrationError::HitRequiresEvidence);
        }
        let kind = kind.into();
        validate_link_kind(&kind)?;
        let label = label.into();
        let body = body.into();
        let source_artefact_ref = source_artefact_ref.into();
        let related_ref = related_ref.into();
        let origin_domain = origin_domain.into();
        let digest = stable_digest(&format!(
            "{kind}|{source_artefact_ref}|{related_ref}|{label}"
        ));
        Ok(Self {
            link_id: format!("{}{}", Self::ID_PREFIX, digest),
            kind,
            label,
            body,
            source_artefact_ref,
            related_ref: Some(related_ref),
            source_revision: None,
            origin_domain,
            evidence_refs,
            provenance,
            uncertainty,
            confidence: 50,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_hit(&self) -> bool {
        Self::HIT_KINDS.iter().any(|k| *k == self.kind)
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_refs.iter().all(|r| r.is_non_actionable())
    }
}

/// Diagnostic retrieval confidence — ≠ truth / decision / action / permission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeRetrievalConfidence {
    pub assessment_id: String,
    pub coverage: u8,
    pub available_surfaces: usize,
    pub requested_surfaces: usize,
    pub gap_count: usize,
    pub conflict_count: usize,
    pub hit_count: usize,
    pub uncertainty: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeRetrievalConfidence {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "knowledge_retrieval_confidence:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Structured knowledge integration artefact — retrieval over evidence, not authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeIntegrationResult {
    pub integration_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: KnowledgeIntegrationStatus,
    pub superseded_at: Option<String>,
    pub frame: KnowledgeRetrievalFrame,
    pub source_revisions: Vec<String>,
    pub query_context: Option<String>,
    pub links: Vec<KnowledgeEvidenceLink>,
    pub gaps: Vec<IntegrationGap>,
    pub confidence: KnowledgeRetrievalConfidence,
    pub completeness: KnowledgeIntegrationCompleteness,
    pub provenance_links: Vec<KnowledgeIntegrationEvidenceRef>,
    pub summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl KnowledgeIntegrationResult {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "knowledge_integration:";

    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: KnowledgeRetrievalFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
    ) -> Result<Self, KnowledgeIntegrationError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let max_hits = frame.max_hits.max(1);
        let query_context = frame.focus.clone();

        let mut links = Vec::new();
        let mut gaps = Vec::new();
        let mut provenance_links = Vec::new();
        let mut source_revisions = Vec::new();
        let mut conflict_count = 0usize;
        let mut requested = 0usize;
        let mut available = 0usize;
        let mut surface_hits: Vec<(String, String)> = Vec::new();

        if frame.include_state {
            requested += 1;
            match state.and_then(|s| s.current.as_ref()) {
                Some(envelope) => {
                    available += 1;
                    source_revisions.push(format!("state:{}", envelope.revision));
                    let refs = vec![KnowledgeIntegrationEvidenceRef::link(
                        "workspace_state_envelope",
                        envelope.state_id.clone(),
                        Some(envelope.revision.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    if envelope.consistency.as_str() == "contradictory" {
                        conflict_count += envelope.contradictions.len().max(1);
                    }
                    let mut uncertainty = envelope.unknowns.clone();
                    if envelope.consistency.as_str() == "contradictory" {
                        uncertainty.push(
                            "Envelope consistency is contradictory — conflict preserved, not resolved"
                                .into(),
                        );
                    }
                    let hit = KnowledgeEvidenceLink::derive_hit(
                        "hit_state",
                        "State envelope hit",
                        format!(
                            "Retrieval hit from unified state envelope {}. Freshness={}, completeness={}, consistency={}, sources={}. Relevance for retrieval only — not factual authority.",
                            envelope.revision,
                            envelope.freshness.as_str(),
                            envelope.completeness.as_str(),
                            envelope.consistency.as_str(),
                            envelope.sources.len()
                        ),
                        envelope.state_id.clone(),
                        Some(envelope.revision.clone()),
                        "workspace_state_envelope",
                        refs,
                        vec![format!(
                            "hit→envelope:{}→revision:{}→domain:workspace_state_envelope",
                            envelope.state_id, envelope.revision
                        )],
                        uncertainty,
                        70,
                    )?;
                    surface_hits.push(("state".into(), hit.link_id.clone()));
                    links.push(hit);
                }
                None => {
                    gaps.push(IntegrationGap::record(
                        "state",
                        "No durable workspace state envelope available — Missing evidence ≠ invented retrieval",
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
                    let refs = vec![KnowledgeIntegrationEvidenceRef::link(
                        "policy_governance",
                        view.meta.evaluation_set_id.clone(),
                        Some(rev.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    let agg = view.meta.aggregate_result.as_str();
                    if agg == "unknown" {
                        gaps.push(IntegrationGap::record(
                            "policy",
                            "Policy aggregate is Unknown — never assume Compliant or invent approval",
                            "high",
                            refs.clone(),
                        ));
                    }
                    let hit = KnowledgeEvidenceLink::derive_hit(
                        "hit_policy",
                        "Policy governance hit",
                        format!(
                            "Retrieval hit from governance evaluation {}. Aggregate result={}. Policy explains authority; Gateway remains final. Retrieval relevance ≠ permission.",
                            view.meta.evaluation_set_id, agg
                        ),
                        view.meta.evaluation_set_id.clone(),
                        Some(rev.clone()),
                        "policy_governance",
                        refs,
                        vec![format!(
                            "hit→policy:{}→revision:{}→domain:policy_governance",
                            view.meta.evaluation_set_id, rev
                        )],
                        if agg == "unknown" {
                            vec!["Unknown policy context must not become approval".into()]
                        } else {
                            vec![]
                        },
                        if agg == "unknown" { 30 } else { 65 },
                    )?;
                    surface_hits.push(("policy".into(), hit.link_id.clone()));
                    links.push(hit);
                }
                None => {
                    gaps.push(IntegrationGap::record(
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
                    let refs = vec![KnowledgeIntegrationEvidenceRef::link(
                        "historical_reconstruction",
                        view.reconstruction_id.clone(),
                        Some(rev.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    if view.completeness.as_str() == "contradictory" {
                        conflict_count += 1;
                    }
                    for g in &view.gaps {
                        gaps.push(IntegrationGap::record(
                            "reconstruction",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![KnowledgeIntegrationEvidenceRef::link(
                                "evidence_gap",
                                g.gap_id.clone(),
                                None,
                            )],
                        ));
                    }
                    let hit = KnowledgeEvidenceLink::derive_hit(
                        "hit_reconstruction",
                        "Historical reconstruction hit",
                        format!(
                            "Retrieval hit from reconstruction {} from {:?} to {:?} — completeness={}, changes={}. Continuity framing only; observed sequence ≠ cause.",
                            view.reconstruction_id,
                            view.from_revision,
                            view.to_revision,
                            view.completeness.as_str(),
                            view.timeline.len()
                        ),
                        view.reconstruction_id.clone(),
                        Some(rev.clone()),
                        "historical_reconstruction",
                        refs,
                        vec![format!(
                            "hit→reconstruction:{}→revision:{}→domain:historical_reconstruction",
                            view.reconstruction_id, rev
                        )],
                        view.gaps.iter().map(|g| g.description.clone()).collect(),
                        60,
                    )?;
                    surface_hits.push(("reconstruction".into(), hit.link_id.clone()));
                    links.push(hit);
                }
                None => {
                    gaps.push(IntegrationGap::record(
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
                    let refs = vec![KnowledgeIntegrationEvidenceRef::link(
                        "temporal_analysis",
                        view.analysis_id.clone(),
                        Some(rev.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    if view.completeness.as_str() == "contradictory" {
                        conflict_count += view.conflict_explanations.len().max(1);
                    }
                    let hit = KnowledgeEvidenceLink::derive_hit(
                        "hit_temporal",
                        "Temporal analysis hit",
                        format!(
                            "Retrieval hit from temporal analysis {} — completeness={}, chain_refs={}, conflicts={}. Temporal organisation only; not a forecast.",
                            view.analysis_id,
                            view.completeness.as_str(),
                            view.chain_summary.ordered_refs.len(),
                            view.conflict_explanations.len()
                        ),
                        view.analysis_id.clone(),
                        Some(rev.clone()),
                        "temporal_analysis",
                        refs,
                        vec![format!(
                            "hit→temporal:{}→revision:{}→domain:temporal_analysis",
                            view.analysis_id, rev
                        )],
                        view.conflict_explanations
                            .iter()
                            .map(|c| c.description.clone())
                            .collect(),
                        55,
                    )?;
                    surface_hits.push(("temporal".into(), hit.link_id.clone()));
                    links.push(hit);
                }
                None => {
                    gaps.push(IntegrationGap::record(
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
                    let refs = vec![KnowledgeIntegrationEvidenceRef::link(
                        "workspace_explanation",
                        package.explanation_id.clone(),
                        Some(package.explanation_id.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    if package.completeness.as_str() == "contradictory" {
                        conflict_count += package.conflicts.len().max(1);
                    }
                    for g in &package.gaps {
                        gaps.push(IntegrationGap::record(
                            "explanation",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![KnowledgeIntegrationEvidenceRef::link(
                                "explanation_gap",
                                g.gap_id.clone(),
                                None,
                            )],
                        ));
                    }
                    let summary_body = if package.narrative.is_empty() {
                        format!(
                            "Explanation package {} — completeness={}, sections={}, gaps={}, conflicts={}. Retrieval input only.",
                            package.explanation_id,
                            package.completeness.as_str(),
                            package.sections.len(),
                            package.gaps.len(),
                            package.conflicts.len()
                        )
                    } else {
                        format!(
                            "Explanation package summary: {}. Completeness={}, sections={}, gaps={}, conflicts={}. Descriptive retrieval only.",
                            package.narrative,
                            package.completeness.as_str(),
                            package.sections.len(),
                            package.gaps.len(),
                            package.conflicts.len()
                        )
                    };
                    let hit = KnowledgeEvidenceLink::derive_hit(
                        "hit_explanation",
                        "Explanation package hit",
                        summary_body,
                        package.explanation_id.clone(),
                        Some(package.explanation_id.clone()),
                        "workspace_explanation",
                        refs,
                        vec![format!(
                            "hit→explanation:{}→domain:workspace_explanation",
                            package.explanation_id
                        )],
                        package
                            .conflicts
                            .iter()
                            .map(|c| c.description.clone())
                            .collect(),
                        package.confidence.coverage,
                    )?;
                    surface_hits.push(("explanation".into(), hit.link_id.clone()));
                    links.push(hit);
                }
                None => {
                    gaps.push(IntegrationGap::record(
                        "explanation",
                        "No durable explanation package available — Missing evidence ≠ invented situation",
                        "medium",
                        vec![],
                    ));
                }
            }
        }

        if frame.include_contextual {
            requested += 1;
            match contextual.and_then(|c| c.current.as_ref()) {
                Some(snap) => {
                    available += 1;
                    source_revisions.push(format!("contextual:{}", snap.understanding_id));
                    let snap_refs = vec![KnowledgeIntegrationEvidenceRef::link(
                        "contextual_understanding",
                        snap.understanding_id.clone(),
                        Some(snap.understanding_id.clone()),
                    )];
                    provenance_links.extend(snap_refs.clone());
                    if snap.completeness.as_str() == "contradictory" {
                        conflict_count += 1;
                    }
                    for g in &snap.gaps {
                        gaps.push(IntegrationGap::record(
                            "contextual",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![KnowledgeIntegrationEvidenceRef::link(
                                "contextual_gap",
                                g.gap_id.clone(),
                                None,
                            )],
                        ));
                    }
                    if snap.themes.is_empty() {
                        let hit = KnowledgeEvidenceLink::derive_hit(
                            "hit_contextual",
                            "Contextual understanding hit",
                            format!(
                                "Retrieval hit from contextual understanding {} — completeness={}, themes=0, gaps={}. Situational framing referenced; not replaced.",
                                snap.understanding_id,
                                snap.completeness.as_str(),
                                snap.gaps.len()
                            ),
                            snap.understanding_id.clone(),
                            Some(snap.understanding_id.clone()),
                            "contextual_understanding",
                            snap_refs,
                            vec![format!(
                                "hit→contextual:{}→domain:contextual_understanding",
                                snap.understanding_id
                            )],
                            vec![],
                            50,
                        )?;
                        surface_hits.push(("contextual".into(), hit.link_id.clone()));
                        links.push(hit);
                    } else {
                        let mut first_theme_linked = false;
                        for theme in &snap.themes {
                            let theme_refs: Vec<KnowledgeIntegrationEvidenceRef> =
                                if theme.evidence_refs.is_empty() {
                                    snap_refs.clone()
                                } else {
                                    theme
                                        .evidence_refs
                                        .iter()
                                        .map(|r| {
                                            KnowledgeIntegrationEvidenceRef::link(
                                                r.origin_domain.clone(),
                                                r.external_ref.clone(),
                                                r.source_revision.clone(),
                                            )
                                        })
                                        .collect()
                                };
                            let hit = KnowledgeEvidenceLink::derive_hit(
                                "hit_theme",
                                theme.title.clone(),
                                theme.body.clone(),
                                theme.theme_id.clone(),
                                Some(snap.understanding_id.clone()),
                                "contextual_understanding",
                                theme_refs,
                                vec![format!(
                                    "hit→theme:{}→understanding:{}→domain:contextual_understanding",
                                    theme.theme_id, snap.understanding_id
                                )],
                                vec![],
                                60,
                            )?;
                            if !first_theme_linked {
                                surface_hits.push(("contextual".into(), hit.link_id.clone()));
                                first_theme_linked = true;
                            } else {
                                surface_hits.push(("contextual_theme".into(), hit.link_id.clone()));
                            }
                            links.push(hit);
                        }
                    }
                }
                None => {
                    gaps.push(IntegrationGap::record(
                        "contextual",
                        "No durable contextual understanding available — Missing evidence ≠ invented knowledge",
                        "medium",
                        vec![],
                    ));
                }
            }
        }

        if frame.include_knowledge_synthesis {
            requested += 1;
            match knowledge_synthesis.and_then(|k| k.current.as_ref()) {
                Some(synth) => {
                    available += 1;
                    source_revisions.push(format!("knowledge_synthesis:{}", synth.synthesis_id));
                    let synth_refs = vec![KnowledgeIntegrationEvidenceRef::link(
                        "knowledge_synthesis",
                        synth.synthesis_id.clone(),
                        Some(synth.synthesis_id.clone()),
                    )];
                    provenance_links.extend(synth_refs.clone());
                    if synth.completeness.as_str() == "contradictory" {
                        conflict_count += 1;
                    }
                    for g in &synth.gaps {
                        gaps.push(IntegrationGap::record(
                            "knowledge_synthesis",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![KnowledgeIntegrationEvidenceRef::link(
                                "knowledge_gap",
                                g.gap_id.clone(),
                                None,
                            )],
                        ));
                    }

                    let mut synthesis_surface_recorded = false;
                    if synth.concepts.is_empty() && synth.clusters.is_empty() {
                        let hit = KnowledgeEvidenceLink::derive_hit(
                            "hit_knowledge_concept",
                            "Knowledge synthesis hit",
                            format!(
                                "Retrieval hit from knowledge synthesis {} — completeness={}, concepts=0, clusters={}, gaps={}. Derived knowledge ≠ truth.",
                                synth.synthesis_id,
                                synth.completeness.as_str(),
                                synth.clusters.len(),
                                synth.gaps.len()
                            ),
                            synth.synthesis_id.clone(),
                            Some(synth.synthesis_id.clone()),
                            "knowledge_synthesis",
                            synth_refs,
                            vec![format!(
                                "hit→synthesis:{}→domain:knowledge_synthesis",
                                synth.synthesis_id
                            )],
                            vec![],
                            45,
                        )?;
                        surface_hits.push(("knowledge_synthesis".into(), hit.link_id.clone()));
                        links.push(hit);
                    } else {
                        for concept in &synth.concepts {
                            let concept_refs: Vec<KnowledgeIntegrationEvidenceRef> =
                                if concept.evidence_refs.is_empty() {
                                    synth_refs.clone()
                                } else {
                                    concept
                                        .evidence_refs
                                        .iter()
                                        .map(|r| {
                                            KnowledgeIntegrationEvidenceRef::link(
                                                r.origin_domain.clone(),
                                                r.external_ref.clone(),
                                                r.source_revision.clone(),
                                            )
                                        })
                                        .collect()
                                };
                            let hit = KnowledgeEvidenceLink::derive_hit(
                                "hit_knowledge_concept",
                                concept.label.clone(),
                                concept.description.clone(),
                                concept.id.clone(),
                                Some(synth.synthesis_id.clone()),
                                "knowledge_synthesis",
                                concept_refs,
                                {
                                    let mut p = concept.provenance.clone();
                                    p.push(format!(
                                        "hit→concept:{}→synthesis:{}→domain:knowledge_synthesis",
                                        concept.id, synth.synthesis_id
                                    ));
                                    p
                                },
                                concept.uncertainty.clone(),
                                concept.confidence,
                            )?;
                            if !synthesis_surface_recorded {
                                surface_hits
                                    .push(("knowledge_synthesis".into(), hit.link_id.clone()));
                                synthesis_surface_recorded = true;
                            } else {
                                surface_hits
                                    .push(("knowledge_concept".into(), hit.link_id.clone()));
                            }
                            links.push(hit);
                        }
                        for cluster in &synth.clusters {
                            let cluster_refs: Vec<KnowledgeIntegrationEvidenceRef> =
                                if cluster.evidence_refs.is_empty() {
                                    synth_refs.clone()
                                } else {
                                    cluster
                                        .evidence_refs
                                        .iter()
                                        .map(|r| {
                                            KnowledgeIntegrationEvidenceRef::link(
                                                r.origin_domain.clone(),
                                                r.external_ref.clone(),
                                                r.source_revision.clone(),
                                            )
                                        })
                                        .collect()
                                };
                            let hit = KnowledgeEvidenceLink::derive_hit(
                                "hit_knowledge_cluster",
                                cluster.label.clone(),
                                cluster.description.clone(),
                                cluster.cluster_id.clone(),
                                Some(synth.synthesis_id.clone()),
                                "knowledge_synthesis",
                                cluster_refs,
                                vec![format!(
                                    "hit→cluster:{}→synthesis:{}→domain:knowledge_synthesis",
                                    cluster.cluster_id, synth.synthesis_id
                                )],
                                vec![],
                                50,
                            )?;
                            if !synthesis_surface_recorded {
                                surface_hits
                                    .push(("knowledge_synthesis".into(), hit.link_id.clone()));
                                synthesis_surface_recorded = true;
                            } else {
                                surface_hits
                                    .push(("knowledge_cluster".into(), hit.link_id.clone()));
                            }
                            links.push(hit);
                        }
                    }
                }
                None => {
                    gaps.push(IntegrationGap::record(
                        "knowledge_synthesis",
                        "No durable knowledge synthesis available — Missing evidence ≠ invented retrieval",
                        "medium",
                        vec![],
                    ));
                }
            }
        }

        if let Some(focus) = &frame.focus {
            let focus_l = focus.to_lowercase();
            links.retain(|l| {
                !l.is_hit()
                    || l.label.to_lowercase().contains(&focus_l)
                    || l.body.to_lowercase().contains(&focus_l)
            });
            surface_hits.retain(|(_, id)| links.iter().any(|l| &l.link_id == id));
        }

        // Prefer hits first; truncate hits to leave room for cross-links later.
        let mut hits: Vec<_> = links.into_iter().filter(|l| l.is_hit()).collect();
        hits.sort_by(|a, b| a.link_id.cmp(&b.link_id));
        if hits.len() > max_hits {
            hits.truncate(max_hits);
            let retained: std::collections::HashSet<_> =
                hits.iter().map(|h| h.link_id.clone()).collect();
            surface_hits.retain(|(_, id)| retained.contains(id));
        }

        // Meaning-only connect links when 2+ hits from different primary surfaces.
        let primary_surfaces: std::collections::HashSet<_> = surface_hits
            .iter()
            .filter(|(s, _)| {
                matches!(
                    s.as_str(),
                    "state"
                        | "policy"
                        | "reconstruction"
                        | "temporal"
                        | "explanation"
                        | "contextual"
                        | "knowledge_synthesis"
                )
            })
            .map(|(s, _)| s.as_str())
            .collect();

        let mut connect_links = Vec::new();
        if hits.len() >= 2 && (primary_surfaces.len() >= 2 || available >= 2) {
            let first = &hits[0];
            let second = &hits[1];
            let kind = if available >= 3 {
                "supported_by"
            } else if primary_surfaces.len() >= 2 {
                "relates_to"
            } else {
                "references"
            };
            let mut pair_refs = first.evidence_refs.clone();
            pair_refs.extend(second.evidence_refs.clone());
            connect_links.push(KnowledgeEvidenceLink::connect(
                kind,
                format!("Cross-surface {} link", kind),
                format!(
                    "Meaning-only join between {} and {} — co-presence observed; retrieval relevance ≠ correctness; correlation ≠ causation.",
                    first.link_id, second.link_id
                ),
                first.link_id.clone(),
                second.link_id.clone(),
                "knowledge_integration",
                pair_refs,
                vec![format!(
                    "link→{}→{}→kind:{}",
                    first.link_id, second.link_id, kind
                )],
                vec![
                    "Multi-surface co-presence observed — relationship is meaning-only".into(),
                ],
            )?);

            if hits.len() >= 3 {
                let third = &hits[2];
                let mut derived_refs = first.evidence_refs.clone();
                derived_refs.extend(third.evidence_refs.clone());
                connect_links.push(KnowledgeEvidenceLink::connect(
                    "derived_from",
                    "Cross-surface derived_from link",
                    format!(
                        "Meaning-only derived_from join between {} and {} — descriptive lineage only.",
                        first.link_id, third.link_id
                    ),
                    first.link_id.clone(),
                    third.link_id.clone(),
                    "knowledge_integration",
                    derived_refs,
                    vec![format!(
                        "link→{}→{}→kind:derived_from",
                        first.link_id, third.link_id
                    )],
                    vec![
                        "Descriptive lineage across retrieved hits — not a causal claim".into(),
                    ],
                )?);
            }

            // shares via supported_by when evidence origin domains overlap
            for i in 0..hits.len() {
                for j in (i + 1)..hits.len() {
                    let a = &hits[i];
                    let b = &hits[j];
                    let a_domains: std::collections::HashSet<_> =
                        a.evidence_refs.iter().map(|r| r.origin_domain.as_str()).collect();
                    let shared: Vec<_> = b
                        .evidence_refs
                        .iter()
                        .filter(|r| a_domains.contains(r.origin_domain.as_str()))
                        .cloned()
                        .collect();
                    if !shared.is_empty()
                        && !(i == 0 && j == 1 && kind == "supported_by")
                    {
                        connect_links.push(KnowledgeEvidenceLink::connect(
                            "supported_by",
                            "Shared-evidence supported_by link",
                            format!(
                                "Hits {} and {} share evidence origin domain(s) — correlation only, not a causal claim.",
                                a.link_id, b.link_id
                            ),
                            a.link_id.clone(),
                            b.link_id.clone(),
                            "knowledge_integration",
                            shared,
                            vec![format!(
                                "link→{}→{}→kind:supported_by",
                                a.link_id, b.link_id
                            )],
                            vec![
                                "Shared evidence origin domain observed — correlation only".into(),
                            ],
                        )?);
                    }
                }
            }
        }

        {
            let mut seen = std::collections::HashSet::new();
            connect_links.retain(|l| seen.insert(l.link_id.clone()));
        }
        connect_links.sort_by(|a, b| a.link_id.cmp(&b.link_id));

        let mut all_links = hits;
        all_links.extend(connect_links);
        all_links.sort_by(|a, b| a.link_id.cmp(&b.link_id));
        if all_links.len() > max_hits {
            // Prefer retaining hits over connect links when truncating.
            let mut preferred: Vec<_> = all_links.iter().filter(|l| l.is_hit()).cloned().collect();
            let mut joins: Vec<_> = all_links.into_iter().filter(|l| !l.is_hit()).collect();
            preferred.sort_by(|a, b| a.link_id.cmp(&b.link_id));
            joins.sort_by(|a, b| a.link_id.cmp(&b.link_id));
            if preferred.len() >= max_hits {
                preferred.truncate(max_hits);
                all_links = preferred;
            } else {
                let room = max_hits - preferred.len();
                joins.truncate(room);
                preferred.extend(joins);
                preferred.sort_by(|a, b| a.link_id.cmp(&b.link_id));
                all_links = preferred;
            }
        }

        gaps.sort_by(|a, b| a.gap_id.cmp(&b.gap_id));
        source_revisions.sort();
        source_revisions.dedup();

        let hit_count = all_links.iter().filter(|l| l.is_hit()).count();
        let completeness =
            derive_completeness(requested, available, &gaps, conflict_count, hit_count);
        let confidence = build_confidence(
            requested,
            available,
            &gaps,
            conflict_count,
            hit_count,
            &source_revisions,
        );
        let limitations = vec![
            "Knowledge integration assembles retrieval over evidence — it does not become the authority"
                .into(),
            "integration ≠ authority".into(),
            "retrieval ≠ truth".into(),
            "relevance ≠ correctness".into(),
            "confidence ≠ permission".into(),
            "Retrieval relevance ≠ factual authority".into(),
            "Gaps and conflicts are preserved, not resolved".into(),
            "Prior integration cache ≠ current truth without revision binding".into(),
        ];
        let summary = build_summary(&all_links, &gaps, completeness);
        let narrative = build_narrative(&all_links, &gaps, &summary, completeness);

        let result = Self {
            integration_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id,
            generated_at,
            status: KnowledgeIntegrationStatus::Current,
            superseded_at: None,
            frame,
            source_revisions,
            query_context,
            links: all_links,
            gaps,
            confidence,
            completeness,
            provenance_links,
            summary,
            narrative,
            limitations,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        result.validate()?;
        Ok(result)
    }

    pub fn compose_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: KnowledgeRetrievalFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
    ) -> Result<Self, KnowledgeIntegrationError> {
        let mut snap = Self::compose(
            workspace_id,
            generated_at,
            frame,
            state,
            policy,
            reconstruction,
            temporal,
            explanation,
            contextual,
            knowledge_synthesis,
        )?;
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}|{}",
            snap.workspace_id,
            snap.source_revisions.join(","),
            snap.links
                .iter()
                .map(|l| l.link_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.gaps
                .iter()
                .map(|g| g.gap_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.query_context.as_deref().unwrap_or(""),
            snap.completeness.as_str()
        ));
        snap.integration_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(snap)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = KnowledgeIntegrationStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.links.iter().all(|l| l.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.confidence.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), KnowledgeIntegrationError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(KnowledgeIntegrationError::AuthorityEffectMustBeNone);
        }
        if self.links.iter().any(|l| l.evidence_refs.is_empty()) {
            return Err(KnowledgeIntegrationError::HitRequiresEvidence);
        }
        for link in &self.links {
            if link.is_hit() {
                validate_hit_kind(&link.kind)?;
            } else {
                validate_link_kind(&link.kind)?;
            }
        }
        if narrative_has_forbidden_patterns(&self.narrative)
            || narrative_has_forbidden_patterns(&self.summary)
            || self
                .links
                .iter()
                .any(|l| {
                    narrative_has_forbidden_patterns(&l.label)
                        || narrative_has_forbidden_patterns(&l.body)
                })
        {
            return Err(KnowledgeIntegrationError::MustNotBeActionable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeIntegrationHistoryEntry {
    pub integration_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub link_count: usize,
    pub hit_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl KnowledgeIntegrationHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_result(snap: &KnowledgeIntegrationResult) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            integration_id: snap.integration_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            link_count: snap.links.len(),
            hit_count: snap.links.iter().filter(|l| l.is_hit()).count(),
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
pub struct KnowledgeIntegrationProjection {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<KnowledgeIntegrationResult>,
    pub history: Vec<KnowledgeIntegrationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl KnowledgeIntegrationProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<KnowledgeIntegrationResult>,
        history: Vec<KnowledgeIntegrationHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> KnowledgeIntegrationSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        KnowledgeIntegrationSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            integration_id: self.current.as_ref().map(|c| c.integration_id.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            link_count: self.current.as_ref().map(|c| c.links.len()).unwrap_or(0),
            hit_count: self
                .current
                .as_ref()
                .map(|c| c.links.iter().filter(|l| l.is_hit()).count())
                .unwrap_or(0),
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
pub struct KnowledgeIntegrationSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub integration_id: Option<String>,
    pub completeness: Option<String>,
    pub link_count: usize,
    pub hit_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub history: Vec<KnowledgeIntegrationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Explanation surface for ExplainKnowledgeIntegration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeIntegrationExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub summary: Option<String>,
    pub link_summaries: Vec<String>,
    pub gaps: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeIntegrationExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_result(snap: &KnowledgeIntegrationResult) -> Self {
        Self {
            explanation_id: format!("knowledge_integration_explanation:{}", snap.integration_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            summary: Some(snap.summary.clone()),
            link_summaries: snap
                .links
                .iter()
                .map(|l| {
                    format!(
                        "{} [{}]: {} → {}",
                        l.kind,
                        l.link_id,
                        l.label,
                        l.source_artefact_ref
                    )
                })
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
    gaps: &[IntegrationGap],
    conflict_count: usize,
    _hit_count: usize,
) -> KnowledgeIntegrationCompleteness {
    if requested == 0 || available == 0 {
        return KnowledgeIntegrationCompleteness::Unavailable;
    }
    if conflict_count > 0 {
        return KnowledgeIntegrationCompleteness::Contradictory;
    }
    if gaps.iter().any(|g| g.severity == "high") {
        return KnowledgeIntegrationCompleteness::Unknown;
    }
    if available < requested || !gaps.is_empty() {
        return KnowledgeIntegrationCompleteness::Partial;
    }
    KnowledgeIntegrationCompleteness::Complete
}

fn build_confidence(
    requested: usize,
    available: usize,
    gaps: &[IntegrationGap],
    conflict_count: usize,
    hit_count: usize,
    source_revisions: &[String],
) -> KnowledgeRetrievalConfidence {
    let coverage = if requested == 0 {
        0
    } else {
        ((available * 100) / requested).min(100) as u8
    };
    let digest = stable_digest(&format!(
        "{requested}|{available}|{}|{}|{}|{}",
        gaps.len(),
        conflict_count,
        hit_count,
        source_revisions.join(",")
    ));
    KnowledgeRetrievalConfidence {
        assessment_id: format!("{}{}", KnowledgeRetrievalConfidence::ID_PREFIX, digest),
        coverage,
        available_surfaces: available,
        requested_surfaces: requested,
        gap_count: gaps.len(),
        conflict_count,
        hit_count,
        uncertainty: gaps
            .iter()
            .map(|g| format!("{}: {}", g.surface, g.description))
            .collect(),
        limitations: vec![
            "Knowledge retrieval confidence is diagnostic coverage only — not truth confidence"
                .into(),
            "Diagnostic confidence ≠ decision authority or action approval".into(),
            "High coverage ≠ permission to act / decide / approve".into(),
            "confidence ≠ permission; relevance ≠ correctness".into(),
        ],
        authority_effect: KnowledgeRetrievalConfidence::AUTHORITY_EFFECT_NONE.into(),
        actionable: false,
    }
}

fn build_summary(
    links: &[KnowledgeEvidenceLink],
    gaps: &[IntegrationGap],
    completeness: KnowledgeIntegrationCompleteness,
) -> String {
    let hits = links.iter().filter(|l| l.is_hit()).count();
    let joins = links.len().saturating_sub(hits);
    format!(
        "Knowledge integration completeness={}; hits={}; joins={}; gaps={}. Retrieval relevance ≠ factual authority.",
        completeness.as_str(),
        hits,
        joins,
        gaps.len()
    )
}

fn build_narrative(
    links: &[KnowledgeEvidenceLink],
    gaps: &[IntegrationGap],
    summary: &str,
    completeness: KnowledgeIntegrationCompleteness,
) -> String {
    let mut parts = Vec::new();
    parts.push(format!(
        "Workspace knowledge integration (completeness={}).",
        completeness.as_str()
    ));
    parts.push(summary.to_string());
    let hits: Vec<_> = links.iter().filter(|l| l.is_hit()).collect();
    if hits.is_empty() {
        parts.push("No knowledge retrieval hits could be assembled from requested surfaces.".into());
    } else {
        for h in hits {
            parts.push(format!("{} — {}", h.label, h.body));
        }
    }
    let joins: Vec<_> = links.iter().filter(|l| !l.is_hit()).collect();
    if !joins.is_empty() {
        parts.push(format!(
            "{} meaning-only join(s) recorded — correlation observed, causal claim not asserted.",
            joins.len()
        ));
    }
    if !gaps.is_empty() {
        parts.push(format!(
            "{} integration gap(s) preserved — Missing evidence ≠ invented certainty.",
            gaps.len()
        ));
    }
    parts.push(
        "Language constraint: integration ≠ authority; retrieval ≠ truth; relevance ≠ correctness; confidence ≠ permission."
            .into(),
    );
    parts.join(" ")
}

fn narrative_has_forbidden_patterns(text: &str) -> bool {
    let lower = text.to_lowercase();
    let forbidden = [
        "causes ",
        " caused ",
        "should execute",
        "recommend that you",
        "because the user",
        "because the system",
        "revision a caused",
        "safe to act",
        "create task",
        "mutate intent",
        "do x",
        "execute y",
        "approve z",
        "you should execute",
        "you should approve",
        "please execute",
        "requires_action",
        "should_execute",
        "leads_to_action",
        "authorises ",
        "authorizes ",
    ];
    forbidden.iter().any(|p| lower.contains(p))
}

pub fn validate_hit_kind(kind: &str) -> Result<(), KnowledgeIntegrationError> {
    let lower = kind.to_lowercase();
    if KnowledgeEvidenceLink::FORBIDDEN_KINDS
        .iter()
        .any(|f| *f == lower)
    {
        return Err(KnowledgeIntegrationError::InvalidLinkKind(kind.into()));
    }
    if KnowledgeEvidenceLink::HIT_KINDS
        .iter()
        .any(|a| *a == lower)
    {
        return Ok(());
    }
    Err(KnowledgeIntegrationError::InvalidLinkKind(kind.into()))
}

pub fn validate_link_kind(kind: &str) -> Result<(), KnowledgeIntegrationError> {
    let lower = kind.to_lowercase();
    if KnowledgeEvidenceLink::FORBIDDEN_KINDS
        .iter()
        .any(|f| *f == lower)
    {
        return Err(KnowledgeIntegrationError::InvalidLinkKind(kind.into()));
    }
    if KnowledgeEvidenceLink::ALLOWED_KINDS
        .iter()
        .any(|a| *a == lower)
    {
        return Ok(());
    }
    Err(KnowledgeIntegrationError::InvalidLinkKind(kind.into()))
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
    fn missing_upstreams_unavailable_with_gaps() {
        let snap = KnowledgeIntegrationResult::compose(
            "ws",
            "t0",
            KnowledgeRetrievalFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(
            snap.completeness,
            KnowledgeIntegrationCompleteness::Unavailable
        );
        assert!(snap.links.is_empty());
        assert_eq!(snap.gaps.len(), 7);
        assert!(snap.is_non_executing());
    }

    #[test]
    fn deterministic_composition_identical_inputs() {
        let empty_state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let left = KnowledgeIntegrationResult::compose_deterministic(
            "ws",
            "t1",
            KnowledgeRetrievalFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let right = KnowledgeIntegrationResult::compose_deterministic(
            "ws",
            "t1",
            KnowledgeRetrievalFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(left.integration_id, right.integration_id);
        assert_eq!(left.links, right.links);
        assert_eq!(left.gaps, right.gaps);
        assert_eq!(left.completeness, right.completeness);
    }

    #[test]
    fn hits_require_evidence() {
        let err = KnowledgeEvidenceLink::derive_hit(
            "hit_state",
            "orphan",
            "no evidence",
            "artefact",
            None,
            "test",
            vec![],
            vec![],
            vec![],
            10,
        )
        .unwrap_err();
        assert_eq!(err, KnowledgeIntegrationError::HitRequiresEvidence);

        let err = KnowledgeEvidenceLink::connect(
            "relates_to",
            "orphan join",
            "no evidence",
            "a",
            "b",
            "test",
            vec![],
            vec![],
            vec![],
        )
        .unwrap_err();
        assert_eq!(err, KnowledgeIntegrationError::HitRequiresEvidence);
    }

    #[test]
    fn forbidden_link_kinds_rejected() {
        for kind in KnowledgeEvidenceLink::FORBIDDEN_KINDS {
            let err = KnowledgeEvidenceLink::connect(
                *kind,
                "bad",
                "forbidden",
                "a",
                "b",
                "test",
                vec![KnowledgeIntegrationEvidenceRef::link("test", "ref", None)],
                vec![],
                vec![],
            )
            .unwrap_err();
            assert!(matches!(
                err,
                KnowledgeIntegrationError::InvalidLinkKind(_)
            ));
        }
    }

    #[test]
    fn allowed_link_kinds_accepted() {
        for kind in KnowledgeEvidenceLink::ALLOWED_KINDS {
            let link = KnowledgeEvidenceLink::connect(
                *kind,
                format!("{kind} link"),
                "meaning-only join",
                "a",
                "b",
                "test",
                vec![KnowledgeIntegrationEvidenceRef::link("test", "ref", None)],
                vec!["descriptive only".into()],
                vec![],
            )
            .unwrap();
            assert_eq!(link.kind, *kind);
            assert!(link.is_non_actionable());
        }
    }

    #[test]
    fn confidence_diagnostic_not_authority() {
        let snap = KnowledgeIntegrationResult::compose(
            "ws",
            "t0",
            KnowledgeRetrievalFrame::all_surfaces(),
            None,
            None,
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
            .any(|l| l.contains("diagnostic")));
        assert!(snap.confidence.limitations.iter().any(|l| {
            l.contains("not truth")
                || l.contains("≠ decision")
                || l.contains("not truth confidence")
                || l.contains("confidence ≠ permission")
        }));
    }

    #[test]
    fn history_separation_non_actionable() {
        let mut snap = KnowledgeIntegrationResult::compose(
            "ws",
            "t0",
            KnowledgeRetrievalFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        snap.mark_superseded("t1");
        let hist = KnowledgeIntegrationHistoryEntry::from_result(&snap).unwrap();
        assert!(hist.is_non_actionable());
        let proj = KnowledgeIntegrationProjection::assemble("ws", None, vec![hist], 1, "t2");
        assert!(proj.is_non_commandable());
    }
}
