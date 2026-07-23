//! Derived workspace observations — a neutral, read-only interpretation of
//! recorded activity (Sprint 17).
//!
//! Observations answer "what is happening over time?" by classifying already
//! recorded events into consumer-friendly records. They are derived,
//! disposable, and non-authoritative: domain events (and their durable audit
//! trail) remain the source of truth. This module is pure — no database, IO, or
//! UI dependencies — so classification stays deterministic and testable.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::actor::ActorType;

/// Broad grouping of an observation by the kind of activity it represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationCategory {
    /// Workspace lifecycle activity (startup, ready, shutdown).
    Lifecycle,
    /// Resource topology changes (workspaces, zones, applications, widgets).
    ResourceChange,
    /// Spatial layout changes.
    LayoutChange,
    /// Settings changes.
    SettingsChange,
}

/// Static, non-inferred importance of an observation category/event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationImportance {
    Low,
    Normal,
    High,
}

/// Observation-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ObservationError {
    #[error("Observation id must not be empty")]
    EmptyId,

    #[error("Observation source event type must not be empty")]
    EmptySource,

    #[error("Observation timestamp must not be empty")]
    EmptyTimestamp,
}

/// A derived, neutral record of something that happened in the workspace.
///
/// Deliberately hides governance internals (command names, capability strings,
/// success flags). It exposes only a consumer-friendly interpretation of an
/// event that already occurred.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    /// Stable identifier, sourced from the underlying recorded event.
    pub id: String,
    /// RFC 3339 timestamp of when the underlying event occurred.
    pub occurred_at: String,
    pub category: ObservationCategory,
    pub importance: ObservationImportance,
    /// Canonical domain event name the observation was derived from.
    pub source_event_type: String,
    /// Neutral, non-sensitive human-readable summary.
    pub summary: String,
    pub actor_type: ActorType,
    pub actor_id: Option<String>,
    /// Canonical `ResourceRef` string when the source event referenced one.
    pub resource_ref: Option<String>,
    /// Already-sanitized metadata carried from the source record, if any.
    pub metadata: Option<String>,
}

impl Observation {
    pub fn validate(&self) -> Result<(), ObservationError> {
        if self.id.trim().is_empty() {
            return Err(ObservationError::EmptyId);
        }
        if self.source_event_type.trim().is_empty() {
            return Err(ObservationError::EmptySource);
        }
        if self.occurred_at.trim().is_empty() {
            return Err(ObservationError::EmptyTimestamp);
        }
        Ok(())
    }
}

/// Classifies a recorded event type into an observation category and importance.
///
/// Returns `Some` only for domain-activity events. Governance bookkeeping
/// (`command.executed`, `command.failed`) and any unknown type return `None`,
/// so the observation stream reflects workspace activity, not command/read
/// accounting.
pub fn classify_event(
    event_type: &str,
) -> Option<(ObservationCategory, ObservationImportance)> {
    use ObservationCategory::*;
    use ObservationImportance::*;

    let classification = match event_type {
        "system.workspace.started" => (Lifecycle, Normal),
        "system.workspace.ready" => (Lifecycle, Normal),
        "system.workspace.shutdown" => (Lifecycle, High),
        "system.settings.changed" => (SettingsChange, Normal),
        "workspace.entity.created" => (ResourceChange, Normal),
        "workspace.entity.updated" => (ResourceChange, Normal),
        "resource.created" => (ResourceChange, Normal),
        "resource.updated" => (ResourceChange, Normal),
        "resource.deleted" => (ResourceChange, High),
        "zone.created" => (ResourceChange, Normal),
        "application.created" => (ResourceChange, Normal),
        "widget.created" => (ResourceChange, Normal),
        "layout.created" => (LayoutChange, Normal),
        "layout.updated" => (LayoutChange, Normal),
        "layout.reset" => (LayoutChange, Normal),
        "layout.deleted" => (LayoutChange, High),
        "layout.snapshot" => (LayoutChange, Low),
        _ => return None,
    };

    Some(classification)
}

/// Produces a deterministic, non-sensitive summary for a classified event.
///
/// Derives entirely from the event type and category; never includes user data.
pub fn neutral_summary(category: ObservationCategory, source_event_type: &str) -> String {
    let phrase = match source_event_type {
        "system.workspace.started" => "Workspace started",
        "system.workspace.ready" => "Workspace ready",
        "system.workspace.shutdown" => "Workspace shutdown",
        "system.settings.changed" => "Settings changed",
        "workspace.entity.created" => "Workspace created",
        "workspace.entity.updated" => "Workspace updated",
        "resource.created" => "Resource created",
        "resource.updated" => "Resource updated",
        "resource.deleted" => "Resource deleted",
        "zone.created" => "Zone created",
        "application.created" => "Application created",
        "widget.created" => "Widget created",
        "layout.created" => "Layout created",
        "layout.updated" => "Layout updated",
        "layout.reset" => "Layout reset",
        "layout.deleted" => "Layout deleted",
        "layout.snapshot" => "Layout snapshot captured",
        _ => match category {
            ObservationCategory::Lifecycle => "Lifecycle activity",
            ObservationCategory::ResourceChange => "Resource change",
            ObservationCategory::LayoutChange => "Layout change",
            ObservationCategory::SettingsChange => "Settings change",
        },
    };

    phrase.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(id: &str, source: &str, occurred_at: &str) -> Observation {
        let (category, importance) = classify_event(source).unwrap();
        Observation {
            id: id.into(),
            occurred_at: occurred_at.into(),
            category,
            importance,
            source_event_type: source.into(),
            summary: neutral_summary(category, source),
            actor_type: ActorType::LocalUser,
            actor_id: Some("local-user".into()),
            resource_ref: None,
            metadata: None,
        }
    }

    #[test]
    fn classifies_known_domain_events() {
        assert_eq!(
            classify_event("system.workspace.started"),
            Some((ObservationCategory::Lifecycle, ObservationImportance::Normal))
        );
        assert_eq!(
            classify_event("system.workspace.shutdown"),
            Some((ObservationCategory::Lifecycle, ObservationImportance::High))
        );
        assert_eq!(
            classify_event("system.settings.changed"),
            Some((ObservationCategory::SettingsChange, ObservationImportance::Normal))
        );
        assert_eq!(
            classify_event("workspace.entity.created"),
            Some((ObservationCategory::ResourceChange, ObservationImportance::Normal))
        );
        assert_eq!(
            classify_event("resource.deleted"),
            Some((ObservationCategory::ResourceChange, ObservationImportance::High))
        );
        assert_eq!(
            classify_event("zone.created"),
            Some((ObservationCategory::ResourceChange, ObservationImportance::Normal))
        );
        assert_eq!(
            classify_event("layout.updated"),
            Some((ObservationCategory::LayoutChange, ObservationImportance::Normal))
        );
        assert_eq!(
            classify_event("layout.deleted"),
            Some((ObservationCategory::LayoutChange, ObservationImportance::High))
        );
        assert_eq!(
            classify_event("layout.snapshot"),
            Some((ObservationCategory::LayoutChange, ObservationImportance::Low))
        );
    }

    #[test]
    fn excludes_command_bookkeeping_and_unknown() {
        assert_eq!(classify_event("command.executed"), None);
        assert_eq!(classify_event("command.failed"), None);
        assert_eq!(classify_event("something.unknown"), None);
        assert_eq!(classify_event(""), None);
    }

    #[test]
    fn summary_is_non_empty_and_deterministic() {
        let first = neutral_summary(ObservationCategory::ResourceChange, "zone.created");
        let second = neutral_summary(ObservationCategory::ResourceChange, "zone.created");
        assert_eq!(first, "Zone created");
        assert_eq!(first, second);
        assert!(!neutral_summary(ObservationCategory::Lifecycle, "system.workspace.shutdown").is_empty());
    }

    #[test]
    fn validate_accepts_complete_observation() {
        let observation = sample("obs-1", "zone.created", "2026-07-23T00:00:00Z");
        assert!(observation.validate().is_ok());
    }

    #[test]
    fn validate_rejects_empty_fields() {
        let mut observation = sample("obs-1", "zone.created", "2026-07-23T00:00:00Z");
        observation.id = String::new();
        assert_eq!(observation.validate(), Err(ObservationError::EmptyId));

        let mut observation = sample("obs-1", "zone.created", "2026-07-23T00:00:00Z");
        observation.source_event_type = String::new();
        assert_eq!(observation.validate(), Err(ObservationError::EmptySource));

        let mut observation = sample("obs-1", "zone.created", "2026-07-23T00:00:00Z");
        observation.occurred_at = String::new();
        assert_eq!(observation.validate(), Err(ObservationError::EmptyTimestamp));
    }
}
