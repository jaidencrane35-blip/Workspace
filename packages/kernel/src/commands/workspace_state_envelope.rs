//! Programme III Batch 1 — Unified Workspace State Envelope commands.
//!
//! Named *Envelope* to avoid colliding with desktop observation `GetWorkspaceState`.

use workspace_domain::{Capability, WorkspaceStateSnapshot, WorkspaceStateSummary};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceStateCompositionService;

pub struct GenerateWorkspaceStateEnvelope {
    pub workspace_id: String,
}

impl GenerateWorkspaceStateEnvelope {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateWorkspaceStateEnvelope {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceStateEnvelope"
    }
}

impl MutationCommand for GenerateWorkspaceStateEnvelope {
    type Output = WorkspaceStateSnapshot;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Self::Output> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceStateCompositionService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
        )
    }
}

pub struct GetWorkspaceStateEnvelope {
    pub workspace_id: String,
}

impl GetWorkspaceStateEnvelope {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceStateEnvelope {
    fn name(&self) -> &'static str {
        "GetWorkspaceStateEnvelope"
    }
}

impl QueryCommand for GetWorkspaceStateEnvelope {
    type Output = WorkspaceStateSnapshot;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Self::Output> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceStateCompositionService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceStateEnvelopeSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceStateEnvelopeSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceStateEnvelopeSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceStateEnvelopeSummary"
    }
}

impl QueryCommand for GetWorkspaceStateEnvelopeSummary {
    type Output = WorkspaceStateSummary;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::work_context_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Self::Output> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        let snap =
            WorkspaceStateCompositionService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}
