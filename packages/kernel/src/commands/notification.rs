//! Notifications capability commands — Intent → Router → NotificationProvider.

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

/// Level 1 notification capability status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationStatusResult {
    pub available: bool,
    pub platform: String,
    pub permission: String,
    pub message: String,
}

/// Level 2 show / dismiss acknowledgement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationOperationResult {
    pub ok: bool,
    pub status: String,
    pub target: Option<String>,
    pub message: String,
    pub preview: Option<String>,
}

fn ensure_ready(ctx: &CommandContext<'_>) -> Result<()> {
    if ctx.state.lifecycle == LifecycleState::Ready {
        Ok(())
    } else {
        Err(KernelError::NotReady)
    }
}

/// Queries whether desktop notifications are available.
pub struct NotificationStatus;

impl crate::commands::Command for NotificationStatus {
    fn name(&self) -> &'static str {
        "NotificationStatus"
    }
}

impl QueryCommand for NotificationStatus {
    type Output = NotificationStatusResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::notify_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<NotificationStatusResult> {
        ensure_ready(ctx)?;
        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::notifications(),
            operation: CapabilityOperation::Status,
            ..Default::default()
        })?;
        Ok(NotificationStatusResult {
            available: response.ok,
            platform: response.format.unwrap_or_else(|| "unknown".into()),
            permission: response.text.unwrap_or_else(|| "unknown".into()),
            message: response
                .message
                .unwrap_or_else(|| "Notification status unavailable.".into()),
        })
    }
}

/// Shows a desktop notification through the Capability Runtime.
pub struct ShowNotification {
    pub title: Option<String>,
    pub text: Option<String>,
    pub category: Option<String>,
    pub priority: Option<String>,
    pub duration: Option<String>,
}

impl ShowNotification {
    pub fn new(
        title: Option<String>,
        text: Option<String>,
        category: Option<String>,
        priority: Option<String>,
        duration: Option<String>,
    ) -> Self {
        Self {
            title,
            text,
            category,
            priority,
            duration,
        }
    }
}

impl crate::commands::Command for ShowNotification {
    fn name(&self) -> &'static str {
        "ShowNotification"
    }
}

impl MutationCommand for ShowNotification {
    type Output = NotificationOperationResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::notify_show()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        let summary = ProviderResultSummary {
            domain: CapabilityDomainId::notifications(),
            operation: CapabilityOperation::Show,
            ok: output.ok,
            format: self.category.clone(),
            bytes: None,
            preview: output.preview.clone(),
            message: Some(output.message.clone()),
            status: Some(output.status.clone()),
            target: output.target.clone(),
            item_count: None,
        };
        Some(json!({ "notification": summary }).to_string())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<NotificationOperationResult> {
        ensure_ready(ctx)?;
        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::notifications(),
            operation: CapabilityOperation::Show,
            title: self.title.clone(),
            text: self.text.clone(),
            category: self.category.clone(),
            priority: self.priority.clone(),
            duration: self.duration.clone(),
            ..Default::default()
        })?;
        Ok(NotificationOperationResult {
            ok: response.ok,
            status: response.status.unwrap_or_else(|| "unknown".into()),
            target: response.target,
            message: response
                .message
                .unwrap_or_else(|| "Notification finished.".into()),
            preview: response.preview,
        })
    }
}

/// Best-effort dismiss of a previously shown notification.
pub struct DismissNotification {
    pub id: Option<String>,
}

impl DismissNotification {
    pub fn new(id: Option<String>) -> Self {
        Self { id }
    }
}

impl crate::commands::Command for DismissNotification {
    fn name(&self) -> &'static str {
        "DismissNotification"
    }
}

impl MutationCommand for DismissNotification {
    type Output = NotificationOperationResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::notify_show()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "notification": {
                    "domain": "notifications",
                    "operation": "dismiss",
                    "ok": output.ok,
                    "status": output.status,
                    "target": output.target,
                }
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<NotificationOperationResult> {
        ensure_ready(ctx)?;
        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::notifications(),
            operation: CapabilityOperation::Dismiss,
            query: self.id.clone(),
            ..Default::default()
        })?;
        Ok(NotificationOperationResult {
            ok: response.ok,
            status: response.status.unwrap_or_else(|| "unknown".into()),
            target: response.target,
            message: response
                .message
                .unwrap_or_else(|| "Dismiss finished.".into()),
            preview: response.preview,
        })
    }
}
