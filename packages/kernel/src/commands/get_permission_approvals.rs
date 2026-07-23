use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::PermissionApprovalService;
use workspace_domain::{Capability, PermissionApprovalRequest};

/// Lists pending and recent permission approval requests.
pub struct GetPermissionApprovals {
    pub limit: Option<usize>,
}

impl GetPermissionApprovals {
    pub fn new(limit: Option<usize>) -> Self {
        Self { limit }
    }
}

impl crate::commands::Command for GetPermissionApprovals {
    fn name(&self) -> &'static str {
        "GetPermissionApprovals"
    }
}

impl QueryCommand for GetPermissionApprovals {
    type Output = Vec<PermissionApprovalRequest>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<PermissionApprovalRequest>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        ctx.with_database(|db| PermissionApprovalService::list_recent(db, self.limit))
    }
}
