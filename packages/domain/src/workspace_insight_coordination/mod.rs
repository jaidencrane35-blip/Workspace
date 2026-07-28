//! Workspace Insight Coordination — Programme III Batch 9.
//!
//! Coordinate understanding. Never create authority.
//! coordination ≠ authority; prioritisation ≠ recommendation;
//! intersection ≠ causation; confidence ≠ permission.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::policy_governance::PolicyGovernanceSnapshot;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_knowledge_integration::KnowledgeIntegrationProjection;
use crate::workspace_knowledge_synthesis::KnowledgeSynthesisProjection;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InsightCoordinationError {
    #[error("invalid insight coordination status: {0}")]
    InvalidStatus(String),

    #[error("invalid insight coordination completeness: {0}")]
    InvalidCompleteness(String),

    #[error("invalid insight intersection relationship: {0}")]
    InvalidRelationship(String),

    #[error("insight coordination artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("insight coordination artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("insight clusters and intersections require evidence references")]
    RequiresEvidence,

    #[error("insight coordination snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightCoordinationStatus {
    Current,
    Superseded,
    Archived,
}

impl InsightCoordinationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, InsightCoordinationError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(InsightCoordinationError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightCoordinationCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl InsightCoordinationCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, InsightCoordinationError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(InsightCoordinationError::InvalidCompleteness(other.into())),
        }
    }
}

/// Frame constraining which upstream surfaces participate in coordination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsightCoordinationFrame {
    pub focus: Option<String>,
    pub include_state: bool,
    pub include_policy: bool,
    pub include_reconstruction: bool,
    pub include_temporal: bool,
    pub include_explanation: bool,
    pub include_contextual: bool,
    pub include_knowledge_synthesis: bool,
    pub include_knowledge_integration: bool,
}

impl InsightCoordinationFrame {
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
            include_knowledge_integration: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsightEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl InsightEvidenceRef {
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

/// Related evidence theme — descriptive only; no causal / action fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsightCluster {
    pub cluster_id: String,
    pub title: String,
    pub body: String,
    pub related_sources: Vec<String>,
    pub evidence_references: Vec<InsightEvidenceRef>,
    pub contributing_domains: Vec<String>,
    pub confidence: u8,
    pub evidence_density: u8,
    pub completeness: String,
    pub attention_rank: u32,
    pub uncertainty: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl InsightCluster {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "insight_cluster:";

    pub fn group(
        title: impl Into<String>,
        body: impl Into<String>,
        related_sources: Vec<String>,
        evidence_references: Vec<InsightEvidenceRef>,
        contributing_domains: Vec<String>,
        confidence: u8,
        evidence_density: u8,
        completeness: impl Into<String>,
        attention_rank: u32,
        uncertainty: Vec<String>,
    ) -> Result<Self, InsightCoordinationError> {
        if evidence_references.is_empty() {
            return Err(InsightCoordinationError::RequiresEvidence);
        }
        let title = title.into();
        let body = body.into();
        let digest = stable_digest(&format!(
            "{title}|{}|{}",
            related_sources.join(","),
            contributing_domains.join(",")
        ));
        Ok(Self {
            cluster_id: format!("{}{}", Self::ID_PREFIX, digest),
            title,
            body,
            related_sources,
            evidence_references,
            contributing_domains,
            confidence: confidence.min(100),
            evidence_density: evidence_density.min(100),
            completeness: completeness.into(),
            attention_rank,
            uncertainty,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_references.iter().all(|r| r.is_non_actionable())
    }
}

/// Cross-domain overlap — meaning-only relationships.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsightIntersection {
    pub intersection_id: String,
    pub label: String,
    pub overlap_explanation: String,
    pub source_refs: Vec<InsightEvidenceRef>,
    pub shared_themes: Vec<String>,
    pub relationship_type: String,
    pub confidence: u8,
    pub provenance: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl InsightIntersection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "insight_intersection:";

    pub const ALLOWED_RELATIONSHIPS: &'static [&'static str] = &[
        "relates_to",
        "overlaps",
        "associated_with",
        "observed_with",
        "shares_evidence",
        "references",
        "derived_from",
        "supported_by",
    ];

    pub const FORBIDDEN_RELATIONSHIPS: &'static [&'static str] = &[
        "causes",
        "triggers",
        "requires",
        "requires_action",
        "should_execute",
        "authorises",
        "leads_to_action",
        "approve",
        "recommend_action",
    ];

    pub fn overlap(
        label: impl Into<String>,
        overlap_explanation: impl Into<String>,
        source_refs: Vec<InsightEvidenceRef>,
        shared_themes: Vec<String>,
        relationship_type: impl Into<String>,
        confidence: u8,
        provenance: Vec<String>,
    ) -> Result<Self, InsightCoordinationError> {
        if source_refs.is_empty() {
            return Err(InsightCoordinationError::RequiresEvidence);
        }
        let relationship_type = relationship_type.into();
        validate_relationship(&relationship_type)?;
        let label = label.into();
        let overlap_explanation = overlap_explanation.into();
        let digest = stable_digest(&format!(
            "{label}|{relationship_type}|{}",
            source_refs
                .iter()
                .map(|r| r.external_ref.clone())
                .collect::<Vec<_>>()
                .join(",")
        ));
        Ok(Self {
            intersection_id: format!("{}{}", Self::ID_PREFIX, digest),
            label,
            overlap_explanation,
            source_refs,
            shared_themes,
            relationship_type,
            confidence: confidence.min(100),
            provenance,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.source_refs.iter().all(|r| r.is_non_actionable())
    }
}

/// Diagnostic attention metadata — never recommendation / action selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsightAttentionSignal {
    pub signal_id: String,
    pub kind: String,
    pub description: String,
    pub magnitude: u8,
    pub related_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl InsightAttentionSignal {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "insight_attention:";

    pub const ALLOWED_KINDS: &'static [&'static str] = &[
        "evidence_concentration",
        "uncertainty_level",
        "missing_context",
        "conflict_density",
    ];

    pub fn signal(
        kind: impl Into<String>,
        description: impl Into<String>,
        magnitude: u8,
        related_refs: Vec<String>,
        uncertainty: Vec<String>,
    ) -> Result<Self, InsightCoordinationError> {
        let kind = kind.into();
        if !Self::ALLOWED_KINDS.iter().any(|k| *k == kind) {
            return Err(InsightCoordinationError::InvalidRelationship(kind));
        }
        let description = description.into();
        let digest = stable_digest(&format!("{kind}|{description}"));
        Ok(Self {
            signal_id: format!("{}{}", Self::ID_PREFIX, digest),
            kind,
            description,
            magnitude: magnitude.min(100),
            related_refs,
            uncertainty,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsightGap {
    pub gap_id: String,
    pub missing_evidence: String,
    pub affected_domains: Vec<String>,
    pub uncertainty_explanation: String,
    pub severity: String,
    pub evidence_refs: Vec<InsightEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl InsightGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "insight_gap:";

    pub fn record(
        missing_evidence: impl Into<String>,
        affected_domains: Vec<String>,
        uncertainty_explanation: impl Into<String>,
        severity: impl Into<String>,
        evidence_refs: Vec<InsightEvidenceRef>,
    ) -> Self {
        let missing_evidence = missing_evidence.into();
        let uncertainty_explanation = uncertainty_explanation.into();
        let digest = stable_digest(&format!(
            "{missing_evidence}|{}",
            affected_domains.join(",")
        ));
        Self {
            gap_id: format!("{}{}", Self::ID_PREFIX, digest),
            missing_evidence,
            affected_domains,
            uncertainty_explanation,
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

/// Diagnostic coordination health rollup — not a scorecard for action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationAssessment {
    pub assessment_id: String,
    pub coverage: u8,
    pub contradictions_detected: usize,
    pub unresolved_areas: usize,
    pub cluster_count: usize,
    pub intersection_count: usize,
    pub attention_signal_count: usize,
    pub available_surfaces: usize,
    pub requested_surfaces: usize,
    pub uncertainty: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CoordinationAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "coordination_assessment:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Derived coordination artefact — understanding only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsightCoordinationSnapshot {
    pub coordination_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: InsightCoordinationStatus,
    pub superseded_at: Option<String>,
    pub frame: InsightCoordinationFrame,
    pub source_revisions: Vec<String>,
    pub clusters: Vec<InsightCluster>,
    pub intersections: Vec<InsightIntersection>,
    pub attention_signals: Vec<InsightAttentionSignal>,
    pub gaps: Vec<InsightGap>,
    pub assessment: CoordinationAssessment,
    pub completeness: InsightCoordinationCompleteness,
    pub provenance_links: Vec<InsightEvidenceRef>,
    pub summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl InsightCoordinationSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "insight_coordination:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: InsightCoordinationFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
    ) -> Result<Self, InsightCoordinationError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();

        let mut clusters = Vec::new();
        let mut intersections = Vec::new();
        let mut attention_signals = Vec::new();
        let mut gaps = Vec::new();
        let mut provenance_links = Vec::new();
        let mut source_revisions = Vec::new();
        let mut available_surfaces: Vec<(String, String, InsightEvidenceRef)> = Vec::new();
        let mut requested = 0usize;
        let mut conflict_count = 0usize;

        let mut push_available =
            |domain: &str,
             artefact_id: String,
             revision: Option<String>,
             title: String,
             body: String,
             density: u8,
             rank: u32| {
                let rev = revision.clone().unwrap_or_else(|| artefact_id.clone());
                source_revisions.push(format!("{domain}:{rev}"));
                let evidence = InsightEvidenceRef::link(domain, artefact_id.clone(), Some(rev));
                provenance_links.push(evidence.clone());
                available_surfaces.push((domain.into(), artefact_id.clone(), evidence.clone()));
                let cluster = InsightCluster::group(
                    title,
                    body,
                    vec![artefact_id],
                    vec![evidence],
                    vec![domain.into()],
                    density.min(90),
                    density,
                    "partial",
                    rank,
                    vec![
                        "Coordination cluster is descriptive only — not truth or action".into(),
                    ],
                )?;
                clusters.push(cluster);
                Ok::<(), InsightCoordinationError>(())
            };

        if frame.include_state {
            requested += 1;
            match state.and_then(|s| s.current.as_ref()) {
                Some(envelope) => {
                    if envelope.consistency.as_str() == "contradictory" {
                        conflict_count += envelope.contradictions.len().max(1);
                    }
                    push_available(
                        "workspace_state_envelope",
                        envelope.state_id.clone(),
                        Some(envelope.revision.clone()),
                        "State evidence concentration".into(),
                        format!(
                            "Multiple evidence readers may observe unified state envelope {}. Freshness={}, completeness={}, consistency={}. Coordination notes presence — not factual authority.",
                            envelope.revision,
                            envelope.freshness.as_str(),
                            envelope.completeness.as_str(),
                            envelope.consistency.as_str()
                        ),
                        70,
                        10,
                    )?;
                }
                None => gaps.push(InsightGap::record(
                    "No durable workspace state envelope available",
                    vec!["workspace_state_envelope".into()],
                    "Missing source remains Missing — never invent state structure",
                    "high",
                    vec![],
                )),
            }
        }

        if frame.include_policy {
            requested += 1;
            match policy.and_then(|p| p.current.as_ref()) {
                Some(view) => {
                    let rev = view
                        .context_revision
                        .clone()
                        .unwrap_or_else(|| view.meta.evaluation_set_id.clone());
                    push_available(
                        "policy_governance",
                        view.meta.evaluation_set_id.clone(),
                        Some(rev),
                        "Policy evaluation presence".into(),
                        format!(
                            "Policy governance evaluation {} is available for coordination. Aggregate={}. Policy explains authority; Gateway remains final. Coordination ≠ permission.",
                            view.meta.evaluation_set_id,
                            view.meta.aggregate_result.as_str()
                        ),
                        65,
                        20,
                    )?;
                }
                None => gaps.push(InsightGap::record(
                    "No durable policy evaluation available",
                    vec!["policy_governance".into()],
                    "Unavailable policy evidence remains Unavailable",
                    "high",
                    vec![],
                )),
            }
        }

        if frame.include_reconstruction {
            requested += 1;
            match reconstruction.and_then(|r| r.current.as_ref()) {
                Some(view) => {
                    if view.completeness.as_str() == "contradictory" {
                        conflict_count += 1;
                    }
                    let rev = view
                        .to_revision
                        .clone()
                        .unwrap_or_else(|| view.reconstruction_id.clone());
                    push_available(
                        "historical_reconstruction",
                        view.reconstruction_id.clone(),
                        Some(rev),
                        "Historical reconstruction presence".into(),
                        format!(
                            "Historical reconstruction {} is available. Completeness={}. Observed sequence ≠ cause.",
                            view.reconstruction_id,
                            view.completeness.as_str()
                        ),
                        60,
                        30,
                    )?;
                }
                None => gaps.push(InsightGap::record(
                    "No durable historical reconstruction available",
                    vec!["historical_reconstruction".into()],
                    "Missing reconstruction remains Missing",
                    "medium",
                    vec![],
                )),
            }
        }

        if frame.include_temporal {
            requested += 1;
            match temporal.and_then(|t| t.current.as_ref()) {
                Some(view) => {
                    push_available(
                        "temporal_intelligence",
                        view.analysis_id.clone(),
                        Some(view.analysis_id.clone()),
                        "Temporal intelligence presence".into(),
                        format!(
                            "Temporal analysis {} is available for coordination. Temporal organisation ≠ forecast.",
                            view.analysis_id
                        ),
                        55,
                        40,
                    )?;
                }
                None => gaps.push(InsightGap::record(
                    "No durable temporal analysis available",
                    vec!["temporal_intelligence".into()],
                    "Missing temporal evidence remains Missing",
                    "medium",
                    vec![],
                )),
            }
        }

        if frame.include_explanation {
            requested += 1;
            match explanation.and_then(|e| e.current.as_ref()) {
                Some(pkg) => {
                    push_available(
                        "workspace_explanation",
                        pkg.explanation_id.clone(),
                        Some(pkg.explanation_id.clone()),
                        "Explanation layer presence".into(),
                        format!(
                            "Explanation package {} is available. Explanation ≠ authority to change reality.",
                            pkg.explanation_id
                        ),
                        55,
                        50,
                    )?;
                }
                None => gaps.push(InsightGap::record(
                    "No durable explanation package available",
                    vec!["workspace_explanation".into()],
                    "Missing explanation remains Missing",
                    "medium",
                    vec![],
                )),
            }
        }

        if frame.include_contextual {
            requested += 1;
            match contextual.and_then(|c| c.current.as_ref()) {
                Some(snap) => {
                    push_available(
                        "contextual_understanding",
                        snap.understanding_id.clone(),
                        Some(snap.understanding_id.clone()),
                        "Contextual understanding presence".into(),
                        format!(
                            "Contextual understanding {} is available. Situational themes for coordination only.",
                            snap.understanding_id
                        ),
                        60,
                        60,
                    )?;
                }
                None => gaps.push(InsightGap::record(
                    "No durable contextual understanding available",
                    vec!["contextual_understanding".into()],
                    "Missing contextual evidence remains Missing",
                    "medium",
                    vec![],
                )),
            }
        }

        if frame.include_knowledge_synthesis {
            requested += 1;
            match knowledge_synthesis.and_then(|k| k.current.as_ref()) {
                Some(synth) => {
                    push_available(
                        "knowledge_synthesis",
                        synth.synthesis_id.clone(),
                        Some(synth.synthesis_id.clone()),
                        "Knowledge synthesis presence".into(),
                        format!(
                            "Knowledge synthesis {} is available. Derived concepts ≠ truth; coordination does not rewrite synthesis.",
                            synth.synthesis_id
                        ),
                        60,
                        70,
                    )?;
                }
                None => gaps.push(InsightGap::record(
                    "No durable knowledge synthesis available",
                    vec!["knowledge_synthesis".into()],
                    "Missing synthesis remains Missing",
                    "medium",
                    vec![],
                )),
            }
        }

        if frame.include_knowledge_integration {
            requested += 1;
            match knowledge_integration.and_then(|k| k.current.as_ref()) {
                Some(integ) => {
                    push_available(
                        "knowledge_integration",
                        integ.integration_id.clone(),
                        Some(integ.integration_id.clone()),
                        "Knowledge integration presence".into(),
                        format!(
                            "Knowledge integration {} is available. Retrieval relevance ≠ factual authority; coordination does not mutate integration.",
                            integ.integration_id
                        ),
                        60,
                        80,
                    )?;
                }
                None => gaps.push(InsightGap::record(
                    "No durable knowledge integration available",
                    vec!["knowledge_integration".into()],
                    "Missing integration remains Missing",
                    "medium",
                    vec![],
                )),
            }
        }

        // Meaning-only intersections between available surfaces (stable pairwise order).
        for i in 0..available_surfaces.len() {
            for j in (i + 1)..available_surfaces.len() {
                let (d1, id1, ref1) = &available_surfaces[i];
                let (d2, id2, ref2) = &available_surfaces[j];
                let relationship = if (*d1 == "policy_governance"
                    && *d2 == "historical_reconstruction")
                    || (*d1 == "historical_reconstruction" && *d2 == "policy_governance")
                {
                    "observed_with"
                } else if (*d1 == "knowledge_synthesis" && *d2 == "contextual_understanding")
                    || (*d1 == "contextual_understanding" && *d2 == "knowledge_synthesis")
                {
                    "overlaps"
                } else if (*d1 == "knowledge_integration" && *d2 == "knowledge_synthesis")
                    || (*d1 == "knowledge_synthesis" && *d2 == "knowledge_integration")
                {
                    "derived_from"
                } else {
                    "relates_to"
                };
                let intersection = InsightIntersection::overlap(
                    format!("{d1} ∩ {d2}"),
                    format!(
                        "Evidence products from {d1} ({id1}) and {d2} ({id2}) are both present. Overlap is meaning-only — intersection ≠ causation."
                    ),
                    vec![ref1.clone(), ref2.clone()],
                    vec![d1.clone(), d2.clone()],
                    relationship,
                    50,
                    vec![format!(
                        "intersection→{d1}:{id1}+{d2}:{id2}→meaning_only:{relationship}"
                    )],
                )?;
                intersections.push(intersection);
            }
        }

        let available = available_surfaces.len();
        if available > 0 {
            attention_signals.push(InsightAttentionSignal::signal(
                "evidence_concentration",
                format!(
                    "Evidence concentration across {available} available Programme III surfaces (diagnostic density only — not an action queue)"
                ),
                ((available * 100) / requested.max(1)) as u8,
                available_surfaces
                    .iter()
                    .map(|(_, id, _)| id.clone())
                    .collect(),
                vec!["Attention metadata ≠ recommendation".into()],
            )?);
        }
        if !gaps.is_empty() {
            attention_signals.push(InsightAttentionSignal::signal(
                "missing_context",
                format!(
                    "{} unresolved understanding area(s) remain visible as gaps",
                    gaps.len()
                ),
                (gaps.len().min(10) * 10) as u8,
                gaps.iter().map(|g| g.gap_id.clone()).collect(),
                vec!["Missing remains Missing — never fabricate resolution".into()],
            )?);
        }
        if conflict_count > 0 {
            attention_signals.push(InsightAttentionSignal::signal(
                "conflict_density",
                format!(
                    "Conflict density signal: {conflict_count} contradiction indicator(s) preserved without resolution"
                ),
                (conflict_count.min(10) * 10) as u8,
                vec![],
                vec!["Conflicts remain conflicts — never invent a winner".into()],
            )?);
        }
        if available > 0 && available < requested {
            attention_signals.push(InsightAttentionSignal::signal(
                "uncertainty_level",
                format!(
                    "Partial coverage: {available}/{requested} requested surfaces available"
                ),
                (((requested - available) * 100) / requested.max(1)) as u8,
                vec![],
                vec!["Unknown remains unknown".into()],
            )?);
        }

        let completeness = derive_completeness(requested, available, &gaps, conflict_count);
        let coverage = if requested == 0 {
            0
        } else {
            ((available * 100) / requested) as u8
        };
        let assessment = CoordinationAssessment {
            assessment_id: format!(
                "{}{}",
                CoordinationAssessment::ID_PREFIX,
                stable_digest(&format!("{workspace_id}|{available}|{requested}|{}", gaps.len()))
            ),
            coverage,
            contradictions_detected: conflict_count,
            unresolved_areas: gaps.len(),
            cluster_count: clusters.len(),
            intersection_count: intersections.len(),
            attention_signal_count: attention_signals.len(),
            available_surfaces: available,
            requested_surfaces: requested,
            uncertainty: vec![
                "Coordination assessment is diagnostic only".into(),
                "prioritisation ≠ recommendation".into(),
            ],
            limitations: vec![
                "coordination ≠ authority".into(),
                "intersection ≠ causation".into(),
                "confidence ≠ permission".into(),
                "attention metadata ≠ action queue".into(),
            ],
            authority_effect: CoordinationAssessment::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        };

        let summary = format!(
            "Insight coordination: {} cluster(s), {} intersection(s), {} gap(s), completeness={}",
            clusters.len(),
            intersections.len(),
            gaps.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Coordinated understanding across Programme III evidence products for workspace {workspace_id}. \
             Available surfaces={available}/{requested}. Conflicts preserved={conflict_count}. \
             This layer coordinates understanding only — it does not create authority."
        );
        let limitations = vec![
            "Insight Coordination coordinates understanding only".into(),
            "It does not make recommendations, create decisions, or execute actions".into(),
            "It does not replace Policy, Knowledge Synthesis, Cognitive Model, or Memory".into(),
            "Diagnostic attention metadata is not an action queue".into(),
        ];

        let mut result = Self {
            coordination_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}",
                    source_revisions.join(",")
                ))
            ),
            workspace_id,
            generated_at,
            status: InsightCoordinationStatus::Current,
            superseded_at: None,
            frame,
            source_revisions,
            clusters,
            intersections,
            attention_signals,
            gaps,
            assessment,
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

    #[allow(clippy::too_many_arguments)]
    pub fn compose_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        frame: InsightCoordinationFrame,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
    ) -> Result<Self, InsightCoordinationError> {
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
            knowledge_integration,
        )?;
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}|{}",
            snap.workspace_id,
            snap.source_revisions.join(","),
            snap.clusters
                .iter()
                .map(|c| c.cluster_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.intersections
                .iter()
                .map(|i| i.intersection_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.gaps
                .iter()
                .map(|g| g.gap_id.clone())
                .collect::<Vec<_>>()
                .join(","),
            snap.completeness.as_str()
        ));
        snap.coordination_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(snap)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = InsightCoordinationStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.clusters.iter().all(|c| c.is_non_actionable())
            && self.intersections.iter().all(|i| i.is_non_actionable())
            && self.attention_signals.iter().all(|s| s.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.assessment.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), InsightCoordinationError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(InsightCoordinationError::AuthorityEffectMustBeNone);
        }
        if self
            .clusters
            .iter()
            .any(|c| c.evidence_references.is_empty() || c.actionable)
        {
            return Err(InsightCoordinationError::RequiresEvidence);
        }
        if self
            .intersections
            .iter()
            .any(|i| i.source_refs.is_empty() || i.actionable)
        {
            return Err(InsightCoordinationError::RequiresEvidence);
        }
        for i in &self.intersections {
            validate_relationship(&i.relationship_type)?;
        }
        if self.attention_signals.iter().any(|s| s.actionable) {
            return Err(InsightCoordinationError::MustNotBeActionable);
        }
        if self.gaps.iter().any(|g| g.actionable) || self.assessment.actionable {
            return Err(InsightCoordinationError::MustNotBeActionable);
        }
        let forbidden_phrases = [
            "do this",
            "execute this",
            "approve this",
            "recommend that you",
            "should_execute",
            "best action",
            "definitely true",
        ];
        let blob = format!(
            "{} {} {} {}",
            self.summary,
            self.narrative,
            self.clusters
                .iter()
                .map(|c| format!("{} {}", c.title, c.body))
                .collect::<Vec<_>>()
                .join(" "),
            self.attention_signals
                .iter()
                .map(|s| s.description.clone())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .to_lowercase();
        for phrase in forbidden_phrases {
            if blob.contains(phrase) {
                return Err(InsightCoordinationError::MustNotBeActionable);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsightCoordinationHistoryEntry {
    pub coordination_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub cluster_count: usize,
    pub intersection_count: usize,
    pub attention_signal_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl InsightCoordinationHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &InsightCoordinationSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            coordination_id: snap.coordination_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            cluster_count: snap.clusters.len(),
            intersection_count: snap.intersections.len(),
            attention_signal_count: snap.attention_signals.len(),
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
pub struct InsightCoordinationProjection {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<InsightCoordinationSnapshot>,
    pub history: Vec<InsightCoordinationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl InsightCoordinationProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<InsightCoordinationSnapshot>,
        history: Vec<InsightCoordinationHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> InsightCoordinationSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        InsightCoordinationSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            coordination_id: self.current.as_ref().map(|c| c.coordination_id.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            cluster_count: self.current.as_ref().map(|c| c.clusters.len()).unwrap_or(0),
            intersection_count: self
                .current
                .as_ref()
                .map(|c| c.intersections.len())
                .unwrap_or(0),
            attention_signal_count: self
                .current
                .as_ref()
                .map(|c| c.attention_signals.len())
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
pub struct InsightCoordinationSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub coordination_id: Option<String>,
    pub completeness: Option<String>,
    pub cluster_count: usize,
    pub intersection_count: usize,
    pub attention_signal_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub history: Vec<InsightCoordinationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsightCoordinationExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub summary: Option<String>,
    pub cluster_summaries: Vec<String>,
    pub intersection_summaries: Vec<String>,
    pub attention_summaries: Vec<String>,
    pub gaps: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl InsightCoordinationExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &InsightCoordinationSnapshot) -> Self {
        Self {
            explanation_id: format!("insight_coordination_explanation:{}", snap.coordination_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            summary: Some(snap.summary.clone()),
            cluster_summaries: snap
                .clusters
                .iter()
                .map(|c| {
                    format!(
                        "{} [{}] density={} confidence={}",
                        c.title, c.cluster_id, c.evidence_density, c.confidence
                    )
                })
                .collect(),
            intersection_summaries: snap
                .intersections
                .iter()
                .map(|i| {
                    format!(
                        "{} [{}] relationship={}",
                        i.label, i.intersection_id, i.relationship_type
                    )
                })
                .collect(),
            attention_summaries: snap
                .attention_signals
                .iter()
                .map(|s| format!("{} [{}] magnitude={}", s.kind, s.signal_id, s.magnitude))
                .collect(),
            gaps: snap
                .gaps
                .iter()
                .map(|g| g.missing_evidence.clone())
                .collect(),
            evidence_refs: snap
                .provenance_links
                .iter()
                .map(|p| p.external_ref.clone())
                .collect(),
            uncertainty: snap.assessment.uncertainty.clone(),
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
    gaps: &[InsightGap],
    conflict_count: usize,
) -> InsightCoordinationCompleteness {
    if requested == 0 || available == 0 {
        return InsightCoordinationCompleteness::Unavailable;
    }
    if conflict_count > 0 {
        return InsightCoordinationCompleteness::Contradictory;
    }
    if available == requested && gaps.is_empty() {
        return InsightCoordinationCompleteness::Complete;
    }
    if available < requested || !gaps.is_empty() {
        return InsightCoordinationCompleteness::Partial;
    }
    InsightCoordinationCompleteness::Unknown
}

pub fn validate_relationship(kind: &str) -> Result<(), InsightCoordinationError> {
    let lower = kind.to_lowercase();
    if InsightIntersection::FORBIDDEN_RELATIONSHIPS
        .iter()
        .any(|f| *f == lower)
    {
        return Err(InsightCoordinationError::InvalidRelationship(kind.into()));
    }
    if InsightIntersection::ALLOWED_RELATIONSHIPS
        .iter()
        .any(|a| *a == lower)
    {
        return Ok(());
    }
    Err(InsightCoordinationError::InvalidRelationship(kind.into()))
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
        let snap = InsightCoordinationSnapshot::compose(
            "ws",
            "t0",
            InsightCoordinationFrame::all_surfaces(),
            None,
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
            InsightCoordinationCompleteness::Unavailable
        );
        assert!(snap.clusters.is_empty());
        assert!(snap.intersections.is_empty());
        assert_eq!(snap.gaps.len(), 8);
        assert!(snap.is_non_executing());
    }

    #[test]
    fn deterministic_composition_identical_inputs() {
        let empty_state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        let left = InsightCoordinationSnapshot::compose_deterministic(
            "ws",
            "t1",
            InsightCoordinationFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let right = InsightCoordinationSnapshot::compose_deterministic(
            "ws",
            "t1",
            InsightCoordinationFrame::all_surfaces(),
            Some(&empty_state),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(left.coordination_id, right.coordination_id);
        assert_eq!(left.clusters, right.clusters);
        assert_eq!(left.gaps, right.gaps);
        assert_eq!(left.completeness, right.completeness);
    }

    #[test]
    fn clusters_require_evidence() {
        let err = InsightCluster::group(
            "orphan",
            "no evidence",
            vec![],
            vec![],
            vec!["test".into()],
            10,
            10,
            "partial",
            1,
            vec![],
        )
        .unwrap_err();
        assert_eq!(err, InsightCoordinationError::RequiresEvidence);
    }

    #[test]
    fn forbidden_relationships_rejected() {
        for kind in InsightIntersection::FORBIDDEN_RELATIONSHIPS {
            let err = InsightIntersection::overlap(
                "bad",
                "forbidden",
                vec![InsightEvidenceRef::link("test", "ref", None)],
                vec![],
                *kind,
                10,
                vec![],
            )
            .unwrap_err();
            assert_eq!(
                err,
                InsightCoordinationError::InvalidRelationship((*kind).into())
            );
        }
    }

    #[test]
    fn allowed_relationships_accepted() {
        for kind in InsightIntersection::ALLOWED_RELATIONSHIPS {
            InsightIntersection::overlap(
                "ok",
                "meaning only",
                vec![InsightEvidenceRef::link("test", "ref", None)],
                vec!["theme".into()],
                *kind,
                40,
                vec!["lineage".into()],
            )
            .unwrap();
        }
    }

    #[test]
    fn attention_signals_diagnostic_not_authority() {
        let signal = InsightAttentionSignal::signal(
            "evidence_concentration",
            "density only",
            50,
            vec![],
            vec![],
        )
        .unwrap();
        assert!(signal.is_non_actionable());
        assert_eq!(signal.authority_effect, "none");
        assert!(!signal.actionable);
    }

    #[test]
    fn history_separation_non_actionable() {
        let mut snap = InsightCoordinationSnapshot::compose(
            "ws",
            "t0",
            InsightCoordinationFrame::all_surfaces(),
            None,
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
        let entry = InsightCoordinationHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        assert!(!entry.actionable);
        assert_eq!(entry.authority_effect, "none");
        let proj = InsightCoordinationProjection::assemble(
            "ws",
            None,
            vec![entry.clone()],
            1,
            "t2",
        );
        assert!(proj.is_non_commandable());
        let summary = proj.summary(10);
        assert_eq!(summary.history_count, 1);
        assert_eq!(summary.history.len(), 1);
    }
}
