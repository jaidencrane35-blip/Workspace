//! Programme IV Batch 9 — Workspace Evidence Reliability Engine commands.

use workspace_domain::{
    Capability, ReliabilityScope, WorkspaceEvidenceReliabilityExplanation,
    WorkspaceEvidenceReliabilityProjection, WorkspaceEvidenceReliabilitySummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceEvidenceReliabilityService;

pub struct GenerateWorkspaceEvidenceReliability {
    pub workspace_id: String,
    pub scope: ReliabilityScope,
}

impl GenerateWorkspaceEvidenceReliability {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope: ReliabilityScope::all_surfaces(),
        }
    }

    pub fn with_scope(workspace_id: impl Into<String>, scope: ReliabilityScope) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            scope,
        }
    }
}

impl Command for GenerateWorkspaceEvidenceReliability {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceEvidenceReliability"
    }
}

impl MutationCommand for GenerateWorkspaceEvidenceReliability {
    type Output = WorkspaceEvidenceReliabilityProjection;

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
        WorkspaceEvidenceReliabilityService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.scope.clone(),
        )
    }
}

pub struct GetWorkspaceEvidenceReliability {
    pub workspace_id: String,
}

impl GetWorkspaceEvidenceReliability {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceEvidenceReliability {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceReliability"
    }
}

impl QueryCommand for GetWorkspaceEvidenceReliability {
    type Output = WorkspaceEvidenceReliabilityProjection;

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
        WorkspaceEvidenceReliabilityService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceEvidenceReliabilitySummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceEvidenceReliabilitySummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceEvidenceReliabilitySummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceReliabilitySummary"
    }
}

impl QueryCommand for GetWorkspaceEvidenceReliabilitySummary {
    type Output = WorkspaceEvidenceReliabilitySummary;

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
            WorkspaceEvidenceReliabilityService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainEvidenceReliability {
    pub workspace_id: String,
}

impl ExplainEvidenceReliability {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainEvidenceReliability {
    fn name(&self) -> &'static str {
        "ExplainEvidenceReliability"
    }
}

impl QueryCommand for ExplainEvidenceReliability {
    type Output = WorkspaceEvidenceReliabilityExplanation;

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
        WorkspaceEvidenceReliabilityService::explain(&ctx.database, self.workspace_id)
    }
}
