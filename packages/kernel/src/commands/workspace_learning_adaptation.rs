//! Programme II Batch 6 — Learning & Adaptation commands.

use workspace_domain::{Capability, LearningSnapshot, LearningSummary};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceLearningAdaptationService;

pub struct GenerateLearningSnapshot {
    pub workspace_id: String,
}

impl GenerateLearningSnapshot {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateLearningSnapshot {
    fn name(&self) -> &'static str {
        "GenerateLearningSnapshot"
    }
}

impl MutationCommand for GenerateLearningSnapshot {
    type Output = LearningSnapshot;

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
        WorkspaceLearningAdaptationService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
        )
    }
}

pub struct GetLearningSnapshot {
    pub workspace_id: String,
}

impl GetLearningSnapshot {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetLearningSnapshot {
    fn name(&self) -> &'static str {
        "GetLearningSnapshot"
    }
}

impl QueryCommand for GetLearningSnapshot {
    type Output = LearningSnapshot;

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
        WorkspaceLearningAdaptationService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetLearningSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetLearningSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetLearningSummary {
    fn name(&self) -> &'static str {
        "GetLearningSummary"
    }
}

impl QueryCommand for GetLearningSummary {
    type Output = LearningSummary;

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
            WorkspaceLearningAdaptationService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}
