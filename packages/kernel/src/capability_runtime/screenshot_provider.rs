use std::sync::Arc;

use workspace_windows_integration::ScreenshotPort;

use super::registry::CapabilityProvider;
use super::types::{
    CapabilityDomainId, CapabilityOperation, ProviderDescriptor, ProviderInvokeRequest,
    ProviderInvokeResponse,
};
use crate::error::{KernelError, Result};

/// Screenshot Capability Provider (P15) — Levels 1–2.
///
/// Adoption: WRAP `xcap` behind [`ScreenshotPort`].
pub struct ScreenshotProvider {
    port: Arc<dyn ScreenshotPort>,
}

impl ScreenshotProvider {
    pub fn new(port: Arc<dyn ScreenshotPort>) -> Self {
        Self { port }
    }

    fn outcome_response(
        operation: CapabilityOperation,
        outcome: workspace_windows_integration::ScreenshotCaptureOutcome,
    ) -> ProviderInvokeResponse {
        ProviderInvokeResponse {
            domain: CapabilityDomainId::screenshots(),
            operation,
            ok: outcome.ok,
            format: Some("png".into()),
            bytes: Some((outcome.width as usize).saturating_mul(outcome.height as usize)),
            text: outcome.path.clone(),
            preview: Some(format!("{}×{}", outcome.width, outcome.height)),
            message: Some(outcome.message),
            status: Some(if outcome.ok {
                if outcome.copied {
                    "copied".into()
                } else {
                    "captured".into()
                }
            } else {
                "not_captured".into()
            }),
            target: Some(outcome.target),
            items: None,
            monitors: None,
        }
    }
}

impl CapabilityProvider for ScreenshotProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            name: "ScreenshotProvider".into(),
            domain: CapabilityDomainId::screenshots(),
            purpose: "Capture desktop, window, or monitor stills under Workspace permission."
                .into(),
            operations: vec![
                "status",
                "capture_desktop",
                "capture_window",
                "capture_monitor",
                "save_png",
                "copy_clipboard",
            ],
            adoption: "WRAP",
        }
    }

    fn invoke(&self, request: ProviderInvokeRequest) -> Result<ProviderInvokeResponse> {
        if request.domain != CapabilityDomainId::screenshots() {
            return Err(KernelError::CapabilityRuntime {
                message: format!(
                    "ScreenshotProvider cannot serve domain '{}'",
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
                    domain: CapabilityDomainId::screenshots(),
                    operation: CapabilityOperation::Status,
                    ok: status.available,
                    format: Some(format!("{} monitors", status.monitor_count)),
                    bytes: Some(status.monitor_count),
                    text: Some(format!(
                        "window={}, clipboard={}",
                        status.can_capture_window, status.can_copy_clipboard
                    )),
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
            CapabilityOperation::CaptureDesktop | CapabilityOperation::SavePng => {
                let outcome = self.port.capture_desktop().map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                Ok(Self::outcome_response(request.operation, outcome))
            }
            CapabilityOperation::CaptureMonitor => {
                let index = request.monitor_index.unwrap_or(1);
                let outcome = self.port.capture_monitor(index).map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                Ok(Self::outcome_response(CapabilityOperation::CaptureMonitor, outcome))
            }
            CapabilityOperation::CaptureWindow => {
                let query = request
                    .query
                    .clone()
                    .or_else(|| request.title.clone())
                    .unwrap_or_default();
                let outcome = self.port.capture_window(&query).map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                Ok(Self::outcome_response(CapabilityOperation::CaptureWindow, outcome))
            }
            CapabilityOperation::CopyClipboard => {
                let path = request
                    .path
                    .clone()
                    .or_else(|| request.query.clone())
                    .unwrap_or_default();
                let outcome = self.port.copy_path_to_clipboard(&path).map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                Ok(Self::outcome_response(CapabilityOperation::CopyClipboard, outcome))
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!(
                    "ScreenshotProvider does not own operation '{}'",
                    other.as_str()
                ),
            }),
        }
    }
}
