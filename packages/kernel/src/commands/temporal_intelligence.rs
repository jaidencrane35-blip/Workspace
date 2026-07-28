//! Programme III Batch 4 — Temporal Intelligence commands.
//!
//! Audit-approved command shape:
//! GenerateTemporalAnalysis / GetTemporalAnalysis /
//! GetTemporalSummary / ExplainTemporalChange

use workspace_domain::{
    Capability, TemporalAnalysisWindow, TemporalChangeExplanation, TemporalIntelligenceSnapshot,
    TemporalIntelligenceSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceTemporalIntelligenceService;

pub struct GenerateTemporalAnalysis {
    pub workspace_id: String,
    pub window: TemporalAnalysisWindow,
}

impl GenerateTemporalAnalysis {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            window: TemporalAnalysisWindow::unbounded(),
        }
    }

    pub fn with_window(workspace_id: impl Into<String>, window: TemporalAnalysisWindow) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            window,
        }
    }
}

impl Command for GenerateTemporalAnalysis {
    fn name(&self) -> &'static str {
        "GenerateTemporalAnalysis"
    }
}

impl MutationCommand for GenerateTemporalAnalysis {
    type Output = TemporalIntelligenceSnapshot;

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
        WorkspaceTemporalIntelligenceService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.window.clone(),
        )
    }
}

pub struct GetTemporalAnalysis {
    pub workspace_id: String,
}

impl GetTemporalAnalysis {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetTemporalAnalysis {
    fn name(&self) -> &'static str {
        "GetTemporalAnalysis"
    }
}

impl QueryCommand for GetTemporalAnalysis {
    type Output = TemporalIntelligenceSnapshot;

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
        WorkspaceTemporalIntelligenceService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetTemporalSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetTemporalSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetTemporalSummary {
    fn name(&self) -> &'static str {
        "GetTemporalSummary"
    }
}

impl QueryCommand for GetTemporalSummary {
    type Output = TemporalIntelligenceSummary;

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
        let snap = WorkspaceTemporalIntelligenceService::load_snapshot(
            &ctx.database,
            self.workspace_id,
        )?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainTemporalChange {
    pub workspace_id: String,
}

impl ExplainTemporalChange {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainTemporalChange {
    fn name(&self) -> &'static str {
        "ExplainTemporalChange"
    }
}

impl QueryCommand for ExplainTemporalChange {
    type Output = TemporalChangeExplanation;

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
        WorkspaceTemporalIntelligenceService::explain(&ctx.database, self.workspace_id)
    }
}
