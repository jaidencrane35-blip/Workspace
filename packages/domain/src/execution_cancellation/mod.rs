//! Governed execution cancellation request — pure domain (Sprint 28).
//!
//! A [`CancellationRequest`] records that cancellation was requested for an
//! existing execution identity. It does **not** interrupt running work — that
//! remains a future boundary. Pure domain only: no database, kernel, or UI.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Cancellation-request validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExecutionCancellationError {
    #[error("Cancellation request id must not be empty")]
    EmptyId,

    #[error("Cancellation execution request id must not be empty")]
    EmptyExecutionRequestId,

    #[error("Cancellation timestamp must not be empty")]
    EmptyTimestamp,

    #[error("Cancellation requested_by must not be empty")]
    EmptyRequestedBy,

    #[error("Invalid cancellation status: {0}")]
    InvalidStatus(String),
}

/// Lifecycle status of a governed cancellation request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancellationStatus {
    Requested,
    Approved,
    Rejected,
}

impl CancellationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ExecutionCancellationError> {
        match value {
            "requested" => Ok(Self::Requested),
            "approved" => Ok(Self::Approved),
            "rejected" => Ok(Self::Rejected),
            _ => Err(ExecutionCancellationError::InvalidStatus(value.to_string())),
        }
    }
}

/// A governed request to cancel a previously identified execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationRequest {
    pub id: String,
    pub execution_request_id: String,
    pub requested_at: String,
    pub requested_by: String,
    /// Sanitized operator-facing reason (may be empty).
    pub reason: String,
    pub status: CancellationStatus,
}

impl CancellationRequest {
    pub fn new(
        execution_request_id: impl Into<String>,
        requested_by: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("cancellation:{}", Uuid::new_v4()),
            execution_request_id: execution_request_id.into(),
            requested_at: Utc::now().to_rfc3339(),
            requested_by: requested_by.into(),
            reason: sanitize_reason(reason),
            status: CancellationStatus::Requested,
        }
    }

    pub fn validate(&self) -> Result<(), ExecutionCancellationError> {
        if self.id.trim().is_empty() {
            return Err(ExecutionCancellationError::EmptyId);
        }
        if self.execution_request_id.trim().is_empty() {
            return Err(ExecutionCancellationError::EmptyExecutionRequestId);
        }
        if self.requested_at.trim().is_empty() {
            return Err(ExecutionCancellationError::EmptyTimestamp);
        }
        if self.requested_by.trim().is_empty() {
            return Err(ExecutionCancellationError::EmptyRequestedBy);
        }
        CancellationStatus::parse(self.status.as_str())?;
        Ok(())
    }
}

/// Trims and bounds reason text for audit-safe metadata (no interpretation).
pub fn sanitize_reason(reason: impl Into<String>) -> String {
    const MAX_LEN: usize = 500;
    let trimmed: String = reason
        .into()
        .chars()
        .filter(|ch| !ch.is_control() || *ch == '\n' || *ch == '\t')
        .collect::<String>()
        .trim()
        .to_string();
    if trimmed.chars().count() <= MAX_LEN {
        trimmed
    } else {
        trimmed.chars().take(MAX_LEN).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_cancellation_request() {
        let request = CancellationRequest::new("execution:s-1", "local-user", "user changed mind");
        assert_eq!(request.status, CancellationStatus::Requested);
        assert_eq!(request.execution_request_id, "execution:s-1");
        assert_eq!(request.requested_by, "local-user");
        assert_eq!(request.reason, "user changed mind");
        assert!(request.id.starts_with("cancellation:"));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn invalid_request_rejection() {
        let mut request = CancellationRequest::new("execution:s-1", "local-user", "ok");
        request.execution_request_id = String::new();
        assert_eq!(
            request.validate(),
            Err(ExecutionCancellationError::EmptyExecutionRequestId)
        );

        request = CancellationRequest::new("execution:s-1", "local-user", "ok");
        request.requested_by = "  ".into();
        assert_eq!(
            request.validate(),
            Err(ExecutionCancellationError::EmptyRequestedBy)
        );

        request = CancellationRequest::new("execution:s-1", "local-user", "ok");
        request.requested_at = String::new();
        assert_eq!(
            request.validate(),
            Err(ExecutionCancellationError::EmptyTimestamp)
        );
    }

    #[test]
    fn status_validation() {
        assert_eq!(
            CancellationStatus::parse("requested"),
            Ok(CancellationStatus::Requested)
        );
        assert_eq!(
            CancellationStatus::parse("approved"),
            Ok(CancellationStatus::Approved)
        );
        assert_eq!(
            CancellationStatus::parse("rejected"),
            Ok(CancellationStatus::Rejected)
        );
        assert_eq!(
            CancellationStatus::parse("stopped"),
            Err(ExecutionCancellationError::InvalidStatus("stopped".into()))
        );
    }

    #[test]
    fn sanitize_reason_truncates_and_strips_controls() {
        let long = "a".repeat(600);
        assert_eq!(sanitize_reason(long).chars().count(), 500);
        assert_eq!(sanitize_reason("  hello\u{0000}  "), "hello");
    }

    #[test]
    fn serializes_round_trip() {
        let request = CancellationRequest::new("execution:s-1", "local-user", "reason");
        let json = serde_json::to_string(&request).unwrap();
        let restored: CancellationRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(request, restored);
    }
}
