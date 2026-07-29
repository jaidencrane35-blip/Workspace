//! Programme IV Batch 13 — Workspace Assistant Retrieval Intelligence commands.

use workspace_domain::{
    AssistantSurfaceScope, Capability, WorkspaceAssistantRetrievalExplanation,
    WorkspaceAssistantRetrievalProjection, WorkspaceAssistantRetrievalSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceAssistantRetrievalService;

pub struct PackageWorkspaceAssistantRetrieval {
    pub workspace_id: String,
    pub human_ask: String,
    pub scope: AssistantSurfaceScope,
}

impl PackageWorkspaceAssistantRetrieval {
    pub fn new(workspace_id: impl Into<String>, human_ask: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            human_ask: human_ask.into(),
            scope: AssistantSurfaceScope::retrieval_default(),
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

impl Command for PackageWorkspaceAssistantRetrieval {
    fn name(&self) -> &'static str {
        "PackageWorkspaceAssistantRetrieval"
    }
}

impl MutationCommand for PackageWorkspaceAssistantRetrieval {
    type Output = WorkspaceAssistantRetrievalProjection;

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
        WorkspaceAssistantRetrievalService::package_retrieval(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.human_ask.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceAssistantRetrieval {
    pub workspace_id: String,
}

impl GetWorkspaceAssistantRetrieval {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceAssistantRetrieval {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantRetrieval"
    }
}

impl QueryCommand for GetWorkspaceAssistantRetrieval {
    type Output = WorkspaceAssistantRetrievalProjection;

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
        WorkspaceAssistantRetrievalService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceAssistantRetrievalSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceAssistantRetrievalSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceAssistantRetrievalSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceAssistantRetrievalSummary"
    }
}

impl QueryCommand for GetWorkspaceAssistantRetrievalSummary {
    type Output = WorkspaceAssistantRetrievalSummary;

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
            WorkspaceAssistantRetrievalService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(projection.summary(self.history_limit))
    }
}

pub struct ExplainAssistantRetrieval {
    pub workspace_id: String,
}

impl ExplainAssistantRetrieval {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainAssistantRetrieval {
    fn name(&self) -> &'static str {
        "ExplainAssistantRetrieval"
    }
}

impl QueryCommand for ExplainAssistantRetrieval {
    type Output = WorkspaceAssistantRetrievalExplanation;

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
        WorkspaceAssistantRetrievalService::explain(&ctx.database, self.workspace_id)
    }
}
