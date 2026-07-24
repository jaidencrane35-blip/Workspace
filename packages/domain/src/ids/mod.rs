use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{DomainError, Result};

macro_rules! define_id {
    ($name:ident) => {
        /// Strongly typed domain identifier.
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self> {
                let value = value.into();
                Self::validate(&value)?;
                Ok(Self(value))
            }

            pub fn generate() -> Self {
                Self(Uuid::new_v4().to_string())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            fn validate(value: &str) -> Result<()> {
                if value.trim().is_empty() {
                    return Err(DomainError::InvalidId);
                }
                Ok(())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl FromStr for $name {
            type Err = DomainError;

            fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
                Self::new(value)
            }
        }
    };
}

define_id!(WorkspaceId);
define_id!(ZoneId);
define_id!(ApplicationId);
define_id!(WidgetId);
define_id!(LayoutId);
define_id!(AuditEventId);
define_id!(ActorId);
define_id!(IntentId);
define_id!(ActionIntentId);
define_id!(PermissionApprovalRequestId);
define_id!(CapabilityGrantId);
define_id!(AiGoalId);
define_id!(AiActionProposalId);
define_id!(AiOrchestratedPlanId);
define_id!(AiPlanStepId);
define_id!(AiAssistantWorkflowId);
define_id!(MemoryEntryId);
define_id!(ModelProviderId);
define_id!(ModelId);
define_id!(UserPreferenceId);
define_id!(ProjectId);
define_id!(TaskId);
define_id!(WorkGoalId);
define_id!(AutomationContractId);
define_id!(TriggerEventId);
define_id!(AutomationIntentProposalId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_ids_serialize_as_strings() {
        let id = WorkspaceId::new("ws-1").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"ws-1\"");
    }

    #[test]
    fn rejects_empty_ids() {
        assert_eq!(WorkspaceId::new("").unwrap_err(), DomainError::InvalidId);
        assert_eq!(ZoneId::new("  ").unwrap_err(), DomainError::InvalidId);
    }

    #[test]
    fn generates_unique_ids() {
        let a = ApplicationId::generate();
        let b = ApplicationId::generate();
        assert_ne!(a, b);
    }

    #[test]
    fn parses_from_str() {
        let id = WidgetId::from_str("widget-42").unwrap();
        assert_eq!(id.as_str(), "widget-42");
    }
}
