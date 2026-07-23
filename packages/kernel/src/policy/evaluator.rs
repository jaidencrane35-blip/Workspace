use super::context::{PolicyContext, PolicyResult};
use super::PermissionPolicy;
use crate::error::Result;

/// Evaluates permission policies — indirection point for future composite rules.
pub trait PolicyEvaluator: Send + Sync {
    fn evaluate(&self, policy: &dyn PermissionPolicy, context: &PolicyContext) -> Result<PolicyResult>;
}

/// Default evaluator that delegates directly to the supplied policy.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DefaultPolicyEvaluator;

impl PolicyEvaluator for DefaultPolicyEvaluator {
    fn evaluate(
        &self,
        policy: &dyn PermissionPolicy,
        context: &PolicyContext,
    ) -> Result<PolicyResult> {
        policy.evaluate(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{AlwaysAllowPolicy, PolicyDecision};
    use crate::security::PermissionSubject;
    use workspace_domain::{Actor, Capability, Intent};

    #[test]
    fn default_evaluator_delegates_to_policy() {
        let context = PolicyContext::new(
            Actor::local_user().id.to_string(),
            Intent::user_request(),
            Capability::settings_write(),
            "UpdateSettings",
            PermissionSubject::Settings,
        );

        let result = DefaultPolicyEvaluator
            .evaluate(&AlwaysAllowPolicy, &context)
            .unwrap();

        assert!(result.is_allowed());
        assert_eq!(result.decision, PolicyDecision::Allow);
    }
}
