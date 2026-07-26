use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::{WorkspaceObservationCaptureResult, WorkspaceObservationService};
use workspace_domain::{Capability, WorkspaceObservationSnapshot, WorkspaceObservationStatus};

/// Capability gate for desktop observation reads (perception only).
pub struct GateObservationRead;

impl crate::commands::Command for GateObservationRead {
    fn name(&self) -> &'static str {
        "GateObservationRead"
    }
}

impl QueryCommand for GateObservationRead {
    type Output = ();

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<()> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        Ok(())
    }
}

/// Captures the live desktop and persists a durable observation snapshot.
pub struct CaptureWorkspaceObservation;

impl crate::commands::Command for CaptureWorkspaceObservation {
    fn name(&self) -> &'static str {
        "CaptureWorkspaceObservation"
    }
}

impl QueryCommand for CaptureWorkspaceObservation {
    type Output = WorkspaceObservationCaptureResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WorkspaceObservationCaptureResult> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceObservationService::capture(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
        )
    }
}

/// Returns the latest persisted desktop observation snapshot.
pub struct GetLatestWorkspaceObservation;

impl crate::commands::Command for GetLatestWorkspaceObservation {
    fn name(&self) -> &'static str {
        "GetLatestWorkspaceObservation"
    }
}

impl QueryCommand for GetLatestWorkspaceObservation {
    type Output = Option<WorkspaceObservationSnapshot>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Option<WorkspaceObservationSnapshot>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceObservationService::get_latest(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
        )
    }
}

/// Returns lightweight observation pipeline status (metadata only).
pub struct GetWorkspaceObservationStatus;

impl crate::commands::Command for GetWorkspaceObservationStatus {
    fn name(&self) -> &'static str {
        "GetWorkspaceObservationStatus"
    }
}

impl QueryCommand for GetWorkspaceObservationStatus {
    type Output = WorkspaceObservationStatus;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WorkspaceObservationStatus> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceObservationService::get_status(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
        )
    }
}

/// Returns one persisted desktop observation snapshot by pass id.
pub struct GetWorkspaceObservationById {
    pub pass_id: String,
}

impl GetWorkspaceObservationById {
    pub fn new(pass_id: impl Into<String>) -> Self {
        Self {
            pass_id: pass_id.into(),
        }
    }
}

impl crate::commands::Command for GetWorkspaceObservationById {
    fn name(&self) -> &'static str {
        "GetWorkspaceObservationById"
    }
}

impl QueryCommand for GetWorkspaceObservationById {
    type Output = Option<WorkspaceObservationSnapshot>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Option<WorkspaceObservationSnapshot>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceObservationService::get_by_id(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
            self.pass_id,
        )
    }
}
