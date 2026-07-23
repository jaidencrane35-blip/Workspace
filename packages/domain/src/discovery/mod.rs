//! Derived capability discovery — read-only view of actor authority (Sprint 16).
//!
//! Distinct from [`crate::capability::CapabilitySet`], which tracks granted
//! capability identifiers for pipeline context. This module exposes what an
//! actor can currently do after policy evaluation.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::actor::ActorType;
use crate::ids::ActorId;
use crate::capability::Capability;
use crate::ids::ActionIntentId;
use crate::intent::ActionIntentCategory;

pub type Result<T> = std::result::Result<T, CapabilityDiscoveryError>;

/// Discovery-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CapabilityDiscoveryError {
    #[error("Duplicate capability in discovery: {0}")]
    DuplicateCapability(String),

    #[error("Duplicate intent in discovery: {0}")]
    DuplicateIntent(String),

    #[error("Actor reference mismatch")]
    ActorMismatch,
}

/// Summary of an action intent and whether it is currently available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailableIntentSummary {
    pub intent_id: ActionIntentId,
    pub name: String,
    pub description: String,
    pub category: ActionIntentCategory,
    pub command_name: String,
    pub capability_required: Capability,
    pub available: bool,
}

/// Derived, non-authoritative view of an actor's current capabilities and actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDiscovery {
    pub actor_id: ActorId,
    pub actor_type: ActorType,
    pub capabilities: Vec<Capability>,
    pub available_intents: Vec<AvailableIntentSummary>,
    pub generated_at: String,
}

impl CapabilityDiscovery {
    pub fn validate(&self) -> Result<()> {
        let mut capability_ids = BTreeSet::new();
        for capability in &self.capabilities {
            if !capability_ids.insert(capability.id.as_str()) {
                return Err(CapabilityDiscoveryError::DuplicateCapability(
                    capability.id.to_string(),
                ));
            }
        }

        let mut intent_ids = BTreeSet::new();
        for intent in &self.available_intents {
            if !intent_ids.insert(intent.intent_id.as_str()) {
                return Err(CapabilityDiscoveryError::DuplicateIntent(
                    intent.intent_id.to_string(),
                ));
            }
        }

        Ok(())
    }

    pub fn available_intent_ids(&self) -> Vec<&ActionIntentId> {
        self.available_intents
            .iter()
            .filter(|intent| intent.available)
            .map(|intent| &intent.intent_id)
            .collect()
    }

    pub fn unavailable_intent_ids(&self) -> Vec<&ActionIntentId> {
        self.available_intents
            .iter()
            .filter(|intent| !intent.available)
            .map(|intent| &intent.intent_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::capability::Capability;

    #[test]
    fn validates_unique_capabilities_and_intents() {
        let discovery = CapabilityDiscovery {
            actor_id: ActorId::new("local-user").unwrap(),
            actor_type: ActorType::LocalUser,
            capabilities: vec![Capability::workspace_read()],
            available_intents: vec![AvailableIntentSummary {
                intent_id: ActionIntentId::new("get-workspace").unwrap(),
                name: "Get Workspace".into(),
                description: "Load a workspace.".into(),
                category: ActionIntentCategory::Workspace,
                command_name: "GetWorkspace".into(),
                capability_required: Capability::workspace_read(),
                available: true,
            }],
            generated_at: Utc::now().to_rfc3339(),
        };

        assert!(discovery.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_capabilities() {
        let discovery = CapabilityDiscovery {
            actor_id: ActorId::new("local-user").unwrap(),
            actor_type: ActorType::LocalUser,
            capabilities: vec![
                Capability::workspace_read(),
                Capability::workspace_read(),
            ],
            available_intents: vec![],
            generated_at: Utc::now().to_rfc3339(),
        };

        assert!(matches!(
            discovery.validate(),
            Err(CapabilityDiscoveryError::DuplicateCapability(_))
        ));
    }
}
