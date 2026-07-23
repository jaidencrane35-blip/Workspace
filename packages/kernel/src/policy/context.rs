use workspace_domain::{Capability, CapabilitySet, Intent};

use crate::security::PermissionSubject;

/// Inputs for policy evaluation — immutable snapshot of a permission check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyContext {
    pub actor_id: String,
    pub intent: Intent,
    pub capability: Capability,
    pub command: &'static str,
    pub subject: PermissionSubject,
    /// Capabilities currently granted to the actor (empty = default deny under CapabilityBoundPolicy).
    pub granted_capabilities: CapabilitySet,
}

impl PolicyContext {
    pub fn new(
        actor_id: impl Into<String>,
        intent: Intent,
        capability: Capability,
        command: &'static str,
        subject: PermissionSubject,
    ) -> Self {
        Self {
            actor_id: actor_id.into(),
            intent,
            capability,
            command,
            subject,
            granted_capabilities: CapabilitySet::new(),
        }
    }

    pub fn with_granted(mut self, granted: CapabilitySet) -> Self {
        self.granted_capabilities = granted;
        self
    }
}

/// Outcome of evaluating a single policy rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny { reason: String },
}

/// Result returned by policy evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyResult {
    pub decision: PolicyDecision,
}

impl PolicyResult {
    pub fn allow() -> Self {
        Self {
            decision: PolicyDecision::Allow,
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            decision: PolicyDecision::Deny {
                reason: reason.into(),
            },
        }
    }

    pub fn is_allowed(&self) -> bool {
        matches!(self.decision, PolicyDecision::Allow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{Actor, Capability, IntentType, ResourceKind};

    #[test]
    fn policy_context_carries_intent_and_capability() {
        let context = PolicyContext::new(
            Actor::local_user().id.to_string(),
            workspace_domain::Intent::user_request(),
            Capability::workspace_write(),
            "CreateWorkspace",
            PermissionSubject::Resource(ResourceKind::Workspace),
        );

        assert_eq!(context.intent.intent_type, IntentType::UserRequest);
        assert_eq!(context.capability.id.as_str(), "workspace.write");
    }
}
