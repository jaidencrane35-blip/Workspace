//! Programme III Batch 2 — Policy & Governance Engine commands.

use workspace_domain::{
    Capability, GovernanceExplanation, PolicyGovernanceSnapshot, PolicyGovernanceSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::PolicyGovernanceService;

pub struct GenerateGovernanceEvaluation {
    pub workspace_id: String,
}

impl GenerateGovernanceEvaluation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GenerateGovernanceEvaluation {
    fn name(&self) -> &'static str {
        "GenerateGovernanceEvaluation"
    }
}

impl MutationCommand for GenerateGovernanceEvaluation {
    type Output = PolicyGovernanceSnapshot;

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
        PolicyGovernanceService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
        )
    }
}

pub struct GetGovernanceEvaluation {
    pub workspace_id: String,
}

impl GetGovernanceEvaluation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetGovernanceEvaluation {
    fn name(&self) -> &'static str {
        "GetGovernanceEvaluation"
    }
}

impl QueryCommand for GetGovernanceEvaluation {
    type Output = PolicyGovernanceSnapshot;

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
        PolicyGovernanceService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetGovernanceSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetGovernanceSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetGovernanceSummary {
    fn name(&self) -> &'static str {
        "GetGovernanceSummary"
    }
}

impl QueryCommand for GetGovernanceSummary {
    type Output = PolicyGovernanceSummary;

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
        let snap = PolicyGovernanceService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainGovernanceDecision {
    pub workspace_id: String,
}

impl ExplainGovernanceDecision {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainGovernanceDecision {
    fn name(&self) -> &'static str {
        "ExplainGovernanceDecision"
    }
}

impl QueryCommand for ExplainGovernanceDecision {
    type Output = GovernanceExplanation;

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
        PolicyGovernanceService::explain(&ctx.database, self.workspace_id)
    }
}
