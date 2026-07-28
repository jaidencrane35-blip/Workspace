//! Programme III Batch 7 — Workspace Knowledge Synthesis commands.
//!
//! GenerateWorkspaceKnowledgeSynthesis / GetWorkspaceKnowledgeSynthesis /
//! GetWorkspaceKnowledgeSummary / ExplainKnowledgeSynthesis

use workspace_domain::{
    Capability, KnowledgeSynthesisExplanation, KnowledgeSynthesisFrame,
    KnowledgeSynthesisProjection, KnowledgeSynthesisSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceKnowledgeSynthesisService;

pub struct GenerateWorkspaceKnowledgeSynthesis {
    pub workspace_id: String,
    pub frame: KnowledgeSynthesisFrame,
}

impl GenerateWorkspaceKnowledgeSynthesis {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame: KnowledgeSynthesisFrame::all_surfaces(),
        }
    }

    pub fn with_frame(workspace_id: impl Into<String>, frame: KnowledgeSynthesisFrame) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            frame,
        }
    }
}

impl Command for GenerateWorkspaceKnowledgeSynthesis {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceKnowledgeSynthesis"
    }
}

impl MutationCommand for GenerateWorkspaceKnowledgeSynthesis {
    type Output = KnowledgeSynthesisProjection;

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
        WorkspaceKnowledgeSynthesisService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.frame.clone(),
        )
    }
}

pub struct GetWorkspaceKnowledgeSynthesis {
    pub workspace_id: String,
}

impl GetWorkspaceKnowledgeSynthesis {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceKnowledgeSynthesis {
    fn name(&self) -> &'static str {
        "GetWorkspaceKnowledgeSynthesis"
    }
}

impl QueryCommand for GetWorkspaceKnowledgeSynthesis {
    type Output = KnowledgeSynthesisProjection;

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
        WorkspaceKnowledgeSynthesisService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceKnowledgeSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceKnowledgeSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceKnowledgeSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceKnowledgeSummary"
    }
}

impl QueryCommand for GetWorkspaceKnowledgeSummary {
    type Output = KnowledgeSynthesisSummary;

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
            WorkspaceKnowledgeSynthesisService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainKnowledgeSynthesis {
    pub workspace_id: String,
}

impl ExplainKnowledgeSynthesis {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainKnowledgeSynthesis {
    fn name(&self) -> &'static str {
        "ExplainKnowledgeSynthesis"
    }
}

impl QueryCommand for ExplainKnowledgeSynthesis {
    type Output = KnowledgeSynthesisExplanation;

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
        WorkspaceKnowledgeSynthesisService::explain(&ctx.database, self.workspace_id)
    }
}
