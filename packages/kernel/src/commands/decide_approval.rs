use serde_json::json;

use crate::commands::context::CommandContext;
use crate::commands::r#trait::MutationCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use crate::services::PermissionApprovalService;
use workspace_domain::{
    ApprovalDecisionKind, ApprovalDecisionResult, Capability, PermissionApprovalRequestId,
};

/// Local-user decision on a pending permission approval (allow once / deny).
pub struct DecideApproval {
    pub request_id: PermissionApprovalRequestId,
    pub decision: ApprovalDecisionKind,
}

impl DecideApproval {
    pub fn new(request_id: PermissionApprovalRequestId, decision: ApprovalDecisionKind) -> Self {
        Self {
            request_id,
            decision,
        }
    }
}

impl crate::commands::Command for DecideApproval {
    fn name(&self) -> &'static str {
        "DecideApproval"
    }
}

impl MutationCommand for DecideApproval {
    type Output = ApprovalDecisionResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::audit_write()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "approval_request_id": output.request.id.to_string(),
                "decision": self.decision.as_str(),
                "status": output.request.status.as_str(),
                "grant_id": output.grant.as_ref().map(|g| g.id.to_string()),
                "grantee_actor_id": output.request.requesting_actor_id,
                "capability": output.request.capability,
                "command_name": output.request.command_name,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<ApprovalDecisionResult> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ctx.with_database(|db| {
            PermissionApprovalService::decide(
                db,
                &ctx.actor_context,
                &self.request_id,
                self.decision,
            )
        })
    }
}
