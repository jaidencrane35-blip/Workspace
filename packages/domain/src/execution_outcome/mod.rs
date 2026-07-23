//! Audit-derived execution outcomes — pure domain types (Sprint 25).
//!
//! An [`ExecutionOutcome`] answers whether a governed execution request completed,
//! failed, or was cancelled by classifying already-recorded audit events. Pure
//! domain only — no database, kernel, or UI dependencies.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::audit::AuditEvent;

/// Outcome status of a governed execution request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOutcomeStatus {
    Completed,
    Failed,
    Cancelled,
}

impl ExecutionOutcomeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ExecutionOutcomeError> {
        match value {
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(ExecutionOutcomeError::InvalidStatus(value.to_string())),
        }
    }
}

/// Outcome-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExecutionOutcomeError {
    #[error("Execution outcome request id must not be empty")]
    EmptyExecutionRequestId,

    #[error("Execution outcome command name must not be empty")]
    EmptyCommandName,

    #[error("Execution outcome timestamp must not be empty")]
    EmptyTimestamp,

    #[error("Invalid execution outcome status: {0}")]
    InvalidStatus(String),
}

/// A derived outcome record for one governed execution request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionOutcome {
    pub execution_request_id: String,
    pub status: ExecutionOutcomeStatus,
    pub command_name: String,
    pub completed_at: String,
    pub success: bool,
    /// Sanitized public error code only — never internal details.
    pub failure_reason: Option<String>,
    pub suggestion_id: Option<String>,
    pub intent_id: Option<String>,
}

impl ExecutionOutcome {
    pub fn validate(&self) -> Result<(), ExecutionOutcomeError> {
        if self.execution_request_id.trim().is_empty() {
            return Err(ExecutionOutcomeError::EmptyExecutionRequestId);
        }
        if self.command_name.trim().is_empty() {
            return Err(ExecutionOutcomeError::EmptyCommandName);
        }
        if self.completed_at.trim().is_empty() {
            return Err(ExecutionOutcomeError::EmptyTimestamp);
        }
        ExecutionOutcomeStatus::parse(self.status.as_str())?;
        Ok(())
    }
}

/// Classifies an audit event into an execution outcome status when applicable.
///
/// Only recognizes `ExecuteIntentRequest` lifecycle signals. Ignores unrelated
/// workspace events and generic command bookkeeping. No inference or scoring.
pub fn classify_execution_outcome_event(
    event: &AuditEvent,
) -> Option<ExecutionOutcomeStatus> {
    if event.command_name.as_deref() != Some("ExecuteIntentRequest") {
        return None;
    }

    match event.event_type.as_str() {
        "command.failed" => Some(ExecutionOutcomeStatus::Failed),
        "command.executed" => {
            if !event.success {
                return Some(ExecutionOutcomeStatus::Failed);
            }
            let metadata = event.metadata.as_deref()?;
            let value = serde_json::from_str::<serde_json::Value>(metadata).ok()?;
            if value.get("execution_request").and_then(|v| v.as_bool()) != Some(true) {
                return None;
            }
            match value.get("execution_status").and_then(|v| v.as_str()) {
                Some("executed") => Some(ExecutionOutcomeStatus::Completed),
                Some("failed") => Some(ExecutionOutcomeStatus::Failed),
                Some("cancelled") => Some(ExecutionOutcomeStatus::Cancelled),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Builds a derived outcome from a classified audit event.
pub fn outcome_from_audit_event(event: &AuditEvent) -> Option<ExecutionOutcome> {
    let status = classify_execution_outcome_event(event)?;
    let metadata_value = event
        .metadata
        .as_deref()
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw).ok());

    let suggestion_id = metadata_value
        .as_ref()
        .and_then(|value| value.get("suggestion_id").and_then(|v| v.as_str()))
        .map(str::to_string);

    let intent_id = metadata_value
        .as_ref()
        .and_then(|value| value.get("intent_id").and_then(|v| v.as_str()))
        .map(str::to_string);

    let execution_request_id = metadata_value
        .as_ref()
        .and_then(|value| value.get("execution_request_id").and_then(|v| v.as_str()))
        .map(str::to_string)
        .or_else(|| {
            suggestion_id
                .as_ref()
                .map(|id| format!("execution:{id}"))
        })
        .unwrap_or_else(|| "execution:unresolved".into());

    let failure_reason = if matches!(status, ExecutionOutcomeStatus::Failed) {
        metadata_value
            .as_ref()
            .and_then(|value| value.get("error_code").and_then(|v| v.as_str()))
            .map(str::to_string)
    } else {
        None
    };

    Some(ExecutionOutcome {
        execution_request_id,
        status,
        command_name: event
            .command_name
            .clone()
            .unwrap_or_else(|| "ExecuteIntentRequest".into()),
        completed_at: event.timestamp.clone(),
        success: matches!(status, ExecutionOutcomeStatus::Completed),
        failure_reason,
        suggestion_id,
        intent_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::ActorType;
    use crate::ids::AuditEventId;

    fn audit_event(
        success: bool,
        command_name: Option<&str>,
        metadata: Option<&str>,
    ) -> AuditEvent {
        AuditEvent {
            id: AuditEventId::generate(),
            timestamp: "2024-01-01T00:00:00Z".into(),
            event_type: "command.executed".into(),
            actor_type: ActorType::LocalUser,
            actor_id: Some("local-user".into()),
            command_name: command_name.map(str::to_string),
            intent_type: None,
            capability: Some("audit.write".into()),
            resource_ref: None,
            success,
            metadata: metadata.map(str::to_string),
        }
    }

    #[test]
    fn valid_outcome_passes_validation() {
        let outcome = ExecutionOutcome {
            execution_request_id: "execution:s-1".into(),
            status: ExecutionOutcomeStatus::Completed,
            command_name: "ExecuteIntentRequest".into(),
            completed_at: "2024-01-01T00:00:00Z".into(),
            success: true,
            failure_reason: None,
            suggestion_id: Some("s-1".into()),
            intent_id: Some("create-zone".into()),
        };
        assert!(outcome.validate().is_ok());
    }

    #[test]
    fn rejects_empty_execution_request_id() {
        let outcome = ExecutionOutcome {
            execution_request_id: "  ".into(),
            status: ExecutionOutcomeStatus::Completed,
            command_name: "ExecuteIntentRequest".into(),
            completed_at: "2024-01-01T00:00:00Z".into(),
            success: true,
            failure_reason: None,
            suggestion_id: None,
            intent_id: None,
        };
        assert_eq!(
            outcome.validate(),
            Err(ExecutionOutcomeError::EmptyExecutionRequestId)
        );
    }

    #[test]
    fn rejects_empty_command_name() {
        let outcome = ExecutionOutcome {
            execution_request_id: "execution:s-1".into(),
            status: ExecutionOutcomeStatus::Completed,
            command_name: String::new(),
            completed_at: "2024-01-01T00:00:00Z".into(),
            success: true,
            failure_reason: None,
            suggestion_id: None,
            intent_id: None,
        };
        assert_eq!(
            outcome.validate(),
            Err(ExecutionOutcomeError::EmptyCommandName)
        );
    }

    #[test]
    fn rejects_empty_timestamp() {
        let outcome = ExecutionOutcome {
            execution_request_id: "execution:s-1".into(),
            status: ExecutionOutcomeStatus::Failed,
            command_name: "ExecuteIntentRequest".into(),
            completed_at: String::new(),
            success: false,
            failure_reason: Some("not_ready".into()),
            suggestion_id: None,
            intent_id: None,
        };
        assert_eq!(
            outcome.validate(),
            Err(ExecutionOutcomeError::EmptyTimestamp)
        );
    }

    #[test]
    fn classifies_successful_executed_request_as_completed() {
        let event = audit_event(
            true,
            Some("ExecuteIntentRequest"),
            Some(
                r#"{"execution_request":true,"suggestion_id":"s-1","intent_id":"create-zone","execution_status":"executed"}"#,
            ),
        );
        assert_eq!(
            classify_execution_outcome_event(&event),
            Some(ExecutionOutcomeStatus::Completed)
        );
        let outcome = outcome_from_audit_event(&event).unwrap();
        assert_eq!(outcome.execution_request_id, "execution:s-1");
        assert!(outcome.success);
    }

    #[test]
    fn classifies_failed_command_as_failed() {
        let event = AuditEvent {
            id: AuditEventId::generate(),
            timestamp: "2024-01-01T00:00:00Z".into(),
            event_type: "command.failed".into(),
            actor_type: ActorType::LocalUser,
            actor_id: Some("local-user".into()),
            command_name: Some("ExecuteIntentRequest".into()),
            intent_type: None,
            capability: Some("audit.write".into()),
            resource_ref: None,
            success: false,
            metadata: Some(
                r#"{"error_code":"suggestion_intent_validation_error","execution_request":true,"suggestion_id":"s-1"}"#.into(),
            ),
        };
        assert_eq!(
            classify_execution_outcome_event(&event),
            Some(ExecutionOutcomeStatus::Failed)
        );
        let outcome = outcome_from_audit_event(&event).unwrap();
        assert_eq!(
            outcome.failure_reason.as_deref(),
            Some("suggestion_intent_validation_error")
        );
        assert!(!outcome.success);
    }

    #[test]
    fn ignores_unrelated_audit_events() {
        let event = audit_event(
            true,
            Some("CreateWorkspace"),
            Some(r#"{"name":"ws"}"#),
        );
        assert!(classify_execution_outcome_event(&event).is_none());
        assert!(outcome_from_audit_event(&event).is_none());
    }

    #[test]
    fn ignores_success_without_execution_request_flag() {
        let event = audit_event(
            true,
            Some("ExecuteIntentRequest"),
            Some(r#"{"suggestion_id":"s-1"}"#),
        );
        assert!(classify_execution_outcome_event(&event).is_none());
    }
}
