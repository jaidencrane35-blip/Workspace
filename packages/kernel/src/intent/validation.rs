//! Validates action intents before command execution.

use workspace_domain::{
    ActionIntentError, ActionIntentRegistry, ActionIntentRequest, Capability,
};

use super::mapping::CommandIntentMapping;
use crate::error::{KernelError, Result};

/// Validates action-intent catalog metadata before commands run through the pipeline.
///
/// Distinct from [`crate::GovernedIntentExecutionService`], which prepares
/// suggestion-driven execution requests (Sprint 24).
pub struct ActionIntentValidationService;

impl ActionIntentValidationService {
    pub fn lookup(request: &ActionIntentRequest) -> Result<workspace_domain::ActionIntentDefinition> {
        ActionIntentRegistry::lookup(&request.intent_id).ok_or_else(|| {
            KernelError::ActionIntentNotFound {
                intent_id: request.intent_id.to_string(),
            }
        })
    }

    pub fn validate_request(
        request: &ActionIntentRequest,
    ) -> Result<workspace_domain::ActionIntentDefinition> {
        let definition = Self::lookup(request)?;
        request
            .validate(&definition)
            .map_err(map_action_intent_error)?;
        Ok(definition)
    }

    pub fn validate_capability_match(
        definition: &workspace_domain::ActionIntentDefinition,
        command_capability: &Capability,
    ) -> Result<()> {
        if definition.matches_capability(command_capability) {
            Ok(())
        } else {
            Err(KernelError::ActionIntentCapabilityMismatch {
                intent_id: definition.id.to_string(),
                expected: definition.capability_required.id.to_string(),
                actual: command_capability.id.to_string(),
            })
        }
    }

    pub fn validate_before_command(
        request: &ActionIntentRequest,
        command_name: &str,
        command_capability: &Capability,
    ) -> Result<String> {
        let definition = Self::validate_request(request)?;

        if definition.command_name != command_name {
            return Err(KernelError::ActionIntentCommandMismatch {
                intent_id: definition.id.to_string(),
                expected_command: definition.command_name.into(),
                actual_command: command_name.into(),
            });
        }

        Self::validate_capability_match(&definition, command_capability)?;
        Ok(definition.command_name.to_string())
    }

    pub fn resolve_command_name(request: &ActionIntentRequest) -> Result<String> {
        let definition = Self::validate_request(request)?;
        CommandIntentMapping::resolve_command_name(&request.intent_id)?;
        Ok(definition.command_name.to_string())
    }
}

fn map_action_intent_error(error: ActionIntentError) -> KernelError {
    match error {
        ActionIntentError::NotFound(intent_id) => KernelError::ActionIntentNotFound { intent_id },
        ActionIntentError::DuplicateId(intent_id) => KernelError::ActionIntentValidation {
            message: format!("duplicate action intent id: {intent_id}"),
        },
        ActionIntentError::MissingTarget => KernelError::ActionIntentValidation {
            message: "action intent requires a target resource".into(),
        },
        ActionIntentError::UnexpectedTarget => KernelError::ActionIntentValidation {
            message: "action intent does not accept a target resource".into(),
        },
        ActionIntentError::InvalidTargetKind { kind } => KernelError::ActionIntentValidation {
            message: format!("invalid target resource kind: {kind}"),
        },
        ActionIntentError::CapabilityMismatch { expected, actual } => {
            KernelError::ActionIntentCapabilityMismatch {
                intent_id: "unknown".into(),
                expected,
                actual,
            }
        }
        ActionIntentError::InvalidMetadata { message } => KernelError::ActionIntentValidation { message },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{
        ActionIntentId, ActionIntentRequest, Capability, ResourceId, ResourceKind, ResourceRef,
    };

    fn zone_ref(id: &str) -> ResourceRef {
        ResourceRef::new(ResourceKind::Zone, ResourceId::new(id).unwrap())
    }

    #[test]
    fn validates_request_before_command() {
        let request = ActionIntentRequest::new(ActionIntentId::new("create-workspace").unwrap());
        let definition = ActionIntentValidationService::validate_request(&request).unwrap();
        assert_eq!(definition.command_name, "CreateWorkspace");
    }

    #[test]
    fn rejects_capability_mismatch() {
        let id = ActionIntentId::new("create-workspace").unwrap();
        let definition = ActionIntentRegistry::lookup(&id).unwrap();
        let error = ActionIntentValidationService::validate_capability_match(
            &definition,
            &Capability::zone_write(),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            KernelError::ActionIntentCapabilityMismatch { .. }
        ));
    }

    #[test]
    fn validate_before_command_checks_name_and_capability() {
        let request = ActionIntentRequest::new(ActionIntentId::new("delete-zone").unwrap())
            .with_target(zone_ref("zone-1"));
        ActionIntentValidationService::validate_before_command(
            &request,
            "DeleteZone",
            &Capability::zone_write(),
        )
        .unwrap();
    }

    #[test]
    fn validate_before_command_rejects_command_name_mismatch() {
        let request = ActionIntentRequest::new(ActionIntentId::new("create-workspace").unwrap());
        let error = ActionIntentValidationService::validate_before_command(
            &request,
            "DeleteZone",
            &Capability::workspace_write(),
        )
        .unwrap_err();
        assert!(matches!(
            error,
            KernelError::ActionIntentCommandMismatch { .. }
        ));
    }
}
