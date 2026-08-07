use std::sync::Arc;

use workspace_windows_integration::BrowserPort;

use super::registry::CapabilityProvider;
use super::types::{
    CapabilityDomainId, CapabilityOperation, ProviderDescriptor, ProviderInvokeRequest,
    ProviderInvokeResponse,
};
use crate::error::{KernelError, Result};

/// Browser Capability Provider (P14) — Levels 1–2.
///
/// Adoption: WRAP `webbrowser` behind [`BrowserPort`].
pub struct BrowserProvider {
    port: Arc<dyn BrowserPort>,
}

impl BrowserProvider {
    pub fn new(port: Arc<dyn BrowserPort>) -> Self {
        Self { port }
    }
}

impl CapabilityProvider for BrowserProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            name: "BrowserProvider".into(),
            domain: CapabilityDomainId::browser(),
            purpose: "Open URLs and query browser availability under Workspace permission."
                .into(),
            operations: vec!["status", "open", "focus"],
            adoption: "WRAP",
        }
    }

    fn invoke(&self, request: ProviderInvokeRequest) -> Result<ProviderInvokeResponse> {
        if request.domain != CapabilityDomainId::browser() {
            return Err(KernelError::CapabilityRuntime {
                message: format!(
                    "BrowserProvider cannot serve domain '{}'",
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
                let list = if status.browsers.is_empty() {
                    "system default".into()
                } else {
                    status.browsers.join(", ")
                };
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::browser(),
                    operation: CapabilityOperation::Status,
                    ok: status.available,
                    format: Some(status.default_handler),
                    bytes: Some(status.browsers.len()),
                    text: Some(list),
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
            CapabilityOperation::Open => {
                let url = request
                    .path
                    .clone()
                    .or_else(|| request.query.clone())
                    .or_else(|| request.text.clone())
                    .unwrap_or_default();
                let outcome = self.port.open_url(&url).map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::browser(),
                    operation: CapabilityOperation::Open,
                    ok: outcome.ok,
                    format: None,
                    bytes: None,
                    text: outcome.target.clone(),
                    preview: Some(outcome.message.clone()),
                    message: Some(outcome.message),
                    status: Some(if outcome.ok {
                        "opened".into()
                    } else {
                        "not_opened".into()
                    }),
                    target: outcome.target,
                    items: None,
                    monitors: None,
                })
            }
            CapabilityOperation::Focus => {
                let query = request.query.clone().unwrap_or_default();
                let outcome = self.port.focus(&query).map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::browser(),
                    operation: CapabilityOperation::Focus,
                    ok: outcome.ok,
                    format: None,
                    bytes: None,
                    text: outcome.target.clone(),
                    preview: Some(outcome.message.clone()),
                    message: Some(outcome.message),
                    status: Some(if outcome.ok {
                        "focused".into()
                    } else {
                        "clarify".into()
                    }),
                    target: outcome.target,
                    items: None,
                    monitors: None,
                })
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!(
                    "BrowserProvider does not own operation '{}'",
                    other.as_str()
                ),
            }),
        }
    }
}
