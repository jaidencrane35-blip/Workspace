//! Programme II Batch 3 — Reasoning Memory commands.

use workspace_domain::{Capability, ReasoningSnapshot, ReasoningSummary};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceReasoningMemoryService;

/// Persist a new reasoning record (supersedes prior current). Never executes.
pub struct GenerateReasoningRecord {
    pub workspace_id: String,
}

impl GenerateReasoningRecord {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateReasoningRecord {
    fn name(&self) -> &'static str {
        "GenerateReasoningRecord"
    }
}

impl MutationCommand for GenerateReasoningRecord {
    type Output = ReasoningSnapshot;

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
        WorkspaceReasoningMemoryService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
        )
    }
}

/// Load durable reasoning snapshot (restart continuity — no fabricate).
pub struct GetReasoningRecord {
    pub workspace_id: String,
}

impl GetReasoningRecord {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetReasoningRecord {
    fn name(&self) -> &'static str {
        "GetReasoningRecord"
    }
}

impl QueryCommand for GetReasoningRecord {
    type Output = ReasoningSnapshot;

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
        WorkspaceReasoningMemoryService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

/// Compact reasoning projection summary.
pub struct GetReasoningSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetReasoningSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetReasoningSummary {
    fn name(&self) -> &'static str {
        "GetReasoningSummary"
    }
}

impl QueryCommand for GetReasoningSummary {
    type Output = ReasoningSummary;

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
            WorkspaceReasoningMemoryService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}
