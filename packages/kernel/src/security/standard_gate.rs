use workspace_domain::ActorType;

use super::gate::{PermissionDecision, PermissionGate, PermissionRequest};
use crate::error::Result;

/// Default Phase 2 gate — allows local user and system; non-human actors require approval.
///
/// `ApprovalRequired` is returned (not silent allow) so the gateway can deny execution
/// until a future approval flow exists.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StandardPermissionGate;

impl PermissionGate for StandardPermissionGate {
    fn authorize(&self, request: &PermissionRequest) -> Result<PermissionDecision> {
        match request.actor.actor_type {
            ActorType::LocalUser | ActorType::System => Ok(PermissionDecision::Allowed),
            ActorType::AIAssistant
            | ActorType::Automation
            | ActorType::Plugin
            | ActorType::RemoteSession => Ok(PermissionDecision::ApprovalRequired {
                reason: format!(
                    "actor type '{:?}' requires explicit approval for '{}'",
                    request.actor.actor_type, request.command
                ),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::PermissionSubject;
    use workspace_domain::{Actor, Capability, Intent, ResourceKind};

    fn request_for(actor: Actor) -> PermissionRequest {
        PermissionRequest {
            actor,
            intent: Intent::user_request(),
            capability: Capability::workspace_write(),
            command: "CreateWorkspace",
            subject: PermissionSubject::Resource(ResourceKind::Workspace),
        }
    }

    #[test]
    fn local_user_is_allowed() {
        let decision = StandardPermissionGate
            .authorize(&request_for(Actor::local_user()))
            .unwrap();
        assert_eq!(decision, PermissionDecision::Allowed);
    }

    #[test]
    fn ai_requires_approval() {
        let decision = StandardPermissionGate
            .authorize(&request_for(Actor::ai_assistant("ai-1").unwrap()))
            .unwrap();
        assert!(matches!(
            decision,
            PermissionDecision::ApprovalRequired { .. }
        ));
    }
}
