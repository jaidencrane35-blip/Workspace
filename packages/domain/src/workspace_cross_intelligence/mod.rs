//! Cross-Workspace Intelligence — Programme III Batch 10.
//!
//! Aggregate understanding. Never centralise authority.
//! aggregation ≠ authority; frequency ≠ priority-as-action;
//! common theme ≠ shared truth; confidence ≠ permission.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;

/// Persistence / projection scope — not a real workspace SoT.
pub const CROSS_WORKSPACE_SCOPE: &str = "cross_workspace";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CrossWorkspaceIntelligenceError {
    #[error("invalid cross-workspace intelligence status: {0}")]
    InvalidStatus(String),

    #[error("invalid cross-workspace intelligence completeness: {0}")]
    InvalidCompleteness(String),

    #[error("cross-workspace artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("cross-workspace artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("cross-workspace patterns require evidence references")]
    RequiresEvidence,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossWorkspaceIntelligenceStatus {
    Current,
    Superseded,
    Archived,
}

impl CrossWorkspaceIntelligenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, CrossWorkspaceIntelligenceError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(CrossWorkspaceIntelligenceError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossWorkspaceCompleteness {
    Complete,
    Partial,
    Unknown,
    Contradictory,
    Unavailable,
}

impl CrossWorkspaceCompleteness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Unknown => "unknown",
            Self::Contradictory => "contradictory",
            Self::Unavailable => "unavailable",
        }
    }

    pub fn parse(value: &str) -> Result<Self, CrossWorkspaceIntelligenceError> {
        match value {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unknown" => Ok(Self::Unknown),
            "contradictory" => Ok(Self::Contradictory),
            "unavailable" => Ok(Self::Unavailable),
            other => Err(CrossWorkspaceIntelligenceError::InvalidCompleteness(
                other.into(),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspaceEvidenceRef {
    pub external_ref: String,
    pub origin_domain: String,
    pub workspace_id: Option<String>,
    pub source_revision: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CrossWorkspaceEvidenceRef {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn link(
        origin_domain: impl Into<String>,
        external_ref: impl Into<String>,
        workspace_id: Option<String>,
        source_revision: Option<String>,
    ) -> Self {
        Self {
            external_ref: external_ref.into(),
            origin_domain: origin_domain.into(),
            workspace_id,
            source_revision,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Light per-workspace contribution gathered via load_snapshot only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceContribution {
    pub workspace_id: String,
    pub workspace_name: String,
    pub available: bool,
    pub surfaces_present: Vec<String>,
    pub surfaces_missing: Vec<String>,
    pub theme_labels: Vec<String>,
    pub risk_labels: Vec<String>,
    pub constraint_labels: Vec<String>,
    pub pattern_labels: Vec<String>,
    pub gap_count: usize,
    pub conflict_indicated: bool,
    pub evidence_refs: Vec<CrossWorkspaceEvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspacePattern {
    pub pattern_id: String,
    pub title: String,
    pub body: String,
    pub participating_workspaces: Vec<String>,
    pub evidence_references: Vec<CrossWorkspaceEvidenceRef>,
    pub confidence: u8,
    pub occurrence_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CrossWorkspacePattern {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cross_workspace_pattern:";

    pub fn aggregate(
        title: impl Into<String>,
        body: impl Into<String>,
        participating_workspaces: Vec<String>,
        evidence_references: Vec<CrossWorkspaceEvidenceRef>,
        confidence: u8,
        occurrence_count: usize,
    ) -> Result<Self, CrossWorkspaceIntelligenceError> {
        if evidence_references.is_empty() || participating_workspaces.is_empty() {
            return Err(CrossWorkspaceIntelligenceError::RequiresEvidence);
        }
        let title = title.into();
        let digest = stable_digest(&format!(
            "{title}|{}",
            participating_workspaces.join(",")
        ));
        Ok(Self {
            pattern_id: format!("{}{}", Self::ID_PREFIX, digest),
            title,
            body: body.into(),
            participating_workspaces,
            evidence_references,
            confidence: confidence.min(100),
            occurrence_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self
                .evidence_references
                .iter()
                .all(|r| r.is_non_actionable())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspaceTheme {
    pub theme_id: String,
    pub label: String,
    pub description: String,
    pub supporting_evidence: Vec<CrossWorkspaceEvidenceRef>,
    pub contributing_workspaces: Vec<String>,
    pub confidence: u8,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CrossWorkspaceTheme {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cross_workspace_theme:";

    pub fn aggregate(
        label: impl Into<String>,
        description: impl Into<String>,
        supporting_evidence: Vec<CrossWorkspaceEvidenceRef>,
        contributing_workspaces: Vec<String>,
        confidence: u8,
    ) -> Result<Self, CrossWorkspaceIntelligenceError> {
        if supporting_evidence.is_empty() || contributing_workspaces.is_empty() {
            return Err(CrossWorkspaceIntelligenceError::RequiresEvidence);
        }
        let label = label.into();
        let digest = stable_digest(&format!("{label}|{}", contributing_workspaces.join(",")));
        Ok(Self {
            theme_id: format!("{}{}", Self::ID_PREFIX, digest),
            label,
            description: description.into(),
            supporting_evidence,
            contributing_workspaces,
            confidence: confidence.min(100),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.supporting_evidence.iter().all(|r| r.is_non_actionable())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspaceRiskSignal {
    pub signal_id: String,
    pub repeated_risk: String,
    pub description: String,
    pub supporting_lineage: Vec<CrossWorkspaceEvidenceRef>,
    pub frequency: usize,
    pub participating_workspaces: Vec<String>,
    pub confidence: u8,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CrossWorkspaceRiskSignal {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cross_workspace_risk:";

    pub fn observe(
        repeated_risk: impl Into<String>,
        description: impl Into<String>,
        supporting_lineage: Vec<CrossWorkspaceEvidenceRef>,
        frequency: usize,
        participating_workspaces: Vec<String>,
        confidence: u8,
    ) -> Result<Self, CrossWorkspaceIntelligenceError> {
        if supporting_lineage.is_empty() {
            return Err(CrossWorkspaceIntelligenceError::RequiresEvidence);
        }
        let repeated_risk = repeated_risk.into();
        let digest = stable_digest(&format!(
            "{repeated_risk}|{}",
            participating_workspaces.join(",")
        ));
        Ok(Self {
            signal_id: format!("{}{}", Self::ID_PREFIX, digest),
            repeated_risk,
            description: description.into(),
            supporting_lineage,
            frequency,
            participating_workspaces,
            confidence: confidence.min(100),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.supporting_lineage.iter().all(|r| r.is_non_actionable())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspaceConstraintPattern {
    pub pattern_id: String,
    pub title: String,
    pub description: String,
    pub evidence_references: Vec<CrossWorkspaceEvidenceRef>,
    pub participating_workspaces: Vec<String>,
    pub frequency: usize,
    pub confidence: u8,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CrossWorkspaceConstraintPattern {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cross_workspace_constraint:";

    pub fn observe(
        title: impl Into<String>,
        description: impl Into<String>,
        evidence_references: Vec<CrossWorkspaceEvidenceRef>,
        participating_workspaces: Vec<String>,
        frequency: usize,
        confidence: u8,
    ) -> Result<Self, CrossWorkspaceIntelligenceError> {
        if evidence_references.is_empty() {
            return Err(CrossWorkspaceIntelligenceError::RequiresEvidence);
        }
        let title = title.into();
        let digest = stable_digest(&format!("{title}|{}", participating_workspaces.join(",")));
        Ok(Self {
            pattern_id: format!("{}{}", Self::ID_PREFIX, digest),
            title,
            description: description.into(),
            evidence_references,
            participating_workspaces,
            frequency,
            confidence: confidence.min(100),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self
                .evidence_references
                .iter()
                .all(|r| r.is_non_actionable())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspaceGap {
    pub gap_id: String,
    pub missing_evidence: String,
    pub affected_workspaces: Vec<String>,
    pub uncertainty_explanation: String,
    pub severity: String,
    pub evidence_refs: Vec<CrossWorkspaceEvidenceRef>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CrossWorkspaceGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cross_workspace_gap:";

    pub fn record(
        missing_evidence: impl Into<String>,
        affected_workspaces: Vec<String>,
        uncertainty_explanation: impl Into<String>,
        severity: impl Into<String>,
        evidence_refs: Vec<CrossWorkspaceEvidenceRef>,
    ) -> Self {
        let missing_evidence = missing_evidence.into();
        let digest = stable_digest(&format!(
            "{missing_evidence}|{}",
            affected_workspaces.join(",")
        ));
        Self {
            gap_id: format!("{}{}", Self::ID_PREFIX, digest),
            missing_evidence,
            affected_workspaces,
            uncertainty_explanation: uncertainty_explanation.into(),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspaceAssessment {
    pub assessment_id: String,
    pub workspace_count: usize,
    pub available_workspace_count: usize,
    pub pattern_count: usize,
    pub theme_count: usize,
    pub risk_signal_count: usize,
    pub constraint_pattern_count: usize,
    pub gap_count: usize,
    pub coverage: u8,
    pub uncertainty: Vec<String>,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CrossWorkspaceAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cross_workspace_assessment:";

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspaceIntelligenceSnapshot {
    pub intelligence_id: String,
    pub scope_id: String,
    pub generated_at: String,
    pub status: CrossWorkspaceIntelligenceStatus,
    pub superseded_at: Option<String>,
    pub participating_workspaces: Vec<String>,
    pub source_revisions: Vec<String>,
    pub patterns: Vec<CrossWorkspacePattern>,
    pub themes: Vec<CrossWorkspaceTheme>,
    pub risk_signals: Vec<CrossWorkspaceRiskSignal>,
    pub constraint_patterns: Vec<CrossWorkspaceConstraintPattern>,
    pub gaps: Vec<CrossWorkspaceGap>,
    pub assessment: CrossWorkspaceAssessment,
    pub completeness: CrossWorkspaceCompleteness,
    pub provenance_links: Vec<CrossWorkspaceEvidenceRef>,
    pub summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl CrossWorkspaceIntelligenceSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cross_workspace_intelligence:";

    pub fn compose(
        generated_at: impl Into<String>,
        contributions: Vec<WorkspaceContribution>,
    ) -> Result<Self, CrossWorkspaceIntelligenceError> {
        let generated_at = generated_at.into();
        let mut patterns = Vec::new();
        let mut themes = Vec::new();
        let mut risk_signals = Vec::new();
        let mut constraint_patterns = Vec::new();
        let mut gaps = Vec::new();
        let mut provenance_links = Vec::new();
        let mut source_revisions = Vec::new();
        let mut participating = Vec::new();

        if contributions.is_empty() {
            gaps.push(CrossWorkspaceGap::record(
                "No workspaces available for aggregation",
                vec![],
                "Unavailable workspaces remain unavailable — never invent global patterns",
                "high",
                vec![],
            ));
        }

        for c in &contributions {
            participating.push(c.workspace_id.clone());
            provenance_links.extend(c.evidence_refs.clone());
            for s in &c.surfaces_present {
                source_revisions.push(format!("{}:{s}", c.workspace_id));
            }
            if !c.available {
                gaps.push(CrossWorkspaceGap::record(
                    format!("Workspace {} unavailable for cross-workspace aggregation", c.workspace_id),
                    vec![c.workspace_id.clone()],
                    "Unavailable remains Unavailable — never invent workspace evidence",
                    "high",
                    c.evidence_refs.clone(),
                ));
            }
            for missing in &c.surfaces_missing {
                gaps.push(CrossWorkspaceGap::record(
                    format!("Surface '{missing}' missing in workspace {}", c.workspace_id),
                    vec![c.workspace_id.clone()],
                    "Missing evidence stays missing",
                    "medium",
                    c.evidence_refs.clone(),
                ));
            }
        }

        // Aggregate label → workspaces maps.
        let theme_map = aggregate_labels(contributions.iter().map(|c| {
            (
                c.workspace_id.clone(),
                c.theme_labels.clone(),
                c.evidence_refs.clone(),
            )
        }));
        for (label, (workspaces, refs)) in theme_map {
            if workspaces.len() >= 2 {
                themes.push(CrossWorkspaceTheme::aggregate(
                    label.clone(),
                    format!(
                        "Theme '{label}' observed across {} workspaces. Common theme ≠ shared truth.",
                        workspaces.len()
                    ),
                    refs,
                    workspaces.clone(),
                    (40 + workspaces.len() * 10).min(90) as u8,
                )?);
                patterns.push(CrossWorkspacePattern::aggregate(
                    format!("Recurring theme: {label}"),
                    format!(
                        "Pattern derived from theme '{label}' across {} workspaces. Aggregation ≠ authority.",
                        workspaces.len()
                    ),
                    workspaces.clone(),
                    themes.last().unwrap().supporting_evidence.clone(),
                    (40 + workspaces.len() * 10).min(90) as u8,
                    workspaces.len(),
                )?);
            }
        }

        let risk_map = aggregate_labels(contributions.iter().map(|c| {
            (
                c.workspace_id.clone(),
                c.risk_labels.clone(),
                c.evidence_refs.clone(),
            )
        }));
        for (label, (workspaces, refs)) in risk_map {
            if workspaces.len() >= 2 {
                risk_signals.push(CrossWorkspaceRiskSignal::observe(
                    label.clone(),
                    format!(
                        "Repeated risk '{label}' across {} workspaces — diagnostic only; never proposes action.",
                        workspaces.len()
                    ),
                    refs,
                    workspaces.len(),
                    workspaces,
                    55,
                )?);
            }
        }

        let constraint_map = aggregate_labels(contributions.iter().map(|c| {
            (
                c.workspace_id.clone(),
                c.constraint_labels.clone(),
                c.evidence_refs.clone(),
            )
        }));
        for (label, (workspaces, refs)) in constraint_map {
            if workspaces.len() >= 2 {
                constraint_patterns.push(CrossWorkspaceConstraintPattern::observe(
                    format!("Constraint: {label}"),
                    format!(
                        "Recurring constraint evidence '{label}' across {} workspaces — no optimisation or planning.",
                        workspaces.len()
                    ),
                    refs,
                    workspaces.clone(),
                    workspaces.len(),
                    50,
                )?);
            }
        }

        let pattern_map = aggregate_labels(contributions.iter().map(|c| {
            (
                c.workspace_id.clone(),
                c.pattern_labels.clone(),
                c.evidence_refs.clone(),
            )
        }));
        for (label, (workspaces, refs)) in pattern_map {
            if workspaces.len() >= 2 {
                patterns.push(CrossWorkspacePattern::aggregate(
                    format!("Recurring pattern: {label}"),
                    format!(
                        "Pattern '{label}' recurs across {} workspaces. Statistics ≠ recommendations.",
                        workspaces.len()
                    ),
                    workspaces.clone(),
                    refs,
                    (45 + workspaces.len() * 8).min(90) as u8,
                    workspaces.len(),
                )?);
            }
        }

        // Surface co-presence pattern (e.g. policy present in multiple workspaces).
        let mut surface_presence: std::collections::BTreeMap<String, Vec<String>> =
            std::collections::BTreeMap::new();
        for c in &contributions {
            for s in &c.surfaces_present {
                surface_presence
                    .entry(s.clone())
                    .or_default()
                    .push(c.workspace_id.clone());
            }
        }
        for (surface, workspaces) in surface_presence {
            if workspaces.len() >= 2 {
                let refs: Vec<_> = contributions
                    .iter()
                    .filter(|c| workspaces.contains(&c.workspace_id))
                    .flat_map(|c| c.evidence_refs.clone())
                    .collect();
                if !refs.is_empty() {
                    patterns.push(CrossWorkspacePattern::aggregate(
                        format!("Surface co-presence: {surface}"),
                        format!(
                            "Surface '{surface}' is present in {} workspaces. Frequency ≠ priority-as-action.",
                            workspaces.len()
                        ),
                        workspaces.clone(),
                        refs,
                        40,
                        workspaces.len(),
                    )?);
                }
            }
        }

        let conflict_workspaces: Vec<_> = contributions
            .iter()
            .filter(|c| c.conflict_indicated)
            .map(|c| c.workspace_id.clone())
            .collect();
        if conflict_workspaces.len() >= 2 {
            let refs: Vec<_> = contributions
                .iter()
                .filter(|c| c.conflict_indicated)
                .flat_map(|c| c.evidence_refs.clone())
                .collect();
            if !refs.is_empty() {
                risk_signals.push(CrossWorkspaceRiskSignal::observe(
                    "repeated_conflict_indicators",
                    format!(
                        "Conflict indicators present in {} workspaces — conflicts preserved, never resolved here.",
                        conflict_workspaces.len()
                    ),
                    refs,
                    conflict_workspaces.len(),
                    conflict_workspaces,
                    60,
                )?);
            }
        }

        let available_count = contributions.iter().filter(|c| c.available).count();
        let workspace_count = contributions.len();
        let completeness = if workspace_count == 0 || available_count == 0 {
            CrossWorkspaceCompleteness::Unavailable
        } else if conflict_workspaces_indicated(&contributions) {
            CrossWorkspaceCompleteness::Contradictory
        } else if available_count == workspace_count
            && gaps.is_empty()
            && contributions.iter().all(|c| c.surfaces_missing.is_empty())
        {
            CrossWorkspaceCompleteness::Complete
        } else if available_count < workspace_count || !gaps.is_empty() {
            CrossWorkspaceCompleteness::Partial
        } else {
            CrossWorkspaceCompleteness::Unknown
        };

        let coverage = if workspace_count == 0 {
            0
        } else {
            ((available_count * 100) / workspace_count) as u8
        };

        let assessment = CrossWorkspaceAssessment {
            assessment_id: format!(
                "{}{}",
                CrossWorkspaceAssessment::ID_PREFIX,
                stable_digest(&format!("{workspace_count}|{available_count}|{}", patterns.len()))
            ),
            workspace_count,
            available_workspace_count: available_count,
            pattern_count: patterns.len(),
            theme_count: themes.len(),
            risk_signal_count: risk_signals.len(),
            constraint_pattern_count: constraint_patterns.len(),
            gap_count: gaps.len(),
            coverage,
            uncertainty: vec![
                "Cross-workspace assessment is diagnostic only".into(),
                "aggregation ≠ authority".into(),
            ],
            limitations: vec![
                "Never centralises authority".into(),
                "Does not replace per-workspace truth".into(),
                "confidence ≠ permission".into(),
                "statistics ≠ recommendations".into(),
            ],
            authority_effect: CrossWorkspaceAssessment::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        };

        let summary = format!(
            "Cross-workspace intelligence: {} workspace(s), {} pattern(s), {} theme(s), {} gap(s), completeness={}",
            workspace_count,
            patterns.len(),
            themes.len(),
            gaps.len(),
            completeness.as_str()
        );
        let narrative = format!(
            "Aggregated understanding across {workspace_count} workspace(s) ({available_count} available). \
             Cross-Workspace Intelligence aggregates understanding only. It never centralises authority \
             or replaces the authority of individual workspaces."
        );

        let mut result = Self {
            intelligence_id: format!(
                "{}{}",
                Self::ID_PREFIX,
                stable_digest(&format!(
                    "{}|{}|{}",
                    participating.join(","),
                    source_revisions.join(","),
                    generated_at
                ))
            ),
            scope_id: CROSS_WORKSPACE_SCOPE.into(),
            generated_at,
            status: CrossWorkspaceIntelligenceStatus::Current,
            superseded_at: None,
            participating_workspaces: participating,
            source_revisions,
            patterns,
            themes,
            risk_signals,
            constraint_patterns,
            gaps,
            assessment,
            completeness,
            provenance_links,
            summary,
            narrative,
            limitations: vec![
                "Cross-Workspace Intelligence aggregates understanding only".into(),
                "It never centralises authority or replaces individual workspace authority".into(),
                "It does not execute, recommend, approve, or plan".into(),
            ],
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        result.validate()?;
        Ok(result)
    }

    pub fn compose_deterministic(
        generated_at: impl Into<String>,
        contributions: Vec<WorkspaceContribution>,
    ) -> Result<Self, CrossWorkspaceIntelligenceError> {
        let mut snap = Self::compose(generated_at, contributions)?;
        let digest = stable_digest(&format!(
            "{}|{}|{}|{}|{}",
            snap.participating_workspaces.join(","),
            snap.patterns
                .iter()
                .map(|p| p.pattern_id.clone())
                .collect::<Vec<_>>()
                .join(","),
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
        snap.intelligence_id = format!("{}{}", Self::ID_PREFIX, digest);
        Ok(snap)
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = CrossWorkspaceIntelligenceStatus::Superseded;
        self.superseded_at = Some(at.into());
        self.terminal = true;
        self.actionable = false;
    }

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.actionable
            && !self.terminal
            && self.patterns.iter().all(|p| p.is_non_actionable())
            && self.themes.iter().all(|t| t.is_non_actionable())
            && self.risk_signals.iter().all(|r| r.is_non_actionable())
            && self.constraint_patterns.iter().all(|c| c.is_non_actionable())
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.assessment.is_non_actionable()
            && self.provenance_links.iter().all(|p| p.is_non_actionable())
    }

    pub fn validate(&self) -> Result<(), CrossWorkspaceIntelligenceError> {
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE || self.actionable {
            return Err(CrossWorkspaceIntelligenceError::AuthorityEffectMustBeNone);
        }
        if self.patterns.iter().any(|p| p.actionable || p.evidence_references.is_empty()) {
            return Err(CrossWorkspaceIntelligenceError::RequiresEvidence);
        }
        if self.themes.iter().any(|t| t.actionable || t.supporting_evidence.is_empty()) {
            return Err(CrossWorkspaceIntelligenceError::RequiresEvidence);
        }
        if self.risk_signals.iter().any(|r| r.actionable) {
            return Err(CrossWorkspaceIntelligenceError::MustNotBeActionable);
        }
        if self.constraint_patterns.iter().any(|c| c.actionable) {
            return Err(CrossWorkspaceIntelligenceError::MustNotBeActionable);
        }
        let forbidden = [
            "do this",
            "execute this",
            "approve this",
            "recommend that you",
            "best action",
            "definitely true",
            "centralise authority",
        ];
        // Allow educational "never centralises authority" — check imperative forms only.
        let blob = format!("{} {}", self.summary, self.narrative).to_lowercase();
        for phrase in ["do this", "execute this", "approve this", "recommend that you", "best action", "definitely true"] {
            if blob.contains(phrase) {
                return Err(CrossWorkspaceIntelligenceError::MustNotBeActionable);
            }
        }
        let _ = forbidden;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspaceIntelligenceHistoryEntry {
    pub intelligence_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub completeness: String,
    pub pattern_count: usize,
    pub theme_count: usize,
    pub risk_signal_count: usize,
    pub constraint_pattern_count: usize,
    pub gap_count: usize,
    pub workspace_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl CrossWorkspaceIntelligenceHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &CrossWorkspaceIntelligenceSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            intelligence_id: snap.intelligence_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            completeness: snap.completeness.as_str().into(),
            pattern_count: snap.patterns.len(),
            theme_count: snap.themes.len(),
            risk_signal_count: snap.risk_signals.len(),
            constraint_pattern_count: snap.constraint_patterns.len(),
            gap_count: snap.gaps.len(),
            workspace_count: snap.participating_workspaces.len(),
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
pub struct CrossWorkspaceIntelligenceProjection {
    pub scope_id: String,
    pub generated_at: String,
    pub current: Option<CrossWorkspaceIntelligenceSnapshot>,
    pub history: Vec<CrossWorkspaceIntelligenceHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl CrossWorkspaceIntelligenceProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        current: Option<CrossWorkspaceIntelligenceSnapshot>,
        history: Vec<CrossWorkspaceIntelligenceHistoryEntry>,
        history_count: usize,
        generated_at: impl Into<String>,
    ) -> Self {
        Self {
            scope_id: CROSS_WORKSPACE_SCOPE.into(),
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

    pub fn summary(&self, history_limit: usize) -> CrossWorkspaceIntelligenceSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        CrossWorkspaceIntelligenceSummary {
            scope_id: self.scope_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            intelligence_id: self.current.as_ref().map(|c| c.intelligence_id.clone()),
            completeness: self
                .current
                .as_ref()
                .map(|c| c.completeness.as_str().into()),
            pattern_count: self.current.as_ref().map(|c| c.patterns.len()).unwrap_or(0),
            theme_count: self.current.as_ref().map(|c| c.themes.len()).unwrap_or(0),
            risk_signal_count: self
                .current
                .as_ref()
                .map(|c| c.risk_signals.len())
                .unwrap_or(0),
            constraint_pattern_count: self
                .current
                .as_ref()
                .map(|c| c.constraint_patterns.len())
                .unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            workspace_count: self
                .current
                .as_ref()
                .map(|c| c.participating_workspaces.len())
                .unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspaceIntelligenceSummary {
    pub scope_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub intelligence_id: Option<String>,
    pub completeness: Option<String>,
    pub pattern_count: usize,
    pub theme_count: usize,
    pub risk_signal_count: usize,
    pub constraint_pattern_count: usize,
    pub gap_count: usize,
    pub workspace_count: usize,
    pub history: Vec<CrossWorkspaceIntelligenceHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWorkspacePatternExplanation {
    pub explanation_id: String,
    pub scope_id: String,
    pub completeness: Option<String>,
    pub summary: Option<String>,
    pub pattern_summaries: Vec<String>,
    pub theme_summaries: Vec<String>,
    pub risk_summaries: Vec<String>,
    pub gaps: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl CrossWorkspacePatternExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_snapshot(snap: &CrossWorkspaceIntelligenceSnapshot) -> Self {
        Self {
            explanation_id: format!(
                "cross_workspace_pattern_explanation:{}",
                snap.intelligence_id
            ),
            scope_id: snap.scope_id.clone(),
            completeness: Some(snap.completeness.as_str().into()),
            summary: Some(snap.summary.clone()),
            pattern_summaries: snap
                .patterns
                .iter()
                .map(|p| {
                    format!(
                        "{} [{}] occurrence={} confidence={}",
                        p.title, p.pattern_id, p.occurrence_count, p.confidence
                    )
                })
                .collect(),
            theme_summaries: snap
                .themes
                .iter()
                .map(|t| format!("{} [{}] confidence={}", t.label, t.theme_id, t.confidence))
                .collect(),
            risk_summaries: snap
                .risk_signals
                .iter()
                .map(|r| {
                    format!(
                        "{} [{}] frequency={}",
                        r.repeated_risk, r.signal_id, r.frequency
                    )
                })
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

fn conflict_workspaces_indicated(contributions: &[WorkspaceContribution]) -> bool {
    contributions.iter().filter(|c| c.conflict_indicated).count() >= 2
}

fn aggregate_labels(
    items: impl Iterator<
        Item = (
            String,
            Vec<String>,
            Vec<CrossWorkspaceEvidenceRef>,
        ),
    >,
) -> std::collections::BTreeMap<String, (Vec<String>, Vec<CrossWorkspaceEvidenceRef>)> {
    let mut map: std::collections::BTreeMap<String, (Vec<String>, Vec<CrossWorkspaceEvidenceRef>)> =
        std::collections::BTreeMap::new();
    for (workspace_id, labels, refs) in items {
        for label in labels {
            let entry = map.entry(label).or_default();
            if !entry.0.contains(&workspace_id) {
                entry.0.push(workspace_id.clone());
            }
            entry.1.extend(refs.clone());
        }
    }
    map
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

    fn contrib(
        id: &str,
        themes: &[&str],
        risks: &[&str],
        constraints: &[&str],
        patterns: &[&str],
        surfaces: &[&str],
    ) -> WorkspaceContribution {
        WorkspaceContribution {
            workspace_id: id.into(),
            workspace_name: format!("WS {id}"),
            available: true,
            surfaces_present: surfaces.iter().map(|s| (*s).into()).collect(),
            surfaces_missing: vec![],
            theme_labels: themes.iter().map(|s| (*s).into()).collect(),
            risk_labels: risks.iter().map(|s| (*s).into()).collect(),
            constraint_labels: constraints.iter().map(|s| (*s).into()).collect(),
            pattern_labels: patterns.iter().map(|s| (*s).into()).collect(),
            gap_count: 0,
            conflict_indicated: false,
            evidence_refs: vec![CrossWorkspaceEvidenceRef::link(
                "test",
                format!("ref:{id}"),
                Some(id.into()),
                None,
            )],
        }
    }

    #[test]
    fn empty_contributions_unavailable() {
        let snap = CrossWorkspaceIntelligenceSnapshot::compose("t0", vec![]).unwrap();
        assert_eq!(snap.completeness, CrossWorkspaceCompleteness::Unavailable);
        assert!(!snap.gaps.is_empty());
        assert!(snap.is_non_executing());
    }

    #[test]
    fn deterministic_identical_inputs() {
        let inputs = vec![
            contrib("a", &["focus"], &["stale"], &["cap"], &["overlap"], &["state"]),
            contrib("b", &["focus"], &["stale"], &["cap"], &["overlap"], &["state"]),
        ];
        let left =
            CrossWorkspaceIntelligenceSnapshot::compose_deterministic("t1", inputs.clone())
                .unwrap();
        let right =
            CrossWorkspaceIntelligenceSnapshot::compose_deterministic("t1", inputs).unwrap();
        assert_eq!(left.intelligence_id, right.intelligence_id);
        assert_eq!(left.patterns, right.patterns);
        assert_eq!(left.themes, right.themes);
    }

    #[test]
    fn recurring_themes_produce_patterns() {
        let snap = CrossWorkspaceIntelligenceSnapshot::compose(
            "t0",
            vec![
                contrib("a", &["shared_theme"], &[], &[], &[], &["state"]),
                contrib("b", &["shared_theme"], &[], &[], &[], &["state"]),
            ],
        )
        .unwrap();
        assert!(!snap.themes.is_empty());
        assert!(snap.themes.iter().all(|t| t.contributing_workspaces.len() >= 2));
        assert!(snap.patterns.iter().any(|p| p.occurrence_count >= 2));
    }

    #[test]
    fn confidence_diagnostic_not_authority() {
        let snap = CrossWorkspaceIntelligenceSnapshot::compose(
            "t0",
            vec![
                contrib("a", &["t"], &["r"], &["c"], &["p"], &["policy"]),
                contrib("b", &["t"], &["r"], &["c"], &["p"], &["policy"]),
            ],
        )
        .unwrap();
        assert!(snap.assessment.is_non_actionable());
        assert!(snap.risk_signals.iter().all(|r| !r.actionable));
        assert_eq!(snap.authority_effect, "none");
    }

    #[test]
    fn history_non_actionable() {
        let mut snap =
            CrossWorkspaceIntelligenceSnapshot::compose("t0", vec![contrib("a", &[], &[], &[], &[], &["state"])])
                .unwrap();
        snap.mark_superseded("t1");
        let entry = CrossWorkspaceIntelligenceHistoryEntry::from_snapshot(&snap).unwrap();
        assert!(entry.is_non_actionable());
        let proj = CrossWorkspaceIntelligenceProjection::assemble(None, vec![entry], 1, "t2");
        assert!(proj.is_non_commandable());
    }

    #[test]
    fn patterns_require_evidence() {
        let err = CrossWorkspacePattern::aggregate(
            "orphan",
            "no evidence",
            vec!["a".into()],
            vec![],
            10,
            1,
        )
        .unwrap_err();
        assert_eq!(err, CrossWorkspaceIntelligenceError::RequiresEvidence);
    }

    #[test]
    fn unavailable_workspace_gap_preserved() {
        let mut c = contrib("gone", &[], &[], &[], &[], &[]);
        c.available = false;
        c.surfaces_missing = vec!["state".into()];
        let snap = CrossWorkspaceIntelligenceSnapshot::compose("t0", vec![c]).unwrap();
        assert!(snap.gaps.iter().any(|g| g.missing_evidence.contains("unavailable")
            || g.missing_evidence.contains("Unavailable")
            || g.uncertainty_explanation.contains("Unavailable")));
    }
}
