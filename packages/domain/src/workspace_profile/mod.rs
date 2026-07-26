//! Workspace Environment Profiles — durable user-owned setups (Phase 6).
//!
//! Answers: what workspace setup does the user consider meaningful?
//! Describes preferred environments. Never launches, restores, automates, or authorizes.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{WorkspaceId, WorkspaceProfileId};

/// Profile-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceProfileError {
    #[error("profiles require a workspace id")]
    MissingWorkspace,

    #[error("profile not found: {0}")]
    NotFound(String),

    #[error("profiles cannot execute, restore, launch, or authorize")]
    CannotExecute,

    #[error("profile validation failed: {0}")]
    Invalid(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Lifecycle status of a user-owned profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceProfileStatus {
    Active,
    Archived,
}

impl WorkspaceProfileStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceProfileError> {
        match value {
            "active" => Ok(Self::Active),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceProfileError::Invalid(format!(
                "unknown profile status: {other}"
            ))),
        }
    }
}

/// Kind of entity a profile member references (never owns).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceProfileMemberType {
    Application,
    Layout,
    Project,
    Task,
    WorkContext,
    Preference,
}

impl WorkspaceProfileMemberType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Application => "application",
            Self::Layout => "layout",
            Self::Project => "project",
            Self::Task => "task",
            Self::WorkContext => "work_context",
            Self::Preference => "preference",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceProfileError> {
        match value {
            "application" => Ok(Self::Application),
            "layout" => Ok(Self::Layout),
            "project" => Ok(Self::Project),
            "task" => Ok(Self::Task),
            "work_context" => Ok(Self::WorkContext),
            "preference" => Ok(Self::Preference),
            other => Err(WorkspaceProfileError::Invalid(format!(
                "unknown member type: {other}"
            ))),
        }
    }
}

/// How a member relates to the profile (descriptive only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceProfileRelationship {
    Expected,
    Preferred,
    Related,
}

impl WorkspaceProfileRelationship {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expected => "expected",
            Self::Preferred => "preferred",
            Self::Related => "related",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceProfileError> {
        match value {
            "expected" => Ok(Self::Expected),
            "preferred" => Ok(Self::Preferred),
            "related" => Ok(Self::Related),
            other => Err(WorkspaceProfileError::Invalid(format!(
                "unknown relationship: {other}"
            ))),
        }
    }
}

/// One referenced member of a profile (references existing systems; owns nothing).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfileMember {
    pub id: String,
    pub profile_id: String,
    pub member_type: WorkspaceProfileMemberType,
    pub reference_id: String,
    pub relationship: WorkspaceProfileRelationship,
    pub evidence: String,
    pub label: String,
    pub authority_effect: String,
}

impl WorkspaceProfileMember {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Durable user-owned workspace environment profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfile {
    pub id: WorkspaceProfileId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub description: String,
    pub status: WorkspaceProfileStatus,
    pub members: Vec<WorkspaceProfileMember>,
    pub created_at: String,
    pub updated_at: String,
    pub authority_effect: String,
}

impl WorkspaceProfile {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Input for creating or replacing members.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfileMemberInput {
    pub member_type: WorkspaceProfileMemberType,
    pub reference_id: String,
    pub relationship: WorkspaceProfileRelationship,
    pub evidence: String,
    pub label: String,
}

/// Alignment band for profile vs current workspace (informational).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceProfileAlignment {
    Aligned,
    Partial,
    Divergent,
    Empty,
}

impl WorkspaceProfileAlignment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Aligned => "aligned",
            Self::Partial => "partial",
            Self::Divergent => "divergent",
            Self::Empty => "empty",
        }
    }
}

/// One matching or differing evidence line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfileEvidence {
    pub label: String,
    pub member_type: String,
    pub reference_id: String,
    pub why: String,
}

/// A difference between profile expectation and current state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfileDifference {
    pub kind: String,
    pub member_type: String,
    pub reference_id: String,
    pub expected: String,
    pub observed: String,
    pub why: String,
}

/// Comparison of one profile against the current workspace (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfileComparison {
    pub profile_id: String,
    pub profile_name: String,
    pub workspace_id: String,
    pub alignment: WorkspaceProfileAlignment,
    pub matching_evidence: Vec<WorkspaceProfileEvidence>,
    pub differences: Vec<WorkspaceProfileDifference>,
    pub missing_members: Vec<WorkspaceProfileMember>,
    pub matched_count: usize,
    pub missing_count: usize,
    pub explanation: String,
    pub authority_effect: String,
}

impl WorkspaceProfileComparison {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Human-facing profile summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSummaryLines {
    pub headline: String,
    pub alignment_line: String,
    pub missing_line: String,
    pub narrative: String,
}

/// Full Profile Model read snapshot — opportunities to understand setups only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfileState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub profile_summary: ProfileSummaryLines,
    pub profiles: Vec<WorkspaceProfile>,
    pub comparisons: Vec<WorkspaceProfileComparison>,
    pub profile_count: usize,
    pub active_count: usize,
    pub best_alignment: Option<WorkspaceProfileAlignment>,
    pub best_profile_name: Option<String>,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceProfileState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceProfileSummary {
        WorkspaceProfileSummary {
            workspace_id: self.workspace_id.clone(),
            workspace_name: self.workspace_name.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            profile_summary: self.profile_summary.clone(),
            profile_count: self.profile_count,
            active_count: self.active_count,
            best_alignment: self.best_alignment.map(|a| a.as_str().to_string()),
            best_profile_name: self.best_profile_name.clone(),
            top_profiles: self.profiles.iter().take(limit).cloned().collect(),
            top_comparisons: self.comparisons.iter().take(limit).cloned().collect(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Intelligence / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfileSummary {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub profile_summary: ProfileSummaryLines,
    pub profile_count: usize,
    pub active_count: usize,
    pub best_alignment: Option<String>,
    pub best_profile_name: Option<String>,
    pub top_profiles: Vec<WorkspaceProfile>,
    pub top_comparisons: Vec<WorkspaceProfileComparison>,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceProfileSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            workspace_name: String::new(),
            generated_at: String::new(),
            label: String::new(),
            profile_summary: ProfileSummaryLines {
                headline: String::new(),
                alignment_line: String::new(),
                missing_line: String::new(),
                narrative: String::new(),
            },
            profile_count: 0,
            active_count: 0,
            best_alignment: None,
            best_profile_name: None,
            top_profiles: Vec::new(),
            top_comparisons: Vec::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceProfileState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Side-by-side comparison of two profile states (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfileStateComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceProfileStateComparison {
    pub fn compare(left: &WorkspaceProfileState, right: &WorkspaceProfileState) -> Self {
        let mut differences = Vec::new();
        if left.profile_count != right.profile_count {
            differences.push(format!(
                "profile_count: {} → {}",
                left.profile_count, right.profile_count
            ));
        }
        if left.best_profile_name != right.best_profile_name {
            differences.push(format!(
                "best_profile: {:?} → {:?}",
                left.best_profile_name, right.best_profile_name
            ));
        }
        if left.best_alignment != right.best_alignment {
            differences.push(format!(
                "best_alignment: {:?} → {:?}",
                left.best_alignment.map(|a| a.as_str()),
                right.best_alignment.map(|a| a.as_str())
            ));
        }
        if left.summary != right.summary {
            differences.push("summary differs".into());
        }
        if differences.is_empty() {
            differences.push("no material profile differences".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
            authority_effect: WorkspaceProfileState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Validate profile / state invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProfileValidation {
    pub valid: bool,
    pub messages: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceProfileValidation {
    pub fn validate_state(state: &WorkspaceProfileState) -> Self {
        let mut messages = Vec::new();
        if state.authority_effect != WorkspaceProfileState::AUTHORITY_EFFECT_NONE {
            messages.push("authority_effect must be none".into());
        }
        if state.explanation.is_empty() {
            messages.push("explanation required".into());
        }
        for profile in &state.profiles {
            if profile.authority_effect != WorkspaceProfile::AUTHORITY_EFFECT_NONE {
                messages.push(format!("profile {} has authority", profile.id));
            }
            for member in &profile.members {
                if member.reference_id.trim().is_empty() {
                    messages.push(format!("profile {} member missing reference", profile.id));
                }
                if member.authority_effect != WorkspaceProfileMember::AUTHORITY_EFFECT_NONE {
                    messages.push(format!("member {} has authority", member.id));
                }
            }
        }
        for cmp in &state.comparisons {
            if cmp.authority_effect != WorkspaceProfileComparison::AUTHORITY_EFFECT_NONE {
                messages.push(format!("comparison {} has authority", cmp.profile_id));
            }
        }
        let valid = messages.is_empty();
        if valid {
            messages.push(format!(
                "Valid: authority none · {} profile(s) · active {} — describes setups only, never executes",
                state.profile_count, state.active_count
            ));
        }
        Self {
            valid,
            messages,
            authority_effect: WorkspaceProfileState::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn validate_profile(profile: &WorkspaceProfile) -> Self {
        let mut messages = Vec::new();
        if profile.name.trim().is_empty() {
            messages.push("name required".into());
        }
        if profile.authority_effect != WorkspaceProfile::AUTHORITY_EFFECT_NONE {
            messages.push("authority_effect must be none".into());
        }
        for member in &profile.members {
            if member.reference_id.trim().is_empty() {
                messages.push(format!("member {} missing reference_id", member.id));
            }
            if member.evidence.trim().is_empty() {
                messages.push(format!("member {} missing evidence", member.id));
            }
        }
        let valid = messages.is_empty();
        if valid {
            messages.push(format!(
                "Valid profile \"{}\" with {} member(s) — user-owned description only",
                profile.name,
                profile.members.len()
            ));
        }
        Self {
            valid,
            messages,
            authority_effect: WorkspaceProfile::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// RFC3339 timestamp helper.
pub fn profile_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

/// Validate workspace id for profile operations.
pub fn validate_profile_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceProfileError> {
    let raw = workspace_id.into();
    if raw.trim().is_empty() {
        return Err(WorkspaceProfileError::MissingWorkspace);
    }
    WorkspaceId::new(raw).map_err(WorkspaceProfileError::Domain)
}

/// Compact one-line summary.
pub fn build_profile_summary(label: &str, count: usize) -> String {
    format!(
        "Workspace Profiles for \"{label}\": {count} setup(s). \
         User-owned descriptions — never launch, restore, or execute."
    )
}
