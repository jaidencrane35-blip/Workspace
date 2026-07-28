//! Programme II Batch 5 — Cognitive Orchestration commands.

use workspace_domain::{
    Capability, WorkspaceOrchestrationSnapshot, WorkspaceOrchestrationSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceCognitiveOrchestrationService;

pub struct GenerateWorkspaceOrchestration {
    pub workspace_id: String,
}

impl GenerateWorkspaceOrchestration {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateWorkspaceOrchestration {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceOrchestration"
    }
}

impl MutationCommand for GenerateWorkspaceOrchestration {
    type Output = WorkspaceOrchestrationSnapshot;

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
        WorkspaceCognitiveOrchestrationService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
        )
    }
}

pub struct GetWorkspaceOrchestration {
    pub workspace_id: String,
}

impl GetWorkspaceOrchestration {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceOrchestration {
    fn name(&self) -> &'static str {
        "GetWorkspaceOrchestration"
    }
}

impl QueryCommand for GetWorkspaceOrchestration {
    type Output = WorkspaceOrchestrationSnapshot;

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
        WorkspaceCognitiveOrchestrationService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceOrchestrationSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceOrchestrationSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceOrchestrationSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceOrchestrationSummary"
    }
}

impl QueryCommand for GetWorkspaceOrchestrationSummary {
    type Output = WorkspaceOrchestrationSummary;

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
        let snap = WorkspaceCognitiveOrchestrationService::load_snapshot(
            &ctx.database,
            self.workspace_id,
        )?;
        Ok(snap.summary(self.history_limit))
    }
}
