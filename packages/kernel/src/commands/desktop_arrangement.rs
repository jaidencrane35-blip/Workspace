//! Desktop arrangement capture / restore / query commands (DAF-1d).

use serde_json::json;

use crate::commands::context::CommandContext;
use crate::commands::resource::ensure_ready;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::DesktopArrangementService;
use workspace_domain::{
    Capability, DesktopArrangement, DesktopArrangementId, DesktopArrangementRestoreResult,
    DesktopWindowFocusResult, ResourceId, ResourceKind, ResourceRef, WorkspaceId,
};

/// Capture currently observed windows into a named desktop arrangement.
pub struct CaptureDesktopArrangement {
    pub workspace_id: WorkspaceId,
    pub arrangement_id: Option<DesktopArrangementId>,
    pub name: String,
    pub description: String,
    /// When true, runs a fresh observation capture before saving membership.
    pub refresh_observation: bool,
}

impl CaptureDesktopArrangement {
    pub fn new(
        workspace_id: WorkspaceId,
        name: String,
        description: String,
        arrangement_id: Option<DesktopArrangementId>,
        refresh_observation: bool,
    ) -> Self {
        Self {
            workspace_id,
            arrangement_id,
            name,
            description,
            refresh_observation,
        }
    }
}

impl crate::commands::Command for CaptureDesktopArrangement {
    fn name(&self) -> &'static str {
        "CaptureDesktopArrangement"
    }
}

impl MutationCommand for CaptureDesktopArrangement {
    type Output = DesktopArrangement;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        Some(ResourceRef::new(
            ResourceKind::Workspace,
            ResourceId::new(output.workspace_id.as_str()).expect("workspace id is non-empty"),
        ))
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "arrangement_id": output.id.as_str(),
                "entry_count": output.entries.len(),
                "name": output.name,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<DesktopArrangement> {
        ensure_ready(ctx)?;
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        DesktopArrangementService::capture(
            &ctx.database,
            &self.workspace_id,
            self.arrangement_id.clone(),
            self.name.clone(),
            self.description.clone(),
            self.refresh_observation,
        )
    }
}

/// Restore a saved desktop arrangement through WindowController.
pub struct RestoreDesktopArrangement {
    pub arrangement_id: DesktopArrangementId,
    pub focus_first: bool,
    /// When true, uses stub WindowController (no OS mutation) — tests / simulation.
    pub(crate) simulate: bool,
}

impl RestoreDesktopArrangement {
    pub fn new(arrangement_id: DesktopArrangementId, focus_first: bool) -> Self {
        Self {
            arrangement_id,
            focus_first,
            simulate: false,
        }
    }

    pub fn simulated(arrangement_id: DesktopArrangementId, focus_first: bool) -> Self {
        Self {
            arrangement_id,
            focus_first,
            simulate: true,
        }
    }
}

impl crate::commands::Command for RestoreDesktopArrangement {
    fn name(&self) -> &'static str {
        "RestoreDesktopArrangement"
    }
}

impl MutationCommand for RestoreDesktopArrangement {
    type Output = DesktopArrangementRestoreResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_restore()
    }

    fn audit_resource_ref(&self, _output: &Self::Output) -> Option<ResourceRef> {
        None
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "arrangement_id": output.arrangement_id,
                "applied_count": output.applied_count,
                "gap_count": output.gap_count,
                "failed_count": output.failed_count,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<DesktopArrangementRestoreResult> {
        ensure_ready(ctx)?;
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        if self.simulate {
            DesktopArrangementService::restore_simulated(
                &ctx.database,
                &self.arrangement_id,
                self.focus_first,
            )
        } else {
            DesktopArrangementService::restore(
                &ctx.database,
                &self.arrangement_id,
                self.focus_first,
            )
        }
    }
}

/// Load one desktop arrangement.
pub struct GetDesktopArrangement {
    pub arrangement_id: DesktopArrangementId,
}

impl GetDesktopArrangement {
    pub fn new(arrangement_id: DesktopArrangementId) -> Self {
        Self { arrangement_id }
    }
}

impl crate::commands::Command for GetDesktopArrangement {
    fn name(&self) -> &'static str {
        "GetDesktopArrangement"
    }
}

impl QueryCommand for GetDesktopArrangement {
    type Output = Option<DesktopArrangement>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Option<DesktopArrangement>> {
        ensure_ready(ctx)?;
        DesktopArrangementService::get(&ctx.database, &self.arrangement_id)
    }
}

/// List desktop arrangements for a workspace.
pub struct ListDesktopArrangements {
    pub workspace_id: WorkspaceId,
    pub limit: usize,
}

impl ListDesktopArrangements {
    pub fn new(workspace_id: WorkspaceId, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            limit: limit.unwrap_or(50),
        }
    }
}

impl crate::commands::Command for ListDesktopArrangements {
    fn name(&self) -> &'static str {
        "ListDesktopArrangements"
    }
}

impl QueryCommand for ListDesktopArrangements {
    type Output = Vec<DesktopArrangement>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<DesktopArrangement>> {
        ensure_ready(ctx)?;
        DesktopArrangementService::list_by_workspace(
            &ctx.database,
            &self.workspace_id,
            self.limit,
        )
    }
}

/// Focus one observed desktop window through WindowController.
pub struct FocusDesktopWindow {
    pub hwnd: String,
    pub(crate) simulate: bool,
}

impl FocusDesktopWindow {
    pub fn new(hwnd: String) -> Self {
        Self {
            hwnd,
            simulate: false,
        }
    }

    pub fn simulated(hwnd: String) -> Self {
        Self {
            hwnd,
            simulate: true,
        }
    }
}

impl crate::commands::Command for FocusDesktopWindow {
    fn name(&self) -> &'static str {
        "FocusDesktopWindow"
    }
}

impl MutationCommand for FocusDesktopWindow {
    type Output = DesktopWindowFocusResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_restore()
    }

    fn audit_resource_ref(&self, _output: &Self::Output) -> Option<ResourceRef> {
        None
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "hwnd": output.hwnd,
                "simulated": output.simulated,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<DesktopWindowFocusResult> {
        ensure_ready(ctx)?;
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        DesktopArrangementService::focus_window(&self.hwnd, self.simulate)
    }
}
