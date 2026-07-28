//! Programme II Batch 8 — Cognitive Autonomy commands.

use workspace_domain::{Capability, CognitiveAutonomySnapshot, CognitiveAutonomySummary};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceCognitiveAutonomyService;

pub struct GenerateCognitiveAutonomy {
    pub workspace_id: String,
}

impl GenerateCognitiveAutonomy {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateCognitiveAutonomy {
    fn name(&self) -> &'static str {
        "GenerateCognitiveAutonomy"
    }
}

impl MutationCommand for GenerateCognitiveAutonomy {
    type Output = CognitiveAutonomySnapshot;

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
        WorkspaceCognitiveAutonomyService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
        )
    }
}

pub struct GetCognitiveAutonomy {
    pub workspace_id: String,
}

impl GetCognitiveAutonomy {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetCognitiveAutonomy {
    fn name(&self) -> &'static str {
        "GetCognitiveAutonomy"
    }
}

impl QueryCommand for GetCognitiveAutonomy {
    type Output = CognitiveAutonomySnapshot;

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
        WorkspaceCognitiveAutonomyService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetCognitiveAutonomySummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetCognitiveAutonomySummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetCognitiveAutonomySummary {
    fn name(&self) -> &'static str {
        "GetCognitiveAutonomySummary"
    }
}

impl QueryCommand for GetCognitiveAutonomySummary {
    type Output = CognitiveAutonomySummary;

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
            WorkspaceCognitiveAutonomyService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}
