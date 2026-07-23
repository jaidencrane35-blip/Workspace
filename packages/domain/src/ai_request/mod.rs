//! AI action request — "what does AI want to do?" (Sprint 46).
//!
//! Pure domain boundary between AI reasoning and system execution.
//! This type does **not** execute commands, grant capabilities, or bypass
//! the Permission Gateway. It only describes a proposed action.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ids::{ActionIntentId, ActorId, ApplicationId};
use crate::resource::{ResourceId, ResourceKind, ResourceRef};

/// Errors when constructing or validating an AI action request.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AiRequestError {
    #[error("AI request actor id must not be empty")]
    EmptyActorId,

    #[error("AI request command name must not be empty")]
    EmptyCommandName,

    #[error("AI request requires a target resource")]
    MissingTarget,

    #[error("AI request target must be an application")]
    InvalidTargetKind,

    #[error("AI request reason exceeds maximum length")]
    ReasonTooLong,

    #[error(transparent)]
    Domain(#[from] crate::errors::DomainError),
}

/// Minimum representation of an AI-proposed action.
///
/// Captures *what* the AI wants — not internal reasoning traces or model output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiActionRequest {
    pub requesting_actor_id: ActorId,
    pub action_intent_id: ActionIntentId,
    pub target_resource: Option<ResourceRef>,
    pub command_name: String,
    pub reason: Option<String>,
    pub created_at: String,
}

const MAX_REASON_LEN: usize = 500;
const LAUNCH_APPLICATION_INTENT: &str = "launch-application";
const LAUNCH_APPLICATION_COMMAND: &str = "LaunchApplication";

impl AiActionRequest {
    /// Proposes a governed application launch for an AI actor.
    pub fn propose_application_launch(
        actor_id: impl Into<String>,
        application_id: &ApplicationId,
        reason: Option<String>,
    ) -> Result<Self, AiRequestError> {
        let requesting_actor_id = ActorId::new(actor_id)?;
        let reason = normalize_reason(reason)?;
        let request = Self {
            requesting_actor_id,
            action_intent_id: ActionIntentId::new(LAUNCH_APPLICATION_INTENT)?,
            target_resource: Some(ResourceRef::new(
                ResourceKind::Application,
                ResourceId::new(application_id.as_str())?,
            )),
            command_name: LAUNCH_APPLICATION_COMMAND.into(),
            reason,
            created_at: Utc::now().to_rfc3339(),
        };
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<(), AiRequestError> {
        if self.requesting_actor_id.as_str().trim().is_empty() {
            return Err(AiRequestError::EmptyActorId);
        }
        if self.command_name.trim().is_empty() {
            return Err(AiRequestError::EmptyCommandName);
        }
        if let Some(reason) = &self.reason {
            if reason.len() > MAX_REASON_LEN {
                return Err(AiRequestError::ReasonTooLong);
            }
        }
        if self.command_name == LAUNCH_APPLICATION_COMMAND {
            let Some(target) = &self.target_resource else {
                return Err(AiRequestError::MissingTarget);
            };
            if target.kind != ResourceKind::Application {
                return Err(AiRequestError::InvalidTargetKind);
            }
        }
        Ok(())
    }

    /// Extracts the application id when this request targets `LaunchApplication`.
    pub fn application_id(&self) -> Result<ApplicationId, AiRequestError> {
        let Some(target) = &self.target_resource else {
            return Err(AiRequestError::MissingTarget);
        };
        if target.kind != ResourceKind::Application {
            return Err(AiRequestError::InvalidTargetKind);
        }
        Ok(ApplicationId::new(target.id.as_str())?)
    }
}

fn normalize_reason(reason: Option<String>) -> Result<Option<String>, AiRequestError> {
    match reason {
        None => Ok(None),
        Some(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                Ok(None)
            } else if trimmed.len() > MAX_REASON_LEN {
                Err(AiRequestError::ReasonTooLong)
            } else {
                Ok(Some(trimmed.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_launch_describes_what_not_how() {
        let app = ApplicationId::new("app-1").unwrap();
        let request = AiActionRequest::propose_application_launch(
            "diagnostic-ai",
            &app,
            Some("User often opens Notepad in this layout".into()),
        )
        .unwrap();

        assert_eq!(request.requesting_actor_id.as_str(), "diagnostic-ai");
        assert_eq!(request.command_name, "LaunchApplication");
        assert_eq!(request.action_intent_id.as_str(), "launch-application");
        assert_eq!(request.application_id().unwrap().as_str(), "app-1");
        assert!(request.reason.as_deref().unwrap().contains("Notepad"));
    }

    #[test]
    fn rejects_oversized_reason() {
        let app = ApplicationId::new("app-1").unwrap();
        let reason = "x".repeat(MAX_REASON_LEN + 1);
        let error = AiActionRequest::propose_application_launch("ai-1", &app, Some(reason))
            .unwrap_err();
        assert_eq!(error, AiRequestError::ReasonTooLong);
    }
}
