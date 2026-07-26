//! Observation event contracts (Sprint 117).
//!
//! Canonical internal representation for future event sources.
//! Not an OS listener — contracts and normalization only.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::workspace_observation::{
    ObservationFreshnessRequirement, ObservationTriggerRequest,
};

/// Observation event validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ObservationEventError {
    #[error("observation event source is required")]
    MissingSource,

    #[error("observation event timestamp is required")]
    MissingTimestamp,

    #[error("observation event timestamp is invalid: {0}")]
    InvalidTimestamp(String),

    #[error("observation event kind is invalid: {0}")]
    InvalidKind(String),

    #[error("observation event is invalid: {0}")]
    Invalid(String),
}

pub type Result<T> = std::result::Result<T, ObservationEventError>;

/// Kind of desktop/observation-related event (contract only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationEventKind {
    WindowChanged,
    FocusChanged,
    MonitorChanged,
    DisplayConfigurationChanged,
    DesktopStateChanged,
    Unknown,
}

impl ObservationEventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WindowChanged => "window_changed",
            Self::FocusChanged => "focus_changed",
            Self::MonitorChanged => "monitor_changed",
            Self::DisplayConfigurationChanged => "display_configuration_changed",
            Self::DesktopStateChanged => "desktop_state_changed",
            Self::Unknown => "unknown",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value.trim() {
            "window_changed" => Ok(Self::WindowChanged),
            "focus_changed" => Ok(Self::FocusChanged),
            "monitor_changed" => Ok(Self::MonitorChanged),
            "display_configuration_changed" => Ok(Self::DisplayConfigurationChanged),
            "desktop_state_changed" => Ok(Self::DesktopStateChanged),
            "unknown" => Ok(Self::Unknown),
            other => Err(ObservationEventError::InvalidKind(other.to_string())),
        }
    }
}

/// Immutable observation event — future sources normalize into this shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationEvent {
    /// Origin label for the event producer (e.g. future win32 adapter id). Not CaptureRequestSource.
    pub source: String,
    pub event_kind: ObservationEventKind,
    /// RFC3339 timestamp.
    pub timestamp: String,
    pub context: Option<String>,
    pub correlation_id: Option<String>,
    /// Opaque JSON object string (defaults to `{}`).
    pub metadata: String,
    pub authority_effect: String,
}

impl ObservationEvent {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const TRIGGER_REASON: &'static str = "normalized_event";
    pub const GATEWAY_CONTEXT_PREFIX: &'static str = "gateway:ObservationEventGateway";

    pub fn new(
        source: impl Into<String>,
        event_kind: ObservationEventKind,
        timestamp: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            event_kind,
            timestamp: timestamp.into(),
            context: None,
            correlation_id: None,
            metadata: "{}".into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }

    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = metadata.into();
        self
    }

    /// Trim / default fields into a canonical form.
    pub fn normalize(mut self) -> Result<Self> {
        self.source = self.source.trim().to_string();
        if self.source.is_empty() {
            return Err(ObservationEventError::MissingSource);
        }

        self.timestamp = self.timestamp.trim().to_string();
        if self.timestamp.is_empty() {
            return Err(ObservationEventError::MissingTimestamp);
        }
        DateTime::parse_from_rfc3339(&self.timestamp)
            .map_err(|error| ObservationEventError::InvalidTimestamp(error.to_string()))?;

        if let Some(context) = self.context.take() {
            let trimmed = context.trim().to_string();
            self.context = if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            };
        }

        if let Some(correlation_id) = self.correlation_id.take() {
            let trimmed = correlation_id.trim().to_string();
            self.correlation_id = if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            };
        }

        let metadata = self.metadata.trim();
        self.metadata = if metadata.is_empty() {
            "{}".into()
        } else {
            metadata.to_string()
        };

        if self.authority_effect.trim().is_empty() {
            self.authority_effect = Self::AUTHORITY_EFFECT_NONE.into();
        } else if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(ObservationEventError::Invalid(
                "authority_effect must be none".into(),
            ));
        }

        Ok(self)
    }

    /// Validate without consuming (normalize clone).
    pub fn validate(&self) -> Result<()> {
        self.clone().normalize().map(|_| ())
    }

    /// Map to an ObservationTriggerRequest (source = Event; admission still rejects today).
    pub fn to_trigger_request(&self) -> ObservationTriggerRequest {
        ObservationTriggerRequest::event(ObservationFreshnessRequirement::NotStale)
            .with_reason(Self::TRIGGER_REASON)
            .with_context(self.gateway_trigger_context())
    }

    /// Compact gateway provenance for ObservationTriggerRequest.context.
    pub fn gateway_trigger_context(&self) -> String {
        let mut parts = vec![
            Self::GATEWAY_CONTEXT_PREFIX.to_string(),
            format!("kind={}", self.event_kind.as_str()),
            format!("source={}", self.source),
            format!("timestamp={}", self.timestamp),
        ];
        if let Some(correlation_id) = &self.correlation_id {
            parts.push(format!("correlation_id={correlation_id}"));
        }
        if let Some(context) = &self.context {
            parts.push(format!("context={context}"));
        }
        if self.metadata != "{}" {
            parts.push(format!("metadata={}", self.metadata));
        }
        parts.join(";")
    }

    pub fn now_rfc3339() -> String {
        Utc::now().to_rfc3339()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_observation::CaptureRequestSource;

    #[test]
    fn normalize_trims_and_defaults() {
        let event = ObservationEvent::new("  win32_probe  ", ObservationEventKind::FocusChanged, " 2026-07-26T12:00:00Z ")
            .with_context("  desktop  ")
            .with_correlation_id("  corr-1  ")
            .with_metadata("  ")
            .normalize()
            .unwrap();
        assert_eq!(event.source, "win32_probe");
        assert_eq!(event.timestamp, "2026-07-26T12:00:00Z");
        assert_eq!(event.context.as_deref(), Some("desktop"));
        assert_eq!(event.correlation_id.as_deref(), Some("corr-1"));
        assert_eq!(event.metadata, "{}");
    }

    #[test]
    fn rejects_missing_source_and_bad_timestamp() {
        assert!(matches!(
            ObservationEvent::new(" ", ObservationEventKind::Unknown, "2026-07-26T12:00:00Z")
                .normalize(),
            Err(ObservationEventError::MissingSource)
        ));
        assert!(matches!(
            ObservationEvent::new("src", ObservationEventKind::Unknown, "not-a-time").normalize(),
            Err(ObservationEventError::InvalidTimestamp(_))
        ));
    }

    #[test]
    fn maps_to_event_trigger_with_gateway_context() {
        let event = ObservationEvent::new(
            "synthetic",
            ObservationEventKind::WindowChanged,
            "2026-07-26T12:00:00Z",
        )
        .with_correlation_id("c-9")
        .normalize()
        .unwrap();
        let request = event.to_trigger_request();
        assert_eq!(request.source, CaptureRequestSource::Event);
        assert_eq!(request.reason.as_deref(), Some("normalized_event"));
        assert_eq!(
            request.freshness_requirement,
            ObservationFreshnessRequirement::NotStale
        );
        let ctx = request.context.as_deref().unwrap();
        assert!(ctx.contains(ObservationEvent::GATEWAY_CONTEXT_PREFIX));
        assert!(ctx.contains("kind=window_changed"));
        assert!(ctx.contains("source=synthetic"));
        assert!(ctx.contains("correlation_id=c-9"));
    }
}
