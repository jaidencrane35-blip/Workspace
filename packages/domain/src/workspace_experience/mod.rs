//! Workspace Experience Layer — presentation model for the Work surface (Phase 6).
//!
//! Consumes `WorkspaceSessionState` only. Owns no data, no intelligence, no authority.
//! Deterministic presentation groupings — not AI reasoning, not a cognition model.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;

/// Experience-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceExperienceError {
    #[error("experience requires a workspace id")]
    MissingWorkspace,

    #[error("experience cannot execute, prepare, restore, or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Experience grouping kinds (presentation only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperienceSectionKind {
    PrimaryFocus,
    TodaysWork,
    SuggestedAttention,
    WaitingOn,
    BlockedWork,
    RecentProgress,
    RecommendedNextStep,
    HelpfulImprovements,
    SessionHealth,
}

impl ExperienceSectionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryFocus => "primary_focus",
            Self::TodaysWork => "todays_work",
            Self::SuggestedAttention => "suggested_attention",
            Self::WaitingOn => "waiting_on",
            Self::BlockedWork => "blocked_work",
            Self::RecentProgress => "recent_progress",
            Self::RecommendedNextStep => "recommended_next_step",
            Self::HelpfulImprovements => "helpful_improvements",
            Self::SessionHealth => "session_health",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::PrimaryFocus => "Primary Focus",
            Self::TodaysWork => "Today's Work",
            Self::SuggestedAttention => "Session decisions",
            Self::WaitingOn => "Waiting On",
            Self::BlockedWork => "Blocked Work",
            Self::RecentProgress => "Recent Progress",
            Self::RecommendedNextStep => "Recommended Next Step",
            Self::HelpfulImprovements => "Helpful Improvements",
            Self::SessionHealth => "Session Health",
        }
    }
}

/// Deterministic presentation visibility (not AI ranking).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperienceVisibility {
    /// Show immediately in the default Work view.
    Immediate,
    /// Emphasize with highlight treatment.
    Highlighted,
    /// Available but collapsed by default.
    Collapsed,
    /// Deferred from the primary surface.
    Deferred,
}

impl ExperienceVisibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::Highlighted => "highlighted",
            Self::Collapsed => "collapsed",
            Self::Deferred => "deferred",
        }
    }
}

/// One presentable item — always a pointer into Session fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceItem {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub visibility: ExperienceVisibility,
    /// Why is this shown here?
    pub why: String,
    /// Which Session field produced this item?
    pub source_session_field: String,
    pub source_ref: String,
    pub authority_effect: String,
}

impl ExperienceItem {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// A presentation grouping over Session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceSection {
    pub kind: ExperienceSectionKind,
    pub title: String,
    pub visibility: ExperienceVisibility,
    pub items: Vec<ExperienceItem>,
    pub item_count: usize,
    pub collapsed_hint: Option<String>,
    pub why: String,
    pub source_session_field: String,
}

/// Human-facing experience summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceSummary {
    pub headline: String,
    pub focus_line: String,
    pub matters_line: String,
    pub blocked_line: String,
    pub ready_line: String,
    pub next_line: String,
    pub narrative: String,
}

/// Presentation importance band for a translated Attention reason.
///
/// Derived from `|weight|` only — never from ranking or score. Experience owns
/// this banding for display; Attention still owns the underlying weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayImportance {
    High,
    Medium,
    Low,
}

impl DisplayImportance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    /// Deterministic band from reason weight magnitude.
    pub fn from_weight(weight: i32) -> Self {
        let magnitude = weight.unsigned_abs();
        if magnitude >= 50 {
            Self::High
        } else if magnitude >= 25 {
            Self::Medium
        } else {
            Self::Low
        }
    }
}

/// Experience translation of one `AttentionReason`.
///
/// Structure only — wording is produced by the Experience resolver, not Domain.
/// Always retain the source `AttentionReason` alongside this; display never replaces
/// structured identity with text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayReason {
    pub title: String,
    pub description: String,
    pub importance: DisplayImportance,
    /// Echo of the source key — identity for UI projection and unknown-key surfacing.
    pub explanation_key: String,
    pub signal: String,
    pub source: String,
    pub weight: i32,
    /// `true` when Experience had a known translation; `false` for safe fallback.
    pub known: bool,
}

/// Which catalog resolution branch produced a `DisplayReason` (Sprint 135).
///
/// Developer diagnostic only — never required for user-facing rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperienceResolverPathKind {
    Exact,
    PrefixSuffix,
    PrefixPattern,
    PrefixFallback,
    Unknown,
    /// Decision reason without `attention_reason` — summary/kind path.
    DecisionNative,
}

impl ExperienceResolverPathKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::PrefixSuffix => "prefix_suffix",
            Self::PrefixPattern => "prefix_pattern",
            Self::PrefixFallback => "prefix_fallback",
            Self::Unknown => "unknown",
            Self::DecisionNative => "decision_native",
        }
    }
}

/// Catalog match descriptor used by Experience translation traces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceResolverPath {
    pub kind: ExperienceResolverPathKind,
    /// Match identity, e.g. `exact:decision.base.outstanding` or
    /// `prefix_pattern:purpose.obstacle.composition:*`.
    pub match_key: String,
}

impl ExperienceResolverPath {
    pub fn new(kind: ExperienceResolverPathKind, match_key: impl Into<String>) -> Self {
        Self {
            kind,
            match_key: match_key.into(),
        }
    }

    /// Compact developer label: `kind:\nmatch_key`.
    pub fn label(&self) -> String {
        format!("{}:\n{}", self.kind.as_str(), self.match_key)
    }
}

/// Developer-only Experience translation trace (Sprint 135).
///
/// Answers: "What cognition produced this displayed explanation?"
/// Does not mutate Domain reasoning. Must not be rendered on Work/Assistant surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceTranslationTrace {
    /// `attention_reason` | `decision_reason`
    pub source_reasoning_type: String,
    /// Stable source identity (explanation_key or decision kind).
    pub source_identifier: String,
    pub explanation_key: String,
    pub resolver_path: ExperienceResolverPath,
    pub display: DisplayReason,
    /// Optional surface tag set by the caller (`operator`, `test`, …).
    pub rendering_surface: Option<String>,
}

/// Full Experience Layer snapshot — presentation only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceExperienceState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub experience_summary: ExperienceSummary,
    pub sections: Vec<ExperienceSection>,
    pub section_count: usize,
    pub immediate_count: usize,
    pub highlighted_count: usize,
    pub collapsed_count: usize,
    pub deferred_count: usize,
    /// Session snapshot this experience was projected from.
    pub session_generated_at: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceExperienceState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceExperienceSummary {
        WorkspaceExperienceSummary {
            workspace_id: self.workspace_id.clone(),
            workspace_name: self.workspace_name.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            experience_summary: self.experience_summary.clone(),
            section_count: self.section_count,
            immediate_count: self.immediate_count,
            highlighted_count: self.highlighted_count,
            collapsed_count: self.collapsed_count,
            deferred_count: self.deferred_count,
            top_sections: self.sections.iter().take(limit).cloned().collect(),
            session_generated_at: self.session_generated_at.clone(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Operator / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceExperienceSummary {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub experience_summary: ExperienceSummary,
    pub section_count: usize,
    pub immediate_count: usize,
    pub highlighted_count: usize,
    pub collapsed_count: usize,
    pub deferred_count: usize,
    pub top_sections: Vec<ExperienceSection>,
    pub session_generated_at: String,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceExperienceSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            workspace_name: String::new(),
            generated_at: String::new(),
            label: String::new(),
            experience_summary: ExperienceSummary {
                headline: String::new(),
                focus_line: String::new(),
                matters_line: String::new(),
                blocked_line: String::new(),
                ready_line: String::new(),
                next_line: String::new(),
                narrative: String::new(),
            },
            section_count: 0,
            immediate_count: 0,
            highlighted_count: 0,
            collapsed_count: 0,
            deferred_count: 0,
            top_sections: Vec::new(),
            session_generated_at: String::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceExperienceState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Side-by-side comparison of two experience snapshots (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceExperienceComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceExperienceComparison {
    pub fn compare(left: &WorkspaceExperienceState, right: &WorkspaceExperienceState) -> Self {
        let mut differences = Vec::new();
        if left.experience_summary.focus_line != right.experience_summary.focus_line {
            differences.push("Primary focus presentation differs.".into());
        }
        if left.immediate_count != right.immediate_count {
            differences.push(format!(
                "Immediate sections: {} → {}",
                left.immediate_count, right.immediate_count
            ));
        }
        if left.highlighted_count != right.highlighted_count {
            differences.push(format!(
                "Highlighted sections: {} → {}",
                left.highlighted_count, right.highlighted_count
            ));
        }
        if left.session_generated_at != right.session_generated_at {
            differences.push("Projected from different Session snapshots.".into());
        }
        if left.experience_summary.blocked_line != right.experience_summary.blocked_line {
            differences.push("Blocked work presentation differs.".into());
        }
        if differences.is_empty() {
            differences.push("No material experience differences.".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
            authority_effect: WorkspaceExperienceState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

pub fn build_experience_summary(label: &str, immediate: usize, highlighted: usize) -> String {
    format!(
        "Experience for \"{label}\" — {immediate} immediate, {highlighted} highlighted section(s). \
         Presentation only; consumes Session; owns no source data; never executes."
    )
}

pub fn experience_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_experience_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceExperienceError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
