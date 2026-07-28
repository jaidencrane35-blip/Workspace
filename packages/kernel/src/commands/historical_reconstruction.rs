//! Programme III Batch 3 — Historical Workspace Reconstruction commands.
//!
//! Audit-approved command shape:
//! GenerateHistoricalWorkspaceView / GetHistoricalWorkspaceView /
//! GetHistoricalWorkspaceSummary / CompareWorkspaceRevisions
//! (+ ExplainHistoricalChange for explanation surface).

use workspace_domain::{
    Capability, HistoricalChangeExplanation, HistoricalReconstructionSnapshot,
    HistoricalReconstructionSummary, RevisionComparison,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceHistoricalReconstructionService;

pub struct GenerateHistoricalWorkspaceView {
    pub workspace_id: String,
}

impl GenerateHistoricalWorkspaceView {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateHistoricalWorkspaceView {
    fn name(&self) -> &'static str {
        "GenerateHistoricalWorkspaceView"
    }
}

impl MutationCommand for GenerateHistoricalWorkspaceView {
    type Output = HistoricalReconstructionSnapshot;

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
        WorkspaceHistoricalReconstructionService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
        )
    }
}

pub struct GetHistoricalWorkspaceView {
    pub workspace_id: String,
}

impl GetHistoricalWorkspaceView {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetHistoricalWorkspaceView {
    fn name(&self) -> &'static str {
        "GetHistoricalWorkspaceView"
    }
}

impl QueryCommand for GetHistoricalWorkspaceView {
    type Output = HistoricalReconstructionSnapshot;

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
        WorkspaceHistoricalReconstructionService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetHistoricalWorkspaceSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetHistoricalWorkspaceSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetHistoricalWorkspaceSummary {
    fn name(&self) -> &'static str {
        "GetHistoricalWorkspaceSummary"
    }
}

impl QueryCommand for GetHistoricalWorkspaceSummary {
    type Output = HistoricalReconstructionSummary;

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
        let snap = WorkspaceHistoricalReconstructionService::load_snapshot(
            &ctx.database,
            self.workspace_id,
        )?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct CompareWorkspaceRevisions {
    pub workspace_id: String,
    pub left_revision: String,
    pub right_revision: String,
}

impl CompareWorkspaceRevisions {
    pub fn new(
        workspace_id: impl Into<String>,
        left_revision: impl Into<String>,
        right_revision: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            left_revision: left_revision.into(),
            right_revision: right_revision.into(),
        }
    }
}

impl Command for CompareWorkspaceRevisions {
    fn name(&self) -> &'static str {
        "CompareWorkspaceRevisions"
    }
}

impl QueryCommand for CompareWorkspaceRevisions {
    type Output = RevisionComparison;

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
        WorkspaceHistoricalReconstructionService::compare_revisions(
            &ctx.database,
            self.workspace_id,
            self.left_revision,
            self.right_revision,
        )
    }
}

pub struct ExplainHistoricalChange {
    pub workspace_id: String,
}

impl ExplainHistoricalChange {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainHistoricalChange {
    fn name(&self) -> &'static str {
        "ExplainHistoricalChange"
    }
}

impl QueryCommand for ExplainHistoricalChange {
    type Output = HistoricalChangeExplanation;

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
        WorkspaceHistoricalReconstructionService::explain(&ctx.database, self.workspace_id)
    }
}
