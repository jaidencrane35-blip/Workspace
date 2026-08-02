use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::SavedContextService;
use workspace_domain::{
    Capability, ResourceKind, SaveContextRequest, SavedContext, SavedContextCaptureScope,
};

/// Declares what saving a context would capture, and what it would not.
///
/// This reads no desktop state and produces no side effects — it exists so the
/// preview the user reviews comes from the same build that performs the capture,
/// rather than from copy held separately in the interface.
pub struct GetSavedContextCaptureScope;

impl crate::commands::Command for GetSavedContextCaptureScope {
    fn name(&self) -> &'static str {
        "GetSavedContextCaptureScope"
    }
}

impl QueryCommand for GetSavedContextCaptureScope {
    type Output = SavedContextCaptureScope;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::workspace_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<SavedContextCaptureScope> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        Ok(SavedContextCaptureScope::current())
    }
}

/// Captures the desktop once and stores it as a named bounded context.
///
/// Declares `workspace.write` because its lasting effect is a new workspace-owned
/// record. It also reads the desktop, so `desktop.read` is required as well; both
/// are checked before anything is observed.
pub struct SaveWorkspaceContext {
    pub request: SaveContextRequest,
}

impl SaveWorkspaceContext {
    pub fn new(request: SaveContextRequest) -> Self {
        Self { request }
    }
}

impl crate::commands::Command for SaveWorkspaceContext {
    fn name(&self) -> &'static str {
        "SaveWorkspaceContext"
    }
}

impl MutationCommand for SaveWorkspaceContext {
    type Output = SavedContext;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn permission_target_id(&self) -> Option<String> {
        Some(self.request.workspace_id.to_string())
    }

    fn required_capability(&self) -> Capability {
        Capability::workspace_write()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            serde_json::json!({
                "saved_context_id": output.id.as_str(),
                "approved_scope": output.approved_scope,
                "handoff_note_present": !output.handoff_note.trim().is_empty(),
                "observation_pass_id": output.observation_pass_id,
                "window_count": output.window_count(),
                "monitor_count": output.monitor_count(),
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<SavedContext> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        // The pipeline authorises the record this command writes. Reading the
        // desktop is a separate effect, so saving must not become a way to reach
        // desktop state without the capability that governs it.
        if !ctx.capability_set.contains(&Capability::desktop_read()) {
            return Err(KernelError::PermissionDenied(
                "saving a context reads the desktop, which requires desktop.read".into(),
            ));
        }

        SavedContextService::save(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
            &self.request,
        )
    }
}
