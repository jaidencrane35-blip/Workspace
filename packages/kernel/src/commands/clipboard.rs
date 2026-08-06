//! Clipboard capability commands — Intent → Router → ClipboardProvider.

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::capability_runtime::{
    runtime, CapabilityDomainId, CapabilityOperation, ProviderInvokeRequest, ProviderResultSummary,
};
use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use workspace_domain::Capability;

/// Audit-safe clipboard read result for IPC / Conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardReadResult {
    pub format: String,
    pub bytes: usize,
    pub preview: String,
    /// Full text for Conversation reply only — not for audit persistence.
    pub text: String,
}

/// Clipboard write acknowledgement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardWriteResult {
    pub format: String,
    pub bytes: usize,
    pub preview: String,
    pub message: String,
}

/// Reads clipboard text through the Capability Runtime.
pub struct ReadClipboard;

impl crate::commands::Command for ReadClipboard {
    fn name(&self) -> &'static str {
        "ReadClipboard"
    }
}

impl QueryCommand for ReadClipboard {
    type Output = ClipboardReadResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::clipboard_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<ClipboardReadResult> {
        ensure_ready(ctx)?;
        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::clipboard(),
            operation: CapabilityOperation::Read,
            text: None,
            ..Default::default()
        })?;
        Ok(ClipboardReadResult {
            format: response.format.unwrap_or_else(|| "text".into()),
            bytes: response.bytes.unwrap_or(0),
            preview: response.preview.unwrap_or_default(),
            text: response.text.unwrap_or_default(),
        })
    }
}

/// Writes clipboard text through the Capability Runtime.
pub struct WriteClipboard {
    pub text: String,
}

impl WriteClipboard {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl crate::commands::Command for WriteClipboard {
    fn name(&self) -> &'static str {
        "WriteClipboard"
    }
}

impl MutationCommand for WriteClipboard {
    type Output = ClipboardWriteResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::clipboard_write()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        let summary = ProviderResultSummary {
            domain: CapabilityDomainId::clipboard(),
            operation: CapabilityOperation::Write,
            ok: true,
            format: Some(output.format.clone()),
            bytes: Some(output.bytes),
            preview: Some(output.preview.clone()),
            message: Some(output.message.clone()),
            status: Some("written".into()),
            target: None,
            item_count: None,
        };
        Some(json!({ "clipboard": summary }).to_string())
    }

    fn audit_failure_metadata(&self) -> Option<String> {
        Some(
            json!({
                "clipboard": {
                    "domain": "clipboard",
                    "operation": "write",
                    "bytes": self.text.len(),
                }
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<ClipboardWriteResult> {
        ensure_ready(ctx)?;
        if self.text.len() > 1_000_000 {
            return Err(KernelError::CapabilityRuntime {
                message: "clipboard write exceeds 1MB limit".into(),
            });
        }
        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::clipboard(),
            operation: CapabilityOperation::Write,
            text: Some(self.text.clone()),
            ..Default::default()
        })?;
        Ok(ClipboardWriteResult {
            format: response.format.unwrap_or_else(|| "text".into()),
            bytes: response.bytes.unwrap_or(0),
            preview: response.preview.unwrap_or_default(),
            message: response
                .message
                .unwrap_or_else(|| "Clipboard updated.".into()),
        })
    }
}

fn ensure_ready(ctx: &CommandContext<'_>) -> Result<()> {
    if ctx.state.lifecycle == LifecycleState::Ready {
        Ok(())
    } else {
        Err(KernelError::NotReady)
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
    fn pipeline_write_then_read_clipboard() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = ready_ctx(&init, &bus);

        CommandPipeline::new(ctx)
            .execute_mutation(WriteClipboard::new("constitutional-p10".into()))
            .unwrap();

        let ctx = ready_ctx(&init, &bus);
        let read = CommandPipeline::new(ctx)
            .execute_query(ReadClipboard)
            .unwrap();
        assert_eq!(read.text, "constitutional-p10");
        assert_eq!(read.format, "text");
        assert!(read.preview.contains("constitutional"));
    }

    #[test]
    fn read_requires_clipboard_capability() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = CommandContext {
            actor_context: ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::new(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &crate::policy::CapabilityBoundPolicy,
        };

        let err = CommandPipeline::new(ctx)
            .execute_query(ReadClipboard)
            .unwrap_err();
        assert!(
            matches!(err, KernelError::PermissionDenied(_))
                || matches!(err, KernelError::ApprovalRequired { .. })
        );
    }
}
