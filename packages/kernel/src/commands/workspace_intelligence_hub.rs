//! Programme III Batch 12 — Workspace Intelligence Hub commands.

use workspace_domain::{
    Capability, IntelligenceHubFrame, WorkspaceIntelligenceHubExplanation,
    WorkspaceIntelligenceHubProjection, WorkspaceIntelligenceHubSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceIntelligenceHubService;

pub struct GenerateWorkspaceIntelligenceHub {
    pub workspace_id: String,
    pub frame: IntelligenceHubFrame,
}

impl GenerateWorkspaceIntelligenceHub {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame: IntelligenceHubFrame::all_surfaces(),
        }
    }
}

impl Command for GenerateWorkspaceIntelligenceHub {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceIntelligenceHub"
    }
}

impl MutationCommand for GenerateWorkspaceIntelligenceHub {
    type Output = WorkspaceIntelligenceHubProjection;

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
        WorkspaceIntelligenceHubService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.frame.clone(),
        )
    }
}

pub struct GetWorkspaceIntelligenceHub {
    pub workspace_id: String,
}

impl GetWorkspaceIntelligenceHub {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceIntelligenceHub {
    fn name(&self) -> &'static str {
        "GetWorkspaceIntelligenceHub"
    }
}

impl QueryCommand for GetWorkspaceIntelligenceHub {
    type Output = WorkspaceIntelligenceHubProjection;

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
        WorkspaceIntelligenceHubService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceIntelligenceHubSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceIntelligenceHubSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceIntelligenceHubSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceIntelligenceHubSummary"
    }
}

impl QueryCommand for GetWorkspaceIntelligenceHubSummary {
    type Output = WorkspaceIntelligenceHubSummary;

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
            WorkspaceIntelligenceHubService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainWorkspaceIntelligence {
    pub workspace_id: String,
}

impl ExplainWorkspaceIntelligence {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainWorkspaceIntelligence {
    fn name(&self) -> &'static str {
        "ExplainWorkspaceIntelligence"
    }
}

impl QueryCommand for ExplainWorkspaceIntelligence {
    type Output = WorkspaceIntelligenceHubExplanation;

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
        WorkspaceIntelligenceHubService::explain(&ctx.database, self.workspace_id)
    }
}
