//! Application Provider commands — operations through Capability Runtime (P11).

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::capability_runtime::{
    runtime, CapabilityDomainId, CapabilityOperation, ProviderInvokeRequest, ProviderInvokeResponse,
    ProviderResultSummary,
};
use crate::commands::context::CommandContext;
use crate::commands::r#trait::MutationCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use workspace_domain::Capability;

/// IPC / Conversation result for an Application Provider operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationOperationResult {
    pub operation: String,
    pub ok: bool,
    pub status: Option<String>,
    pub target: Option<String>,
    pub message: Option<String>,
    pub preview: Option<String>,
    pub items: Option<Vec<crate::capability_runtime::ApplicationWindowItem>>,
}

impl From<ProviderInvokeResponse> for ApplicationOperationResult {
    fn from(response: ProviderInvokeResponse) -> Self {
        Self {
            operation: response.operation.as_str().into(),
            ok: response.ok,
            status: response.status,
            target: response.target,
            message: response.message,
            preview: response.preview,
            items: response.items,
        }
    }
}

/// Executes one Application Provider operation through the frozen pipeline.
pub struct ExecuteApplicationOperation {
    pub operation: CapabilityOperation,
    pub query: Option<String>,
    pub path: Option<String>,
    pub hwnd: Option<String>,
}

impl ExecuteApplicationOperation {
    pub fn new(
        operation: CapabilityOperation,
        query: Option<String>,
        path: Option<String>,
        hwnd: Option<String>,
    ) -> Self {
        Self {
            operation,
            query,
            path,
            hwnd,
        }
    }
}

impl crate::commands::Command for ExecuteApplicationOperation {
    fn name(&self) -> &'static str {
        "ExecuteApplicationOperation"
    }
}

impl MutationCommand for ExecuteApplicationOperation {
    type Output = ApplicationOperationResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(workspace_domain::ResourceKind::Application)
    }

    fn required_capability(&self) -> Capability {
        match self.operation {
            CapabilityOperation::Launch => Capability::application_launch(),
            CapabilityOperation::Enumerate | CapabilityOperation::Find => {
                Capability::application_read()
            }
            CapabilityOperation::Focus => Capability::application_focus(),
            CapabilityOperation::Close => Capability::application_close(),
            CapabilityOperation::Minimize => Capability::application_minimize(),
            CapabilityOperation::Restore => Capability::application_restore(),
            other => Capability::new(
                format!("application.{}", other.as_str()),
                workspace_domain::CapabilityScope::Application,
            )
            .unwrap_or_else(|_| Capability::application_read()),
        }
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        let summary = ProviderResultSummary {
            domain: CapabilityDomainId::application(),
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
        Some(json!({ "application": summary }).to_string())
    }

    fn audit_failure_metadata(&self) -> Option<String> {
        Some(
            json!({
                "application": {
                    "domain": "application",
                    "operation": self.operation.as_str(),
                    "query": self.query,
                }
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<ApplicationOperationResult> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        match self.operation {
            CapabilityOperation::Launch
            | CapabilityOperation::Enumerate
            | CapabilityOperation::Focus
            | CapabilityOperation::Close
            | CapabilityOperation::Minimize
            | CapabilityOperation::Restore
            | CapabilityOperation::Find => {}
            other => {
                return Err(KernelError::CapabilityRuntime {
                    message: format!(
                        "ExecuteApplicationOperation cannot run '{}'",
                        other.as_str()
                    ),
                });
            }
        }

        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::application(),
            operation: self.operation,
            text: None,
            query: self.query.clone(),
            path: self.path.clone(),
            hwnd: self.hwnd.clone(),
            pid: None,
        })?;
        Ok(ApplicationOperationResult::from(response))
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
    fn pipeline_enumerates_applications() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let result = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(ExecuteApplicationOperation::new(
                CapabilityOperation::Enumerate,
                None,
                None,
                None,
            ))
            .unwrap();
        assert!(result.ok);
        assert!(result.items.as_ref().unwrap().len() >= 1);
    }

    #[test]
    fn pipeline_launches_notepad_alias() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let result = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(ExecuteApplicationOperation::new(
                CapabilityOperation::Launch,
                Some("notepad".into()),
                None,
                None,
            ))
            .unwrap();
        assert!(result.ok);
        assert_eq!(result.target.as_deref(), Some("notepad.exe"));
    }
}
