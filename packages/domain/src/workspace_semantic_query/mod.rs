//! Workspace Semantic Query Engine — Programme IV Batch 1.
//!
//! Retrieve meaning. Never create meaning.
//! retrieval ≠ synthesis; match ≠ recommendation; lineage ≠ inferred provenance.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::policy_governance::PolicyGovernanceSnapshot;
use crate::workspace_contextual_understanding::ContextualUnderstandingProjection;
use crate::workspace_cross_intelligence::CrossWorkspaceIntelligenceProjection;
use crate::workspace_decision_support::WorkspaceDecisionSupportProjection;
use crate::workspace_explanation::WorkspaceExplanationSnapshot;
use crate::workspace_historical_reconstruction::HistoricalReconstructionSnapshot;
use crate::workspace_insight_coordination::InsightCoordinationProjection;
use crate::workspace_intelligence_hub::WorkspaceIntelligenceHubProjection;
use crate::workspace_knowledge_integration::KnowledgeIntegrationProjection;
use crate::workspace_knowledge_synthesis::KnowledgeSynthesisProjection;
use crate::workspace_state_envelope::WorkspaceStateSnapshot;
use crate::workspace_temporal_intelligence::TemporalIntelligenceSnapshot;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SemanticQueryError {
    #[error("invalid semantic query status: {0}")]
    InvalidStatus(String),

    #[error("invalid semantic query completeness: {0}")]
    InvalidCompleteness(String),

    #[error("invalid semantic relevance: {0}")]
    InvalidRelevance(String),

    #[error("semantic query artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("semantic query artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("semantic matches require evidence references")]
    RequiresEvidence,

    #[error("semantic query text must not be empty")]
    EmptyQuery,

    #[error("semantic query snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticQueryStatus {
    Current,
    Superseded,
    Archived,
}

impl SemanticQueryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, SemanticQueryError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(SemanticQueryError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticQueryCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl SemanticQueryCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, SemanticQueryError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(SemanticQueryError::InvalidCompleteness(other.into())),
        }
    }
}

/// Diagnostic relevance only — never ranking-as-authority or recommendation weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRelevance {
    ExactToken,
    PartialToken,
    SourceLabel,
    DiagnosticOnly,
}

impl SemanticRelevance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactToken => "exact_token",
            Self::PartialToken => "partial_token",
            Self::SourceLabel => "source_label",
            Self::DiagnosticOnly => "diagnostic_only",
        }
    }

    pub fn parse(value: &str) -> Result<Self, SemanticQueryError> {
        match value {
            "exact_token" => Ok(Self::ExactToken),
            "partial_token" => Ok(Self::PartialToken),
            "source_label" => Ok(Self::SourceLabel),
            "diagnostic_only" => Ok(Self::DiagnosticOnly),
            other => Err(SemanticQueryError::InvalidRelevance(other.into())),
        }
    }
}

/// Scope / filter frame for which upstream surfaces participate in retrieval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticQueryScope {
    pub include_state: bool,
    pub include_policy: bool,
    pub include_reconstruction: bool,
    pub include_temporal: bool,
    pub include_explanation: bool,
    pub include_contextual: bool,
    pub include_knowledge_synthesis: bool,
    pub include_knowledge_integration: bool,
    pub include_insight: bool,
    pub include_cross_workspace: bool,
    pub include_decision_support: bool,
    pub include_intelligence_hub: bool,
}

impl SemanticQueryScope {
    pub fn all_surfaces() -> Self {
        Self {
            include_state: true,
            include_policy: true,
            include_reconstruction: true,
            include_temporal: true,
            include_explanation: true,
            include_contextual: true,
            include_knowledge_synthesis: true,
            include_knowledge_integration: true,
            include_insight: true,
            include_cross_workspace: true,
            include_decision_support: true,
            include_intelligence_hub: true,
        }
    }
}

/// Non-executable semantic query request — never becomes a command or plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticQuery {
    pub query: String,
    pub scope: SemanticQueryScope,
    pub filters: Vec<String>,
    pub provenance_requirements: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub executable: bool,
}

impl SemanticQuery {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn request(
        query: impl Into<String>,
        scope: SemanticQueryScope,
        filters: Vec<String>,
        provenance_requirements: Vec<String>,
    ) -> Result<Self, SemanticQueryError> {
        let query = query.into().trim().to_string();
        if query.is_empty() {
            return Err(SemanticQueryError::EmptyQuery);
        }
        Ok(Self {
            query,
            scope,
            filters,
            provenance_requirements,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            executable: false,
        })
    }

    pub fn is_non_executable(&self) -> bool {
        !self.actionable
            && !self.executable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticQueryEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl SemanticQueryEvidenceRef {
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

/// Matched source evidence — diagnostic relevance only; never ranking-as-authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticMatch {
    pub match_id: String,
    pub matched_source: String,
    pub relevance: String,
    pub evidence_ref: SemanticQueryEvidenceRef,
    pub originating_revision: Option<String>,
    pub matched_tokens: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl SemanticMatch {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "semantic_match:";

    pub fn record(
        matched_source: impl Into<String>,
        relevance: SemanticRelevance,
        evidence_ref: SemanticQueryEvidenceRef,
        originating_revision: Option<String>,
        matched_tokens: Vec<String>,
    ) -> Result<Self, SemanticQueryError> {
        if !evidence_ref.is_non_actionable() {
            return Err(SemanticQueryError::MustNotBeActionable);
        }
        let matched_source = matched_source.into();
        let digest = stable_digest(&format!(
            "{matched_source}|{}|{}",
            evidence_ref.external_ref,
            matched_tokens.join(",")
        ));
        Ok(Self {
            match_id: format!("{}{}", Self::ID_PREFIX, digest),
            matched_source,
            relevance: relevance.as_str().into(),
            evidence_ref,
            originating_revision,
            matched_tokens,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.evidence_ref.is_non_actionable()
    }
}

/// Retrieved evidence package — no interpretation, no recommendations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticQueryResult {
    pub result_id: String,
    pub matches: Vec<SemanticMatch>,
    pub match_count: usize,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl SemanticQueryResult {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "semantic_query_result:";

    pub fn from_matches(matches: Vec<SemanticMatch>) -> Self {
        let digest = stable_digest(&format!(
            "matches={}",
            matches
                .iter()
                .map(|m| m.match_id.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ));
        Self {
            result_id: format!("{}{}", Self::ID_PREFIX, digest),
            match_count: matches.len(),
            matches,
            notes: vec![
                "Semantic query result contains retrieved evidence only".into(),
                "match is not a decision or ranking authority".into(),
                "relevance is diagnostic only".into(),
            ],
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.matches.iter().all(|m| m.is_non_actionable())
    }
}

/// Unavailable / incomplete / missing lineage — never filled automatically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalGap {
    pub gap_id: String,
    pub unavailable_source: String,
    pub gap_kind: String,
    pub uncertainty_explanation: String,
    pub evidence_refs: Vec<SemanticQueryEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl RetrievalGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "retrieval_gap:";
    pub const KIND_UNAVAILABLE: &'static str = "unavailable_source";
    pub const KIND_INCOMPLETE: &'static str = "incomplete_retrieval";
    pub const KIND_MISSING_LINEAGE: &'static str = "missing_lineage";

    pub fn record(
        unavailable_source: impl Into<String>,
        gap_kind: impl Into<String>,
        uncertainty_explanation: impl Into<String>,
        evidence_refs: Vec<SemanticQueryEvidenceRef>,
    ) -> Self {
        let unavailable_source = unavailable_source.into();
        let gap_kind = gap_kind.into();
        let uncertainty_explanation = uncertainty_explanation.into();
        let digest = stable_digest(&format!("{unavailable_source}|{gap_kind}"));
        Self {
            gap_id: format!("{}{}", Self::ID_PREFIX, digest),
            unavailable_source,
            gap_kind,
            uncertainty_explanation,
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

/// Tracks exactly which upstream snapshots contributed — never inferred.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalLineage {
    pub lineage_id: String,
    pub contributing_snapshots: Vec<String>,
    pub revisions: Vec<String>,
    pub evidence_refs: Vec<SemanticQueryEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl RetrievalLineage {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "retrieval_lineage:";

    pub fn from_contributors(
        contributing_snapshots: Vec<String>,
        revisions: Vec<String>,
        evidence_refs: Vec<SemanticQueryEvidenceRef>,
    ) -> Self {
        let digest = stable_digest(&format!(
            "{}|{}",
            contributing_snapshots.join(","),
            revisions.join(",")
        ));
        Self {
            lineage_id: format!("{}{}", Self::ID_PREFIX, digest),
            contributing_snapshots,
            revisions,
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

/// Retrieval diagnostics — completeness and gap reporting only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalDiagnostics {
    pub diagnostics_id: String,
    pub sources_requested: usize,
    pub sources_available: usize,
    pub sources_unavailable: usize,
    pub match_count: usize,
    pub gap_count: usize,
    pub uncertainty: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl RetrievalDiagnostics {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "retrieval_diagnostics:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Durable semantic query retrieval artefact — observational evidence only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSemanticQuerySnapshot {
    pub query_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: SemanticQueryStatus,
    pub superseded_at: Option<String>,
    pub query: SemanticQuery,
    pub result: SemanticQueryResult,
    pub gaps: Vec<RetrievalGap>,
    pub lineage: RetrievalLineage,
    pub diagnostics: RetrievalDiagnostics,
    pub completeness: SemanticQueryCompleteness,
    pub provenance_links: Vec<SemanticQueryEvidenceRef>,
    pub source_revisions: Vec<String>,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceSemanticQuerySnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "semantic_query:";

    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        query: SemanticQuery,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        insight: Option<&InsightCoordinationProjection>,
        cross_workspace: Option<&CrossWorkspaceIntelligenceProjection>,
        decision_support: Option<&WorkspaceDecisionSupportProjection>,
        intelligence_hub: Option<&WorkspaceIntelligenceHubProjection>,
    ) -> Result<Self, SemanticQueryError> {
        if !query.is_non_executable() {
            return Err(SemanticQueryError::MustNotBeActionable);
        }
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let tokens = tokenize(&query.query);
        let mut acc = RetrievalAccumulator::default();

        if query.scope.include_state {
            retrieve_surface(
                &mut acc,
                &tokens,
                "workspace_state_envelope",
                state.and_then(|s| s.current.as_ref()).map(|e| {
                    SourceEvidence {
                        artefact_ref: e.state_id.clone(),
                        revision: Some(e.revision.clone()),
                        corpus: vec![
                            e.state_id.clone(),
                            "workspace_state_envelope".into(),
                            "state".into(),
                            e.consistency.as_str().into(),
                        ],
                    }
                }),
            )?;
        }
        if query.scope.include_policy {
            retrieve_surface(
                &mut acc,
                &tokens,
                "policy_governance",
                policy.and_then(|p| p.current.as_ref()).map(|v| {
                    let rev = v
                        .context_revision
                        .clone()
                        .unwrap_or_else(|| v.meta.evaluation_set_id.clone());
                    SourceEvidence {
                        artefact_ref: v.meta.evaluation_set_id.clone(),
                        revision: Some(rev),
                        corpus: vec![
                            v.meta.evaluation_set_id.clone(),
                            "policy_governance".into(),
                            "policy".into(),
                            "governance".into(),
                        ],
                    }
                }),
            )?;
        }
        if query.scope.include_reconstruction {
            retrieve_surface(
                &mut acc,
                &tokens,
                "historical_reconstruction",
                reconstruction.and_then(|r| r.current.as_ref()).map(|v| {
                    let rev = v
                        .to_revision
                        .clone()
                        .unwrap_or_else(|| v.reconstruction_id.clone());
                    SourceEvidence {
                        artefact_ref: v.reconstruction_id.clone(),
                        revision: Some(rev),
                        corpus: vec![
                            v.reconstruction_id.clone(),
                            "historical_reconstruction".into(),
                            "history".into(),
                            "reconstruction".into(),
                        ],
                    }
                }),
            )?;
        }
        if query.scope.include_temporal {
            retrieve_surface(
                &mut acc,
                &tokens,
                "temporal_intelligence",
                temporal.and_then(|t| t.current.as_ref()).map(|v| SourceEvidence {
                    artefact_ref: v.analysis_id.clone(),
                    revision: Some(v.analysis_id.clone()),
                    corpus: vec![
                        v.analysis_id.clone(),
                        "temporal_intelligence".into(),
                        "temporal".into(),
                        "time".into(),
                    ],
                }),
            )?;
        }
        if query.scope.include_explanation {
            retrieve_surface(
                &mut acc,
                &tokens,
                "workspace_explanation",
                explanation.and_then(|e| e.current.as_ref()).map(|p| SourceEvidence {
                    artefact_ref: p.explanation_id.clone(),
                    revision: Some(p.explanation_id.clone()),
                    corpus: vec![
                        p.explanation_id.clone(),
                        "workspace_explanation".into(),
                        "explanation".into(),
                    ],
                }),
            )?;
        }
        if query.scope.include_contextual {
            retrieve_surface(
                &mut acc,
                &tokens,
                "contextual_understanding",
                contextual.and_then(|c| c.current.as_ref()).map(|s| SourceEvidence {
                    artefact_ref: s.understanding_id.clone(),
                    revision: Some(s.understanding_id.clone()),
                    corpus: vec![
                        s.understanding_id.clone(),
                        "contextual_understanding".into(),
                        "context".into(),
                        "understanding".into(),
                    ],
                }),
            )?;
        }
        if query.scope.include_knowledge_synthesis {
            retrieve_surface(
                &mut acc,
                &tokens,
                "knowledge_synthesis",
                knowledge_synthesis
                    .and_then(|k| k.current.as_ref())
                    .map(|s| SourceEvidence {
                        artefact_ref: s.synthesis_id.clone(),
                        revision: Some(s.synthesis_id.clone()),
                        corpus: vec![
                            s.synthesis_id.clone(),
                            "knowledge_synthesis".into(),
                            "knowledge".into(),
                            "synthesis".into(),
                        ],
                    }),
            )?;
        }
        if query.scope.include_knowledge_integration {
            retrieve_surface(
                &mut acc,
                &tokens,
                "knowledge_integration",
                knowledge_integration
                    .and_then(|k| k.current.as_ref())
                    .map(|i| SourceEvidence {
                        artefact_ref: i.integration_id.clone(),
                        revision: Some(i.integration_id.clone()),
                        corpus: vec![
                            i.integration_id.clone(),
                            "knowledge_integration".into(),
                            "knowledge".into(),
                            "integration".into(),
                        ],
                    }),
            )?;
        }
        if query.scope.include_insight {
            retrieve_surface(
                &mut acc,
                &tokens,
                "insight_coordination",
                insight.and_then(|i| i.current.as_ref()).map(|c| SourceEvidence {
                    artefact_ref: c.coordination_id.clone(),
                    revision: Some(c.coordination_id.clone()),
                    corpus: vec![
                        c.coordination_id.clone(),
                        "insight_coordination".into(),
                        "insight".into(),
                        "coordination".into(),
                    ],
                }),
            )?;
        }
        if query.scope.include_cross_workspace {
            retrieve_surface(
                &mut acc,
                &tokens,
                "cross_workspace_intelligence",
                cross_workspace
                    .and_then(|c| c.current.as_ref())
                    .map(|i| SourceEvidence {
                        artefact_ref: i.intelligence_id.clone(),
                        revision: Some(i.intelligence_id.clone()),
                        corpus: vec![
                            i.intelligence_id.clone(),
                            "cross_workspace_intelligence".into(),
                            "cross_workspace".into(),
                            "intelligence".into(),
                        ],
                    }),
            )?;
        }
        if query.scope.include_decision_support {
            retrieve_surface(
                &mut acc,
                &tokens,
                "workspace_decision_support",
                decision_support
                    .and_then(|d| d.current.as_ref())
                    .map(|s| SourceEvidence {
                        artefact_ref: s.support_id.clone(),
                        revision: Some(s.support_id.clone()),
                        corpus: vec![
                            s.support_id.clone(),
                            "workspace_decision_support".into(),
                            "decision_support".into(),
                            "tradeoff".into(),
                        ],
                    }),
            )?;
        }
        if query.scope.include_intelligence_hub {
            retrieve_surface(
                &mut acc,
                &tokens,
                "workspace_intelligence_hub",
                intelligence_hub
                    .and_then(|h| h.current.as_ref())
                    .map(|hub| SourceEvidence {
                        artefact_ref: hub.hub_id.clone(),
                        revision: Some(hub.hub_id.clone()),
                        corpus: vec![
                            hub.hub_id.clone(),
                            "workspace_intelligence_hub".into(),
                            "intelligence_hub".into(),
                            "hub".into(),
                            "intelligence".into(),
                        ],
                    }),
            )?;
        }

        // Apply surface filters: if filters present, retain only matching sources in matches/gaps notes.
        if !query.filters.is_empty() {
            let filters_lower: Vec<String> = query
                .filters
                .iter()
                .map(|f| f.to_ascii_lowercase())
                .collect();
            acc.matches.retain(|m| {
                filters_lower.iter().any(|f| {
                    m.matched_source.to_ascii_lowercase().contains(f)
                        || m.evidence_ref
                            .origin_domain
                            .to_ascii_lowercase()
                            .contains(f)
                })
            });
        }

        // Provenance requirements: record gap when required provenance token absent from lineage.
        for req in &query.provenance_requirements {
            let req_l = req.to_ascii_lowercase();
            let satisfied = acc.lineage_revisions.iter().any(|r| r.to_ascii_lowercase().contains(&req_l))
                || acc
                    .lineage_evidence
                    .iter()
                    .any(|e| e.external_ref.to_ascii_lowercase().contains(&req_l));
            if !satisfied {
                acc.gaps.push(RetrievalGap::record(
                    req.clone(),
                    RetrievalGap::KIND_MISSING_LINEAGE,
                    format!(
                        "Provenance requirement '{req}' not satisfied by retrieved lineage — never invent lineage"
                    ),
                    vec![],
                ));
            }
        }

        let lineage = RetrievalLineage::from_contributors(
            acc.contributing_snapshots.clone(),
            acc.lineage_revisions.clone(),
            acc.lineage_evidence.clone(),
        );
        let result = SemanticQueryResult::from_matches(acc.matches);
        let completeness = derive_completeness(acc.requested, acc.available, &acc.gaps);
        let diagnostics = RetrievalDiagnostics {
            diagnostics_id: format!(
                "{}{}",
                RetrievalDiagnostics::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{}|{}|{}",
                    acc.requested,
                    acc.available,
                    result.match_count
                ))
            ),
            sources_requested: acc.requested,
            sources_available: acc.available,
            sources_unavailable: acc.requested.saturating_sub(acc.available),
            match_count: result.match_count,
            gap_count: acc.gaps.len(),
            uncertainty: vec![
                "Retrieval diagnostics are observational only".into(),
                "Unknown remains unknown".into(),
                "Unavailable remains unavailable".into(),
            ],
            limitations: vec![
                "retrieve meaning — never create meaning".into(),
                "match is not a decision or ranking authority".into(),
                "lineage ≠ inferred provenance".into(),
            ],
            authority_effect: RetrievalDiagnostics::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        };

        let narrative_summary = format!(
            "Semantic query retrieval: {} match(es), {} gap(s), completeness={}",
            result.match_count,
            acc.gaps.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Retrieved existing Programme II/III evidence for workspace {workspace_id} \
             against query '{}'. Available sources={}/{} . \
             This layer retrieves evidence only — it never creates knowledge or new meaning.",
            query.query, acc.available, acc.requested
        );
        let limitations = vec![
            "Workspace Semantic Query Engine retrieves existing evidence only".into(),
            "It never synthesises new truth or creates meaning".into(),
            "It never becomes the authority for any upstream intelligence layer".into(),
            "Missing evidence remains missing — never invent matches or lineage".into(),
        ];

        let mut snap = Self {
            query_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{workspace_id}|{generated_at}|{}|{}",
                    query.query,
                    acc.lineage_revisions.join(",")
                ))
            ),
            workspace_id,
            generated_at,
            status: SemanticQueryStatus::Current,
            superseded_at: None,
            query,
            result,
            gaps: acc.gaps,
            lineage,
            diagnostics,
            completeness,
            provenance_links: acc.provenance_links,
            source_revisions: acc.lineage_revisions,
            narrative_summary,
            narrative,
            limitations,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        snap.validate()?;
        Ok(snap)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compose_deterministic(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        query: SemanticQuery,
        state: Option<&WorkspaceStateSnapshot>,
        policy: Option<&PolicyGovernanceSnapshot>,
        reconstruction: Option<&HistoricalReconstructionSnapshot>,
        temporal: Option<&TemporalIntelligenceSnapshot>,
        explanation: Option<&WorkspaceExplanationSnapshot>,
        contextual: Option<&ContextualUnderstandingProjection>,
        knowledge_synthesis: Option<&KnowledgeSynthesisProjection>,
        knowledge_integration: Option<&KnowledgeIntegrationProjection>,
        insight: Option<&InsightCoordinationProjection>,
        cross_workspace: Option<&CrossWorkspaceIntelligenceProjection>,
        decision_support: Option<&WorkspaceDecisionSupportProjection>,
        intelligence_hub: Option<&WorkspaceIntelligenceHubProjection>,
    ) -> Result<Self, SemanticQueryError> {
        Self::compose(
            workspace_id,
            generated_at,
            query,
            state,
            policy,
            reconstruction,
            temporal,
            explanation,
            contextual,
            knowledge_synthesis,
            knowledge_integration,
            insight,
            cross_workspace,
            decision_support,
            intelligence_hub,
        )
    }

    pub fn mark_superseded(&mut self, superseded_at: impl Into<String>) {
        self.status = SemanticQueryStatus::Superseded;
        self.superseded_at = Some(superseded_at.into());
        self.terminal = true;
        self.actionable = false;
        self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
    }

    pub fn is_non_executing(&self) -> bool {
        !self.actionable
            && !self.terminal
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.query.is_non_executable()
            && self.result.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.lineage.is_non_actionable()
            && self.diagnostics.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), SemanticQueryError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(SemanticQueryError::AuthorityEffectMustBeNone);
        }
        if self.actionable {
            return Err(SemanticQueryError::MustNotBeActionable);
        }
        if !self.query.is_non_executable() {
            return Err(SemanticQueryError::MustNotBeActionable);
        }
        if !self.result.is_non_actionable() {
            return Err(SemanticQueryError::MustNotBeActionable);
        }
        for m in &self.result.matches {
            if m.evidence_ref.external_ref.is_empty() {
                return Err(SemanticQueryError::RequiresEvidence);
            }
        }
        if !self.lineage.is_non_actionable()
            || !self.diagnostics.is_non_actionable()
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || self.provenance_links.iter().any(|p| !p.is_non_actionable())
        {
            return Err(SemanticQueryError::MustNotBeActionable);
        }
        reject_forbidden_phrases(&self.narrative_summary)?;
        reject_forbidden_phrases(&self.narrative)?;
        for note in &self.result.notes {
            reject_forbidden_phrases(note)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticQueryHistoryEntry {
    pub query_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub match_count: usize,
    pub gap_count: usize,
    pub source_revision_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl SemanticQueryHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceSemanticQuerySnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            query_id: snap.query_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            match_count: snap.result.match_count,
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
            && (self.status == "superseded" || self.status == "archived")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSemanticQueryProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceSemanticQuerySnapshot>,
    pub history: Vec<SemanticQueryHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceSemanticQueryProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceSemanticQuerySnapshot>,
        history: Vec<SemanticQueryHistoryEntry>,
        history_count: usize,
        projected_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            current,
            history,
            history_count,
            projected_at: projected_at.into(),
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceSemanticQuerySummary {
        WorkspaceSemanticQuerySummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().to_string()),
            match_count: self
                .current
                .as_ref()
                .map(|c| c.result.match_count)
                .unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            narrative_summary: self
                .current
                .as_ref()
                .map(|c| c.narrative_summary.clone()),
            history: self.history.iter().take(history_limit).cloned().collect(),
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSemanticQuerySummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub completeness: Option<String>,
    pub match_count: usize,
    pub gap_count: usize,
    pub narrative_summary: Option<String>,
    pub history: Vec<SemanticQueryHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSemanticQueryExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub completeness: Option<String>,
    pub narrative_summary: Option<String>,
    pub match_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub lineage_summaries: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceSemanticQueryExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &WorkspaceSemanticQuerySnapshot) -> Self {
        Self {
            explanation_id: format!("semantic_query_explanation:{}", snap.query_id),
            workspace_id: snap.workspace_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            narrative_summary: Some(snap.narrative_summary.clone()),
            match_summaries: snap
                .result
                .matches
                .iter()
                .map(|m| {
                    format!(
                        "{} relevance={} tokens={}",
                        m.matched_source,
                        m.relevance,
                        m.matched_tokens.join(",")
                    )
                })
                .collect(),
            gap_summaries: snap
                .gaps
                .iter()
                .map(|g| format!("{} ({})", g.unavailable_source, g.gap_kind))
                .collect(),
            lineage_summaries: snap.lineage.contributing_snapshots.clone(),
            evidence_refs: snap
                .provenance_links
                .iter()
                .map(|p| p.external_ref.clone())
                .collect(),
            uncertainty: snap.diagnostics.uncertainty.clone(),
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

struct SourceEvidence {
    artefact_ref: String,
    revision: Option<String>,
    corpus: Vec<String>,
}

#[derive(Default)]
struct RetrievalAccumulator {
    requested: usize,
    available: usize,
    matches: Vec<SemanticMatch>,
    gaps: Vec<RetrievalGap>,
    contributing_snapshots: Vec<String>,
    lineage_revisions: Vec<String>,
    lineage_evidence: Vec<SemanticQueryEvidenceRef>,
    provenance_links: Vec<SemanticQueryEvidenceRef>,
}

fn retrieve_surface(
    acc: &mut RetrievalAccumulator,
    tokens: &[String],
    surface: &str,
    evidence: Option<SourceEvidence>,
) -> Result<(), SemanticQueryError> {
    acc.requested += 1;
    match evidence {
        Some(ev) => {
            acc.available += 1;
            let link = SemanticQueryEvidenceRef::link(
                surface,
                ev.artefact_ref.clone(),
                ev.revision.clone(),
            );
            acc.contributing_snapshots.push(format!("{surface}:{}", ev.artefact_ref));
            if let Some(rev) = &ev.revision {
                acc.lineage_revisions.push(rev.clone());
            }
            acc.lineage_evidence.push(link.clone());
            acc.provenance_links.push(link.clone());

            let corpus_lower: Vec<String> = ev
                .corpus
                .iter()
                .flat_map(|c| tokenize(c))
                .collect();
            let mut matched_tokens = Vec::new();
            for t in tokens {
                if corpus_lower.iter().any(|c| c == t) {
                    matched_tokens.push(t.clone());
                } else if corpus_lower.iter().any(|c| c.contains(t) || t.contains(c.as_str())) {
                    matched_tokens.push(t.clone());
                }
            }
            matched_tokens.sort();
            matched_tokens.dedup();

            if !matched_tokens.is_empty() {
                let relevance = if matched_tokens.len() == tokens.len() && !tokens.is_empty() {
                    SemanticRelevance::ExactToken
                } else if matched_tokens.iter().any(|t| corpus_lower.iter().any(|c| c == t)) {
                    SemanticRelevance::PartialToken
                } else {
                    SemanticRelevance::SourceLabel
                };
                acc.matches.push(SemanticMatch::record(
                    surface,
                    relevance,
                    link,
                    ev.revision,
                    matched_tokens,
                )?);
            }
        }
        None => {
            acc.gaps.push(RetrievalGap::record(
                surface,
                RetrievalGap::KIND_UNAVAILABLE,
                format!(
                    "Upstream source '{surface}' unavailable — never invent matches or lineage"
                ),
                vec![],
            ));
        }
    }
    Ok(())
}

fn derive_completeness(
    requested: usize,
    available: usize,
    gaps: &[RetrievalGap],
) -> SemanticQueryCompleteness {
    if requested == 0 {
        return SemanticQueryCompleteness::Unknown;
    }
    if available == 0 {
        return SemanticQueryCompleteness::Unavailable;
    }
    if available == requested && gaps.is_empty() {
        return SemanticQueryCompleteness::Complete;
    }
    if available < requested || !gaps.is_empty() {
        return SemanticQueryCompleteness::Partial;
    }
    SemanticQueryCompleteness::Unknown
}

fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_lowercase())
        .collect()
}

fn reject_forbidden_phrases(text: &str) -> Result<(), SemanticQueryError> {
    let lower = text.to_ascii_lowercase();
    const FORBIDDEN: &[&str] = &[
        "recommend",
        "should do",
        "best choice",
        "optimal",
        "execute now",
        "approve",
        "dispatch",
        "automate",
    ];
    for phrase in FORBIDDEN {
        if lower.contains(phrase) {
            return Err(SemanticQueryError::MustNotBeActionable);
        }
    }
    Ok(())
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

    fn empty_query() -> SemanticQuery {
        SemanticQuery::request(
            "knowledge state",
            SemanticQueryScope::all_surfaces(),
            vec![],
            vec![],
        )
        .unwrap()
    }

    #[test]
    fn unavailable_sources_produce_gaps_not_matches() {
        let snap = WorkspaceSemanticQuerySnapshot::compose(
            "ws",
            "t0",
            empty_query(),
            None,
            None,
            None,
            None,
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
        assert_eq!(snap.completeness, SemanticQueryCompleteness::Unavailable);
        assert_eq!(snap.gaps.len(), 12);
        assert!(snap.result.matches.is_empty());
        assert!(snap.is_non_executing());
        assert!(snap.lineage.contributing_snapshots.is_empty());
    }

    #[test]
    fn identical_snapshots_produce_identical_retrieval() {
        let q = empty_query();
        let left = WorkspaceSemanticQuerySnapshot::compose_deterministic(
            "ws",
            "t1",
            q.clone(),
            None,
            None,
            None,
            None,
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
        let right = WorkspaceSemanticQuerySnapshot::compose_deterministic(
            "ws",
            "t1",
            q,
            None,
            None,
            None,
            None,
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
        assert_eq!(left.query_id, right.query_id);
        assert_eq!(left.result.matches, right.result.matches);
        assert_eq!(left.gaps, right.gaps);
        assert_eq!(left.lineage, right.lineage);
    }

    #[test]
    fn history_non_actionable() {
        let mut snap = WorkspaceSemanticQuerySnapshot::compose(
            "ws",
            "t0",
            empty_query(),
            None,
            None,
            None,
            None,
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
        let entry = SemanticQueryHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj = WorkspaceSemanticQueryProjection::assemble("ws", None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn lineage_preserved_from_available_sources_only() {
        let state = WorkspaceStateSnapshot::assemble("ws", None, vec![], 0, "t0");
        // empty current — still unavailable for state envelope content
        let snap = WorkspaceSemanticQuerySnapshot::compose(
            "ws",
            "t0",
            SemanticQuery::request(
                "state",
                SemanticQueryScope {
                    include_state: true,
                    include_policy: false,
                    include_reconstruction: false,
                    include_temporal: false,
                    include_explanation: false,
                    include_contextual: false,
                    include_knowledge_synthesis: false,
                    include_knowledge_integration: false,
                    include_insight: false,
                    include_cross_workspace: false,
                    include_decision_support: false,
                    include_intelligence_hub: false,
                },
                vec![],
                vec![],
            )
            .unwrap(),
            Some(&state),
            None,
            None,
            None,
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
        // No current envelope → gap; lineage must not invent contributors
        assert!(snap.lineage.contributing_snapshots.is_empty());
        assert_eq!(snap.gaps.len(), 1);
        assert_eq!(snap.gaps[0].gap_kind, RetrievalGap::KIND_UNAVAILABLE);
    }

    #[test]
    fn missing_provenance_requirement_records_gap() {
        let snap = WorkspaceSemanticQuerySnapshot::compose(
            "ws",
            "t0",
            SemanticQuery::request(
                "knowledge",
                SemanticQueryScope {
                    include_state: false,
                    include_policy: false,
                    include_reconstruction: false,
                    include_temporal: false,
                    include_explanation: false,
                    include_contextual: false,
                    include_knowledge_synthesis: false,
                    include_knowledge_integration: false,
                    include_insight: false,
                    include_cross_workspace: false,
                    include_decision_support: false,
                    include_intelligence_hub: false,
                },
                vec![],
                vec!["required_revision_xyz".into()],
            )
            .unwrap(),
            None,
            None,
            None,
            None,
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
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == RetrievalGap::KIND_MISSING_LINEAGE));
    }

    #[test]
    fn query_must_be_non_executable() {
        let mut q = empty_query();
        q.executable = true;
        let err = WorkspaceSemanticQuerySnapshot::compose(
            "ws",
            "t0",
            q,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(err.is_err());
    }

    #[test]
    fn empty_query_rejected() {
        let err = SemanticQuery::request("  ", SemanticQueryScope::all_surfaces(), vec![], vec![]);
        assert_eq!(err, Err(SemanticQueryError::EmptyQuery));
    }
}
