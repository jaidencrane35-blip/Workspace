//! Derived suggestion lifecycle — read-only projection over governance history
//! (Sprint 22).
//!
//! Lifecycle records answer "what happened to a proposal over time?" by
//! classifying already-recorded audit events. They are derived, disposable,
//! and non-authoritative: the audit trail remains the source of truth. Pure
//! domain types only — no database, kernel, or UI dependencies.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::actor::ActorType;
use crate::audit::AuditEvent;
use crate::resource::{ResourceId, ResourceKind, ResourceRef};

/// Lifecycle stage of a suggestion derived from governance history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionLifecycleState {
    Created,
    Presented,
    Accepted,
    Rejected,
    Expired,
}

impl SuggestionLifecycleState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Presented => "presented",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }

    pub fn parse(value: &str) -> Result<Self, SuggestionLifecycleError> {
        match value {
            "created" => Ok(Self::Created),
            "presented" => Ok(Self::Presented),
            "accepted" => Ok(Self::Accepted),
            "rejected" => Ok(Self::Rejected),
            "expired" => Ok(Self::Expired),
            _ => Err(SuggestionLifecycleError::InvalidState(value.to_string())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Accepted | Self::Rejected | Self::Expired)
    }

    pub fn allows_transition(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Created, Self::Presented)
                | (Self::Created, Self::Accepted)
                | (Self::Created, Self::Rejected)
                | (Self::Created, Self::Expired)
                | (Self::Presented, Self::Presented)
                | (Self::Presented, Self::Accepted)
                | (Self::Presented, Self::Rejected)
                | (Self::Presented, Self::Expired)
        )
    }
}

/// Lifecycle-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SuggestionLifecycleError {
    #[error("Suggestion lifecycle suggestion id must not be empty")]
    EmptySuggestionId,

    #[error("Suggestion lifecycle timestamp must not be empty")]
    EmptyTimestamp,

    #[error("Invalid suggestion lifecycle state: {0}")]
    InvalidState(String),

    #[error("Invalid suggestion lifecycle transition for {suggestion_id}: {from} -> {to}")]
    InvalidTransition {
        suggestion_id: String,
        from: String,
        to: String,
    },
}

/// A derived lifecycle record for one suggestion at one point in time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestionLifecycleRecord {
    pub suggestion_id: String,
    pub state: SuggestionLifecycleState,
    pub occurred_at: String,
    pub actor_type: ActorType,
    pub actor_id: Option<String>,
    pub related_resource_ref: Option<ResourceRef>,
    pub metadata: Option<String>,
}

impl SuggestionLifecycleRecord {
    pub fn validate(&self) -> Result<(), SuggestionLifecycleError> {
        if self.suggestion_id.trim().is_empty() {
            return Err(SuggestionLifecycleError::EmptySuggestionId);
        }
        if self.occurred_at.trim().is_empty() {
            return Err(SuggestionLifecycleError::EmptyTimestamp);
        }
        Ok(())
    }
}

pub fn validate_suggestion_lifecycle_sequence(
    records: &[SuggestionLifecycleRecord],
) -> Result<(), SuggestionLifecycleError> {
    let mut current_by_suggestion: HashMap<&str, SuggestionLifecycleState> = HashMap::new();
    for record in records.iter().rev() {
        if let Some(current) = current_by_suggestion.get(record.suggestion_id.as_str()) {
            if !current.allows_transition(record.state) {
                return Err(SuggestionLifecycleError::InvalidTransition {
                    suggestion_id: record.suggestion_id.clone(),
                    from: current.as_str().into(),
                    to: record.state.as_str().into(),
                });
            }
        }
        current_by_suggestion.insert(record.suggestion_id.as_str(), record.state);
    }
    Ok(())
}

/// Classifies an audit event into a suggestion lifecycle state when applicable.
///
/// Only recognizes explicit suggestion lifecycle signals. Ignores command
/// bookkeeping and unrelated workspace events. No inference or scoring.
pub fn classify_suggestion_lifecycle_event(
    event: &AuditEvent,
) -> Option<SuggestionLifecycleState> {
    if event.event_type != "command.executed" || !event.success {
        return None;
    }

    if let Some(metadata) = &event.metadata {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(metadata) {
            if let Some(lifecycle) = value.get("lifecycle").and_then(|v| v.as_str()) {
                if let Ok(state) = SuggestionLifecycleState::parse(lifecycle) {
                    return Some(state);
                }
            }
            if let Some(decision) = value.get("decision").and_then(|v| v.as_str()) {
                return match decision {
                    "accepted" => Some(SuggestionLifecycleState::Accepted),
                    "rejected" => Some(SuggestionLifecycleState::Rejected),
                    _ => None,
                };
            }
        }
    }

    match event.command_name.as_deref() {
        Some("AcceptSuggestion") => Some(SuggestionLifecycleState::Accepted),
        Some("RejectSuggestion") => Some(SuggestionLifecycleState::Rejected),
        Some("GetSuggestions") => Some(SuggestionLifecycleState::Presented),
        _ => None,
    }
}

/// Extracts a suggestion identifier from audit metadata when present.
pub fn extract_suggestion_id(metadata: Option<&str>) -> Option<String> {
    let metadata = metadata?;
    let value = serde_json::from_str::<serde_json::Value>(metadata).ok()?;
    value
        .get("suggestion_id")
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

/// Parses a canonical `kind:id` resource reference string.
pub fn parse_canonical_resource_ref(value: &str) -> Option<ResourceRef> {
    let (kind_str, id_str) = value.split_once(':')?;
    let kind = match kind_str {
        "workspace" => ResourceKind::Workspace,
        "zone" => ResourceKind::Zone,
        "application" => ResourceKind::Application,
        "widget" => ResourceKind::Widget,
        _ => return None,
    };
    let id = ResourceId::new(id_str).ok()?;
    Some(ResourceRef::new(kind, id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::Actor;

    fn audit_event(command_name: &str, metadata: Option<&str>) -> AuditEvent {
        AuditEvent::from_actor("command.executed", &Actor::local_user(), true)
            .with_command_name(command_name)
            .with_metadata(metadata.unwrap_or("{}"))
    }

    #[test]
    fn classifies_accept_and_reject_commands() {
        let accept = audit_event(
            "AcceptSuggestion",
            Some(r#"{"suggestion_id":"resource-growth:workspace:ws-1","decision":"accepted"}"#),
        );
        let reject = audit_event(
            "RejectSuggestion",
            Some(r#"{"suggestion_id":"resource-growth:workspace:ws-1","decision":"rejected"}"#),
        );

        assert_eq!(
            classify_suggestion_lifecycle_event(&accept),
            Some(SuggestionLifecycleState::Accepted)
        );
        assert_eq!(
            classify_suggestion_lifecycle_event(&reject),
            Some(SuggestionLifecycleState::Rejected)
        );
    }

    #[test]
    fn classifies_explicit_lifecycle_metadata() {
        let created = audit_event(
            "SuggestionLifecycle",
            Some(r#"{"suggestion_id":"s-1","lifecycle":"created"}"#),
        );
        assert_eq!(
            classify_suggestion_lifecycle_event(&created),
            Some(SuggestionLifecycleState::Created)
        );
    }

    #[test]
    fn ignores_unrelated_and_failed_events() {
        let unrelated = AuditEvent::from_actor("workspace.entity.created", &Actor::local_user(), true);
        assert!(classify_suggestion_lifecycle_event(&unrelated).is_none());

        let failed = AuditEvent::from_actor("command.executed", &Actor::local_user(), false)
            .with_command_name("AcceptSuggestion");
        assert!(classify_suggestion_lifecycle_event(&failed).is_none());
    }

    #[test]
    fn validates_lifecycle_record_fields() {
        let record = SuggestionLifecycleRecord {
            suggestion_id: "resource-growth:workspace:ws-1".into(),
            state: SuggestionLifecycleState::Accepted,
            occurred_at: "2026-07-24T00:00:00Z".into(),
            actor_type: ActorType::LocalUser,
            actor_id: Some("local-user".into()),
            related_resource_ref: None,
            metadata: None,
        };
        assert!(record.validate().is_ok());

        let mut invalid = record.clone();
        invalid.suggestion_id.clear();
        assert_eq!(
            invalid.validate(),
            Err(SuggestionLifecycleError::EmptySuggestionId)
        );
    }

    #[test]
    fn rejects_invalid_state_strings() {
        assert!(matches!(
            SuggestionLifecycleState::parse("unknown"),
            Err(SuggestionLifecycleError::InvalidState(_))
        ));
    }

    #[test]
    fn extracts_suggestion_id_from_metadata() {
        let id = extract_suggestion_id(Some(r#"{"suggestion_id":"s-1"}"#));
        assert_eq!(id.as_deref(), Some("s-1"));
    }

    #[test]
    fn sequence_validation_keeps_terminal_states_terminal() {
        let presented = SuggestionLifecycleRecord {
            suggestion_id: "s-1".into(),
            state: SuggestionLifecycleState::Presented,
            occurred_at: "2026-07-24T00:00:00Z".into(),
            actor_type: ActorType::LocalUser,
            actor_id: Some("local-user".into()),
            related_resource_ref: None,
            metadata: None,
        };
        let mut accepted = presented.clone();
        accepted.state = SuggestionLifecycleState::Accepted;
        accepted.occurred_at = "2026-07-24T00:01:00Z".into();
        assert!(validate_suggestion_lifecycle_sequence(&[
            accepted.clone(),
            presented.clone(),
        ])
        .is_ok());
        assert!(matches!(
            validate_suggestion_lifecycle_sequence(&[presented, accepted]),
            Err(SuggestionLifecycleError::InvalidTransition { .. })
        ));
    }
}
