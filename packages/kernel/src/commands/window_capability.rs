//! Window Provider commands — operations through Capability Runtime (P12).

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::capability_runtime::{
    runtime, ApplicationWindowItem, CapabilityDomainId, CapabilityOperation, MonitorItem,
    ProviderInvokeRequest, ProviderInvokeResponse, ProviderResultSummary,
};
use crate::commands::context::CommandContext;
use crate::commands::r#trait::MutationCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use workspace_domain::Capability;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOperationResult {
    pub operation: String,
    pub ok: bool,
    pub status: Option<String>,
    pub target: Option<String>,
    pub message: Option<String>,
    pub preview: Option<String>,
    pub text: Option<String>,
    pub items: Option<Vec<ApplicationWindowItem>>,
    pub monitors: Option<Vec<MonitorItem>>,
}

impl From<ProviderInvokeResponse> for WindowOperationResult {
    fn from(response: ProviderInvokeResponse) -> Self {
        Self {
            operation: response.operation.as_str().into(),
            ok: response.ok,
            status: response.status,
            target: response.target,
            message: response.message,
            preview: response.preview,
            text: response.text,
            items: response.items,
            monitors: response.monitors,
        }
    }
}

pub struct ExecuteWindowOperation {
    pub operation: CapabilityOperation,
    pub query: Option<String>,
    pub path: Option<String>,
    pub hwnd: Option<String>,
    pub pid: Option<u32>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub monitor_index: Option<i32>,
    pub snap: Option<String>,
}

impl ExecuteWindowOperation {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation: CapabilityOperation,
        query: Option<String>,
        path: Option<String>,
        hwnd: Option<String>,
        pid: Option<u32>,
        x: Option<i32>,
        y: Option<i32>,
        width: Option<i32>,
        height: Option<i32>,
        monitor_index: Option<i32>,
        snap: Option<String>,
    ) -> Self {
        Self {
            operation,
            query,
            path,
            hwnd,
            pid,
            x,
            y,
            width,
            height,
            monitor_index,
            snap,
        }
    }
}

impl crate::commands::Command for ExecuteWindowOperation {
    fn name(&self) -> &'static str {
        "ExecuteWindowOperation"
    }
}

impl MutationCommand for ExecuteWindowOperation {
    type Output = WindowOperationResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        match self.operation {
            CapabilityOperation::Enumerate
            | CapabilityOperation::Find
            | CapabilityOperation::Active
            | CapabilityOperation::Bounds
            | CapabilityOperation::Monitors
            | CapabilityOperation::EnumerateControls
            | CapabilityOperation::FindControl => Capability::window_read(),
            CapabilityOperation::Focus => Capability::window_focus(),
            CapabilityOperation::Minimize
            | CapabilityOperation::Restore
            | CapabilityOperation::Maximize => Capability::window_state(),
            CapabilityOperation::Move
            | CapabilityOperation::Resize
            | CapabilityOperation::Center
            | CapabilityOperation::Snap => Capability::window_place(),
            other => Capability::new(
                format!("window.{}", other.as_str()),
                workspace_domain::CapabilityScope::System,
            )
            .unwrap_or_else(|_| Capability::window_read()),
        }
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        let summary = ProviderResultSummary {
            domain: CapabilityDomainId::window(),
            operation: self.operation,
            ok: output.ok,
            format: None,
            bytes: output.items.as_ref().map(|items| items.len()),
            preview: output.preview.clone(),
            message: output.message.clone(),
            status: output.status.clone(),
            target: output.target.clone(),
            item_count: output.items.as_ref().map(|items| items.len()),
        };
        Some(json!({ "window": summary }).to_string())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<WindowOperationResult> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        match self.operation {
            CapabilityOperation::Enumerate
            | CapabilityOperation::Find
            | CapabilityOperation::Active
            | CapabilityOperation::Bounds
            | CapabilityOperation::Monitors
            | CapabilityOperation::EnumerateControls
            | CapabilityOperation::FindControl
            | CapabilityOperation::Focus
            | CapabilityOperation::Minimize
            | CapabilityOperation::Restore
            | CapabilityOperation::Maximize
            | CapabilityOperation::Move
            | CapabilityOperation::Resize
            | CapabilityOperation::Center
            | CapabilityOperation::Snap => {}
            other => {
                return Err(KernelError::CapabilityRuntime {
                    message: format!(
                        "ExecuteWindowOperation cannot run '{}'",
                        other.as_str()
                    ),
                });
            }
        }

        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::window(),
            operation: self.operation,
            text: None,
            query: self.query.clone(),
            path: self.path.clone(),
            hwnd: self.hwnd.clone(),
            pid: self.pid,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            monitor_index: self.monitor_index,
            snap: self.snap.clone(),
            ..Default::default()
        })?;
        Ok(WindowOperationResult::from(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

    fn ready_ctx<'a>(
        init: &'a crate::commands::initialize::InitializeWorkspaceResult,
        bus: &'a EventBus,
    ) -> CommandContext<'a> {
        CommandContext {
            actor_context: ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::local_user_standard(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        }
    }

    #[test]
    fn pipeline_snaps_fixture_window() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let result = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(ExecuteWindowOperation::new(
                CapabilityOperation::Snap,
                Some("Fixture Focus".into()),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some("left".into()),
            ))
            .unwrap();
        assert!(result.ok);
        assert_eq!(result.status.as_deref(), Some("snapped"));
    }
}
