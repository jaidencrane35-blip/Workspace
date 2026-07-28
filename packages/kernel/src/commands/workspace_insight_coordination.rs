//! Programme III Batch 9 — Workspace Insight Coordination commands.

use workspace_domain::{
    Capability, InsightCoordinationExplanation, InsightCoordinationFrame,
    InsightCoordinationProjection, InsightCoordinationSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceInsightCoordinationService;

pub struct GenerateInsightCoordination {
    pub workspace_id: String,
    pub frame: InsightCoordinationFrame,
}

impl GenerateInsightCoordination {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame: InsightCoordinationFrame::all_surfaces(),
        }
    }
}

impl Command for GenerateInsightCoordination {
    fn name(&self) -> &'static str {
        "GenerateInsightCoordination"
    }
}

impl MutationCommand for GenerateInsightCoordination {
    type Output = InsightCoordinationProjection;

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
        WorkspaceInsightCoordinationService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.frame.clone(),
        )
    }
}

pub struct GetInsightCoordination {
    pub workspace_id: String,
}

impl GetInsightCoordination {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetInsightCoordination {
    fn name(&self) -> &'static str {
        "GetInsightCoordination"
    }
}

impl QueryCommand for GetInsightCoordination {
    type Output = InsightCoordinationProjection;

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
        WorkspaceInsightCoordinationService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetInsightCoordinationSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetInsightCoordinationSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetInsightCoordinationSummary {
    fn name(&self) -> &'static str {
        "GetInsightCoordinationSummary"
    }
}

impl QueryCommand for GetInsightCoordinationSummary {
    type Output = InsightCoordinationSummary;

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
            WorkspaceInsightCoordinationService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainInsightCoordination {
    pub workspace_id: String,
}

impl ExplainInsightCoordination {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainInsightCoordination {
    fn name(&self) -> &'static str {
        "ExplainInsightCoordination"
    }
}

impl QueryCommand for ExplainInsightCoordination {
    type Output = InsightCoordinationExplanation;

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
        WorkspaceInsightCoordinationService::explain(&ctx.database, self.workspace_id)
    }
}
