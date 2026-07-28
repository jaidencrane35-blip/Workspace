//! Programme IV Batch 1 — Workspace Semantic Query Engine commands.

use workspace_domain::{
    Capability, SemanticQuery, SemanticQueryScope, WorkspaceSemanticQueryExplanation,
    WorkspaceSemanticQueryProjection, WorkspaceSemanticQuerySummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceSemanticQueryService;

pub struct GenerateWorkspaceSemanticQuery {
    pub workspace_id: String,
    pub query: SemanticQuery,
}

impl GenerateWorkspaceSemanticQuery {
    pub fn new(workspace_id: impl Into<String>, query_text: impl Into<String>) -> Self {
        let query = SemanticQuery::request(
            query_text,
            SemanticQueryScope::all_surfaces(),
            vec![],
            vec![],
        )
        .unwrap_or_else(|_| SemanticQuery {
            query: "workspace".into(),
            scope: SemanticQueryScope::all_surfaces(),
            filters: vec![],
            provenance_requirements: vec![],
            authority_effect: SemanticQuery::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            executable: false,
        });
        Self {
            workspace_id: workspace_id.into(),
            query,
        }
    }

    pub fn with_query(workspace_id: impl Into<String>, query: SemanticQuery) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            query,
        }
    }
}

impl Command for GenerateWorkspaceSemanticQuery {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceSemanticQuery"
    }
}

impl MutationCommand for GenerateWorkspaceSemanticQuery {
    type Output = WorkspaceSemanticQueryProjection;

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
        WorkspaceSemanticQueryService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.query.clone(),
        )
    }
}

pub struct GetWorkspaceSemanticQuery {
    pub workspace_id: String,
}

impl GetWorkspaceSemanticQuery {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceSemanticQuery {
    fn name(&self) -> &'static str {
        "GetWorkspaceSemanticQuery"
    }
}

impl QueryCommand for GetWorkspaceSemanticQuery {
    type Output = WorkspaceSemanticQueryProjection;

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
        WorkspaceSemanticQueryService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceSemanticQuerySummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceSemanticQuerySummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceSemanticQuerySummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceSemanticQuerySummary"
    }
}

impl QueryCommand for GetWorkspaceSemanticQuerySummary {
    type Output = WorkspaceSemanticQuerySummary;

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
            WorkspaceSemanticQueryService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainWorkspaceSemanticQuery {
    pub workspace_id: String,
}

impl ExplainWorkspaceSemanticQuery {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainWorkspaceSemanticQuery {
    fn name(&self) -> &'static str {
        "ExplainWorkspaceSemanticQuery"
    }
}

impl QueryCommand for ExplainWorkspaceSemanticQuery {
    type Output = WorkspaceSemanticQueryExplanation;

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
        WorkspaceSemanticQueryService::explain(&ctx.database, self.workspace_id)
    }
}
