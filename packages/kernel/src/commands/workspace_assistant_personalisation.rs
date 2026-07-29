//! Programme IV Batch 16 — Workspace Assistant Personalisation Boundary commands.

use workspace_domain::{
    AssistantSurfaceScope, Capability, WorkspaceAssistantPersonalisationExplanation,
    WorkspaceAssistantPersonalisationProjection, WorkspaceAssistantPersonalisationSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceAssistantPersonalisationService;

pub struct PackageWorkspaceAssistantPersonalisation {
    pub workspace_id: String,
    pub human_ask: String,
    pub scope: AssistantSurfaceScope,
}

impl PackageWorkspaceAssistantPersonalisation {
    pub fn new(workspace_id: impl Into<String>, human_ask: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            human_ask: human_ask.into(),
            scope: AssistantSurfaceScope::personalisation_default(),
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

impl Command for PackageWorkspaceAssistantPersonalisation {
    fn name(&self) -> &'static str {
        "PackageWorkspaceAssistantPersonalisation"
    }
}

impl MutationCommand for PackageWorkspaceAssistantPersonalisation {
    type Output = WorkspaceAssistantPersonalisationProjection;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    /// Dual-channel assistant evidence packaging — same as Batches 11–15.
    /// Does NOT use personalization.write; preference SoT mutations remain
    /// exclusive to personalization commands.
    fn required_capability(&self) -> Capability {
        Capability::work_context_write()
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Self::Output> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceAssistantPersonalisationService::package_personalisation(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.human_ask.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceAssistantPersonalisation {
    pub workspace_id: String,
}

impl GetWorkspaceAssistantPersonalisation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceAssistantPersonalisation {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantPersonalisation"
    }
}

impl QueryCommand for GetWorkspaceAssistantPersonalisation {
    type Output = WorkspaceAssistantPersonalisationProjection;

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
        WorkspaceAssistantPersonalisationService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceAssistantPersonalisationSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceAssistantPersonalisationSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceAssistantPersonalisationSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantPersonalisationSummary"
    }
}

impl QueryCommand for GetWorkspaceAssistantPersonalisationSummary {
    type Output = WorkspaceAssistantPersonalisationSummary;

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
        let projection = WorkspaceAssistantPersonalisationService::load_snapshot(
            &ctx.database,
            self.workspace_id,
        )?;
        Ok(projection.summary(self.history_limit))
    }
}

pub struct ExplainAssistantPersonalisation {
    pub workspace_id: String,
}

impl ExplainAssistantPersonalisation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainAssistantPersonalisation {
    fn name(&self) -> &'static str {
        "ExplainAssistantPersonalisation"
    }
}

impl QueryCommand for ExplainAssistantPersonalisation {
    type Output = WorkspaceAssistantPersonalisationExplanation;

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
        WorkspaceAssistantPersonalisationService::explain(&ctx.database, self.workspace_id)
    }
}
