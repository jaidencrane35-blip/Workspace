//! Programme IV Batch 15 — Workspace Assistant Interaction Intelligence commands.

use workspace_domain::{
    AssistantSurfaceScope, Capability, WorkspaceAssistantInteractionExplanation,
    WorkspaceAssistantInteractionProjection, WorkspaceAssistantInteractionSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceAssistantInteractionService;

pub struct PackageWorkspaceAssistantInteraction {
    pub workspace_id: String,
    pub human_ask: String,
    pub scope: AssistantSurfaceScope,
}

impl PackageWorkspaceAssistantInteraction {
    pub fn new(workspace_id: impl Into<String>, human_ask: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            human_ask: human_ask.into(),
            scope: AssistantSurfaceScope::interaction_default(),
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

impl Command for PackageWorkspaceAssistantInteraction {
    fn name(&self) -> &'static str {
        "PackageWorkspaceAssistantInteraction"
    }
}

impl MutationCommand for PackageWorkspaceAssistantInteraction {
    type Output = WorkspaceAssistantInteractionProjection;

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
        WorkspaceAssistantInteractionService::package_interaction(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.human_ask.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceAssistantInteraction {
    pub workspace_id: String,
}

impl GetWorkspaceAssistantInteraction {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceAssistantInteraction {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantInteraction"
    }
}

impl QueryCommand for GetWorkspaceAssistantInteraction {
    type Output = WorkspaceAssistantInteractionProjection;

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
        WorkspaceAssistantInteractionService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceAssistantInteractionSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceAssistantInteractionSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceAssistantInteractionSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantInteractionSummary"
    }
}

impl QueryCommand for GetWorkspaceAssistantInteractionSummary {
    type Output = WorkspaceAssistantInteractionSummary;

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
            WorkspaceAssistantInteractionService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(projection.summary(self.history_limit))
    }
}

pub struct ExplainAssistantInteraction {
    pub workspace_id: String,
}

impl ExplainAssistantInteraction {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainAssistantInteraction {
    fn name(&self) -> &'static str {
        "ExplainAssistantInteraction"
    }
}

impl QueryCommand for ExplainAssistantInteraction {
    type Output = WorkspaceAssistantInteractionExplanation;

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
        WorkspaceAssistantInteractionService::explain(&ctx.database, self.workspace_id)
    }
}
