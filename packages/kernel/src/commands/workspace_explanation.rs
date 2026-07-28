//! Programme III Batch 5 — Workspace Explanation Layer commands.
//!
//! Audit-approved command shape:
//! GenerateWorkspaceExplanation / GetWorkspaceExplanation /
//! GetWorkspaceExplanationSummary / ExplainWorkspaceSituation

use workspace_domain::{
    Capability, ExplanationScope, WorkspaceExplanationSnapshot, WorkspaceExplanationSummary,
    WorkspaceSituationExplanation,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceExplanationService;

pub struct GenerateWorkspaceExplanation {
    pub workspace_id: String,
    pub scope: ExplanationScope,
}

impl GenerateWorkspaceExplanation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope: ExplanationScope::all_surfaces(),
        }
    }

    pub fn with_scope(workspace_id: impl Into<String>, scope: ExplanationScope) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope,
        }
    }
}

impl Command for GenerateWorkspaceExplanation {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceExplanation"
    }
}

impl MutationCommand for GenerateWorkspaceExplanation {
    type Output = WorkspaceExplanationSnapshot;

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
        WorkspaceExplanationService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceExplanation {
    pub workspace_id: String,
}

impl GetWorkspaceExplanation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceExplanation {
    fn name(&self) -> &'static str {
        "GetWorkspaceExplanation"
    }
}

impl QueryCommand for GetWorkspaceExplanation {
    type Output = WorkspaceExplanationSnapshot;

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
        WorkspaceExplanationService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceExplanationSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceExplanationSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceExplanationSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceExplanationSummary"
    }
}

impl QueryCommand for GetWorkspaceExplanationSummary {
    type Output = WorkspaceExplanationSummary;

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
            WorkspaceExplanationService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainWorkspaceSituation {
    pub workspace_id: String,
}

impl ExplainWorkspaceSituation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainWorkspaceSituation {
    fn name(&self) -> &'static str {
        "ExplainWorkspaceSituation"
    }
}

impl QueryCommand for ExplainWorkspaceSituation {
    type Output = WorkspaceSituationExplanation;

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
        WorkspaceExplanationService::explain(&ctx.database, self.workspace_id)
    }
}
