//! Programme IV Batch 3 — Workspace Evidence Trace Engine commands.

use workspace_domain::{
    Capability, EvidenceTraceRequest, EvidenceTraceScope, WorkspaceEvidenceTraceExplanation,
    WorkspaceEvidenceTraceProjection, WorkspaceEvidenceTraceSummary,
};

use crate::commands::r#trait::{Command, MutationCommand, QueryCommand};
use crate::commands::CommandContext;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceEvidenceTraceService;

pub struct GenerateWorkspaceEvidenceTrace {
    pub workspace_id: String,
    pub request: EvidenceTraceRequest,
}

impl GenerateWorkspaceEvidenceTrace {
    pub fn new(workspace_id: impl Into<String>, target_reference: impl Into<String>) -> Self {
        let request = EvidenceTraceRequest::request(
            target_reference,
            EvidenceTraceScope::all_surfaces(),
            3,
            vec![],
        )
        .unwrap_or_else(|_| EvidenceTraceRequest {
            request_id: "evidence_trace_request:fallback".into(),
            target_reference: "workspace".into(),
            trace_scope: EvidenceTraceScope::all_surfaces(),
            maximum_depth: 3,
            provenance_constraints: vec![],
            authority_effect: EvidenceTraceRequest::AUTHORITY_EFFECT_NONE.into(),
            actionable: false,
            executable: false,
        });
        Self {
            workspace_id: workspace_id.into(),
            request,
        }
    }

    pub fn with_request(workspace_id: impl Into<String>, request: EvidenceTraceRequest) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            request,
        }
    }
}

impl Command for GenerateWorkspaceEvidenceTrace {
    fn name(&self) -> &'static str {
        "GenerateWorkspaceEvidenceTrace"
    }
}

impl MutationCommand for GenerateWorkspaceEvidenceTrace {
    type Output = WorkspaceEvidenceTraceProjection;

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
        WorkspaceEvidenceTraceService::generate(
            &ctx.database,
            &ctx.actor_context,
            self.workspace_id.clone(),
            self.request.clone(),
        )
    }
}

pub struct GetWorkspaceEvidenceTrace {
    pub workspace_id: String,
}

impl GetWorkspaceEvidenceTrace {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for GetWorkspaceEvidenceTrace {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceTrace"
    }
}

impl QueryCommand for GetWorkspaceEvidenceTrace {
    type Output = WorkspaceEvidenceTraceProjection;

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
        WorkspaceEvidenceTraceService::load_snapshot(&ctx.database, self.workspace_id)
    }
}

pub struct GetWorkspaceEvidenceTraceSummary {
    pub workspace_id: String,
    pub history_limit: usize,
}

impl GetWorkspaceEvidenceTraceSummary {
    pub fn new(workspace_id: impl Into<String>, history_limit: usize) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            history_limit,
        }
    }
}

impl Command for GetWorkspaceEvidenceTraceSummary {
    fn name(&self) -> &'static str {
        "GetWorkspaceEvidenceTraceSummary"
    }
}

impl QueryCommand for GetWorkspaceEvidenceTraceSummary {
    type Output = WorkspaceEvidenceTraceSummary;

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
            WorkspaceEvidenceTraceService::load_snapshot(&ctx.database, self.workspace_id)?;
        Ok(snap.summary(self.history_limit))
    }
}

pub struct ExplainEvidenceTrace {
    pub workspace_id: String,
}

impl ExplainEvidenceTrace {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

impl Command for ExplainEvidenceTrace {
    fn name(&self) -> &'static str {
        "ExplainEvidenceTrace"
    }
}

impl QueryCommand for ExplainEvidenceTrace {
    type Output = WorkspaceEvidenceTraceExplanation;

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
        WorkspaceEvidenceTraceService::explain(&ctx.database, self.workspace_id)
    }
}
