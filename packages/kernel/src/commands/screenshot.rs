//! Screenshot capability commands — Intent → Router → ScreenshotProvider.

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotStatusResult {
    pub available: bool,
    pub monitor_count: usize,
    pub can_capture_window: bool,
    pub can_copy_clipboard: bool,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotOperationResult {
    pub ok: bool,
    pub status: String,
    pub target: Option<String>,
    pub path: Option<String>,
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

pub struct ScreenshotStatus;

impl crate::commands::Command for ScreenshotStatus {
    fn name(&self) -> &'static str {
        "ScreenshotStatus"
    }
}

impl QueryCommand for ScreenshotStatus {
    type Output = ScreenshotStatusResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::screenshot_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<ScreenshotStatusResult> {
        ensure_ready(ctx)?;
        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::screenshots(),
            operation: CapabilityOperation::Status,
            ..Default::default()
        })?;
        Ok(ScreenshotStatusResult {
            available: response.ok,
            monitor_count: response.bytes.unwrap_or(0),
            can_capture_window: response
                .text
                .as_deref()
                .unwrap_or("")
                .contains("window=true"),
            can_copy_clipboard: response
                .text
                .as_deref()
                .unwrap_or("")
                .contains("clipboard=true"),
            message: response
                .message
                .unwrap_or_else(|| "Screenshot status unavailable.".into()),
        })
    }
}

pub struct ExecuteScreenshotOperation {
    pub operation: CapabilityOperation,
    pub query: Option<String>,
    pub path: Option<String>,
    pub monitor_index: Option<i32>,
}

impl ExecuteScreenshotOperation {
    pub fn new(
        operation: CapabilityOperation,
        query: Option<String>,
        path: Option<String>,
        monitor_index: Option<i32>,
    ) -> Self {
        Self {
            operation,
            query,
            path,
            monitor_index,
        }
    }
}

impl crate::commands::Command for ExecuteScreenshotOperation {
    fn name(&self) -> &'static str {
        "ExecuteScreenshotOperation"
    }
}

impl MutationCommand for ExecuteScreenshotOperation {
    type Output = ScreenshotOperationResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::screenshot_capture()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        let summary = ProviderResultSummary {
            domain: CapabilityDomainId::screenshots(),
            operation: self.operation,
            ok: output.ok,
            format: Some("png".into()),
            bytes: None,
            preview: output.preview.clone(),
            message: Some(output.message.clone()),
            status: Some(output.status.clone()),
            target: output.target.clone(),
            item_count: None,
        };
        Some(json!({ "screenshot": summary }).to_string())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<ScreenshotOperationResult> {
        ensure_ready(ctx)?;
        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::screenshots(),
            operation: self.operation,
            query: self.query.clone(),
            path: self.path.clone(),
            title: self.query.clone(),
            monitor_index: self.monitor_index,
            ..Default::default()
        })?;
        Ok(ScreenshotOperationResult {
            ok: response.ok,
            status: response.status.unwrap_or_else(|| "unknown".into()),
            target: response.target,
            path: response.text,
            message: response
                .message
                .unwrap_or_else(|| "Screenshot operation finished.".into()),
            preview: response.preview,
        })
    }
}
