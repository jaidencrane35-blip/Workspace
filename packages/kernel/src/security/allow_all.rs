use super::gate::{PermissionDecision, PermissionGate, PermissionRequest};
use crate::error::Result;

/// Default permission gate — allows all mutations until approval storage exists.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AllowAllPermissionGate;

impl PermissionGate for AllowAllPermissionGate {
    fn authorize(&self, _request: &PermissionRequest) -> Result<PermissionDecision> {
        Ok(PermissionDecision::Allowed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::PermissionSubject;
    use workspace_domain::{Actor, Capability, Intent};

    #[test]
    fn allows_all_mutations() {
        let gate = AllowAllPermissionGate;
        let request = PermissionRequest {
            actor: Actor::local_user(),
            intent: Intent::user_request(),
            capability: Capability::workspace_write(),
            command: "CreateWorkspace",
            subject: PermissionSubject::Workspace,
        };

        assert_eq!(
            gate.authorize(&request).unwrap(),
            PermissionDecision::Allowed
        );
        assert!(gate.require(&request).is_ok());
    }

    #[test]
    fn permission_request_carries_actor_intent_and_capability() {
        let request = PermissionRequest {
            actor: Actor::local_user(),
            intent: Intent::user_request(),
            capability: Capability::settings_write(),
            command: "UpdateSettings",
            subject: PermissionSubject::Settings,
        };

        assert_eq!(request.actor, Actor::local_user());
        assert_eq!(request.intent.intent_type, workspace_domain::IntentType::UserRequest);
        assert_eq!(request.capability.id.as_str(), "settings.write");
    }
}
