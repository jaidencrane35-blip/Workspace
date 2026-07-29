//! Workspace Assistant Personalisation Boundary — Programme IV Batch 16.
//!
//! Adapt presentation from explicit preferences. Never invent who the user is.
//!
//! **Spelling note:** Rust module and Programme IV boundary types use British
//! `personalisation`. This package **composes** the American-spelled foundation
//! types from `ai_personalization` (`UserPreference`, `UserPreferenceProfile`,
//! `AiPersonalizationAwareness`) — it does not reimplement preference SoT.
//!
//! Ownership clarification beside Batches 11–15 — not a user model, personality
//! engine, hidden profile database, memory SoT, or autonomous adapter.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ai_personalization::{
    AiPersonalizationAwareness, PreferenceSource, UserPreference, UserPreferenceProfile,
};
use crate::errors::DomainError;
use crate::workspace_assistant_context::WorkspaceAssistantContextProjection;
use crate::workspace_assistant_explanation::WorkspaceAssistantExplanationProjection;
use crate::workspace_assistant_interaction::WorkspaceAssistantInteractionProjection;
use crate::workspace_assistant_retrieval::WorkspaceAssistantRetrievalProjection;
use crate::workspace_assistant_surface::{AssistantSurfaceScope, WorkspaceAssistantSurfaceProjection};
use crate::workspace_evidence_contract::{
    reject_forbidden_phrases_with, stable_digest, AUTHORITY_EFFECT_NONE,
};

const FORBIDDEN_PERSONALISATION_PHRASES: &[&str] = &[
    "you are the kind of",
    "your personality",
    "i inferred",
    "based on your behaviour",
    "based on your behavior",
    "demographic",
    "psychological",
    "i learned that you",
    "secretly",
    "without telling you",
    "you should",
    "approve",
    "execute now",
    "decision is",
    "trust me",
];

/// Assistant personalisation item kinds (string constants).
pub mod item_kind {
    pub const EXPLICIT_PREFERENCE: &str = "explicit_preference";
    pub const ADAPTATION_METADATA: &str = "adaptation_metadata";
    pub const STYLE_CONFIGURATION: &str = "style_configuration";
    pub const GAP: &str = "gap";
}

const ORIGIN_AI_PERSONALIZATION: &str = "ai_personalization";
const PACKAGE_SURFACE: &str = "workspace_assistant_surface";
const PACKAGE_CONTEXT: &str = "workspace_assistant_context";
const PACKAGE_RETRIEVAL: &str = "workspace_assistant_retrieval";
const PACKAGE_EXPLANATION: &str = "workspace_assistant_explanation";
const PACKAGE_INTERACTION: &str = "workspace_assistant_interaction";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AssistantPersonalisationError {
    #[error("invalid assistant personalisation status: {0}")]
    InvalidStatus(String),

    #[error("assistant personalisation artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("assistant personalisation artefacts must not be actionable")]
    MustNotBeActionable,

    #[error("assistant personalisation text contains forbidden phrase: {0}")]
    ForbiddenPhrase(String),

    #[error("assistant personalisation snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantPersonalisationStatus {
    Current,
    Superseded,
    Archived,
}

impl AssistantPersonalisationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AssistantPersonalisationError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(AssistantPersonalisationError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Explicit preference summary used as packaging input — never inferred.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplicitPreferenceSummary {
    pub preference_id: String,
    pub category: String,
    pub value_summary: String,
    pub source: String,
    pub key: Option<String>,
}

impl ExplicitPreferenceSummary {
    pub fn from_preference(pref: &UserPreference) -> Self {
        Self {
            preference_id: pref.id.as_str().to_string(),
            category: pref.category.as_str().to_string(),
            value_summary: pref.value.clone(),
            source: pref.source.as_str().to_string(),
            key: Some(pref.key.clone()),
        }
    }

    pub fn from_profile(profile: &UserPreferenceProfile) -> Vec<Self> {
        profile
            .preferences
            .iter()
            .filter(|p| p.is_active())
            .map(Self::from_preference)
            .collect()
    }

    pub fn from_awareness(awareness: &AiPersonalizationAwareness) -> Vec<Self> {
        awareness
            .preferences
            .iter()
            .filter(|p| p.is_active())
            .map(Self::from_preference)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantPersonalisationRequest {
    pub request_id: String,
    pub human_ask: String,
    /// Reused from Batch 11 — do not redefine scope flags.
    pub scope: AssistantSurfaceScope,
    /// Factual notes naming which preference / assistant packages were consulted.
    pub pathway_notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantPersonalisationRequest {
    fn from_ask_and_scope(human_ask: &str, scope: &AssistantSurfaceScope) -> Self {
        let pathway_notes = vec![
            "Request packaging records explicit preference presentation pathways only".into(),
            "Pathway notes adapt presentation from explicit preferences — never invent who the user is".into(),
            format!(
                "Scope flags retained from AssistantSurfaceScope (personalisation_default pathways selected={})",
                scope.include_explanation
                    || scope.include_semantic_query
                    || scope.include_contextual
                    || scope.include_state
            ),
        ];
        let request_id = format!(
            "assistant_personalisation_request:{}",
            stable_digest(&format!(
                "{}|{}|{}|{}|{}",
                human_ask,
                scope.include_explanation,
                scope.include_semantic_query,
                scope.include_contextual,
                scope.include_state
            ))
        );
        Self {
            request_id,
            human_ask: human_ask.to_string(),
            scope: scope.clone(),
            pathway_notes,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantPersonalisationItem {
    pub item_id: String,
    pub kind: String,
    /// Preference id from AI Personalization Foundation when present.
    pub preference_ref: Option<String>,
    /// e.g. "ai_personalization"
    pub origin_domain: String,
    /// user_defined / user_confirmed / imported / none
    pub source: Option<String>,
    /// Factual display of recorded pref — never inferred.
    pub excerpt: String,
    /// Explainable presentation adaptation only.
    pub adaptation_note: Option<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantPersonalisationItem {
    fn package(
        kind: impl Into<String>,
        preference_ref: Option<String>,
        origin_domain: impl Into<String>,
        source: Option<String>,
        excerpt: impl Into<String>,
        adaptation_note: Option<String>,
    ) -> Self {
        let kind = kind.into();
        let origin_domain = origin_domain.into();
        let excerpt = excerpt.into();
        let item_id = format!(
            "assistant_personalisation_item:{}",
            stable_digest(&format!(
                "{}|{}|{}|{}|{}|{}",
                kind,
                preference_ref.as_deref().unwrap_or(""),
                origin_domain,
                source.as_deref().unwrap_or(""),
                excerpt,
                adaptation_note.as_deref().unwrap_or("")
            ))
        );
        Self {
            item_id,
            kind,
            preference_ref,
            origin_domain,
            source,
            excerpt,
            adaptation_note,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    fn from_explicit(pref: &ExplicitPreferenceSummary) -> Self {
        let source = if pref.source.trim().is_empty() {
            None
        } else {
            Some(pref.source.clone())
        };
        let adaptation_note = Some(format!(
            "Presentation may adapt using explicit preference '{}' (source {}) — never invent identity",
            pref.preference_id,
            pref.source
        ));
        let key_part = pref
            .key
            .as_ref()
            .map(|k| format!(" key='{}'", k))
            .unwrap_or_default();
        Self::package(
            item_kind::EXPLICIT_PREFERENCE,
            Some(pref.preference_id.clone()),
            ORIGIN_AI_PERSONALIZATION,
            source,
            format!(
                "Recorded preference id='{}' category='{}'{} value='{}' source='{}'",
                pref.preference_id, pref.category, key_part, pref.value_summary, pref.source
            ),
            adaptation_note,
        )
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantPersonalisationAdaptation {
    pub adaptation_id: String,
    pub personalization_enabled: bool,
    pub applied_preference_refs: Vec<String>,
    /// Must say adapt from explicit prefs only / never invent identity.
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantPersonalisationAdaptation {
    fn assemble(personalization_enabled: bool, applied_preference_refs: Vec<String>) -> Self {
        let notes = if !personalization_enabled {
            vec![
                "Personalization is disabled — presentation remains neutral".into(),
                "Adapt presentation from explicit preferences only; never invent who the user is".into(),
                "Disabled state is preserved; packaging never overrides the disable flag".into(),
            ]
        } else if applied_preference_refs.is_empty() {
            vec![
                "Personalization is enabled but no explicit preferences are recorded".into(),
                "Adapt presentation from explicit preferences only; never invent who the user is".into(),
                "No invented default-as-identity preferences are applied".into(),
            ]
        } else {
            vec![
                "Presentation adaptation cites only explicit recorded preferences".into(),
                "Adapt presentation from explicit preferences only; never invent who the user is".into(),
                format!(
                    "Applied preference refs: {}",
                    applied_preference_refs.join(", ")
                ),
            ]
        };
        let adaptation_id = format!(
            "assistant_personalisation_adaptation:{}",
            stable_digest(&format!(
                "{}|{}",
                personalization_enabled,
                applied_preference_refs.join(",")
            ))
        );
        Self {
            adaptation_id,
            personalization_enabled,
            applied_preference_refs,
            notes,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantPersonalisationLineage {
    pub lineage_id: String,
    pub contributing_artefacts: Vec<String>,
    pub revisions: Vec<String>,
    pub source_refs: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantPersonalisationLineage {
    fn assemble(
        preference_refs: &[String],
        assistant_package_ids: Vec<String>,
        source_refs: Vec<String>,
    ) -> Self {
        let mut contributing_artefacts = preference_refs.to_vec();
        // Assistant package ids are presentation consumers — not preference sources.
        for id in &assistant_package_ids {
            if !contributing_artefacts.contains(id) {
                contributing_artefacts.push(id.clone());
            }
        }
        contributing_artefacts.sort();
        contributing_artefacts.dedup();
        let mut revisions = Vec::new();
        for id in &assistant_package_ids {
            revisions.push(format!("presentation_consumer:{}", id));
        }
        revisions.sort();
        revisions.dedup();
        let mut source_refs = source_refs;
        source_refs.sort();
        source_refs.dedup();
        let lineage_id = format!(
            "assistant_personalisation_lineage:{}",
            stable_digest(&format!(
                "{}|{}|{}",
                contributing_artefacts.join(","),
                revisions.join(","),
                source_refs.join(",")
            ))
        );
        Self {
            lineage_id,
            contributing_artefacts,
            revisions,
            source_refs,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantPersonalisationDiagnostics {
    pub consulted_packages: Vec<String>,
    pub unavailable_packages: Vec<String>,
    pub personalization_enabled: bool,
    pub notes: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantPersonalisationDiagnostics {
    fn assemble(
        consulted_packages: Vec<String>,
        unavailable_packages: Vec<String>,
        personalization_enabled: bool,
    ) -> Self {
        Self {
            consulted_packages,
            unavailable_packages,
            personalization_enabled,
            notes: vec![
                "Assistant personalisation packages explicit presentation preferences only".into(),
                "Adapt presentation from explicit preferences only; never invent who the user is".into(),
                "No hidden profile, personality inference, or autonomous adaptation authority is created here".into(),
            ],
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantPersonalisationGap {
    pub gap_id: String,
    pub gap_kind: String,
    pub description: String,
    pub affected_packages: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl AssistantPersonalisationGap {
    pub const KIND_EMPTY_ASK: &'static str = "empty_ask";
    pub const KIND_PERSONALIZATION_DISABLED: &'static str = "personalization_disabled";
    pub const KIND_NO_PREFERENCES_RECORDED: &'static str = "no_preferences_recorded";
    pub const KIND_UNAVAILABLE_PACKAGE: &'static str = "unavailable_package";
    pub const KIND_INCOMPLETE_PACKAGE: &'static str = "incomplete_package";

    fn record(
        gap_kind: impl Into<String>,
        description: impl Into<String>,
        affected_packages: Vec<String>,
    ) -> Self {
        let gap_kind = gap_kind.into();
        let description = description.into();
        Self {
            gap_id: format!(
                "assistant_personalisation_gap:{}",
                stable_digest(&format!(
                    "{}|{}|{}",
                    gap_kind,
                    description,
                    affected_packages.join(",")
                ))
            ),
            gap_kind,
            description,
            affected_packages,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }

    pub fn is_non_actionable(&self) -> bool {
        !self.actionable && self.authority_effect == AUTHORITY_EFFECT_NONE
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantPersonalisationSnapshot {
    pub personalisation_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub status: AssistantPersonalisationStatus,
    pub superseded_at: Option<String>,
    pub request: AssistantPersonalisationRequest,
    pub items: Vec<AssistantPersonalisationItem>,
    pub adaptation: AssistantPersonalisationAdaptation,
    pub lineage: AssistantPersonalisationLineage,
    pub gaps: Vec<AssistantPersonalisationGap>,
    pub diagnostics: AssistantPersonalisationDiagnostics,
    pub narrative_summary: String,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
    pub terminal: bool,
}

impl WorkspaceAssistantPersonalisationSnapshot {
    pub const ID_PREFIX: &'static str = "assistant_personalisation:";

    /// Package presentation preferences from explicit recorded prefs + Batches 11–15.
    ///
    /// Items are built ONLY from explicit recorded preferences. Disabled personalization
    /// yields a neutral adaptation. Enabled with no prefs yields a gap — never invents
    /// defaults-as-identity. Assistant package ids appear in lineage as presentation
    /// consumers only — not as preference sources.
    pub fn package(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
        personalization_enabled: bool,
        preferences: &[ExplicitPreferenceSummary],
        surface: Option<&WorkspaceAssistantSurfaceProjection>,
        context: Option<&WorkspaceAssistantContextProjection>,
        retrieval: Option<&WorkspaceAssistantRetrievalProjection>,
        explanation: Option<&WorkspaceAssistantExplanationProjection>,
        interaction: Option<&WorkspaceAssistantInteractionProjection>,
    ) -> Result<Self, AssistantPersonalisationError> {
        let workspace_id = workspace_id.into();
        let generated_at = generated_at.into();
        let human_ask = human_ask.into();

        let request = AssistantPersonalisationRequest::from_ask_and_scope(&human_ask, &scope);

        let mut items = Vec::new();
        let mut gaps = Vec::new();
        let mut consulted = vec![ORIGIN_AI_PERSONALIZATION.to_string()];
        let mut unavailable = Vec::new();
        let mut applied_preference_refs = Vec::new();
        let mut assistant_package_ids = Vec::new();
        let mut source_refs = vec![ORIGIN_AI_PERSONALIZATION.to_string()];

        if human_ask.trim().is_empty() {
            gaps.push(AssistantPersonalisationGap::record(
                AssistantPersonalisationGap::KIND_EMPTY_ASK,
                "Human ask is blank; personalisation package records an empty-ask gap without inventing preference claims",
                vec!["request".into()],
            ));
            items.push(AssistantPersonalisationItem::package(
                item_kind::GAP,
                None,
                "request",
                Some("none".into()),
                "No human ask text was provided; the package records the gap and does not invent identity claims",
                None,
            ));
        }

        // Explicit preferences only — never invent.
        if personalization_enabled {
            for pref in preferences {
                // Only accept known explicit sources; skip anything that looks invented.
                let source_ok = matches!(
                    pref.source.as_str(),
                    "user_defined" | "user_confirmed" | "imported"
                ) || PreferenceSource::parse(&pref.source).is_ok();
                if !source_ok || pref.preference_id.trim().is_empty() {
                    continue;
                }
                applied_preference_refs.push(pref.preference_id.clone());
                items.push(AssistantPersonalisationItem::from_explicit(pref));
            }
            applied_preference_refs.sort();
            applied_preference_refs.dedup();

            if applied_preference_refs.is_empty() {
                gaps.push(AssistantPersonalisationGap::record(
                    AssistantPersonalisationGap::KIND_NO_PREFERENCES_RECORDED,
                    "Personalization is enabled but no explicit preferences are recorded; packaging never invents default-as-identity preferences",
                    vec![ORIGIN_AI_PERSONALIZATION.into()],
                ));
                items.push(AssistantPersonalisationItem::package(
                    item_kind::GAP,
                    None,
                    ORIGIN_AI_PERSONALIZATION,
                    Some("none".into()),
                    "No explicit preferences recorded; gap remains gap without invented identity defaults",
                    None,
                ));
            } else {
                items.push(AssistantPersonalisationItem::package(
                    item_kind::ADAPTATION_METADATA,
                    None,
                    ORIGIN_AI_PERSONALIZATION,
                    Some("user_defined".into()),
                    format!(
                        "Adaptation metadata cites {} explicit preference ref(s) for presentation only",
                        applied_preference_refs.len()
                    ),
                    Some(
                        "Adapt presentation from explicit preferences only; never invent who the user is"
                            .into(),
                    ),
                ));
                items.push(AssistantPersonalisationItem::package(
                    item_kind::STYLE_CONFIGURATION,
                    None,
                    ORIGIN_AI_PERSONALIZATION,
                    Some("user_defined".into()),
                    "Style configuration reflects only explicitly recorded preference values",
                    Some(
                        "Style knobs apply when explicitly set; packaging never invents personality"
                            .into(),
                    ),
                ));
            }
        } else {
            gaps.push(AssistantPersonalisationGap::record(
                AssistantPersonalisationGap::KIND_PERSONALIZATION_DISABLED,
                "Personalization is disabled; packaging preserves neutral presentation and never overrides the disable flag",
                vec![ORIGIN_AI_PERSONALIZATION.into()],
            ));
            items.push(AssistantPersonalisationItem::package(
                item_kind::GAP,
                None,
                ORIGIN_AI_PERSONALIZATION,
                Some("none".into()),
                "Personalization disabled — presentation remains neutral without applying preference adaptations",
                None,
            ));
            // Do not apply preference refs when disabled.
            applied_preference_refs.clear();
        }

        // Batches 11–15 as presentation consumers (lineage only — not preference sources).
        consulted.push(PACKAGE_SURFACE.to_string());
        match surface.and_then(|s| s.current.as_ref()) {
            Some(package) => {
                assistant_package_ids.push(package.surface_id.clone());
                source_refs.push(format!("presentation_consumer:{}", package.surface_id));
            }
            None => {
                unavailable.push(PACKAGE_SURFACE.to_string());
                gaps.push(unavailable_gap(PACKAGE_SURFACE));
            }
        }

        consulted.push(PACKAGE_CONTEXT.to_string());
        match context.and_then(|c| c.current.as_ref()) {
            Some(package) => {
                assistant_package_ids.push(package.context_id.clone());
                source_refs.push(format!("presentation_consumer:{}", package.context_id));
            }
            None => {
                unavailable.push(PACKAGE_CONTEXT.to_string());
                gaps.push(unavailable_gap(PACKAGE_CONTEXT));
            }
        }

        consulted.push(PACKAGE_RETRIEVAL.to_string());
        match retrieval.and_then(|r| r.current.as_ref()) {
            Some(package) => {
                assistant_package_ids.push(package.retrieval_id.clone());
                source_refs.push(format!("presentation_consumer:{}", package.retrieval_id));
            }
            None => {
                unavailable.push(PACKAGE_RETRIEVAL.to_string());
                gaps.push(unavailable_gap(PACKAGE_RETRIEVAL));
            }
        }

        consulted.push(PACKAGE_EXPLANATION.to_string());
        match explanation.and_then(|e| e.current.as_ref()) {
            Some(package) => {
                assistant_package_ids.push(package.explanation_id.clone());
                source_refs.push(format!("presentation_consumer:{}", package.explanation_id));
            }
            None => {
                unavailable.push(PACKAGE_EXPLANATION.to_string());
                gaps.push(unavailable_gap(PACKAGE_EXPLANATION));
            }
        }

        consulted.push(PACKAGE_INTERACTION.to_string());
        match interaction.and_then(|i| i.current.as_ref()) {
            Some(package) => {
                assistant_package_ids.push(package.interaction_id.clone());
                source_refs.push(format!("presentation_consumer:{}", package.interaction_id));
            }
            None => {
                unavailable.push(PACKAGE_INTERACTION.to_string());
                gaps.push(unavailable_gap(PACKAGE_INTERACTION));
            }
        }

        if !unavailable.is_empty() {
            gaps.push(AssistantPersonalisationGap::record(
                AssistantPersonalisationGap::KIND_INCOMPLETE_PACKAGE,
                format!(
                    "Personalisation package is incomplete; unavailable assistant packages: {}",
                    unavailable.join(", ")
                ),
                unavailable.clone(),
            ));
        }

        // Stable secondary order by item_id (deterministic).
        items.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.item_id.cmp(&b.item_id)));
        gaps.sort_by(|a, b| {
            a.gap_kind
                .cmp(&b.gap_kind)
                .then_with(|| a.gap_id.cmp(&b.gap_id))
        });

        let adaptation =
            AssistantPersonalisationAdaptation::assemble(personalization_enabled, applied_preference_refs.clone());
        let lineage = AssistantPersonalisationLineage::assemble(
            &applied_preference_refs,
            assistant_package_ids,
            source_refs,
        );
        let diagnostics = AssistantPersonalisationDiagnostics::assemble(
            consulted,
            unavailable,
            personalization_enabled,
        );

        let narrative_summary = format!(
            "Assistant personalisation: {} item(s), {} applied preference(s), personalization_enabled={}, {} gap(s)",
            items.len(),
            adaptation.applied_preference_refs.len(),
            personalization_enabled,
            gaps.len()
        );
        let narrative = "The assistant personalisation package adapts presentation from explicit preferences only for humans. It packages recorded preference display and explainable presentation adaptation metadata; it never invents who the user is, never infers personality or identity, never creates hidden profiles, never predicts behaviour, never autonomously adapts, never replaces memory or preference SoT owners, never influences permissions, and never decides or executes.".to_string();
        let limitations = vec![
            "Charter confirmation: Adapt presentation from explicit preferences. Never invent who the user is.".into(),
            "Personalisation packages are non-actionable evidence and cannot replace AI Personalization Foundation or Batches 11–15 owners".into(),
            "Preference writes remain existing authorised preference commands only — packaging never mutates user_preferences".into(),
            "Assistant package ids in lineage are presentation consumers — not preference sources".into(),
        ];

        let personalisation_id = format!(
            "{}{}",
            Self::ID_PREFIX,
            stable_digest(&format!(
                "{}|{}|{}|{}|{}|{}|{}",
                workspace_id,
                generated_at,
                request.request_id,
                adaptation.adaptation_id,
                lineage.lineage_id,
                items
                    .iter()
                    .map(|i| i.item_id.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
                gaps.iter()
                    .map(|g| g.gap_id.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        );

        let snap = Self {
            personalisation_id,
            workspace_id,
            generated_at,
            status: AssistantPersonalisationStatus::Current,
            superseded_at: None,
            request,
            items,
            adaptation,
            lineage,
            gaps,
            diagnostics,
            narrative_summary,
            narrative,
            limitations,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            terminal: false,
        };
        snap.validate()?;
        Ok(snap)
    }

    pub fn mark_superseded(&mut self, superseded_at: impl Into<String>) {
        self.status = AssistantPersonalisationStatus::Superseded;
        self.superseded_at = Some(superseded_at.into());
        self.terminal = true;
        self.actionable = false;
        self.authority_effect = AUTHORITY_EFFECT_NONE.into();
    }

    pub fn is_non_executing(&self) -> bool {
        !self.actionable
            && !self.terminal
            && self.authority_effect == AUTHORITY_EFFECT_NONE
            && self.request.is_non_actionable()
            && self.items.iter().all(|i| i.is_non_actionable())
            && self.adaptation.is_non_actionable()
            && self.lineage.is_non_actionable()
            && self.gaps.iter().all(|g| g.is_non_actionable())
            && self.diagnostics.is_non_actionable()
    }

    pub fn validate(&self) -> Result<(), AssistantPersonalisationError> {
        if self.authority_effect != AUTHORITY_EFFECT_NONE {
            return Err(AssistantPersonalisationError::AuthorityEffectMustBeNone);
        }
        if self.actionable
            || !self.request.is_non_actionable()
            || self.items.iter().any(|i| !i.is_non_actionable())
            || !self.adaptation.is_non_actionable()
            || !self.lineage.is_non_actionable()
            || self.gaps.iter().any(|g| !g.is_non_actionable())
            || !self.diagnostics.is_non_actionable()
        {
            return Err(AssistantPersonalisationError::MustNotBeActionable);
        }
        reject_forbidden(&self.narrative_summary)?;
        reject_forbidden(&self.narrative)?;
        for item in &self.limitations {
            reject_forbidden(item)?;
        }
        for item in &self.items {
            reject_forbidden(&item.excerpt)?;
            if let Some(note) = &item.adaptation_note {
                reject_forbidden(note)?;
            }
        }
        for gap in &self.gaps {
            reject_forbidden(&gap.description)?;
        }
        for note in &self.diagnostics.notes {
            reject_forbidden(note)?;
        }
        for note in &self.adaptation.notes {
            reject_forbidden(note)?;
        }
        for note in &self.request.pathway_notes {
            reject_forbidden(note)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantPersonalisationHistoryEntry {
    pub personalisation_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub item_count: usize,
    pub gap_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl AssistantPersonalisationHistoryEntry {
    pub fn from_snapshot(snap: &WorkspaceAssistantPersonalisationSnapshot) -> Option<Self> {
        if !snap.status.is_terminal() {
            return None;
        }
        Some(Self {
            personalisation_id: snap.personalisation_id.clone(),
            status: snap.status.as_str().into(),
            created_at: snap.generated_at.clone(),
            superseded_at: snap.superseded_at.clone(),
            item_count: snap.items.len(),
            gap_count: snap.gaps.len(),
            terminal: true,
            actionable: false,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        self.terminal
            && !self.actionable
            && self.authority_effect == AUTHORITY_EFFECT_NONE
            && matches!(self.status.as_str(), "superseded" | "archived")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantPersonalisationProjection {
    pub workspace_id: String,
    pub current: Option<WorkspaceAssistantPersonalisationSnapshot>,
    pub history: Vec<AssistantPersonalisationHistoryEntry>,
    pub history_count: usize,
    pub projected_at: String,
    pub authority_effect: String,
}

impl WorkspaceAssistantPersonalisationProjection {
    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceAssistantPersonalisationSnapshot>,
        history: Vec<AssistantPersonalisationHistoryEntry>,
        history_count: usize,
        projected_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            current,
            history,
            history_count,
            projected_at: projected_at.into(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_non_commandable(&self) -> bool {
        self.authority_effect == AUTHORITY_EFFECT_NONE
            && self.history.iter().all(|h| h.is_non_actionable())
            && self
                .current
                .as_ref()
                .map(|c| c.is_non_executing())
                .unwrap_or(true)
    }

    pub fn summary(&self, history_limit: usize) -> WorkspaceAssistantPersonalisationSummary {
        WorkspaceAssistantPersonalisationSummary {
            workspace_id: self.workspace_id.clone(),
            has_current: self.current.is_some(),
            item_count: self.current.as_ref().map(|c| c.items.len()).unwrap_or(0),
            gap_count: self.current.as_ref().map(|c| c.gaps.len()).unwrap_or(0),
            personalization_enabled: self
                .current
                .as_ref()
                .map(|c| c.adaptation.personalization_enabled),
            narrative_summary: self.current.as_ref().map(|c| c.narrative_summary.clone()),
            history: self.history.iter().take(history_limit).cloned().collect(),
            history_count: self.history_count,
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantPersonalisationSummary {
    pub workspace_id: String,
    pub has_current: bool,
    pub item_count: usize,
    pub gap_count: usize,
    pub personalization_enabled: Option<bool>,
    pub narrative_summary: Option<String>,
    pub history: Vec<AssistantPersonalisationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAssistantPersonalisationExplanation {
    pub explanation_id: String,
    pub workspace_id: String,
    pub narrative_summary: Option<String>,
    pub item_summaries: Vec<String>,
    pub adaptation_summaries: Vec<String>,
    pub gap_summaries: Vec<String>,
    pub uncertainty: Vec<String>,
    pub narrative: String,
    pub limitations: Vec<String>,
    pub authority_effect: String,
    pub actionable: bool,
}

impl WorkspaceAssistantPersonalisationExplanation {
    pub fn from_snapshot(snap: &WorkspaceAssistantPersonalisationSnapshot) -> Self {
        Self {
            explanation_id: format!("assistant_personalisation_meta:{}", snap.personalisation_id),
            workspace_id: snap.workspace_id.clone(),
            narrative_summary: Some(snap.narrative_summary.clone()),
            item_summaries: snap
                .items
                .iter()
                .map(|i| {
                    format!(
                        "{} -> {} [{}]",
                        i.kind,
                        i.preference_ref.as_deref().unwrap_or("none"),
                        i.item_id
                    )
                })
                .collect(),
            adaptation_summaries: {
                let mut summaries = snap.adaptation.applied_preference_refs.clone();
                summaries.push(format!(
                    "personalization_enabled={}",
                    snap.adaptation.personalization_enabled
                ));
                summaries.extend(snap.adaptation.notes.clone());
                summaries
            },
            gap_summaries: snap
                .gaps
                .iter()
                .map(|g| format!("{}: {}", g.gap_kind, g.description))
                .collect(),
            uncertainty: snap.diagnostics.unavailable_packages.clone(),
            narrative: snap.narrative.clone(),
            limitations: snap.limitations.clone(),
            authority_effect: AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
        }
    }
}

fn unavailable_gap(package: &str) -> AssistantPersonalisationGap {
    AssistantPersonalisationGap::record(
        AssistantPersonalisationGap::KIND_UNAVAILABLE_PACKAGE,
        format!(
            "Capability package '{}' is unavailable for this personalisation package",
            package
        ),
        vec![package.into()],
    )
}

fn reject_forbidden(text: &str) -> Result<(), AssistantPersonalisationError> {
    reject_forbidden_phrases_with(text, FORBIDDEN_PERSONALISATION_PHRASES)
        .map_err(|phrase| AssistantPersonalisationError::ForbiddenPhrase(phrase.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package_none(
        human_ask: &str,
        enabled: bool,
        prefs: &[ExplicitPreferenceSummary],
    ) -> WorkspaceAssistantPersonalisationSnapshot {
        WorkspaceAssistantPersonalisationSnapshot::package(
            "ws",
            "2026-07-29T00:00:00Z",
            human_ask,
            AssistantSurfaceScope::personalisation_default(),
            enabled,
            prefs,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap()
    }

    fn sample_pref(id: &str) -> ExplicitPreferenceSummary {
        ExplicitPreferenceSummary {
            preference_id: id.into(),
            category: "communication".into(),
            value_summary: "prefer concise citations".into(),
            source: "user_defined".into(),
            key: Some("citation_density".into()),
        }
    }

    #[test]
    fn package_is_non_actionable() {
        let snap = package_none("how are preferences packaged?", true, &[]);
        assert!(snap.is_non_executing());
        assert_eq!(snap.authority_effect, AUTHORITY_EFFECT_NONE);
        assert!(!snap.actionable);
        assert!(snap.validate().is_ok());
    }

    #[test]
    fn identical_inputs_have_identical_outputs() {
        let prefs = vec![sample_pref("pref-1")];
        let left = package_none("same ask", true, &prefs);
        let right = package_none("same ask", true, &prefs);
        assert_eq!(left.personalisation_id, right.personalisation_id);
        assert_eq!(left.items, right.items);
        assert_eq!(left.request, right.request);
        assert_eq!(left.adaptation, right.adaptation);
    }

    #[test]
    fn disabled_personalization_is_preserved() {
        let prefs = vec![sample_pref("pref-1")];
        let snap = package_none("ask", false, &prefs);
        assert!(snap.gaps.iter().any(|g| {
            g.gap_kind == AssistantPersonalisationGap::KIND_PERSONALIZATION_DISABLED
        }));
        assert!(!snap.adaptation.personalization_enabled);
        assert!(snap.adaptation.applied_preference_refs.is_empty());
        assert!(!snap.diagnostics.personalization_enabled);
        // Must not apply prefs when disabled.
        assert!(snap
            .items
            .iter()
            .filter(|i| i.kind == item_kind::EXPLICIT_PREFERENCE)
            .all(|i| i.preference_ref.is_none()));
    }

    #[test]
    fn no_preferences_records_gap() {
        let snap = package_none("ask", true, &[]);
        assert!(snap.gaps.iter().any(|g| {
            g.gap_kind == AssistantPersonalisationGap::KIND_NO_PREFERENCES_RECORDED
        }));
        assert!(snap.adaptation.applied_preference_refs.is_empty());
        assert!(snap
            .items
            .iter()
            .all(|i| i.kind != item_kind::EXPLICIT_PREFERENCE));
    }

    #[test]
    fn forbidden_inference_language_is_rejected() {
        assert!(reject_forbidden("you are the kind of").is_err());
        assert!(reject_forbidden("your personality").is_err());
        assert!(reject_forbidden("i inferred").is_err());
        assert!(reject_forbidden("based on your behaviour").is_err());
        assert!(reject_forbidden("based on your behavior").is_err());
        assert!(reject_forbidden("demographic").is_err());
        assert!(reject_forbidden("psychological").is_err());
        assert!(reject_forbidden("i learned that you").is_err());
        assert!(reject_forbidden("secretly").is_err());
        assert!(reject_forbidden("without telling you").is_err());
        assert!(reject_forbidden("you should").is_err());
        assert!(reject_forbidden("approve").is_err());
        assert!(reject_forbidden("execute now").is_err());
        assert!(reject_forbidden("decision is").is_err());
        assert!(reject_forbidden("trust me").is_err());
        assert!(reject_forbidden("adapt presentation from explicit preferences").is_ok());
        assert!(reject_forbidden("never invent who the user is").is_ok());
    }

    #[test]
    fn no_invented_preference_refs() {
        let snap = package_none("ask", true, &[]);
        assert!(snap.adaptation.applied_preference_refs.is_empty());
        for item in &snap.items {
            if item.kind == item_kind::EXPLICIT_PREFERENCE {
                panic!("must not invent explicit preference items");
            }
            assert!(
                item.preference_ref.is_none(),
                "must not invent preference refs: {:?}",
                item.preference_ref
            );
        }
        assert!(snap.narrative.contains("never invents who the user is"));
        assert!(snap
            .narrative
            .contains("adapts presentation from explicit preferences only"));
    }

    #[test]
    fn projection_is_non_commandable() {
        let projection = WorkspaceAssistantPersonalisationProjection::assemble(
            "ws",
            Some(package_none(
                "ask",
                true,
                &[sample_pref("pref-a")],
            )),
            vec![],
            0,
            "2026-07-29T00:00:00Z",
        );
        assert!(projection.is_non_commandable());
        let summary = projection.summary(5);
        assert!(!summary.actionable);
        assert_eq!(summary.authority_effect, AUTHORITY_EFFECT_NONE);
    }

    #[test]
    fn items_only_from_explicit_sources() {
        let prefs = vec![
            sample_pref("pref-1"),
            ExplicitPreferenceSummary {
                preference_id: "pref-2".into(),
                category: "layout".into(),
                value_summary: "compact".into(),
                source: "user_confirmed".into(),
                key: Some("density".into()),
            },
            ExplicitPreferenceSummary {
                preference_id: "invented".into(),
                category: "workflow".into(),
                value_summary: "should be skipped".into(),
                source: "inferred_behaviour".into(),
                key: None,
            },
        ];
        let snap = package_none("ask", true, &prefs);
        let explicit: Vec<_> = snap
            .items
            .iter()
            .filter(|i| i.kind == item_kind::EXPLICIT_PREFERENCE)
            .collect();
        assert_eq!(explicit.len(), 2);
        assert!(explicit.iter().all(|i| {
            matches!(
                i.source.as_deref(),
                Some("user_defined") | Some("user_confirmed") | Some("imported")
            )
        }));
        assert!(!snap
            .adaptation
            .applied_preference_refs
            .contains(&"invented".to_string()));
        assert_eq!(snap.adaptation.applied_preference_refs.len(), 2);
    }

    #[test]
    fn empty_ask_records_gap() {
        let snap = package_none("  ", true, &[]);
        assert!(snap
            .gaps
            .iter()
            .any(|g| g.gap_kind == AssistantPersonalisationGap::KIND_EMPTY_ASK));
        assert_eq!(snap.request.human_ask, "  ");
    }

    #[test]
    fn personalization_default_alias_matches() {
        assert_eq!(
            AssistantSurfaceScope::personalisation_default(),
            AssistantSurfaceScope::personalization_default()
        );
    }
}
