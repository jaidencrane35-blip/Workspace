//! Informational action catalog — "what actions exist?" (Sprint 52).
//!
//! Distinct from [`crate::discovery::CapabilityDiscovery`], which answers
//! "what may this actor execute right now?". Catalog entries never grant
//! authority. Seeing an action does not authorize it.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::capability::Capability;
use crate::ids::ActionIntentId;
use crate::intent::{
    ActionIntentCategory, ActionIntentDefinition, ActionIntentRegistry, TargetRequirement,
};
use crate::resource::ResourceKind;

/// Catalog validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ActionCatalogError {
    #[error("Duplicate action catalog entry: {0}")]
    DuplicateEntry(String),
}

/// One catalogued workspace action (metadata only — not an authorization decision).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionCatalogEntry {
    pub intent_id: ActionIntentId,
    pub name: String,
    pub description: String,
    pub category: ActionIntentCategory,
    pub command_name: String,
    pub capability_required: Capability,
    pub target_requirement: TargetRequirement,
    pub allowed_target_kinds: Vec<ResourceKind>,
}

impl ActionCatalogEntry {
    pub fn from_definition(definition: &ActionIntentDefinition) -> Self {
        Self {
            intent_id: definition.id.clone(),
            name: definition.name.clone(),
            description: definition.description.clone(),
            category: definition.category,
            command_name: definition.command_name.to_string(),
            capability_required: definition.capability_required.clone(),
            target_requirement: definition.target_requirement,
            allowed_target_kinds: definition.allowed_target_kinds.to_vec(),
        }
    }

    /// Human-readable capability requirement for AI explanations.
    pub fn capability_explanation(&self) -> String {
        format!(
            "Action '{}' ({}) requires capability '{}'.",
            self.name,
            self.command_name,
            self.capability_required.id.as_str()
        )
    }
}

/// Complete informational catalog of registered actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionCatalog {
    pub entries: Vec<ActionCatalogEntry>,
    pub generated_at: String,
}

impl ActionCatalog {
    /// Builds the catalog from the static action registry — no policy, no gate.
    pub fn from_registry() -> Self {
        let mut entries: Vec<_> = ActionIntentRegistry::all()
            .iter()
            .map(ActionCatalogEntry::from_definition)
            .collect();
        entries.sort_by(|left, right| left.intent_id.as_str().cmp(right.intent_id.as_str()));
        Self {
            entries,
            generated_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn validate(&self) -> Result<(), ActionCatalogError> {
        let mut seen = std::collections::BTreeSet::new();
        for entry in &self.entries {
            if !seen.insert(entry.intent_id.as_str()) {
                return Err(ActionCatalogError::DuplicateEntry(
                    entry.intent_id.to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn find_by_intent_id(&self, intent_id: &str) -> Option<&ActionCatalogEntry> {
        self.entries
            .iter()
            .find(|entry| entry.intent_id.as_str() == intent_id)
    }

    pub fn find_by_command(&self, command_name: &str) -> Option<&ActionCatalogEntry> {
        self.entries
            .iter()
            .find(|entry| entry.command_name == command_name)
    }
}

/// AI-facing awareness of catalogued actions (still not authorization).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiActionAwareness {
    pub catalog: ActionCatalog,
    /// Intents the AI understands exist (informational).
    pub known_intent_ids: Vec<String>,
}

impl AiActionAwareness {
    pub fn from_catalog(catalog: ActionCatalog) -> Self {
        let known_intent_ids = catalog
            .entries
            .iter()
            .map(|entry| entry.intent_id.to_string())
            .collect();
        Self {
            catalog,
            known_intent_ids,
        }
    }

    pub fn explain_capability_for_command(&self, command_name: &str) -> Option<String> {
        self.catalog
            .find_by_command(command_name)
            .map(ActionCatalogEntry::capability_explanation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_lists_launch_without_implying_authority() {
        let catalog = ActionCatalog::from_registry();
        catalog.validate().unwrap();
        let launch = catalog.find_by_intent_id("launch-application").unwrap();
        assert_eq!(launch.command_name, "LaunchApplication");
        assert_eq!(
            launch.capability_required.id.as_str(),
            "application.launch"
        );
        assert!(launch
            .capability_explanation()
            .contains("application.launch"));
    }

    #[test]
    fn awareness_exposes_known_actions() {
        let awareness = AiActionAwareness::from_catalog(ActionCatalog::from_registry());
        assert!(awareness
            .known_intent_ids
            .iter()
            .any(|id| id == "launch-application"));
        assert!(awareness
            .explain_capability_for_command("LaunchApplication")
            .unwrap()
            .contains("application.launch"));
    }
}
