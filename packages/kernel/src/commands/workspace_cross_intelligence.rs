//! Programme III Batch 10 — Cross-Workspace Intelligence commands.

use workspace_domain::{
    Capability, CrossWorkspaceIntelligenceProjection, CrossWorkspaceIntelligenceSummary,
    CrossWorkspacePatternExplanation,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceCrossIntelligenceService;

pub struct GenerateCrossWorkspaceIntelligence;

impl GenerateCrossWorkspaceIntelligence {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GenerateCrossWorkspaceIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

impl Command for GenerateCrossWorkspaceIntelligence {
    fn name(&self) -> &'static str {
        "GenerateCrossWorkspaceIntelligence"
    }
}

impl MutationCommand for GenerateCrossWorkspaceIntelligence {
    type Output = CrossWorkspaceIntelligenceProjection;

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
        WorkspaceCrossIntelligenceService::generate(&ctx.database, &ctx.actor_context)
    }
}

pub struct GetCrossWorkspaceIntelligence;

impl GetCrossWorkspaceIntelligence {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GetCrossWorkspaceIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

impl Command for GetCrossWorkspaceIntelligence {
    fn name(&self) -> &'static str {
        "GetCrossWorkspaceIntelligence"
    }
}

impl QueryCommand for GetCrossWorkspaceIntelligence {
    type Output = CrossWorkspaceIntelligenceProjection;

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
        WorkspaceCrossIntelligenceService::load_snapshot(&ctx.database)
    }
}

pub struct GetCrossWorkspaceSummary {
    pub history_limit: usize,
}

impl GetCrossWorkspaceSummary {
    pub fn new(history_limit: usize) -> Self {
        Self { history_limit }
    }
}

impl Command for GetCrossWorkspaceSummary {
    fn name(&self) -> &'static str {
        "GetCrossWorkspaceSummary"
    }
}

impl QueryCommand for GetCrossWorkspaceSummary {
    type Output = CrossWorkspaceIntelligenceSummary;

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
        let snap = WorkspaceCrossIntelligenceService::load_snapshot(&ctx.database)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainCrossWorkspacePattern;

impl ExplainCrossWorkspacePattern {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ExplainCrossWorkspacePattern {
    fn default() -> Self {
        Self::new()
    }
}

impl Command for ExplainCrossWorkspacePattern {
    fn name(&self) -> &'static str {
        "ExplainCrossWorkspacePattern"
    }
}

impl QueryCommand for ExplainCrossWorkspacePattern {
    type Output = CrossWorkspacePatternExplanation;

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
        WorkspaceCrossIntelligenceService::explain(&ctx.database)
    }
}
