//! Programme IV Batch 14 — Workspace Assistant Explanation Intelligence commands.

use workspace_domain::{
    AssistantSurfaceScope, Capability, WorkspaceAssistantExplanationExplanation,
    WorkspaceAssistantExplanationProjection, WorkspaceAssistantExplanationSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceAssistantExplanationService;

pub struct PackageWorkspaceAssistantExplanation {
    pub workspace_id: String,
    pub human_ask: String,
    pub scope: AssistantSurfaceScope,
}

impl PackageWorkspaceAssistantExplanation {
    pub fn new(workspace_id: impl Into<String>, human_ask: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            human_ask: human_ask.into(),
            scope: AssistantSurfaceScope::explanation_default(),
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

impl Command for PackageWorkspaceAssistantExplanation {
    fn name(&self) -> &'static str {
        "PackageWorkspaceAssistantExplanation"
    }
}

impl MutationCommand for PackageWorkspaceAssistantExplanation {
    type Output = WorkspaceAssistantExplanationProjection;

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
        WorkspaceAssistantExplanationService::package_explanation(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.human_ask.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceAssistantExplanation {
    pub workspace_id: String,
}

impl GetWorkspaceAssistantExplanation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceAssistantExplanation {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantExplanation"
    }
}

impl QueryCommand for GetWorkspaceAssistantExplanation {
    type Output = WorkspaceAssistantExplanationProjection;

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
        WorkspaceAssistantExplanationService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceAssistantExplanationSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceAssistantExplanationSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceAssistantExplanationSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantExplanationSummary"
    }
}

impl QueryCommand for GetWorkspaceAssistantExplanationSummary {
    type Output = WorkspaceAssistantExplanationSummary;

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
            WorkspaceAssistantExplanationService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(projection.summary(self.history_limit))
    }
}

pub struct ExplainAssistantExplanation {
    pub workspace_id: String,
}

impl ExplainAssistantExplanation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainAssistantExplanation {
    fn name(&self) -> &'static str {
        "ExplainAssistantExplanation"
    }
}

impl QueryCommand for ExplainAssistantExplanation {
    type Output = WorkspaceAssistantExplanationExplanation;

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
        WorkspaceAssistantExplanationService::explain(&ctx.database, self.workspace_id)
    }
}
