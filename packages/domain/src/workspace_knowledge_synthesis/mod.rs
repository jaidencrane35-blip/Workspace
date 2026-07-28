//! Workspace Knowledge Synthesis — Programme III Batch 7.
//!
//! Synthesize understanding from evidence. Do not create reality.
//! Derived knowledge ≠ truth. Relationships ≠ causation. Confidence ≠ authority.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::policy_governance::PolicyGovernanceSnapshot;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum KnowledgeSynthesisError {
    #[error("invalid knowledge synthesis status: {0}")]
    InvalidStatus(String),

    #[error("invalid knowledge completeness: {0}")]
    InvalidCompleteness(String),

    #[error("invalid knowledge relationship kind: {0}")]
    InvalidRelationshipKind(String),

    #[error("knowledge synthesis artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("knowledge synthesis artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("knowledge concepts require at least one evidence reference")]
    ConceptRequiresEvidence,

    #[error("knowledge synthesis snapshot not found")]
    SnapshotNotFound,

    #[error("knowledge synthesis incomplete — required evidence unavailable")]
    SynthesisIncomplete,

    #[error("knowledge synthesis failed — corrupt or unusable evidence")]
    SynthesisFailed,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeSynthesisStatus {
    Current,
    Superseded,
    Archived,
}

impl KnowledgeSynthesisStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, KnowledgeSynthesisError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(KnowledgeSynthesisError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl KnowledgeCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, KnowledgeSynthesisError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(KnowledgeSynthesisError::InvalidCompleteness(other.into())),
        }
    }
}

/// Frame for knowledge synthesis — constrains reading; does not invent coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeSynthesisFrame {
    pub focus: Option<String>,
    pub include_state: bool,
    pub include_policy: bool,
    pub include_reconstruction: bool,
    pub include_temporal: bool,
    pub include_explanation: bool,
    pub include_contextual: bool,
    pub max_concepts: usize,
}

impl KnowledgeSynthesisFrame {
    pub const DEFAULT_MAX_CONCEPTS: usize = 32;

    pub fn all_surfaces() -> Self {
        Self {
            focus: None,
            include_state: true,
            include_policy: true,
            include_reconstruction: true,
            include_temporal: true,
            include_explanation: true,
            include_contextual: true,
            max_concepts: Self::DEFAULT_MAX_CONCEPTS,
        }
    }
}

/// Provenance anchor — statement → evidence → revision → origin domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeEvidenceReference {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeEvidenceReference {
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
pub struct KnowledgeGap {
    pub gap_id: String,
    pub surface: String,
    pub description: String,
    pub severity: String,
    pub evidence_refs: Vec<KnowledgeEvidenceReference>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "knowledge_gap:";

    pub fn record(
        surface: impl Into<String>,
        description: impl Into<String>,
        severity: impl Into<String>,
        evidence_refs: Vec<KnowledgeEvidenceReference>,
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

/// Evidence-backed derived concept — descriptive only; never invented truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeConcept {
    pub id: String,
    pub label: String,
    pub description: String,
    pub evidence_refs: Vec<KnowledgeEvidenceReference>,
    pub confidence: u8,
    pub uncertainty: Vec<String>,
    pub provenance: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeConcept {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "knowledge_concept:";

    pub fn derive(
        label: impl Into<String>,
        description: impl Into<String>,
        evidence_refs: Vec<KnowledgeEvidenceReference>,
        confidence: u8,
        uncertainty: Vec<String>,
        provenance: Vec<String>,
    ) -> Result<Self, KnowledgeSynthesisError> {
        if evidence_refs.is_empty() {
            return Err(KnowledgeSynthesisError::ConceptRequiresEvidence);
        }
        let label = label.into();
        let description = description.into();
        let digest = stable_digest(&format!("{label}|{description}"));
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, digest),
            label,
            description,
            evidence_refs,
            confidence: confidence.min(100),
            uncertainty,
            provenance,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_refs.iter().all(|r| r.is_non_actionable())
    }
}

/// Descriptive grouping of related concepts — never priority / required action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeCluster {
    pub cluster_id: String,
    pub kind: String,
    pub label: String,
    pub description: String,
    pub concept_ids: Vec<String>,
    pub evidence_refs: Vec<KnowledgeEvidenceReference>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeCluster {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "knowledge_cluster:";

    pub fn group(
        kind: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
        concept_ids: Vec<String>,
        evidence_refs: Vec<KnowledgeEvidenceReference>,
    ) -> Self {
        let kind = kind.into();
        let label = label.into();
        let description = description.into();
        let digest = stable_digest(&format!("{kind}|{label}|{description}"));
        Self {
            cluster_id: format!("{}{}", Self::ID_PREFIX, digest),
            kind,
            label,
            description,
            concept_ids,
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

/// Meaning-only connection — correlation permitted; causation / action forbidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeRelationship {
    pub relationship_id: String,
    pub kind: String,
    pub from_ref: String,
    pub to_ref: String,
    pub evidence_refs: Vec<KnowledgeEvidenceReference>,
    pub uncertainty: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeRelationship {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "knowledge_relationship:";

    pub const ALLOWED_KINDS: &'static [&'static str] = &[
        "relates_to",
        "overlaps",
        "associated_with",
        "observed_with",
        "shares_evidence",
    ];

    pub const FORBIDDEN_KINDS: &'static [&'static str] = &[
        "causes",
        "requires",
        "should_trigger",
        "leads_to_action",
        "requires_action",
        "should_execute",
        "approve",
        "deny",
        "dispatch",
    ];

    pub fn connect(
        kind: impl Into<String>,
        from_ref: impl Into<String>,
        to_ref: impl Into<String>,
        evidence_refs: Vec<KnowledgeEvidenceReference>,
        uncertainty: Vec<String>,
    ) -> Result<Self, KnowledgeSynthesisError> {
        let kind = kind.into();
        validate_relationship_kind(&kind)?;
        let from_ref = from_ref.into();
        let to_ref = to_ref.into();
        let digest = stable_digest(&format!("{kind}|{from_ref}|{to_ref}"));
        Ok(Self {
            relationship_id: format!("{}{}", Self::ID_PREFIX, digest),
            kind,
            from_ref,
            to_ref,
            evidence_refs,
            uncertainty,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_refs.iter().all(|r| r.is_non_actionable())
    }
}

/// Diagnostic knowledge confidence — ≠ truth confidence / decision authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeConfidence {
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

impl KnowledgeConfidence {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "knowledge_confidence:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Structured knowledge synthesis artefact — derived evidence, not memory truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceKnowledgeSynthesis {
    pub synthesis_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: KnowledgeSynthesisStatus,
    pub superseded_at: Option<String>,
    pub frame: KnowledgeSynthesisFrame,
    pub source_revisions: Vec<String>,
    pub concepts: Vec<KnowledgeConcept>,
    pub clusters: Vec<KnowledgeCluster>,
    pub relationships: Vec<KnowledgeRelationship>,
    pub gaps: Vec<KnowledgeGap>,
    pub confidence: KnowledgeConfidence,
    pub completeness: KnowledgeCompleteness,
    pub provenance_links: Vec<KnowledgeEvidenceReference>,
    pub summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceKnowledgeSynthesis {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "knowledge_synthesis:";

    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: KnowledgeSynthesisFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
    ) -> Result<Self, KnowledgeSynthesisError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let max_concepts = frame.max_concepts.max(1);

        let mut concepts = Vec::new();
        let mut clusters = Vec::new();
        let mut relationships = Vec::new();
        let mut gaps = Vec::new();
        let mut provenance_links = Vec::new();
        let mut source_revisions = Vec::new();
        let mut conflict_count = 0usize;
        let mut requested = 0usize;
        let mut available = 0usize;
        let mut surface_concepts: Vec<(String, String)> = Vec::new();

        if frame.include_state {
            requested += 1;
            match state.and_then(|s| s.current.as_ref()) {
                Some(envelope) => {
                    available += 1;
                    source_revisions.push(format!("state:{}", envelope.revision));
                    let refs = vec![KnowledgeEvidenceReference::link(
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
                    let concept = KnowledgeConcept::derive(
                        "Current state posture",
                        format!(
                            "Derived concept from unified state envelope {}. Freshness={}, completeness={}, consistency={}, sources={}. Descriptive synthesis only — not a health verdict or action.",
                            envelope.revision,
                            envelope.freshness.as_str(),
                            envelope.completeness.as_str(),
                            envelope.consistency.as_str(),
                            envelope.sources.len()
                        ),
                        refs,
                        70,
                        uncertainty,
                        vec![format!(
                            "statement→envelope:{}→revision:{}→domain:workspace_state_envelope",
                            envelope.state_id, envelope.revision
                        )],
                    )?;
                    surface_concepts.push(("operational".into(), concept.id.clone()));
                    concepts.push(concept);
                }
                None => {
                    gaps.push(KnowledgeGap::record(
                        "state",
                        "No durable workspace state envelope available — Missing evidence ≠ invented knowledge",
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
                    let refs = vec![KnowledgeEvidenceReference::link(
                        "policy_governance",
                        view.meta.evaluation_set_id.clone(),
                        Some(rev.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    let agg = view.meta.aggregate_result.as_str();
                    if agg == "unknown" {
                        gaps.push(KnowledgeGap::record(
                            "policy",
                            "Policy aggregate is Unknown — never assume Compliant or invent approval",
                            "high",
                            refs.clone(),
                        ));
                    }
                    let concept = KnowledgeConcept::derive(
                        "Governance posture",
                        format!(
                            "Derived concept from governance evaluation {}. Aggregate result={}. Policy explains authority; Gateway remains final.",
                            view.meta.evaluation_set_id, agg
                        ),
                        refs,
                        if agg == "unknown" { 30 } else { 65 },
                        if agg == "unknown" {
                            vec!["Unknown policy context must not become approval".into()]
                        } else {
                            vec![]
                        },
                        vec![format!(
                            "statement→policy:{}→revision:{}→domain:policy_governance",
                            view.meta.evaluation_set_id, rev
                        )],
                    )?;
                    surface_concepts.push(("capability".into(), concept.id.clone()));
                    concepts.push(concept);
                }
                None => {
                    gaps.push(KnowledgeGap::record(
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
                    let refs = vec![KnowledgeEvidenceReference::link(
                        "historical_reconstruction",
                        view.reconstruction_id.clone(),
                        Some(rev.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    if view.completeness.as_str() == "contradictory" {
                        conflict_count += 1;
                    }
                    for g in &view.gaps {
                        gaps.push(KnowledgeGap::record(
                            "reconstruction",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![KnowledgeEvidenceReference::link(
                                "evidence_gap",
                                g.gap_id.clone(),
                                None,
                            )],
                        ));
                    }
                    let concept = KnowledgeConcept::derive(
                        "Historical continuity",
                        format!(
                            "Derived concept from reconstruction {} from {:?} to {:?} — completeness={}, changes={}. Continuity framing only; observed sequence ≠ cause.",
                            view.reconstruction_id,
                            view.from_revision,
                            view.to_revision,
                            view.completeness.as_str(),
                            view.timeline.len()
                        ),
                        refs,
                        60,
                        view.gaps.iter().map(|g| g.description.clone()).collect(),
                        vec![format!(
                            "statement→reconstruction:{}→revision:{}→domain:historical_reconstruction",
                            view.reconstruction_id, rev
                        )],
                    )?;
                    surface_concepts.push(("historical".into(), concept.id.clone()));
                    concepts.push(concept);
                }
                None => {
                    gaps.push(KnowledgeGap::record(
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
                    let refs = vec![KnowledgeEvidenceReference::link(
                        "temporal_analysis",
                        view.analysis_id.clone(),
                        Some(rev.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    if view.completeness.as_str() == "contradictory" {
                        conflict_count += view.conflict_explanations.len().max(1);
                    }
                    let concept = KnowledgeConcept::derive(
                        "Temporal organisation",
                        format!(
                            "Derived concept from temporal analysis {} — completeness={}, chain_refs={}, conflicts={}. Temporal organisation only; not a forecast.",
                            view.analysis_id,
                            view.completeness.as_str(),
                            view.chain_summary.ordered_refs.len(),
                            view.conflict_explanations.len()
                        ),
                        refs,
                        55,
                        view.conflict_explanations
                            .iter()
                            .map(|c| c.description.clone())
                            .collect(),
                        vec![format!(
                            "statement→temporal:{}→revision:{}→domain:temporal_analysis",
                            view.analysis_id, rev
                        )],
                    )?;
                    surface_concepts.push(("historical".into(), concept.id.clone()));
                    concepts.push(concept);
                }
                None => {
                    gaps.push(KnowledgeGap::record(
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
                    let refs = vec![KnowledgeEvidenceReference::link(
                        "workspace_explanation",
                        package.explanation_id.clone(),
                        Some(package.explanation_id.clone()),
                    )];
                    provenance_links.extend(refs.clone());
                    if package.completeness.as_str() == "contradictory" {
                        conflict_count += package.conflicts.len().max(1);
                    }
                    for g in &package.gaps {
                        gaps.push(KnowledgeGap::record(
                            "explanation",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![KnowledgeEvidenceReference::link(
                                "explanation_gap",
                                g.gap_id.clone(),
                                None,
                            )],
                        ));
                    }
                    let summary_body = if package.narrative.is_empty() {
                        format!(
                            "Explanation package {} — completeness={}, sections={}, gaps={}, conflicts={}. Synthesis input only.",
                            package.explanation_id,
                            package.completeness.as_str(),
                            package.sections.len(),
                            package.gaps.len(),
                            package.conflicts.len()
                        )
                    } else {
                        format!(
                            "Explanation package summary: {}. Completeness={}, sections={}, gaps={}, conflicts={}. Descriptive synthesis only.",
                            package.narrative,
                            package.completeness.as_str(),
                            package.sections.len(),
                            package.gaps.len(),
                            package.conflicts.len()
                        )
                    };
                    let concept = KnowledgeConcept::derive(
                        "Explanation synthesis",
                        summary_body,
                        refs,
                        package.confidence.coverage,
                        package
                            .conflicts
                            .iter()
                            .map(|c| c.description.clone())
                            .collect(),
                        vec![format!(
                            "statement→explanation:{}→domain:workspace_explanation",
                            package.explanation_id
                        )],
                    )?;
                    surface_concepts.push(("situational".into(), concept.id.clone()));
                    concepts.push(concept);
                }
                None => {
                    gaps.push(KnowledgeGap::record(
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
                    let snap_refs = vec![KnowledgeEvidenceReference::link(
                        "contextual_understanding",
                        snap.understanding_id.clone(),
                        Some(snap.understanding_id.clone()),
                    )];
                    provenance_links.extend(snap_refs.clone());
                    if snap.completeness.as_str() == "contradictory" {
                        conflict_count += 1;
                    }
                    for g in &snap.gaps {
                        gaps.push(KnowledgeGap::record(
                            "contextual",
                            g.description.clone(),
                            g.severity.clone(),
                            vec![KnowledgeEvidenceReference::link(
                                "contextual_gap",
                                g.gap_id.clone(),
                                None,
                            )],
                        ));
                    }
                    if snap.themes.is_empty() {
                        let concept = KnowledgeConcept::derive(
                            "Contextual understanding",
                            format!(
                                "Derived concept from contextual understanding {} — completeness={}, themes=0, gaps={}. Situational framing referenced; not replaced.",
                                snap.understanding_id,
                                snap.completeness.as_str(),
                                snap.gaps.len()
                            ),
                            snap_refs,
                            50,
                            vec![],
                            vec![format!(
                                "statement→contextual:{}→domain:contextual_understanding",
                                snap.understanding_id
                            )],
                        )?;
                        surface_concepts.push(("situational".into(), concept.id.clone()));
                        concepts.push(concept);
                    } else {
                        for theme in &snap.themes {
                            let theme_refs: Vec<KnowledgeEvidenceReference> = if theme
                                .evidence_refs
                                .is_empty()
                            {
                                snap_refs.clone()
                            } else {
                                theme
                                    .evidence_refs
                                    .iter()
                                    .map(|r| {
                                        KnowledgeEvidenceReference::link(
                                            r.origin_domain.clone(),
                                            r.external_ref.clone(),
                                            r.source_revision.clone(),
                                        )
                                    })
                                    .collect()
                            };
                            let concept = KnowledgeConcept::derive(
                                theme.title.clone(),
                                theme.body.clone(),
                                theme_refs,
                                60,
                                vec![],
                                vec![format!(
                                    "statement→theme:{}→understanding:{}→domain:contextual_understanding",
                                    theme.theme_id, snap.understanding_id
                                )],
                            )?;
                            let cluster_kind = match theme.kind.as_str() {
                                "current_state" | "governance_posture" => {
                                    if theme.kind == "governance_posture" {
                                        "capability"
                                    } else {
                                        "operational"
                                    }
                                }
                                "historical_continuity" | "temporal_pressure" => "historical",
                                _ => "situational",
                            };
                            surface_concepts.push((cluster_kind.into(), concept.id.clone()));
                            concepts.push(concept);
                        }
                    }
                }
                None => {
                    gaps.push(KnowledgeGap::record(
                        "contextual",
                        "No durable contextual understanding available — Missing evidence ≠ invented knowledge",
                        "medium",
                        vec![],
                    ));
                }
            }
        }

        if let Some(focus) = &frame.focus {
            let focus_l = focus.to_lowercase();
            concepts.retain(|c| {
                c.label.to_lowercase().contains(&focus_l)
                    || c.description.to_lowercase().contains(&focus_l)
            });
            surface_concepts.retain(|(_, id)| concepts.iter().any(|c| &c.id == id));
        }

        concepts.sort_by(|a, b| a.id.cmp(&b.id));
        if concepts.len() > max_concepts {
            concepts.truncate(max_concepts);
            let retained: std::collections::HashSet<_> =
                concepts.iter().map(|c| c.id.clone()).collect();
            surface_concepts.retain(|(_, id)| retained.contains(id));
        }

        // Group concepts into descriptive clusters by surface kind.
        let mut by_kind: std::collections::BTreeMap<String, Vec<String>> =
            std::collections::BTreeMap::new();
        for (kind, id) in &surface_concepts {
            by_kind.entry(kind.clone()).or_default().push(id.clone());
        }
        for (kind, mut ids) in by_kind {
            ids.sort();
            ids.dedup();
            if ids.is_empty() {
                continue;
            }
            let cluster_refs: Vec<_> = concepts
                .iter()
                .filter(|c| ids.contains(&c.id))
                .flat_map(|c| c.evidence_refs.clone())
                .collect();
            let label = match kind.as_str() {
                "operational" => "Operational themes",
                "historical" => "Historical themes",
                "capability" => "Capability themes",
                _ => "Situational themes",
            };
            clusters.push(KnowledgeCluster::group(
                kind.clone(),
                label,
                format!(
                    "Descriptive cluster grouping {} concept(s) under {} themes. Does not imply priority, importance, or required action.",
                    ids.len(),
                    kind
                ),
                ids,
                cluster_refs,
            ));
        }

        // Meaning-only relationships when multiple concepts / shared evidence domains exist.
        if concepts.len() >= 2 {
            // shares_evidence for pairs that share an origin domain
            for i in 0..concepts.len() {
                for j in (i + 1)..concepts.len() {
                    let a = &concepts[i];
                    let b = &concepts[j];
                    let a_domains: std::collections::HashSet<_> =
                        a.evidence_refs.iter().map(|r| r.origin_domain.as_str()).collect();
                    let shared: Vec<_> = b
                        .evidence_refs
                        .iter()
                        .filter(|r| a_domains.contains(r.origin_domain.as_str()))
                        .cloned()
                        .collect();
                    if !shared.is_empty() {
                        relationships.push(KnowledgeRelationship::connect(
                            "shares_evidence",
                            a.id.clone(),
                            b.id.clone(),
                            shared,
                            vec![
                                "Shared evidence origin domain observed — correlation only, not a causal claim"
                                    .into(),
                            ],
                        )?);
                    }
                }
            }

            // Cross-surface association when multiple surfaces contributed concepts
            let distinct_kinds: std::collections::HashSet<_> =
                surface_concepts.iter().map(|(k, _)| k.as_str()).collect();
            if distinct_kinds.len() >= 2 || available >= 2 {
                let first = &concepts[0];
                let second = &concepts[1];
                let kind = if available >= 3 {
                    "associated_with"
                } else if distinct_kinds.len() >= 2 {
                    "relates_to"
                } else {
                    "observed_with"
                };
                let mut pair_refs = first.evidence_refs.clone();
                pair_refs.extend(second.evidence_refs.clone());
                relationships.push(KnowledgeRelationship::connect(
                    kind,
                    first.id.clone(),
                    second.id.clone(),
                    pair_refs,
                    vec![
                        "Multi-surface co-presence observed — relationship is meaning-only".into(),
                    ],
                )?);
                if concepts.len() >= 3 {
                    let third = &concepts[2];
                    let mut overlap_refs = first.evidence_refs.clone();
                    overlap_refs.extend(third.evidence_refs.clone());
                    relationships.push(KnowledgeRelationship::connect(
                        "overlaps",
                        first.id.clone(),
                        third.id.clone(),
                        overlap_refs,
                        vec!["Descriptive overlap across synthesised concepts — not a causal claim".into()],
                    )?);
                }
            }
        }

        // Deduplicate relationships by id, then sort all collections.
        {
            let mut seen = std::collections::HashSet::new();
            relationships.retain(|r| seen.insert(r.relationship_id.clone()));
        }
        clusters.sort_by(|a, b| a.cluster_id.cmp(&b.cluster_id));
        relationships.sort_by(|a, b| a.relationship_id.cmp(&b.relationship_id));
        gaps.sort_by(|a, b| a.gap_id.cmp(&b.gap_id));
        source_revisions.sort();
        source_revisions.dedup();

        let completeness =
            derive_completeness(requested, available, &gaps, conflict_count, &concepts);
        let confidence =
            build_confidence(requested, available, &gaps, conflict_count, &source_revisions);
        let limitations = vec![
            "Knowledge synthesis derives structured representations from evidence — it does not create reality"
                .into(),
            "Derived knowledge ≠ truth".into(),
            "Relationships ≠ causal claims".into(),
            "Confidence ≠ authority".into(),
            "Conflicts and gaps are preserved, not resolved".into(),
            "Prior synthesis cache ≠ current truth without revision binding".into(),
        ];
        let summary = build_summary(&concepts, &clusters, &relationships, &gaps, completeness);
        let narrative = build_narrative(
            &concepts,
            &clusters,
            &relationships,
            &gaps,
            &summary,
            completeness,
        );

        let synthesis = Self {
            synthesis_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id,
            generated_at,
            status: KnowledgeSynthesisStatus::Current,
            superseded_at: None,
            frame,
            source_revisions,
            concepts,
            clusters,
            relationships,
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
        synthesis.validate()?;
        Ok(synthesis)
    }

    pub fn compose_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: KnowledgeSynthesisFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
    ) -> Result<Self, KnowledgeSynthesisError> {
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
        )?;
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}|{}|{}",
            snap.workspace_id,
            snap.source_revisions.join(","),
            snap.concepts
                .iter()
                .map(|c| c.id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.clusters
                .iter()
                .map(|c| c.cluster_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.relationships
                .iter()
                .map(|r| r.relationship_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.gaps
                .iter()
                .map(|g| g.gap_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.completeness.as_str()
        ));
        snap.synthesis_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(snap)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = KnowledgeSynthesisStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.concepts.iter().all(|c| c.is_non_actionable())
            && self.clusters.iter().all(|c| c.is_non_actionable())
            && self.relationships.iter().all(|r| r.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.confidence.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), KnowledgeSynthesisError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(KnowledgeSynthesisError::AuthorityEffectMustBeNone);
        }
        if self.concepts.iter().any(|c| c.evidence_refs.is_empty()) {
            return Err(KnowledgeSynthesisError::ConceptRequiresEvidence);
        }
        for rel in &self.relationships {
            validate_relationship_kind(&rel.kind)?;
        }
        if narrative_has_forbidden_patterns(&self.narrative)
            || narrative_has_forbidden_patterns(&self.summary)
            || self
                .concepts
                .iter()
                .any(|c| narrative_has_forbidden_patterns(&c.description))
            || self
                .clusters
                .iter()
                .any(|c| narrative_has_forbidden_patterns(&c.description))
        {
            return Err(KnowledgeSynthesisError::MustNotBeActionable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeSynthesisHistoryEntry {
    pub synthesis_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub concept_count: usize,
    pub cluster_count: usize,
    pub relationship_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl KnowledgeSynthesisHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_synthesis(snap: &WorkspaceKnowledgeSynthesis) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            synthesis_id: snap.synthesis_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            concept_count: snap.concepts.len(),
            cluster_count: snap.clusters.len(),
            relationship_count: snap.relationships.len(),
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
pub struct KnowledgeSynthesisProjection {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<WorkspaceKnowledgeSynthesis>,
    pub history: Vec<KnowledgeSynthesisHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl KnowledgeSynthesisProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceKnowledgeSynthesis>,
        history: Vec<KnowledgeSynthesisHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> KnowledgeSynthesisSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        KnowledgeSynthesisSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            synthesis_id: self.current.as_ref().map(|c| c.synthesis_id.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            concept_count: self.current.as_ref().map(|c| c.concepts.len()).unwrap_or(0),
            cluster_count: self.current.as_ref().map(|c| c.clusters.len()).unwrap_or(0),
            relationship_count: self
                .current
                .as_ref()
                .map(|c| c.relationships.len())
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
pub struct KnowledgeSynthesisSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub synthesis_id: Option<String>,
    pub completeness: Option<String>,
    pub concept_count: usize,
    pub cluster_count: usize,
    pub relationship_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub history: Vec<KnowledgeSynthesisHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Explanation surface for ExplainKnowledgeSynthesis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeSynthesisExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub summary: Option<String>,
    pub concept_summaries: Vec<String>,
    pub cluster_summaries: Vec<String>,
    pub relationship_summaries: Vec<String>,
    pub gaps: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl KnowledgeSynthesisExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_synthesis(snap: &WorkspaceKnowledgeSynthesis) -> Self {
        Self {
            explanation_id: format!("knowledge_synthesis_explanation:{}", snap.synthesis_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            summary: Some(snap.summary.clone()),
            concept_summaries: snap
                .concepts
                .iter()
                .map(|c| format!("{}: {}", c.label, c.description))
                .collect(),
            cluster_summaries: snap
                .clusters
                .iter()
                .map(|c| format!("{}: {}", c.kind, c.label))
                .collect(),
            relationship_summaries: snap
                .relationships
                .iter()
                .map(|r| format!("{}: {} → {}", r.kind, r.from_ref, r.to_ref))
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
    gaps: &[KnowledgeGap],
    conflict_count: usize,
    _concepts: &[KnowledgeConcept],
) -> KnowledgeCompleteness {
    if requested == 0 || available == 0 {
        return KnowledgeCompleteness::Unavailable;
    }
    if conflict_count > 0 {
        return KnowledgeCompleteness::Contradictory;
    }
    if gaps.iter().any(|g| g.severity == "high") {
        return KnowledgeCompleteness::Unknown;
    }
    if available < requested || !gaps.is_empty() {
        return KnowledgeCompleteness::Partial;
    }
    KnowledgeCompleteness::Complete
}

fn build_confidence(
    requested: usize,
    available: usize,
    gaps: &[KnowledgeGap],
    conflict_count: usize,
    source_revisions: &[String],
) -> KnowledgeConfidence {
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
    KnowledgeConfidence {
        assessment_id: format!("{}{}", KnowledgeConfidence::ID_PREFIX, digest),
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
            "Knowledge confidence is diagnostic coverage only — not truth confidence".into(),
            "Diagnostic confidence ≠ decision authority or action approval".into(),
            "High coverage ≠ safe to act / decide / approve".into(),
        ],
        authority_effect: KnowledgeConfidence::AUTHORITY_EFFECT_NONE.into(),
        actionable: false,
    }
}

fn build_summary(
    concepts: &[KnowledgeConcept],
    clusters: &[KnowledgeCluster],
    relationships: &[KnowledgeRelationship],
    gaps: &[KnowledgeGap],
    completeness: KnowledgeCompleteness,
) -> String {
    format!(
        "Knowledge synthesis completeness={}; concepts={}; clusters={}; relationships={}; gaps={}. Derived representations are descriptive only.",
        completeness.as_str(),
        concepts.len(),
        clusters.len(),
        relationships.len(),
        gaps.len()
    )
}

fn build_narrative(
    concepts: &[KnowledgeConcept],
    clusters: &[KnowledgeCluster],
    relationships: &[KnowledgeRelationship],
    gaps: &[KnowledgeGap],
    summary: &str,
    completeness: KnowledgeCompleteness,
) -> String {
    let mut parts = Vec::new();
    parts.push(format!(
        "Workspace knowledge synthesis (completeness={}).",
        completeness.as_str()
    ));
    parts.push(summary.to_string());
    if concepts.is_empty() {
        parts.push("No knowledge concepts could be derived from requested surfaces.".into());
    } else {
        for c in concepts {
            parts.push(format!("{} — {}", c.label, c.description));
        }
    }
    if !clusters.is_empty() {
        parts.push(format!(
            "{} descriptive cluster(s) group related concepts without implying priority.",
            clusters.len()
        ));
    }
    if !relationships.is_empty() {
        parts.push(format!(
            "{} meaning-only relationship(s) recorded — correlation observed, causal claim not asserted.",
            relationships.len()
        ));
    }
    if !gaps.is_empty() {
        parts.push(format!(
            "{} knowledge gap(s) preserved — Missing evidence ≠ invented certainty.",
            gaps.len()
        ));
    }
    parts.push(
        "Language constraint: derived knowledge ≠ truth; relationships ≠ causal claims; confidence ≠ authority."
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
        "should_trigger",
    ];
    forbidden.iter().any(|p| lower.contains(p))
}

pub fn validate_relationship_kind(kind: &str) -> Result<(), KnowledgeSynthesisError> {
    let lower = kind.to_lowercase();
    if KnowledgeRelationship::FORBIDDEN_KINDS
        .iter()
        .any(|f| *f == lower)
    {
        return Err(KnowledgeSynthesisError::InvalidRelationshipKind(kind.into()));
    }
    if KnowledgeRelationship::ALLOWED_KINDS
        .iter()
        .any(|a| *a == lower)
    {
        return Ok(());
    }
    Err(KnowledgeSynthesisError::InvalidRelationshipKind(kind.into()))
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
        let snap = WorkspaceKnowledgeSynthesis::compose(
            "ws",
            "t0",
            KnowledgeSynthesisFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(snap.completeness, KnowledgeCompleteness::Unavailable);
        assert!(snap.concepts.is_empty());
        assert_eq!(snap.gaps.len(), 6);
        assert!(snap.is_non_executing());
    }

    #[test]
    fn deterministic_composition_identical_inputs() {
        let empty_state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let left = WorkspaceKnowledgeSynthesis::compose_deterministic(
            "ws",
            "t1",
            KnowledgeSynthesisFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let right = WorkspaceKnowledgeSynthesis::compose_deterministic(
            "ws",
            "t1",
            KnowledgeSynthesisFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(left.synthesis_id, right.synthesis_id);
        assert_eq!(left.concepts, right.concepts);
        assert_eq!(left.clusters, right.clusters);
        assert_eq!(left.relationships, right.relationships);
        assert_eq!(left.gaps, right.gaps);
    }

    #[test]
    fn concepts_require_evidence() {
        let err = KnowledgeConcept::derive(
            "orphan",
            "no evidence",
            vec![],
            10,
            vec![],
            vec![],
        )
        .unwrap_err();
        assert_eq!(err, KnowledgeSynthesisError::ConceptRequiresEvidence);
    }

    #[test]
    fn forbidden_relationship_kinds_rejected() {
        for kind in KnowledgeRelationship::FORBIDDEN_KINDS {
            let err = KnowledgeRelationship::connect(
                *kind,
                "a",
                "b",
                vec![KnowledgeEvidenceReference::link("test", "ref", None)],
                vec![],
            )
            .unwrap_err();
            assert!(matches!(
                err,
                KnowledgeSynthesisError::InvalidRelationshipKind(_)
            ));
        }
    }

    #[test]
    fn allowed_relationship_kinds_accepted() {
        for kind in KnowledgeRelationship::ALLOWED_KINDS {
            let rel = KnowledgeRelationship::connect(
                *kind,
                "a",
                "b",
                vec![KnowledgeEvidenceReference::link("test", "ref", None)],
                vec!["descriptive only".into()],
            )
            .unwrap();
            assert_eq!(rel.kind, *kind);
            assert!(rel.is_non_actionable());
        }
    }

    #[test]
    fn confidence_diagnostic_not_authority() {
        let snap = WorkspaceKnowledgeSynthesis::compose(
            "ws",
            "t0",
            KnowledgeSynthesisFrame::all_surfaces(),
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
        assert!(snap
            .confidence
            .limitations
            .iter()
            .any(|l| l.contains("not truth") || l.contains("≠ decision") || l.contains("not truth confidence")));
    }

    #[test]
    fn history_separation_non_actionable() {
        let mut snap = WorkspaceKnowledgeSynthesis::compose(
            "ws",
            "t0",
            KnowledgeSynthesisFrame::all_surfaces(),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        snap.mark_superseded("t1");
        let hist = KnowledgeSynthesisHistoryEntry::from_synthesis(&snap).unwrap();
        assert!(hist.is_non_actionable());
        let proj = KnowledgeSynthesisProjection::assemble("ws", None, vec![hist], 1, "t2");
        assert!(proj.is_non_commandable());
    }
}
