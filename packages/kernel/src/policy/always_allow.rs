use super::context::{PolicyContext, PolicyResult};
use super::PermissionPolicy;
use crate::error::Result;

/// Default policy — allows all operations until real policy storage exists.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AlwaysAllowPolicy;

impl PermissionPolicy for AlwaysAllowPolicy {
    fn evaluate(&self, _context: &PolicyContext) -> Result<PolicyResult> {
        Ok(PolicyResult::allow())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{DefaultPolicyEvaluator, PolicyDecision, PolicyEvaluator};
    use crate::security::PermissionSubject;
    use workspace_domain::{Actor, Capability, Intent};

    #[test]
    fn always_allows_any_context() {
        let context = PolicyContext::new(
            Actor::local_user().id.to_string(),
            Intent::user_request(),
            Capability::workspace_write(),
            "CreateWorkspace",
            PermissionSubject::Workspace,
        );

        let result = AlwaysAllowPolicy.evaluate(&context).unwrap();
        assert!(result.is_allowed());

        let via_evaluator = DefaultPolicyEvaluator
            .evaluate(&AlwaysAllowPolicy, &context)
            .unwrap();
        assert!(via_evaluator.is_allowed());
    }
}
