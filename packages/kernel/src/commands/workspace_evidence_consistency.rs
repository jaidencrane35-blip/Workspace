//! Programme IV Batch 5 — Workspace Evidence Consistency Engine commands.

use workspace_domain::{
    Capability, ConsistencyScope, WorkspaceEvidenceConsistencyExplanation,
    WorkspaceEvidenceConsistencyProjection, WorkspaceEvidenceConsistencySummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceEvidenceConsistencyService;

pub struct GenerateWorkspaceEvidenceConsistency {
    pub workspace_id: String,
    pub scope: ConsistencyScope,
}

impl GenerateWorkspaceEvidenceConsistency {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope: ConsistencyScope::all_surfaces(),
        }
    }

    pub fn with_scope(workspace_id: impl Into<String>, scope: ConsistencyScope) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope,
        }
    }
}

impl Command for GenerateWorkspaceEvidenceConsistency {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceEvidenceConsistency"
    }
}

impl MutationCommand for GenerateWorkspaceEvidenceConsistency {
    type Output = WorkspaceEvidenceConsistencyProjection;

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
        WorkspaceEvidenceConsistencyService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceEvidenceConsistency {
    pub workspace_id: String,
}

impl GetWorkspaceEvidenceConsistency {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceEvidenceConsistency {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceConsistency"
    }
}

impl QueryCommand for GetWorkspaceEvidenceConsistency {
    type Output = WorkspaceEvidenceConsistencyProjection;

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
        WorkspaceEvidenceConsistencyService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceEvidenceConsistencySummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceEvidenceConsistencySummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceEvidenceConsistencySummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceConsistencySummary"
    }
}

impl QueryCommand for GetWorkspaceEvidenceConsistencySummary {
    type Output = WorkspaceEvidenceConsistencySummary;

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
            WorkspaceEvidenceConsistencyService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainEvidenceConsistency {
    pub workspace_id: String,
}

impl ExplainEvidenceConsistency {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainEvidenceConsistency {
    fn name(&self) -> &'static str {
        "ExplainEvidenceConsistency"
    }
}

impl QueryCommand for ExplainEvidenceConsistency {
    type Output = WorkspaceEvidenceConsistencyExplanation;

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
        WorkspaceEvidenceConsistencyService::explain(&ctx.database, self.workspace_id)
    }
}
