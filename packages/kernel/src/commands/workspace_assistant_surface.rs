//! Programme IV Batch 11 — Workspace Assistant Surface commands.

use workspace_domain::{
    AssistantSurfaceScope, Capability, WorkspaceAssistantSurfaceExplanation,
    WorkspaceAssistantSurfaceProjection, WorkspaceAssistantSurfaceSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceAssistantSurfaceService;

pub struct ComposeWorkspaceAssistantTurn {
    pub workspace_id: String,
    pub human_ask: String,
    pub scope: AssistantSurfaceScope,
}

impl ComposeWorkspaceAssistantTurn {
    pub fn new(workspace_id: impl Into<String>, human_ask: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            human_ask: human_ask.into(),
            scope: AssistantSurfaceScope::presentation_default(),
        }
    }

    pub fn with_scope(
        workspace_id: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            human_ask: human_ask.into(),
            scope,
        }
    }
}

impl Command for ComposeWorkspaceAssistantTurn {
    fn name(&self) -> &'static str {
        "ComposeWorkspaceAssistantTurn"
    }
}

impl MutationCommand for ComposeWorkspaceAssistantTurn {
    type Output = WorkspaceAssistantSurfaceProjection;

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
        WorkspaceAssistantSurfaceService::compose_turn(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.human_ask.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceAssistantSurface {
    pub workspace_id: String,
}

impl GetWorkspaceAssistantSurface {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceAssistantSurface {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantSurface"
    }
}

impl QueryCommand for GetWorkspaceAssistantSurface {
    type Output = WorkspaceAssistantSurfaceProjection;

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
        WorkspaceAssistantSurfaceService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceAssistantSurfaceSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceAssistantSurfaceSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceAssistantSurfaceSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantSurfaceSummary"
    }
}

impl QueryCommand for GetWorkspaceAssistantSurfaceSummary {
    type Output = WorkspaceAssistantSurfaceSummary;

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
            WorkspaceAssistantSurfaceService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(projection.summary(self.history_limit))
    }
}

pub struct ExplainAssistantSurface {
    pub workspace_id: String,
}

impl ExplainAssistantSurface {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainAssistantSurface {
    fn name(&self) -> &'static str {
        "ExplainAssistantSurface"
    }
}

impl QueryCommand for ExplainAssistantSurface {
    type Output = WorkspaceAssistantSurfaceExplanation;

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
        WorkspaceAssistantSurfaceService::explain(&ctx.database, self.workspace_id)
    }
}
