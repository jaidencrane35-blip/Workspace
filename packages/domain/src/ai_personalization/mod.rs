//! Governed personalization — explicit user preferences only (Sprints 64–65).
//!
//! Personalization answers "What does this user prefer?"
//! It must never answer "What is this AI allowed to do?"
//!
//! No behavioral profiling or automatic long-term inference in this batch.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ai_planning::AiPlan;
use crate::errors::DomainError;
use crate::ids::{UserPreferenceId, WorkspaceId};

/// Personalization-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AiPersonalizationError {
    #[error("Preference key must not be empty")]
    EmptyKey,

    #[error("Preference value must not be empty")]
    EmptyValue,

    #[error("Preference source must not be empty")]
    EmptySource,

    #[error("Preference value exceeds maximum length")]
    ValueTooLong,

    #[error("Invalid preference category: {0}")]
    InvalidCategory(String),

    #[error("Invalid preference source: {0}")]
    InvalidSource(String),

    #[error("Preference is not editable")]
    NotEditable,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

const MAX_KEY_LEN: usize = 120;
const MAX_VALUE_LEN: usize = 500;
const MAX_LABEL_LEN: usize = 200;

/// Preference category separation (Sprint 64).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreferenceCategory {
    Workflow,
    Application,
    Layout,
    Communication,
    Planning,
}

impl PreferenceCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Workflow => "workflow",
            Self::Application => "application",
            Self::Layout => "layout",
            Self::Communication => "communication",
            Self::Planning => "planning",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AiPersonalizationError> {
        match value {
            "workflow" => Ok(Self::Workflow),
            "application" => Ok(Self::Application),
            "layout" => Ok(Self::Layout),
            "communication" => Ok(Self::Communication),
            "planning" => Ok(Self::Planning),
            other => Err(AiPersonalizationError::InvalidCategory(other.to_string())),
        }
    }
}

/// Explicit preference provenance — no automatic inference in this batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreferenceSource {
    UserDefined,
    UserConfirmed,
    Imported,
}

impl PreferenceSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UserDefined => "user_defined",
            Self::UserConfirmed => "user_confirmed",
            Self::Imported => "imported",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AiPersonalizationError> {
        match value {
            "user_defined" => Ok(Self::UserDefined),
            "user_confirmed" => Ok(Self::UserConfirmed),
            "imported" => Ok(Self::Imported),
            other => Err(AiPersonalizationError::InvalidSource(other.to_string())),
        }
    }
}

/// A single explicit, inspectable user preference — never authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPreference {
    pub id: UserPreferenceId,
    pub category: PreferenceCategory,
    pub key: String,
    /// Human-readable preference statement / value.
    pub value: String,
    /// Optional display label (e.g. application name).
    pub label: Option<String>,
    pub source: PreferenceSource,
    pub confidence: u8,
    pub scope_workspace_id: Option<WorkspaceId>,
    pub editable: bool,
    /// Optional structured payload (e.g. application_id). Policy-safe only.
    pub attributes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

impl UserPreference {
    pub fn new(
        category: PreferenceCategory,
        key: impl Into<String>,
        value: impl Into<String>,
        source: PreferenceSource,
        scope_workspace_id: Option<WorkspaceId>,
        label: Option<String>,
        attributes: Option<String>,
    ) -> Result<Self, AiPersonalizationError> {
        let key = normalize(key.into(), MAX_KEY_LEN)?;
        let value = normalize(value.into(), MAX_VALUE_LEN)?;
        if key.is_empty() {
            return Err(AiPersonalizationError::EmptyKey);
        }
        if value.is_empty() {
            return Err(AiPersonalizationError::EmptyValue);
        }
        let label = match label {
            None => None,
            Some(raw) => {
                let trimmed = normalize(raw, MAX_LABEL_LEN)?;
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            }
        };
        let now = Utc::now().to_rfc3339();
        Ok(Self {
            id: UserPreferenceId::generate(),
            category,
            key,
            value,
            label,
            source,
            confidence: 4, // explicit preferences start high (informational only)
            scope_workspace_id,
            editable: true,
            attributes,
            created_at: now.clone(),
            updated_at: now,
            deleted: false,
        })
    }

    pub fn apply_update(
        &mut self,
        value: Option<String>,
        label: Option<Option<String>>,
        attributes: Option<Option<String>>,
        confidence: Option<u8>,
    ) -> Result<(), AiPersonalizationError> {
        if !self.editable {
            return Err(AiPersonalizationError::NotEditable);
        }
        if let Some(value) = value {
            let value = normalize(value, MAX_VALUE_LEN)?;
            if value.is_empty() {
                return Err(AiPersonalizationError::EmptyValue);
            }
            self.value = value;
        }
        if let Some(label) = label {
            self.label = match label {
                None => None,
                Some(raw) => {
                    let trimmed = normalize(raw, MAX_LABEL_LEN)?;
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed)
                    }
                }
            };
        }
        if let Some(attributes) = attributes {
            self.attributes = attributes;
        }
        if let Some(confidence) = confidence {
            self.confidence = confidence.min(4);
        }
        self.updated_at = Utc::now().to_rfc3339();
        Ok(())
    }

    pub fn mark_deleted(&mut self) {
        self.deleted = true;
        self.updated_at = Utc::now().to_rfc3339();
    }

    pub fn is_active(&self) -> bool {
        !self.deleted
    }
}

/// Inspectable preference profile for a user/session context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPreferenceProfile {
    pub preferences: Vec<UserPreference>,
    /// Whether personalization is enabled for planning (mirrors settings).
    pub personalization_enabled: bool,
    pub assembled_at: String,
}

impl UserPreferenceProfile {
    pub fn from_preferences(
        preferences: Vec<UserPreference>,
        personalization_enabled: bool,
    ) -> Self {
        Self {
            preferences,
            personalization_enabled,
            assembled_at: Utc::now().to_rfc3339(),
        }
    }
}

/// Side-by-side diagnostic: personalized vs neutral planning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonalizedPlanComparison {
    pub personalized: AiPlan,
    pub neutral: AiPlan,
}

/// Bounded preference hints injected into planning (never authority).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiPersonalizationAwareness {
    pub preferences: Vec<UserPreference>,
    pub enabled: bool,
    pub assembled_at: String,
}

impl AiPersonalizationAwareness {
    pub fn from_preferences(preferences: Vec<UserPreference>, enabled: bool) -> Self {
        Self {
            preferences: preferences.into_iter().filter(|p| p.is_active()).collect(),
            enabled,
            assembled_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn empty_disabled() -> Self {
        Self {
            preferences: Vec::new(),
            enabled: false,
            assembled_at: Utc::now().to_rfc3339(),
        }
    }

    /// Preferred application ids from application preferences (informational).
    pub fn preferred_application_ids(&self) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }
        let mut ids = Vec::new();
        for preference in &self.preferences {
            if preference.category != PreferenceCategory::Application {
                continue;
            }
            if preference.key != "preferred_application"
                && preference.key != "default_editor"
                && preference.key != "default_application"
            {
                continue;
            }
            if let Some(attributes) = &preference.attributes {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(attributes) {
                    if let Some(id) = value.get("application_id").and_then(|v| v.as_str()) {
                        if !ids.iter().any(|existing| existing == id) {
                            ids.push(id.to_string());
                        }
                    }
                }
            }
        }
        ids
    }

    /// Explanation snippets for applied preferences (deterministic, bounded).
    pub fn applied_explanations(&self) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }
        self.preferences
            .iter()
            .filter(|preference| preference.is_active())
            .take(10)
            .filter_map(|preference| explanation_for(preference))
            .collect()
    }

    pub fn explanation_for_application(&self, application_id: &str) -> Option<String> {
        if !self.enabled {
            return None;
        }
        self.preferences.iter().find_map(|preference| {
            if !preference.is_active() {
                return None;
            }
            let matches = preference
                .attributes
                .as_ref()
                .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw).ok())
                .and_then(|value| {
                    value
                        .get("application_id")
                        .and_then(|v| v.as_str())
                        .map(|id| id == application_id)
                })
                .unwrap_or(false);
            if matches {
                explanation_for(preference)
            } else {
                None
            }
        })
    }
}

fn explanation_for(preference: &UserPreference) -> Option<String> {
    let label = preference
        .label
        .clone()
        .unwrap_or_else(|| preference.value.clone());
    match preference.key.as_str() {
        "preferred_application" | "default_editor" | "default_application" => Some(format!(
            "Preferred because you marked {label} as your default editor."
        )),
        "workflow_style" => Some(format!(
            "Preferred because you set workflow style to '{label}'."
        )),
        "planning_style" => Some(format!(
            "Preferred because you set planning style to '{label}'."
        )),
        _ => Some(format!(
            "Preferred because of your {} preference: {}.",
            preference.category.as_str(),
            label
        )),
    }
}

fn normalize(value: String, max_len: usize) -> Result<String, AiPersonalizationError> {
    let trimmed = value.trim();
    if trimmed.len() > max_len {
        return Err(AiPersonalizationError::ValueTooLong);
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preferred_application_ranks_and_explains() {
        let preference = UserPreference::new(
            PreferenceCategory::Application,
            "default_editor",
            "VS Code is my default editor",
            PreferenceSource::UserDefined,
            None,
            Some("VS Code".into()),
            Some(r#"{"application_id":"app-1"}"#.into()),
        )
        .unwrap();
        let awareness = AiPersonalizationAwareness::from_preferences(vec![preference], true);
        assert_eq!(awareness.preferred_application_ids(), vec!["app-1".to_string()]);
        assert!(awareness
            .explanation_for_application("app-1")
            .unwrap()
            .contains("VS Code"));
    }

    #[test]
    fn disabled_personalization_has_no_influence() {
        let preference = UserPreference::new(
            PreferenceCategory::Application,
            "default_editor",
            "VS Code",
            PreferenceSource::UserDefined,
            None,
            Some("VS Code".into()),
            Some(r#"{"application_id":"app-1"}"#.into()),
        )
        .unwrap();
        let awareness = AiPersonalizationAwareness::from_preferences(vec![preference], false);
        assert!(awareness.preferred_application_ids().is_empty());
        assert!(awareness.applied_explanations().is_empty());
    }
}
