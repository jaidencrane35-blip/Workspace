//! Programme II Batch 7 — Cognitive Agent Cast commands.

use workspace_domain::{
    Capability, CognitiveAgentCastSnapshot, CognitiveAgentCastSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceCognitiveAgentCastService;

pub struct GenerateCognitiveAgentCast {
    pub workspace_id: String,
}

impl GenerateCognitiveAgentCast {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateCognitiveAgentCast {
    fn name(&self) -> &'static str {
        "GenerateCognitiveAgentCast"
    }
}

impl MutationCommand for GenerateCognitiveAgentCast {
    type Output = CognitiveAgentCastSnapshot;

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
        WorkspaceCognitiveAgentCastService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
        )
    }
}

pub struct GetCognitiveAgentCast {
    pub workspace_id: String,
}

impl GetCognitiveAgentCast {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetCognitiveAgentCast {
    fn name(&self) -> &'static str {
        "GetCognitiveAgentCast"
    }
}

impl QueryCommand for GetCognitiveAgentCast {
    type Output = CognitiveAgentCastSnapshot;

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
        WorkspaceCognitiveAgentCastService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetCognitiveAgentCastSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetCognitiveAgentCastSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetCognitiveAgentCastSummary {
    fn name(&self) -> &'static str {
        "GetCognitiveAgentCastSummary"
    }
}

impl QueryCommand for GetCognitiveAgentCastSummary {
    type Output = CognitiveAgentCastSummary;

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
            WorkspaceCognitiveAgentCastService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}
