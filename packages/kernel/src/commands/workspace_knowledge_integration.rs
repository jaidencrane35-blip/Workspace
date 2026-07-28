//! Programme III Batch 8 — Workspace Knowledge Integration commands.

use workspace_domain::{
    Capability, KnowledgeIntegrationExplanation, KnowledgeIntegrationProjection,
    KnowledgeIntegrationResult, KnowledgeIntegrationSummary, KnowledgeRetrievalFrame,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceKnowledgeIntegrationService;

pub struct GenerateWorkspaceKnowledgeIntegration {
    pub workspace_id: String,
    pub frame: KnowledgeRetrievalFrame,
}

impl GenerateWorkspaceKnowledgeIntegration {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame: KnowledgeRetrievalFrame::all_surfaces(),
        }
    }

    pub fn with_frame(workspace_id: impl Into<String>, frame: KnowledgeRetrievalFrame) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame,
        }
    }
}

impl Command for GenerateWorkspaceKnowledgeIntegration {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceKnowledgeIntegration"
    }
}

impl MutationCommand for GenerateWorkspaceKnowledgeIntegration {
    type Output = KnowledgeIntegrationProjection;

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
        WorkspaceKnowledgeIntegrationService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.frame.clone(),
        )
    }
}

pub struct GetWorkspaceKnowledgeIntegration {
    pub workspace_id: String,
}

impl GetWorkspaceKnowledgeIntegration {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceKnowledgeIntegration {
    fn name(&self) -> &'static str {
        "GetWorkspaceKnowledgeIntegration"
    }
}

impl QueryCommand for GetWorkspaceKnowledgeIntegration {
    type Output = KnowledgeIntegrationProjection;

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
        WorkspaceKnowledgeIntegrationService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceKnowledgeIntegrationSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceKnowledgeIntegrationSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceKnowledgeIntegrationSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceKnowledgeIntegrationSummary"
    }
}

impl QueryCommand for GetWorkspaceKnowledgeIntegrationSummary {
    type Output = KnowledgeIntegrationSummary;

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
            WorkspaceKnowledgeIntegrationService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainKnowledgeIntegration {
    pub workspace_id: String,
}

impl ExplainKnowledgeIntegration {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainKnowledgeIntegration {
    fn name(&self) -> &'static str {
        "ExplainKnowledgeIntegration"
    }
}

impl QueryCommand for ExplainKnowledgeIntegration {
    type Output = KnowledgeIntegrationExplanation;

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
        WorkspaceKnowledgeIntegrationService::explain(&ctx.database, self.workspace_id)
    }
}

pub struct RetrieveWorkspaceKnowledge {
    pub workspace_id: String,
    pub frame: KnowledgeRetrievalFrame,
}

impl RetrieveWorkspaceKnowledge {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame: KnowledgeRetrievalFrame::all_surfaces(),
        }
    }

    pub fn with_frame(workspace_id: impl Into<String>, frame: KnowledgeRetrievalFrame) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame,
        }
    }
}

impl Command for RetrieveWorkspaceKnowledge {
    fn name(&self) -> &'static str {
        "RetrieveWorkspaceKnowledge"
    }
}

impl QueryCommand for RetrieveWorkspaceKnowledge {
    type Output = KnowledgeIntegrationResult;

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
        WorkspaceKnowledgeIntegrationService::retrieve(
            &ctx.database,
            self.workspace_id,
            self.frame,
        )
    }
}
