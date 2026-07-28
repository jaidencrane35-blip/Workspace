//! Programme III Batch 11 — Workspace Decision Support commands.

use workspace_domain::{
    Capability, DecisionSupportFrame, WorkspaceDecisionSupportExplanation,
    WorkspaceDecisionSupportProjection, WorkspaceDecisionSupportSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceDecisionSupportService;

pub struct GenerateWorkspaceDecisionSupport {
    pub workspace_id: String,
    pub frame: DecisionSupportFrame,
}

impl GenerateWorkspaceDecisionSupport {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame: DecisionSupportFrame::all_surfaces(),
        }
    }
}

impl Command for GenerateWorkspaceDecisionSupport {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceDecisionSupport"
    }
}

impl MutationCommand for GenerateWorkspaceDecisionSupport {
    type Output = WorkspaceDecisionSupportProjection;

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
        WorkspaceDecisionSupportService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.frame.clone(),
        )
    }
}

pub struct GetWorkspaceDecisionSupport {
    pub workspace_id: String,
}

impl GetWorkspaceDecisionSupport {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceDecisionSupport {
    fn name(&self) -> &'static str {
        "GetWorkspaceDecisionSupport"
    }
}

impl QueryCommand for GetWorkspaceDecisionSupport {
    type Output = WorkspaceDecisionSupportProjection;

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
        WorkspaceDecisionSupportService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceDecisionSupportSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceDecisionSupportSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceDecisionSupportSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceDecisionSupportSummary"
    }
}

impl QueryCommand for GetWorkspaceDecisionSupportSummary {
    type Output = WorkspaceDecisionSupportSummary;

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
            WorkspaceDecisionSupportService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainWorkspaceDecisionSupport {
    pub workspace_id: String,
}

impl ExplainWorkspaceDecisionSupport {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainWorkspaceDecisionSupport {
    fn name(&self) -> &'static str {
        "ExplainWorkspaceDecisionSupport"
    }
}

impl QueryCommand for ExplainWorkspaceDecisionSupport {
    type Output = WorkspaceDecisionSupportExplanation;

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
        WorkspaceDecisionSupportService::explain(&ctx.database, self.workspace_id)
    }
}
