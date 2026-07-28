//! Programme II Batch 4 — Cognitive Graph commands.

use workspace_domain::{Capability, CognitiveGraphSnapshot, CognitiveGraphSummary};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceCognitiveGraphService;

pub struct GenerateCognitiveGraph {
    pub workspace_id: String,
}

impl GenerateCognitiveGraph {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateCognitiveGraph {
    fn name(&self) -> &'static str {
        "GenerateCognitiveGraph"
    }
}

impl MutationCommand for GenerateCognitiveGraph {
    type Output = CognitiveGraphSnapshot;

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
        WorkspaceCognitiveGraphService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
        )
    }
}

pub struct GetCognitiveGraph {
    pub workspace_id: String,
}

impl GetCognitiveGraph {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetCognitiveGraph {
    fn name(&self) -> &'static str {
        "GetCognitiveGraph"
    }
}

impl QueryCommand for GetCognitiveGraph {
    type Output = CognitiveGraphSnapshot;

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
        WorkspaceCognitiveGraphService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetCognitiveGraphSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetCognitiveGraphSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetCognitiveGraphSummary {
    fn name(&self) -> &'static str {
        "GetCognitiveGraphSummary"
    }
}

impl QueryCommand for GetCognitiveGraphSummary {
    type Output = CognitiveGraphSummary;

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
            WorkspaceCognitiveGraphService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}
