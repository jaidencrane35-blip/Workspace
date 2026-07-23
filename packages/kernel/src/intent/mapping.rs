//! Maps action intents to existing command names.

use workspace_domain::{ActionIntentId, ActionIntentRegistry};

use crate::error::{KernelError, Result};

/// Resolves action intent identifiers to command names.
pub struct CommandIntentMapping;

impl CommandIntentMapping {
    pub fn resolve_command_name(intent_id: &ActionIntentId) -> Result<&'static str> {
        ActionIntentRegistry::lookup(intent_id)
            .map(|definition| definition.command_name)
            .ok_or_else(|| KernelError::ActionIntentNotFound {
                intent_id: intent_id.to_string(),
            })
    }

    pub fn resolve_for_command(
        command_name: &str,
    ) -> Option<workspace_domain::ActionIntentDefinition> {
        ActionIntentRegistry::all()
            .into_iter()
            .find(|definition| definition.command_name == command_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{ActionIntentId, Capability};

    #[test]
    fn maps_create_workspace_intent_to_command() {
        let id = ActionIntentId::new("create-workspace").unwrap();
        assert_eq!(
            CommandIntentMapping::resolve_command_name(&id).unwrap(),
            "CreateWorkspace"
        );
    }

    #[test]
    fn missing_mapping_fails() {
        let id = ActionIntentId::new("unknown-intent").unwrap();
        assert!(matches!(
            CommandIntentMapping::resolve_command_name(&id),
            Err(KernelError::ActionIntentNotFound { .. })
        ));
    }

    #[test]
    fn reverse_lookup_for_create_workspace() {
        let definition =
            CommandIntentMapping::resolve_for_command("CreateWorkspace").unwrap();
        assert!(definition.matches_capability(&Capability::workspace_write()));
    }
}