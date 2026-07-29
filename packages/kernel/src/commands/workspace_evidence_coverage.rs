//! Programme IV Batch 4 — Workspace Evidence Coverage Engine commands.

use workspace_domain::{
    Capability, CoverageScope, WorkspaceEvidenceCoverageExplanation,
    WorkspaceEvidenceCoverageProjection, WorkspaceEvidenceCoverageSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceEvidenceCoverageService;

pub struct GenerateWorkspaceEvidenceCoverage {
    pub workspace_id: String,
    pub scope: CoverageScope,
}

impl GenerateWorkspaceEvidenceCoverage {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope: CoverageScope::all_surfaces(),
        }
    }

    pub fn with_scope(workspace_id: impl Into<String>, scope: CoverageScope) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope,
        }
    }
}

impl Command for GenerateWorkspaceEvidenceCoverage {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceEvidenceCoverage"
    }
}

impl MutationCommand for GenerateWorkspaceEvidenceCoverage {
    type Output = WorkspaceEvidenceCoverageProjection;

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
        WorkspaceEvidenceCoverageService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceEvidenceCoverage {
    pub workspace_id: String,
}

impl GetWorkspaceEvidenceCoverage {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceEvidenceCoverage {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceCoverage"
    }
}

impl QueryCommand for GetWorkspaceEvidenceCoverage {
    type Output = WorkspaceEvidenceCoverageProjection;

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
        WorkspaceEvidenceCoverageService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceEvidenceCoverageSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceEvidenceCoverageSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceEvidenceCoverageSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceCoverageSummary"
    }
}

impl QueryCommand for GetWorkspaceEvidenceCoverageSummary {
    type Output = WorkspaceEvidenceCoverageSummary;

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
            WorkspaceEvidenceCoverageService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainEvidenceCoverage {
    pub workspace_id: String,
}

impl ExplainEvidenceCoverage {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainEvidenceCoverage {
    fn name(&self) -> &'static str {
        "ExplainEvidenceCoverage"
    }
}

impl QueryCommand for ExplainEvidenceCoverage {
    type Output = WorkspaceEvidenceCoverageExplanation;

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
        WorkspaceEvidenceCoverageService::explain(&ctx.database, self.workspace_id)
    }
}
