//! Programme IV Batch 12 — Workspace Assistant Context Intelligence commands.

use workspace_domain::{
    AssistantSurfaceScope, Capability, WorkspaceAssistantContextExplanation,
    WorkspaceAssistantContextProjection, WorkspaceAssistantContextSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceAssistantContextService;

pub struct PackageWorkspaceAssistantContext {
    pub workspace_id: String,
    pub scope: AssistantSurfaceScope,
}

impl PackageWorkspaceAssistantContext {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope: AssistantSurfaceScope::presentation_default(),
        }
    }

    pub fn with_scope(workspace_id: impl Into<String>, scope: AssistantSurfaceScope) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope,
        }
    }
}

impl Command for PackageWorkspaceAssistantContext {
    fn name(&self) -> &'static str {
        "PackageWorkspaceAssistantContext"
    }
}

impl MutationCommand for PackageWorkspaceAssistantContext {
    type Output = WorkspaceAssistantContextProjection;

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
        WorkspaceAssistantContextService::package_context(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceAssistantContext {
    pub workspace_id: String,
}

impl GetWorkspaceAssistantContext {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceAssistantContext {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantContext"
    }
}

impl QueryCommand for GetWorkspaceAssistantContext {
    type Output = WorkspaceAssistantContextProjection;

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
        WorkspaceAssistantContextService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceAssistantContextSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceAssistantContextSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceAssistantContextSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantContextSummary"
    }
}

impl QueryCommand for GetWorkspaceAssistantContextSummary {
    type Output = WorkspaceAssistantContextSummary;

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
        let projection =
            WorkspaceAssistantContextService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(projection.summary(self.history_limit))
    }
}

pub struct ExplainAssistantContext {
    pub workspace_id: String,
}

impl ExplainAssistantContext {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainAssistantContext {
    fn name(&self) -> &'static str {
        "ExplainAssistantContext"
    }
}

impl QueryCommand for ExplainAssistantContext {
    type Output = WorkspaceAssistantContextExplanation;

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
        WorkspaceAssistantContextService::explain(&ctx.database, self.workspace_id)
    }
}
