//! Programme IV Batch 7 — Workspace Evidence Freshness Engine commands.

use workspace_domain::{
    Capability, FreshnessScope, WorkspaceEvidenceFreshnessExplanation,
    WorkspaceEvidenceFreshnessProjection, WorkspaceEvidenceFreshnessSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceEvidenceFreshnessService;

pub struct GenerateWorkspaceEvidenceFreshness {
    pub workspace_id: String,
    pub scope: FreshnessScope,
}

impl GenerateWorkspaceEvidenceFreshness {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope: FreshnessScope::all_surfaces(),
        }
    }

    pub fn with_scope(workspace_id: impl Into<String>, scope: FreshnessScope) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope,
        }
    }
}

impl Command for GenerateWorkspaceEvidenceFreshness {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceEvidenceFreshness"
    }
}

impl MutationCommand for GenerateWorkspaceEvidenceFreshness {
    type Output = WorkspaceEvidenceFreshnessProjection;

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
        WorkspaceEvidenceFreshnessService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceEvidenceFreshness {
    pub workspace_id: String,
}

impl GetWorkspaceEvidenceFreshness {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceEvidenceFreshness {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceFreshness"
    }
}

impl QueryCommand for GetWorkspaceEvidenceFreshness {
    type Output = WorkspaceEvidenceFreshnessProjection;

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
        WorkspaceEvidenceFreshnessService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceEvidenceFreshnessSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceEvidenceFreshnessSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceEvidenceFreshnessSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceFreshnessSummary"
    }
}

impl QueryCommand for GetWorkspaceEvidenceFreshnessSummary {
    type Output = WorkspaceEvidenceFreshnessSummary;

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
            WorkspaceEvidenceFreshnessService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainEvidenceFreshness {
    pub workspace_id: String,
}

impl ExplainEvidenceFreshness {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainEvidenceFreshness {
    fn name(&self) -> &'static str {
        "ExplainEvidenceFreshness"
    }
}

impl QueryCommand for ExplainEvidenceFreshness {
    type Output = WorkspaceEvidenceFreshnessExplanation;

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
        WorkspaceEvidenceFreshnessService::explain(&ctx.database, self.workspace_id)
    }
}
