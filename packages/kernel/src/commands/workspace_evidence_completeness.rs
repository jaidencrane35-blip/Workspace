//! Programme IV Batch 8 — Workspace Evidence Completeness Engine commands.

use workspace_domain::{
    Capability, CompletenessScope, WorkspaceEvidenceCompletenessExplanation,
    WorkspaceEvidenceCompletenessProjection, WorkspaceEvidenceCompletenessSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceEvidenceCompletenessService;

pub struct GenerateWorkspaceEvidenceCompleteness {
    pub workspace_id: String,
    pub scope: CompletenessScope,
}

impl GenerateWorkspaceEvidenceCompleteness {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope: CompletenessScope::all_surfaces(),
        }
    }

    pub fn with_scope(workspace_id: impl Into<String>, scope: CompletenessScope) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope,
        }
    }
}

impl Command for GenerateWorkspaceEvidenceCompleteness {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceEvidenceCompleteness"
    }
}

impl MutationCommand for GenerateWorkspaceEvidenceCompleteness {
    type Output = WorkspaceEvidenceCompletenessProjection;

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
        WorkspaceEvidenceCompletenessService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceEvidenceCompleteness {
    pub workspace_id: String,
}

impl GetWorkspaceEvidenceCompleteness {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceEvidenceCompleteness {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceCompleteness"
    }
}

impl QueryCommand for GetWorkspaceEvidenceCompleteness {
    type Output = WorkspaceEvidenceCompletenessProjection;

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
        WorkspaceEvidenceCompletenessService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceEvidenceCompletenessSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceEvidenceCompletenessSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceEvidenceCompletenessSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceCompletenessSummary"
    }
}

impl QueryCommand for GetWorkspaceEvidenceCompletenessSummary {
    type Output = WorkspaceEvidenceCompletenessSummary;

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
            WorkspaceEvidenceCompletenessService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainEvidenceCompleteness {
    pub workspace_id: String,
}

impl ExplainEvidenceCompleteness {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainEvidenceCompleteness {
    fn name(&self) -> &'static str {
        "ExplainEvidenceCompleteness"
    }
}

impl QueryCommand for ExplainEvidenceCompleteness {
    type Output = WorkspaceEvidenceCompletenessExplanation;

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
        WorkspaceEvidenceCompletenessService::explain(&ctx.database, self.workspace_id)
    }
}
