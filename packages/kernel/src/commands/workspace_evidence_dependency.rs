//! Programme IV Batch 6 — Workspace Evidence Dependency Engine commands.

use workspace_domain::{
    Capability, DependencyScope, WorkspaceEvidenceDependencyExplanation,
    WorkspaceEvidenceDependencyProjection, WorkspaceEvidenceDependencySummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceEvidenceDependencyService;

pub struct GenerateWorkspaceEvidenceDependency {
    pub workspace_id: String,
    pub scope: DependencyScope,
}

impl GenerateWorkspaceEvidenceDependency {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope: DependencyScope::all_surfaces(),
        }
    }

    pub fn with_scope(workspace_id: impl Into<String>, scope: DependencyScope) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope,
        }
    }
}

impl Command for GenerateWorkspaceEvidenceDependency {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceEvidenceDependency"
    }
}

impl MutationCommand for GenerateWorkspaceEvidenceDependency {
    type Output = WorkspaceEvidenceDependencyProjection;

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
        WorkspaceEvidenceDependencyService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceEvidenceDependency {
    pub workspace_id: String,
}

impl GetWorkspaceEvidenceDependency {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceEvidenceDependency {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceDependency"
    }
}

impl QueryCommand for GetWorkspaceEvidenceDependency {
    type Output = WorkspaceEvidenceDependencyProjection;

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
        WorkspaceEvidenceDependencyService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceEvidenceDependencySummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceEvidenceDependencySummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceEvidenceDependencySummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceDependencySummary"
    }
}

impl QueryCommand for GetWorkspaceEvidenceDependencySummary {
    type Output = WorkspaceEvidenceDependencySummary;

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
            WorkspaceEvidenceDependencyService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainEvidenceDependency {
    pub workspace_id: String,
}

impl ExplainEvidenceDependency {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainEvidenceDependency {
    fn name(&self) -> &'static str {
        "ExplainEvidenceDependency"
    }
}

impl QueryCommand for ExplainEvidenceDependency {
    type Output = WorkspaceEvidenceDependencyExplanation;

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
        WorkspaceEvidenceDependencyService::explain(&ctx.database, self.workspace_id)
    }
}
