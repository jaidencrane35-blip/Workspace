//! Approval-gated suggestion → intent bridge — pure domain types (Sprint 23).
//!
//! A [`SuggestionIntentRequest`] records that an accepted suggestion may become
//! an actionable intent. It is NOT execution: the bridge stops before command
//! dispatch. Pure domain only — no database, kernel, or UI dependencies.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::actor::ActorType;
use crate::ids::ActionIntentId;
use crate::resource::ResourceRef;
use crate::suggestion::SuggestionType;

/// Bridge-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SuggestionIntentError {
    #[error("Suggestion intent suggestion id must not be empty")]
    EmptySuggestionId,

    #[error("Suggestion intent timestamp must not be empty")]
    EmptyTimestamp,

    #[error("Suggestion intent requires an action intent id")]
    MissingIntentId,

    #[error("No action intent mapping exists for suggestion type {0:?}")]
    UnmappedSuggestionType(SuggestionType),

    #[error("Suggestion '{0}' is not in accepted lifecycle state")]
    NotAccepted(String),

    #[error("Suggestion '{0}' was rejected")]
    Rejected(String),
}

/// An approval-gated bridge record linking an accepted suggestion to an intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestionIntentRequest {
    pub suggestion_id: String,
    pub intent_id: ActionIntentId,
    pub resource_ref: Option<ResourceRef>,
    pub created_at: String,
    pub actor_type: ActorType,
    pub metadata: Option<String>,
}

impl SuggestionIntentRequest {
    pub fn new(
        suggestion_id: String,
        intent_id: ActionIntentId,
        resource_ref: Option<ResourceRef>,
        actor_type: ActorType,
    ) -> Self {
        Self {
            suggestion_id,
            intent_id,
            resource_ref,
            created_at: Utc::now().to_rfc3339(),
            actor_type,
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: Option<String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn validate(&self) -> Result<(), SuggestionIntentError> {
        if self.suggestion_id.trim().is_empty() {
            return Err(SuggestionIntentError::EmptySuggestionId);
        }
        if self.created_at.trim().is_empty() {
            return Err(SuggestionIntentError::EmptyTimestamp);
        }
        if self.intent_id.as_str().trim().is_empty() {
            return Err(SuggestionIntentError::MissingIntentId);
        }
        Ok(())
    }
}

/// Deterministic, static mapping from suggestion type to action intent.
///
/// No AI, no learning, no database registry. Returns `None` when no binding
/// exists (callers should treat as validation failure).
pub fn map_suggestion_type_to_intent(
    suggestion_type: SuggestionType,
) -> Result<ActionIntentId, SuggestionIntentError> {
    let id = match suggestion_type {
        SuggestionType::ResourceGrowth => "create-zone",
        SuggestionType::LayoutActivity => "get-layout-snapshot",
        SuggestionType::WorkspaceActivity => "get-audit-history",
    };
    ActionIntentId::new(id)
        .map_err(|_| SuggestionIntentError::UnmappedSuggestionType(suggestion_type))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::{ResourceId, ResourceKind};

    fn workspace_ref() -> ResourceRef {
        ResourceRef::new(ResourceKind::Workspace, ResourceId::new("ws-1").unwrap())
    }

    #[test]
    fn valid_request_passes_validation() {
        let request = SuggestionIntentRequest::new(
            "resource-growth:workspace:ws-1".into(),
            ActionIntentId::new("create-zone").unwrap(),
            Some(workspace_ref()),
            ActorType::LocalUser,
        );
        assert!(request.validate().is_ok());
    }

    #[test]
    fn rejects_empty_suggestion_id() {
        let request = SuggestionIntentRequest::new(
            "   ".into(),
            ActionIntentId::new("create-zone").unwrap(),
            None,
            ActorType::LocalUser,
        );
        assert_eq!(
            request.validate(),
            Err(SuggestionIntentError::EmptySuggestionId)
        );
    }

    #[test]
    fn rejects_empty_timestamp() {
        let mut request = SuggestionIntentRequest::new(
            "s-1".into(),
            ActionIntentId::new("create-zone").unwrap(),
            None,
            ActorType::LocalUser,
        );
        request.created_at = String::new();
        assert_eq!(
            request.validate(),
            Err(SuggestionIntentError::EmptyTimestamp)
        );
    }

    #[test]
    fn rejects_missing_intent_id() {
        let request: SuggestionIntentRequest = serde_json::from_str(
            r#"{
                "suggestion_id": "s-1",
                "intent_id": "",
                "resource_ref": null,
                "created_at": "2024-01-01T00:00:00Z",
                "actor_type": "local_user",
                "metadata": null
            }"#,
        )
        .unwrap();
        assert_eq!(
            request.validate(),
            Err(SuggestionIntentError::MissingIntentId)
        );
    }

    #[test]
    fn static_mapping_covers_all_suggestion_types() {
        assert_eq!(
            map_suggestion_type_to_intent(SuggestionType::ResourceGrowth).unwrap().as_str(),
            "create-zone"
        );
        assert_eq!(
            map_suggestion_type_to_intent(SuggestionType::LayoutActivity)
                .unwrap()
                .as_str(),
            "get-layout-snapshot"
        );
        assert_eq!(
            map_suggestion_type_to_intent(SuggestionType::WorkspaceActivity)
                .unwrap()
                .as_str(),
            "get-audit-history"
        );
    }
}
