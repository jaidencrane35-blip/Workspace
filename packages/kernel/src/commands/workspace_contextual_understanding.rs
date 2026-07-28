//! Programme III Batch 6 — Contextual Workspace Understanding commands.
//!
//! GenerateContextualWorkspaceUnderstanding / GetContextualWorkspaceUnderstanding /
//! GetContextualWorkspaceUnderstandingSummary / ExplainWorkspaceContext

use workspace_domain::{
    Capability, ContextFrame, ContextualUnderstandingProjection, ContextualUnderstandingSummary,
    WorkspaceContextExplanation,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceContextualUnderstandingService;

pub struct GenerateContextualWorkspaceUnderstanding {
    pub workspace_id: String,
    pub frame: ContextFrame,
}

impl GenerateContextualWorkspaceUnderstanding {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame: ContextFrame::all_surfaces(),
        }
    }

    pub fn with_frame(workspace_id: impl Into<String>, frame: ContextFrame) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame,
        }
    }
}

impl Command for GenerateContextualWorkspaceUnderstanding {
    fn name(&self) -> &'static str {
        "GenerateContextualWorkspaceUnderstanding"
    }
}

impl MutationCommand for GenerateContextualWorkspaceUnderstanding {
    type Output = ContextualUnderstandingProjection;

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
        WorkspaceContextualUnderstandingService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.frame.clone(),
        )
    }
}

pub struct GetContextualWorkspaceUnderstanding {
    pub workspace_id: String,
}

impl GetContextualWorkspaceUnderstanding {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetContextualWorkspaceUnderstanding {
    fn name(&self) -> &'static str {
        "GetContextualWorkspaceUnderstanding"
    }
}

impl QueryCommand for GetContextualWorkspaceUnderstanding {
    type Output = ContextualUnderstandingProjection;

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
        WorkspaceContextualUnderstandingService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetContextualWorkspaceUnderstandingSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetContextualWorkspaceUnderstandingSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetContextualWorkspaceUnderstandingSummary {
    fn name(&self) -> &'static str {
        "GetContextualWorkspaceUnderstandingSummary"
    }
}

impl QueryCommand for GetContextualWorkspaceUnderstandingSummary {
    type Output = ContextualUnderstandingSummary;

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
        let snap = WorkspaceContextualUnderstandingService::load_snapshot(
            &ctx.database,
            self.workspace_id,
        )?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainWorkspaceContext {
    pub workspace_id: String,
}

impl ExplainWorkspaceContext {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainWorkspaceContext {
    fn name(&self) -> &'static str {
        "ExplainWorkspaceContext"
    }
}

impl QueryCommand for ExplainWorkspaceContext {
    type Output = WorkspaceContextExplanation;

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
        WorkspaceContextualUnderstandingService::explain(&ctx.database, self.workspace_id)
    }
}
