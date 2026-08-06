use std::sync::Arc;

use workspace_windows_integration::ClipboardPort;

use super::registry::CapabilityProvider;
use super::types::{
    text_preview, CapabilityDomainId, CapabilityOperation, ProviderDescriptor,
    ProviderInvokeRequest, ProviderInvokeResponse,
};
use crate::error::{KernelError, Result};

/// Reference Capability Provider (P10) — Clipboard domain.
///
/// Adoption: WRAP `arboard` behind [`ClipboardPort`].
/// Operations owned: Read, Write.
pub struct ClipboardProvider {
    port: Arc<dyn ClipboardPort>,
}

impl ClipboardProvider {
    pub fn new(port: Arc<dyn ClipboardPort>) -> Self {
        Self { port }
    }
}

impl CapabilityProvider for ClipboardProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            name: "ClipboardProvider".into(),
            domain: CapabilityDomainId::clipboard(),
            purpose: "Read and write clipboard text under Workspace permission and audit."
                .into(),
            operations: vec!["read", "write"],
            adoption: "WRAP",
        }
    }

    fn invoke(&self, request: ProviderInvokeRequest) -> Result<ProviderInvokeResponse> {
        if request.domain != CapabilityDomainId::clipboard() {
            return Err(KernelError::CapabilityRuntime {
                message: format!(
                    "ClipboardProvider cannot serve domain '{}'",
                    request.domain.as_str()
                ),
            });
        }

        match request.operation {
            CapabilityOperation::Read => {
                let text = self.port.read_text().map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                let bytes = text.len();
                let preview = text_preview(&text, 120);
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::clipboard(),
                    operation: CapabilityOperation::Read,
                    ok: true,
                    format: Some("text".into()),
                    bytes: Some(bytes),
                    text: Some(text),
                    preview: Some(preview),
                    message: None,
                    status: Some("read".into()),
                    target: None,
                    items: None,
                })
            }
            CapabilityOperation::Write => {
                let text = request.text.ok_or_else(|| KernelError::CapabilityRuntime {
                    message: "clipboard write requires text".into(),
                })?;
                self.port.write_text(&text).map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                let bytes = text.len();
                let preview = text_preview(&text, 80);
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::clipboard(),
                    operation: CapabilityOperation::Write,
                    ok: true,
                    format: Some("text".into()),
                    bytes: Some(bytes),
                    text: None,
                    preview: Some(preview),
                    message: Some("Clipboard updated.".into()),
                    status: Some("written".into()),
                    target: None,
                    items: None,
                })
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!(
                    "ClipboardProvider does not own operation '{}'",
                    other.as_str()
                ),
            }),
        }
    }
}
