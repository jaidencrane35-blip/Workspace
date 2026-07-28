//! Programme II Batch 2 — Cognitive Planning Engine commands.

use std::sync::{Arc, Mutex};

use workspace_domain::{Capability, PlanningSnapshot, PlanningSummary};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::{
    AssistantWorkflowStore, OrchestratedPlanStore, WorkspacePlanningService,
};

/// Persist a new planning snapshot (supersedes prior active plan). Never executes.
pub struct GeneratePlanningSnapshot {
    pub workspace_id: String,
}

impl GeneratePlanningSnapshot {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GeneratePlanningSnapshot {
    fn name(&self) -> &'static str {
        "GeneratePlanningSnapshot"
    }
}

impl MutationCommand for GeneratePlanningSnapshot {
    type Output = PlanningSnapshot;

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
        // CommandContext does not carry AI plan stores; empty stores still allow
        // Attention/Purpose/RE/DE composition over durable workspace state.
        let orchestrated_plans = Arc::new(Mutex::new(OrchestratedPlanStore::new()));
        let assistant_workflows = Arc::new(Mutex::new(AssistantWorkflowStore::new()));
        WorkspacePlanningService::generate(
            &ctx.database,
            &ctx.actor_context,
            &orchestrated_plans,
            &assistant_workflows,
            self.workspace_id.clone(),
        )
    }
}

/// Load durable planning snapshot (restart continuity — no replay).
pub struct GetPlanningSnapshot {
    pub workspace_id: String,
}

impl GetPlanningSnapshot {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetPlanningSnapshot {
    fn name(&self) -> &'static str {
        "GetPlanningSnapshot"
    }
}

impl QueryCommand for GetPlanningSnapshot {
    type Output = PlanningSnapshot;

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
        WorkspacePlanningService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

/// Compact planning projection summary.
pub struct GetPlanningSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetPlanningSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetPlanningSummary {
    fn name(&self) -> &'static str {
        "GetPlanningSummary"
    }
}

impl QueryCommand for GetPlanningSummary {
    type Output = PlanningSummary;

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
            WorkspacePlanningService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}
