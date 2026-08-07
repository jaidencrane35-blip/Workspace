use std::sync::Arc;

use workspace_windows_integration::{NotificationPort, NotificationShowRequest};

use super::registry::CapabilityProvider;
use super::types::{
    CapabilityDomainId, CapabilityOperation, ProviderDescriptor, ProviderInvokeRequest,
    ProviderInvokeResponse,
};
use crate::error::{KernelError, Result};

/// Notifications Capability Provider (P13) — Levels 1–2.
///
/// Adoption: WRAP WinRT toast APIs behind [`NotificationPort`].
pub struct NotificationProvider {
    port: Arc<dyn NotificationPort>,
}

impl NotificationProvider {
    pub fn new(port: Arc<dyn NotificationPort>) -> Self {
        Self { port }
    }
}

impl CapabilityProvider for NotificationProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            name: "NotificationProvider".into(),
            domain: CapabilityDomainId::notifications(),
            purpose: "Show and query desktop notifications under Workspace permission and audit."
                .into(),
            operations: vec!["status", "show", "dismiss"],
            adoption: "WRAP",
        }
    }

    fn invoke(&self, request: ProviderInvokeRequest) -> Result<ProviderInvokeResponse> {
        if request.domain != CapabilityDomainId::notifications() {
            return Err(KernelError::CapabilityRuntime {
                message: format!(
                    "NotificationProvider cannot serve domain '{}'",
                    request.domain.as_str()
                ),
            });
        }

        match request.operation {
            CapabilityOperation::Status => {
                let status = self.port.status().map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::notifications(),
                    operation: CapabilityOperation::Status,
                    ok: status.available,
                    format: Some(status.platform),
                    bytes: None,
                    text: Some(status.permission),
                    preview: Some(status.message.clone()),
                    message: Some(status.message),
                    status: Some(if status.available {
                        "available".into()
                    } else {
                        "unavailable".into()
                    }),
                    target: None,
                    items: None,
                    monitors: None,
                })
            }
            CapabilityOperation::Show => {
                let title = request
                    .title
                    .clone()
                    .or_else(|| request.query.clone())
                    .unwrap_or_default();
                let body = request.text.clone().unwrap_or_default();
                let outcome = self
                    .port
                    .show(&NotificationShowRequest {
                        title,
                        body,
                        category: request.category.clone(),
                        priority: request.priority.clone(),
                        duration: request.duration.clone(),
                    })
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::notifications(),
                    operation: CapabilityOperation::Show,
                    ok: outcome.shown,
                    format: request.category.clone(),
                    bytes: None,
                    text: Some(outcome.id.clone()),
                    preview: Some(outcome.message.clone()),
                    message: Some(outcome.message),
                    status: Some(if outcome.shown {
                        "shown".into()
                    } else {
                        "not_shown".into()
                    }),
                    target: Some(outcome.id),
                    items: None,
                    monitors: None,
                })
            }
            CapabilityOperation::Dismiss => {
                let id = request
                    .query
                    .clone()
                    .or_else(|| request.text.clone())
                    .unwrap_or_default();
                if id.trim().is_empty() {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::notifications(),
                        operation: CapabilityOperation::Dismiss,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: None,
                        message: Some(
                            "Tell me which notification to dismiss.".into(),
                        ),
                        status: Some("clarify".into()),
                        target: None,
                        items: None,
                        monitors: None,
                    });
                }
                let outcome = self.port.dismiss(&id).map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                let ok = outcome
                    .message
                    .to_ascii_lowercase()
                    .contains("dismissed");
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::notifications(),
                    operation: CapabilityOperation::Dismiss,
                    ok,
                    format: None,
                    bytes: None,
                    text: Some(outcome.id.clone()),
                    preview: Some(outcome.message.clone()),
                    message: Some(outcome.message),
                    status: Some(if ok {
                        "dismissed".into()
                    } else {
                        "not_dismissed".into()
                    }),
                    target: Some(outcome.id),
                    items: None,
                    monitors: None,
                })
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!(
                    "NotificationProvider does not own operation '{}'",
                    other.as_str()
                ),
            }),
        }
    }
}
