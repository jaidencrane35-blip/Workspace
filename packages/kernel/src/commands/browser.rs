//! Browser capability commands — Intent → Router → BrowserProvider.

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
pub struct BrowserStatusResult {
    pub available: bool,
    pub default_handler: String,
    pub browsers: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserOperationResult {
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

pub struct BrowserStatus;

impl crate::commands::Command for BrowserStatus {
    fn name(&self) -> &'static str {
        "BrowserStatus"
    }
}

impl QueryCommand for BrowserStatus {
    type Output = BrowserStatusResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::browser_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<BrowserStatusResult> {
        ensure_ready(ctx)?;
        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::browser(),
            operation: CapabilityOperation::Status,
            ..Default::default()
        })?;
        Ok(BrowserStatusResult {
            available: response.ok,
            default_handler: response.format.unwrap_or_else(|| "unknown".into()),
            browsers: response.text.unwrap_or_default(),
            message: response
                .message
                .unwrap_or_else(|| "Browser status unavailable.".into()),
        })
    }
}

pub struct OpenBrowserUrl {
    pub url: Option<String>,
}

impl OpenBrowserUrl {
    pub fn new(url: Option<String>) -> Self {
        Self { url }
    }
}

impl crate::commands::Command for OpenBrowserUrl {
    fn name(&self) -> &'static str {
        "OpenBrowserUrl"
    }
}

impl MutationCommand for OpenBrowserUrl {
    type Output = BrowserOperationResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::browser_open()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        let summary = ProviderResultSummary {
            domain: CapabilityDomainId::browser(),
            operation: CapabilityOperation::Open,
            ok: output.ok,
            format: None,
            bytes: None,
            preview: output.preview.clone(),
            message: Some(output.message.clone()),
            status: Some(output.status.clone()),
            target: output.target.clone(),
            item_count: None,
        };
        Some(json!({ "browser": summary }).to_string())
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<BrowserOperationResult> {
        ensure_ready(ctx)?;
        let response = runtime().invoke(ProviderInvokeRequest {
            domain: CapabilityDomainId::browser(),
            operation: CapabilityOperation::Open,
            path: self.url.clone(),
            query: self.url.clone(),
            ..Default::default()
        })?;
        Ok(BrowserOperationResult {
            ok: response.ok,
            status: response.status.unwrap_or_else(|| "unknown".into()),
            target: response.target,
            message: response
                .message
                .unwrap_or_else(|| "Browser open finished.".into()),
            preview: response.preview,
        })
    }
}
