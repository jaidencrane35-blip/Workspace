//! Governed AI memory — intelligence enhancement only (Sprints 60–61).
//!
//! Memory improves context, recommendations, and planning quality.
//! Memory must never create authority, permissions, or execution ability.
//!
//! Aligns with DEC-014 Memory Policy: inspectable, removable, no hidden learning.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ids::{MemoryEntryId, WorkspaceId};

/// Memory-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AiMemoryError {
    #[error("Memory summary must not be empty")]
    EmptySummary,

    #[error("Memory summary exceeds maximum length")]
    SummaryTooLong,

    #[error("Memory source must not be empty")]
    EmptySource,

    #[error("Memory key must not be empty")]
    EmptyKey,

    #[error("Invalid memory type: {0}")]
    InvalidType(String),

    #[error("Invalid memory lifecycle state: {0}")]
    InvalidLifecycle(String),

    #[error(transparent)]
    Domain(#[from] crate::errors::DomainError),
}

const MAX_SUMMARY_LEN: usize = 500;
const MAX_KEY_LEN: usize = 120;
const MAX_SOURCE_LEN: usize = 120;

/// Memory category separation (Sprint 60).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryType {
    /// Temporary conversation/task context.
    Session,
    /// Workspace-scoped facts (preferred apps, layouts, project context).
    Workspace,
    /// Explicit user preference choices.
    UserPreference,
    /// Stable application/system knowledge (capability descriptions, docs).
    SystemKnowledge,
}

impl MemoryType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Session => "session",
            Self::Workspace => "workspace",
            Self::UserPreference => "user_preference",
            Self::SystemKnowledge => "system_knowledge",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AiMemoryError> {
        match value {
            "session" => Ok(Self::Session),
            "workspace" => Ok(Self::Workspace),
            "user_preference" => Ok(Self::UserPreference),
            "system_knowledge" => Ok(Self::SystemKnowledge),
            other => Err(AiMemoryError::InvalidType(other.to_string())),
        }
    }
}

/// Lifecycle of a stored memory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryLifecycleState {
    Created,
    Updated,
    Viewed,
    Deleted,
}

impl MemoryLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Viewed => "viewed",
            Self::Deleted => "deleted",
        }
    }

    pub fn parse(value: &str) -> Result<Self, AiMemoryError> {
        match value {
            "created" => Ok(Self::Created),
            "updated" => Ok(Self::Updated),
            "viewed" => Ok(Self::Viewed),
            "deleted" => Ok(Self::Deleted),
            other => Err(AiMemoryError::InvalidLifecycle(other.to_string())),
        }
    }
}

/// Confidence/quality metadata (L0–L4 alignment; informational only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryMetadata {
    pub confidence_level: u8,
    pub occurrence_count: u32,
    pub user_visible: bool,
    /// Optional structured payload (policy-safe; no CoT / credentials).
    pub attributes: Option<String>,
}

impl MemoryMetadata {
    pub fn default_visible() -> Self {
        Self {
            confidence_level: 2,
            occurrence_count: 1,
            user_visible: true,
            attributes: None,
        }
    }

    pub fn with_attribute_json(mut self, attributes: impl Into<String>) -> Self {
        self.attributes = Some(attributes.into());
        self
    }
}

/// A single governed memory entry — never authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: MemoryEntryId,
    pub memory_type: MemoryType,
    pub key: String,
    pub summary: String,
    pub source: String,
    pub workspace_id: Option<WorkspaceId>,
    pub lifecycle: MemoryLifecycleState,
    pub metadata: MemoryMetadata,
    pub created_at: String,
    pub updated_at: String,
}

impl MemoryEntry {
    pub fn new(
        memory_type: MemoryType,
        key: impl Into<String>,
        summary: impl Into<String>,
        source: impl Into<String>,
        workspace_id: Option<WorkspaceId>,
        metadata: MemoryMetadata,
    ) -> Result<Self, AiMemoryError> {
        let key = normalize_required(key.into(), MAX_KEY_LEN, true)?;
        let summary = normalize_required(summary.into(), MAX_SUMMARY_LEN, false)?;
        let source = normalize_required(source.into(), MAX_SOURCE_LEN, true)?;
        if key.is_empty() {
            return Err(AiMemoryError::EmptyKey);
        }
        if summary.is_empty() {
            return Err(AiMemoryError::EmptySummary);
        }
        if source.is_empty() {
            return Err(AiMemoryError::EmptySource);
        }
        let now = Utc::now().to_rfc3339();
        Ok(Self {
            id: MemoryEntryId::generate(),
            memory_type,
            key,
            summary,
            source,
            workspace_id,
            lifecycle: MemoryLifecycleState::Created,
            metadata,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn mark_updated(&mut self) {
        self.lifecycle = MemoryLifecycleState::Updated;
        self.updated_at = Utc::now().to_rfc3339();
    }

    pub fn mark_viewed(&mut self) {
        self.lifecycle = MemoryLifecycleState::Viewed;
        self.updated_at = Utc::now().to_rfc3339();
    }

    pub fn mark_deleted(&mut self) {
        self.lifecycle = MemoryLifecycleState::Deleted;
        self.updated_at = Utc::now().to_rfc3339();
    }

    pub fn is_active(&self) -> bool {
        self.lifecycle != MemoryLifecycleState::Deleted
    }
}

/// Bounded hints injected into planning context (not long-term authority).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMemoryAwareness {
    pub entries: Vec<MemoryEntry>,
    pub assembled_at: String,
}

impl AiMemoryAwareness {
    pub fn from_entries(entries: Vec<MemoryEntry>) -> Self {
        Self {
            entries,
            assembled_at: Utc::now().to_rfc3339(),
        }
    }

    /// Preferred application ids from workspace/preference memories (informational).
    pub fn preferred_application_ids(&self) -> Vec<String> {
        let mut ids = Vec::new();
        for entry in &self.entries {
            if !entry.is_active() {
                continue;
            }
            if entry.key == "preferred_application" {
                if let Some(attributes) = &entry.metadata.attributes {
                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(attributes) {
                        if let Some(id) = value.get("application_id").and_then(|v| v.as_str()) {
                            if !ids.iter().any(|existing| existing == id) {
                                ids.push(id.to_string());
                            }
                        }
                    }
                }
            }
        }
        ids
    }

    pub fn planning_notes(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter(|entry| entry.is_active() && entry.metadata.user_visible)
            .take(10)
            .map(|entry| format!("[{}] {}", entry.memory_type.as_str(), entry.summary))
            .collect()
    }
}

fn normalize_required(
    value: String,
    max_len: usize,
    _as_key: bool,
) -> Result<String, AiMemoryError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    if trimmed.len() > max_len {
        return Err(AiMemoryError::SummaryTooLong);
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_workspace_memory_without_authority_fields() {
        let entry = MemoryEntry::new(
            MemoryType::Workspace,
            "preferred_application",
            "User often launches VS Code in this workspace",
            "diagnostic",
            None,
            MemoryMetadata::default_visible()
                .with_attribute_json(r#"{"application_id":"app-1"}"#),
        )
        .unwrap();
        assert!(entry.is_active());
        assert_eq!(entry.memory_type, MemoryType::Workspace);
    }

    #[test]
    fn awareness_extracts_preferred_application_ids() {
        let entry = MemoryEntry::new(
            MemoryType::Workspace,
            "preferred_application",
            "Prefer VS Code",
            "diagnostic",
            None,
            MemoryMetadata::default_visible()
                .with_attribute_json(r#"{"application_id":"app-1"}"#),
        )
        .unwrap();
        let awareness = AiMemoryAwareness::from_entries(vec![entry]);
        assert_eq!(awareness.preferred_application_ids(), vec!["app-1".to_string()]);
    }
}
