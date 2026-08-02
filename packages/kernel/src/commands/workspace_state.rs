//! GetWorkspaceState / GetWorkspaceRuntimeState — read-only runtime projection.

use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::{WorkspaceRuntimeStateService, WorkspaceStateEngine};
use workspace_domain::{Capability, WorkspaceRuntimeState, WorkspaceState};

/// Returns the canonical WorkspaceState projection (observation + delta).
pub struct GetWorkspaceState;

impl crate::commands::Command for GetWorkspaceState {
    fn name(&self) -> &'static str {
        "GetWorkspaceState"
    }
}

impl QueryCommand for GetWorkspaceState {
    type Output = WorkspaceState;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WorkspaceState> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceStateEngine::get_current(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
        )
    }
}

/// Returns the full live [`WorkspaceRuntimeState`] bundle (desktop + cache + execution).
pub struct GetWorkspaceRuntimeState;

impl crate::commands::Command for GetWorkspaceRuntimeState {
    fn name(&self) -> &'static str {
        "GetWorkspaceRuntimeState"
    }
}

impl QueryCommand for GetWorkspaceRuntimeState {
    type Output = WorkspaceRuntimeState;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::desktop_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WorkspaceRuntimeState> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        WorkspaceRuntimeStateService::current(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::pipeline::CommandPipeline;
    use crate::commands::CommandHandler;
    use crate::WorkspaceKernel;
    use workspace_database::ObservationPassRepository;
    use workspace_domain::{
        ActorContext, IntentContext, ObservedMonitor, ObservedWindow, WorkspaceObservationPass,
        WorkspaceObservationSnapshot,
    };

    fn persist_one(kernel: &WorkspaceKernel) {
        let pass_id = "pass-cmd";
        let snap = WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.into(),
                captured_at: "2026-07-26T11:00:00Z".into(),
                schema_version: 1,
                source: "test_inject".into(),
                foreground_hwnd: Some("0x9".into()),
                window_count: 1,
                monitor_count: 1,
                duration_ms: Some(1),
                metadata_json: "{}".into(),
                authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
            },
            monitors: vec![ObservedMonitor {
                id: format!("{pass_id}-mon"),
                pass_id: pass_id.into(),
                monitor_index: 0,
                name: "Primary".into(),
                x: 0,
                y: 0,
                width: 800,
                height: 600,
                work_x: 0,
                work_y: 0,
                work_w: 800,
                work_h: 560,
                is_primary: true,
                dpi_scale: None,
                authority_effect: ObservedMonitor::AUTHORITY_EFFECT_NONE.into(),
            }],
            windows: vec![ObservedWindow {
                id: format!("{pass_id}-win"),
                pass_id: pass_id.into(),
                hwnd: "0x9".into(),
                stable_window_id: Some("stable-cmd".into()),
                title: "Command Window".into(),
                process_id: 7,
                process_name: Some("cmd.exe".into()),
                x: 1,
                y: 2,
                width: 100,
                height: 100,
                monitor_id: Some(format!("{pass_id}-mon")),
                visible: true,
                minimized: false,
                focused: true,
                z_order: Some(0),
                authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
            }],
            identities: Vec::new(),
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        };
        let db = kernel.shared_database();
        ObservationPassRepository::new(&db.lock().unwrap())
            .insert_snapshot(&snap)
            .unwrap();
    }

    #[test]
    fn get_workspace_state_command_empty_and_populated() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let empty = CommandHandler::get_workspace_state(&kernel, local.clone(), intent.clone())
            .unwrap();
        assert!(empty.metadata.observation_pass_id.is_none());

        persist_one(&kernel);
        let state = CommandPipeline::new(kernel.command_context(local, intent))
            .execute_query(GetWorkspaceState)
            .unwrap();
        assert_eq!(state.metadata.observation_pass_id.as_deref(), Some("pass-cmd"));
        assert_eq!(
            state.focused_window.as_ref().and_then(|w| w.stable_window_id.as_deref()),
            Some("stable-cmd")
        );
        assert_eq!(state.authority_effect, "none");
    }
}
