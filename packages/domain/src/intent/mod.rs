mod action;

pub use action::{
    ActionIntentCategory, ActionIntentDefinition, ActionIntentError, ActionIntentMetadata,
    ActionIntentRegistry, ActionIntentRequest, TargetRequirement,
};
pub use crate::ids::ActionIntentId;

use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::ids::IntentId;

/// Why an action is being performed (execution motivation, not authentication).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentType {
    UserRequest,
    AISuggestion,
    Automation,
    PluginRequest,
    SystemStartup,
    SystemShutdown,
    ScheduledTask,
    ExternalIntegration,
}

/// Lightweight, optional metadata describing intent (no secrets or payloads).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct IntentMetadata {
    pub label: Option<String>,
}

/// Immutable execution intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Intent {
    pub id: IntentId,
    pub intent_type: IntentType,
    pub metadata: IntentMetadata,
}

/// Execution intent attached to a command or permission check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentContext {
    pub intent: Intent,
}

/// Well-known identifier for user-initiated actions.
pub const USER_REQUEST_INTENT_ID: &str = "user-request";

/// Well-known identifier for kernel startup.
pub const SYSTEM_STARTUP_INTENT_ID: &str = "system-startup";

/// Well-known identifier for kernel shutdown.
pub const SYSTEM_SHUTDOWN_INTENT_ID: &str = "system-shutdown";

/// Well-known identifier for AI-proposed actions (motivation intent).
pub const AI_SUGGESTION_INTENT_ID: &str = "ai-suggestion";

impl Intent {
    pub fn new(id: IntentId, intent_type: IntentType, metadata: IntentMetadata) -> Self {
        Self {
            id,
            intent_type,
            metadata,
        }
    }

    pub fn user_request() -> Self {
        Self {
            id: IntentId::new(USER_REQUEST_INTENT_ID).expect("user request intent id is valid"),
            intent_type: IntentType::UserRequest,
            metadata: IntentMetadata {
                label: Some("User Request".into()),
            },
        }
    }

    pub fn system_startup() -> Self {
        Self {
            id: IntentId::new(SYSTEM_STARTUP_INTENT_ID).expect("system startup intent id is valid"),
            intent_type: IntentType::SystemStartup,
            metadata: IntentMetadata {
                label: Some("System Startup".into()),
            },
        }
    }

    pub fn system_shutdown() -> Self {
        Self {
            id: IntentId::new(SYSTEM_SHUTDOWN_INTENT_ID)
                .expect("system shutdown intent id is valid"),
            intent_type: IntentType::SystemShutdown,
            metadata: IntentMetadata {
                label: Some("System Shutdown".into()),
            },
        }
    }

    /// Motivation intent for AI-proposed actions (no authority implied).
    pub fn ai_suggestion() -> Self {
        Self {
            id: IntentId::new(AI_SUGGESTION_INTENT_ID).expect("ai suggestion intent id is valid"),
            intent_type: IntentType::AISuggestion,
            metadata: IntentMetadata {
                label: Some("AI Suggestion".into()),
            },
        }
    }

    /// Named AI suggestion intent when a distinct id is required.
    pub fn ai_suggestion_named(id: impl Into<String>) -> Result<Self> {
        Ok(Self {
            id: IntentId::new(id)?,
            intent_type: IntentType::AISuggestion,
            metadata: IntentMetadata::default(),
        })
    }

    pub fn automation(id: impl Into<String>) -> Result<Self> {
        Ok(Self {
            id: IntentId::new(id)?,
            intent_type: IntentType::Automation,
            metadata: IntentMetadata::default(),
        })
    }

    pub fn plugin_request(id: impl Into<String>) -> Result<Self> {
        Ok(Self {
            id: IntentId::new(id)?,
            intent_type: IntentType::PluginRequest,
            metadata: IntentMetadata::default(),
        })
    }
}

impl IntentContext {
    pub fn new(intent: Intent) -> Self {
        Self { intent }
    }

    pub fn user_request() -> Self {
        Self::new(Intent::user_request())
    }

    pub fn system_startup() -> Self {
        Self::new(Intent::system_startup())
    }

    pub fn system_shutdown() -> Self {
        Self::new(Intent::system_shutdown())
    }

    /// AI-proposed action motivation — carries no privileges.
    pub fn ai_suggestion() -> Self {
        Self::new(Intent::ai_suggestion())
    }

    pub fn ai_suggestion_with_label(label: impl Into<String>) -> Self {
        let mut intent = Intent::ai_suggestion();
        intent.metadata.label = Some(label.into());
        Self::new(intent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_active_intent_types() {
        assert_eq!(Intent::user_request().intent_type, IntentType::UserRequest);
        assert_eq!(Intent::system_startup().intent_type, IntentType::SystemStartup);
        assert_eq!(Intent::system_shutdown().intent_type, IntentType::SystemShutdown);
    }

    #[test]
    fn intent_context_round_trips_through_serialization() {
        let context = IntentContext::user_request();
        let json = serde_json::to_string(&context).unwrap();
        let restored: IntentContext = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, context);
    }

    #[test]
    fn ai_suggestion_intent_is_first_class() {
        let context = IntentContext::ai_suggestion();
        assert_eq!(context.intent.intent_type, IntentType::AISuggestion);
        assert_eq!(context.intent.id.as_str(), AI_SUGGESTION_INTENT_ID);
        assert_eq!(
            Intent::ai_suggestion_named("ai-suggest-1")
                .unwrap()
                .intent_type,
            IntentType::AISuggestion
        );
    }
}
