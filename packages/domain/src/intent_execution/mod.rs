//! Governed intent execution boundary — pure domain types (Sprint 24).
//!
//! An [`IntentExecutionRequest`] records an explicit execution request derived
//! from an approved suggestion intent bridge. Domain only — no database, kernel,
//! or UI dependencies.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::actor::ActorType;
use crate::ids::ActionIntentId;

/// Lifecycle status of a governed execution request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentExecutionStatus {
    Requested,
    Authorized,
    Executed,
    Failed,
    Cancelled,
}

impl IntentExecutionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Authorized => "authorized",
            Self::Executed => "executed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(value: &str) -> Result<Self, IntentExecutionError> {
        match value {
            "requested" => Ok(Self::Requested),
            "authorized" => Ok(Self::Authorized),
            "executed" => Ok(Self::Executed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(IntentExecutionError::InvalidStatus(value.to_string())),
        }
    }
}

/// Execution-boundary validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum IntentExecutionError {
    #[error("Intent execution id must not be empty")]
    EmptyId,

    #[error("Intent execution suggestion id must not be empty")]
    EmptySuggestionId,

    #[error("Intent execution requires an action intent id")]
    MissingIntentId,

    #[error("Intent execution timestamp must not be empty")]
    EmptyTimestamp,

    #[error("Invalid intent execution status: {0}")]
    InvalidStatus(String),

    #[error("Intent execution '{id}' cannot transition from {from:?} to {to:?}")]
    InvalidTransition {
        id: String,
        from: IntentExecutionStatus,
        to: IntentExecutionStatus,
    },
}

/// An explicit, governed execution request for a mapped action intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentExecutionRequest {
    pub id: String,
    pub suggestion_id: String,
    pub action_intent_id: ActionIntentId,
    pub actor_type: ActorType,
    pub actor_id: Option<String>,
    pub status: IntentExecutionStatus,
    pub created_at: String,
}

impl IntentExecutionRequest {
    pub fn new(
        id: String,
        suggestion_id: String,
        action_intent_id: ActionIntentId,
        actor_type: ActorType,
        actor_id: Option<String>,
    ) -> Self {
        Self {
            id,
            suggestion_id,
            action_intent_id,
            actor_type,
            actor_id,
            status: IntentExecutionStatus::Requested,
            created_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn validate(&self) -> Result<(), IntentExecutionError> {
        if self.id.trim().is_empty() {
            return Err(IntentExecutionError::EmptyId);
        }
        if self.suggestion_id.trim().is_empty() {
            return Err(IntentExecutionError::EmptySuggestionId);
        }
        if self.action_intent_id.as_str().trim().is_empty() {
            return Err(IntentExecutionError::MissingIntentId);
        }
        if self.created_at.trim().is_empty() {
            return Err(IntentExecutionError::EmptyTimestamp);
        }
        IntentExecutionStatus::parse(self.status.as_str())?;
        Ok(())
    }

    pub fn authorize(self) -> Result<Self, IntentExecutionError> {
        self.transition(IntentExecutionStatus::Authorized)
    }

    pub fn mark_executed(self) -> Result<Self, IntentExecutionError> {
        self.transition(IntentExecutionStatus::Executed)
    }

    pub fn mark_failed(self) -> Result<Self, IntentExecutionError> {
        self.transition(IntentExecutionStatus::Failed)
    }

    fn transition(mut self, to: IntentExecutionStatus) -> Result<Self, IntentExecutionError> {
        let allowed = matches!(
            (self.status, to),
            (IntentExecutionStatus::Requested, IntentExecutionStatus::Authorized)
                | (IntentExecutionStatus::Authorized, IntentExecutionStatus::Executed)
                | (IntentExecutionStatus::Authorized, IntentExecutionStatus::Failed)
                | (IntentExecutionStatus::Requested, IntentExecutionStatus::Cancelled)
                | (IntentExecutionStatus::Authorized, IntentExecutionStatus::Cancelled)
        );
        if !allowed {
            return Err(IntentExecutionError::InvalidTransition {
                id: self.id.clone(),
                from: self.status,
                to,
            });
        }
        self.status = to;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_request() -> IntentExecutionRequest {
        IntentExecutionRequest::new(
            "execution:s-1".into(),
            "s-1".into(),
            ActionIntentId::new("create-zone").unwrap(),
            ActorType::LocalUser,
            Some("local-user".into()),
        )
    }

    #[test]
    fn valid_request_passes_validation() {
        assert!(sample_request().validate().is_ok());
    }

    #[test]
    fn rejects_empty_id() {
        let mut request = sample_request();
        request.id = "  ".into();
        assert_eq!(
            request.validate(),
            Err(IntentExecutionError::EmptyId)
        );
    }

    #[test]
    fn rejects_empty_suggestion_id() {
        let mut request = sample_request();
        request.suggestion_id.clear();
        assert_eq!(
            request.validate(),
            Err(IntentExecutionError::EmptySuggestionId)
        );
    }

    #[test]
    fn rejects_empty_timestamp() {
        let mut request = sample_request();
        request.created_at.clear();
        assert_eq!(
            request.validate(),
            Err(IntentExecutionError::EmptyTimestamp)
        );
    }

    #[test]
    fn rejects_missing_intent_id() {
        let request: IntentExecutionRequest = serde_json::from_str(
            r#"{
                "id": "execution:s-1",
                "suggestion_id": "s-1",
                "action_intent_id": "",
                "actor_type": "local_user",
                "actor_id": null,
                "status": "requested",
                "created_at": "2024-01-01T00:00:00Z"
            }"#,
        )
        .unwrap();
        assert_eq!(
            request.validate(),
            Err(IntentExecutionError::MissingIntentId)
        );
    }

    #[test]
    fn status_transitions_follow_governed_path() {
        let authorized = sample_request().authorize().unwrap();
        assert_eq!(authorized.status, IntentExecutionStatus::Authorized);

        let executed = authorized.mark_executed().unwrap();
        assert_eq!(executed.status, IntentExecutionStatus::Executed);
    }

    #[test]
    fn invalid_status_transition_rejected() {
        let request = sample_request();
        assert!(matches!(
            request.mark_executed(),
            Err(IntentExecutionError::InvalidTransition { .. })
        ));
    }
}
