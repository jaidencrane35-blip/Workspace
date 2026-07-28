//! Programme IV Batch 2 — Workspace Evidence Navigation Engine commands.

use workspace_domain::{
    Capability, EvidenceNavigationSession, TraversalScope, WorkspaceEvidenceNavigationExplanation,
    WorkspaceEvidenceNavigationProjection, WorkspaceEvidenceNavigationSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceEvidenceNavigationService;

pub struct GenerateWorkspaceEvidenceNavigation {
    pub workspace_id: String,
    pub session: EvidenceNavigationSession,
}

impl GenerateWorkspaceEvidenceNavigation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        let session = EvidenceNavigationSession::request(
            "navigate evidence",
            vec![],
            TraversalScope::all_surfaces(),
            3,
        )
        .unwrap_or_else(|_| EvidenceNavigationSession {
            session_id: "evidence_nav_session:fallback".into(),
            navigation_request: "navigate evidence".into(),
            entry_references: vec![],
            traversal_scope: TraversalScope::all_surfaces(),
            traversal_depth: 3,
            authority_effect: EvidenceNavigationSession::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            executable: false,
        });
        Self {
            workspace_id: workspace_id.into(),
            session,
        }
    }

    pub fn with_session(
        workspace_id: impl Into<String>,
        session: EvidenceNavigationSession,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            session,
        }
    }
}

impl Command for GenerateWorkspaceEvidenceNavigation {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceEvidenceNavigation"
    }
}

impl MutationCommand for GenerateWorkspaceEvidenceNavigation {
    type Output = WorkspaceEvidenceNavigationProjection;

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
        WorkspaceEvidenceNavigationService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.session.clone(),
        )
    }
}

pub struct GetWorkspaceEvidenceNavigation {
    pub workspace_id: String,
}

impl GetWorkspaceEvidenceNavigation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceEvidenceNavigation {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceNavigation"
    }
}

impl QueryCommand for GetWorkspaceEvidenceNavigation {
    type Output = WorkspaceEvidenceNavigationProjection;

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
        WorkspaceEvidenceNavigationService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceEvidenceNavigationSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceEvidenceNavigationSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceEvidenceNavigationSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceNavigationSummary"
    }
}

impl QueryCommand for GetWorkspaceEvidenceNavigationSummary {
    type Output = WorkspaceEvidenceNavigationSummary;

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
            WorkspaceEvidenceNavigationService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainEvidenceNavigation {
    pub workspace_id: String,
}

impl ExplainEvidenceNavigation {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainEvidenceNavigation {
    fn name(&self) -> &'static str {
        "ExplainEvidenceNavigation"
    }
}

impl QueryCommand for ExplainEvidenceNavigation {
    type Output = WorkspaceEvidenceNavigationExplanation;

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
        WorkspaceEvidenceNavigationService::explain(&ctx.database, self.workspace_id)
    }
}
