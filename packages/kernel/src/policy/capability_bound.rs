use super::context::{PolicyContext, PolicyResult};
use super::PermissionPolicy;
use crate::error::Result;

/// Default Phase 2 policy — allows only capabilities present in the granted set.
///
/// Unknown / missing capabilities are denied (no silent allow-all).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityBoundPolicy;

impl PermissionPolicy for CapabilityBoundPolicy {
    fn evaluate(&self, context: &PolicyContext) -> Result<PolicyResult> {
        if context
            .granted_capabilities
            .contains(&context.capability)
        {
            Ok(PolicyResult::allow())
        } else {
            Ok(PolicyResult::deny(format!(
                "capability '{}' not granted for command '{}'",
                context.capability.id, context.command
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::PermissionSubject;
    use workspace_domain::{Actor, Capability, CapabilitySet, Intent, ResourceKind};

    fn context_with_granted(granted: CapabilitySet) -> PolicyContext {
        PolicyContext::new(
            Actor::local_user().id.to_string(),
            Intent::user_request(),
            Capability::workspace_write(),
            "CreateWorkspace",
            PermissionSubject::Resource(ResourceKind::Workspace),
        )
        .with_granted(granted)
    }

    #[test]
    fn allows_when_capability_is_granted() {
        let result = CapabilityBoundPolicy
            .evaluate(&context_with_granted(CapabilitySet::local_user_standard()))
            .unwrap();
        assert!(result.is_allowed());
    }

    #[test]
    fn denies_when_capability_is_missing() {
        let result = CapabilityBoundPolicy
            .evaluate(&context_with_granted(CapabilitySet::new()))
            .unwrap();
        assert!(!result.is_allowed());
    }
}
