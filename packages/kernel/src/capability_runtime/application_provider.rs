//! Application Provider (P11) — owns application operations, not isolated features.
//!
//! Adoption: ADAPT Workspace contracts + WRAP/ADAPT Win32 via existing ports.

use std::sync::Arc;

use workspace_windows_integration::{
    DesktopWindowSnapshot, MutatorEffectOutcome, ProcessLaunchRequest, ProcessLauncher,
    WindowEnumerator, WindowMutator,
};

use super::registry::CapabilityProvider;
use super::types::{
    text_preview, ApplicationWindowItem, CapabilityDomainId, CapabilityOperation,
    ProviderDescriptor, ProviderInvokeRequest, ProviderInvokeResponse,
};
use crate::error::{KernelError, Result};

pub struct ApplicationPorts {
    pub launcher: Arc<dyn ProcessLauncher>,
    pub enumerator: Arc<dyn WindowEnumerator>,
    pub mutator: Arc<dyn WindowMutator>,
}

/// Application Capability Provider — Launch, Enumerate, Focus, Close, Minimize, Restore, Find.
pub struct ApplicationProvider {
    ports: ApplicationPorts,
}

impl ApplicationProvider {
    pub fn new(ports: ApplicationPorts) -> Self {
        Self { ports }
    }

    fn item_from_snapshot(window: &DesktopWindowSnapshot) -> ApplicationWindowItem {
        ApplicationWindowItem {
            hwnd: window.hwnd.clone(),
            title: window.title.clone(),
            process_id: window.process_id,
            minimized: window.minimized,
            focused: window.focused,
        }
    }

    fn match_windows(&self, query: &str) -> Result<Vec<DesktopWindowSnapshot>> {
        let needle = query.trim().to_ascii_lowercase();
        if needle.is_empty() {
            return Ok(Vec::new());
        }
        let windows = self
            .ports
            .enumerator
            .enumerate_windows()
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })?;
        Ok(windows
            .into_iter()
            .filter(|window| {
                window.title.to_ascii_lowercase().contains(&needle)
                    || format!("{}", window.process_id) == needle
                    || window.hwnd.to_ascii_lowercase() == needle
            })
            .collect())
    }

    fn resolve_target(
        &self,
        request: &ProviderInvokeRequest,
    ) -> Result<Option<DesktopWindowSnapshot>> {
        if let Some(hwnd) = request.hwnd.as_deref() {
            let windows = self
                .ports
                .enumerator
                .enumerate_windows()
                .map_err(|error| KernelError::WindowsIntegration {
                    message: error.to_string(),
                })?;
            return Ok(windows.into_iter().find(|window| window.hwnd == hwnd));
        }
        let query = request
            .query
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| KernelError::CapabilityRuntime {
                message: "application operation requires query or hwnd".into(),
            })?;
        let matches = self.match_windows(query)?;
        Ok(matches.into_iter().next())
    }

    fn resolve_launch_executable(request: &ProviderInvokeRequest) -> Result<String> {
        if let Some(path) = request
            .path
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return Ok(path.to_string());
        }
        let query = request
            .query
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| KernelError::CapabilityRuntime {
                message: "application launch requires query or path".into(),
            })?;
        Ok(launch_alias(query))
    }

    fn effect_response(
        operation: CapabilityOperation,
        status: &str,
        target: &str,
        outcome: MutatorEffectOutcome,
        message: impl Into<String>,
    ) -> Result<ProviderInvokeResponse> {
        let ok = matches!(
            outcome,
            MutatorEffectOutcome::Committed | MutatorEffectOutcome::OutcomeUnknown
        );
        Ok(ProviderInvokeResponse {
            domain: CapabilityDomainId::application(),
            operation,
            ok,
            format: None,
            bytes: None,
            text: None,
            preview: Some(text_preview(target, 80)),
            message: Some(message.into()),
            status: Some(status.into()),
            target: Some(target.into()),
            items: None,
        })
    }
}

impl CapabilityProvider for ApplicationProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            name: "ApplicationProvider".into(),
            domain: CapabilityDomainId::application(),
            purpose: "Launch, find, focus, and control desktop applications under Workspace governance."
                .into(),
            operations: vec![
                "launch",
                "enumerate",
                "focus",
                "close",
                "minimize",
                "restore",
                "find",
            ],
            adoption: "ADAPT",
        }
    }

    fn invoke(&self, request: ProviderInvokeRequest) -> Result<ProviderInvokeResponse> {
        if request.domain != CapabilityDomainId::application() {
            return Err(KernelError::CapabilityRuntime {
                message: format!(
                    "ApplicationProvider cannot serve domain '{}'",
                    request.domain.as_str()
                ),
            });
        }

        match request.operation {
            CapabilityOperation::Enumerate => {
                let windows = self
                    .ports
                    .enumerator
                    .enumerate_windows()
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?;
                let items: Vec<_> = windows.iter().map(Self::item_from_snapshot).collect();
                let preview = items
                    .iter()
                    .take(8)
                    .map(|item| item.title.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::application(),
                    operation: CapabilityOperation::Enumerate,
                    ok: true,
                    format: Some("application_list".into()),
                    bytes: Some(items.len()),
                    text: None,
                    preview: Some(text_preview(&preview, 160)),
                    message: Some(format!("{} running application window(s).", items.len())),
                    status: Some("enumerated".into()),
                    target: None,
                    items: Some(items),
                })
            }
            CapabilityOperation::Find => {
                let query = request
                    .query
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| KernelError::CapabilityRuntime {
                        message: "application find requires query".into(),
                    })?;
                let matches = self.match_windows(query)?;
                let items: Vec<_> = matches.iter().map(Self::item_from_snapshot).collect();
                let status = if items.is_empty() {
                    "not_found"
                } else {
                    "found"
                };
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::application(),
                    operation: CapabilityOperation::Find,
                    ok: !items.is_empty(),
                    format: Some("application_list".into()),
                    bytes: Some(items.len()),
                    text: None,
                    preview: Some(text_preview(query, 80)),
                    message: Some(format!("{status}: {} match(es) for “{query}”.", items.len())),
                    status: Some(status.into()),
                    target: Some(query.into()),
                    items: Some(items),
                })
            }
            CapabilityOperation::Launch => {
                let executable = Self::resolve_launch_executable(&request)?;
                let outcome = self
                    .ports
                    .launcher
                    .launch(&ProcessLaunchRequest {
                        executable: executable.clone(),
                        args: Vec::new(),
                    })
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?;
                let status = if outcome.simulated {
                    "launched_simulated"
                } else {
                    "launched"
                };
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::application(),
                    operation: CapabilityOperation::Launch,
                    ok: true,
                    format: None,
                    bytes: None,
                    text: None,
                    preview: Some(text_preview(&executable, 80)),
                    message: Some(format!("Launch requested for {executable}.")),
                    status: Some(status.into()),
                    target: Some(executable),
                    items: None,
                })
            }
            CapabilityOperation::Focus => {
                let Some(window) = self.resolve_target(&request)? else {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::application(),
                        operation: CapabilityOperation::Focus,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: request.query.clone(),
                        message: Some("No matching application window to focus.".into()),
                        status: Some("not_found".into()),
                        target: request.query.clone(),
                        items: None,
                    });
                };
                let outcome = self
                    .ports
                    .mutator
                    .focus_window(&window.hwnd)
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?;
                Self::effect_response(
                    CapabilityOperation::Focus,
                    "focused",
                    &window.title,
                    outcome,
                    format!("Focused “{}”.", window.title),
                )
            }
            CapabilityOperation::Close => {
                let Some(window) = self.resolve_target(&request)? else {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::application(),
                        operation: CapabilityOperation::Close,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: request.query.clone(),
                        message: Some("No matching application window to close.".into()),
                        status: Some("not_found".into()),
                        target: request.query.clone(),
                        items: None,
                    });
                };
                let outcome = self
                    .ports
                    .mutator
                    .close_window(&window.hwnd)
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?;
                Self::effect_response(
                    CapabilityOperation::Close,
                    "closed",
                    &window.title,
                    outcome,
                    format!("Close requested for “{}”.", window.title),
                )
            }
            CapabilityOperation::Minimize => {
                let Some(window) = self.resolve_target(&request)? else {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::application(),
                        operation: CapabilityOperation::Minimize,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: request.query.clone(),
                        message: Some("No matching application window to minimize.".into()),
                        status: Some("not_found".into()),
                        target: request.query.clone(),
                        items: None,
                    });
                };
                // Prefer dedicated minimize; fall back path unused on ports that
                // only implement place (kept for trait completeness).
                let outcome = self
                    .ports
                    .mutator
                    .minimize_window(&window.hwnd)
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?;
                Self::effect_response(
                    CapabilityOperation::Minimize,
                    "minimized",
                    &window.title,
                    outcome,
                    format!("Minimized “{}”.", window.title),
                )
            }
            CapabilityOperation::Restore => {
                let Some(window) = self.resolve_target(&request)? else {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::application(),
                        operation: CapabilityOperation::Restore,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: request.query.clone(),
                        message: Some("No matching application window to restore.".into()),
                        status: Some("not_found".into()),
                        target: request.query.clone(),
                        items: None,
                    });
                };
                let outcome = self
                    .ports
                    .mutator
                    .restore_window(&window.hwnd)
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?;
                Self::effect_response(
                    CapabilityOperation::Restore,
                    "restored",
                    &window.title,
                    outcome,
                    format!("Restored “{}”.", window.title),
                )
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!(
                    "ApplicationProvider does not own operation '{}'",
                    other.as_str()
                ),
            }),
        }
    }
}

fn launch_alias(query: &str) -> String {
    let normalized = query.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "notepad" => "notepad.exe".into(),
        "explorer" | "file explorer" | "files" => "explorer.exe".into(),
        "cmd" | "command prompt" | "terminal cmd" => "cmd.exe".into(),
        "powershell" | "pwsh" => "powershell.exe".into(),
        "chrome" | "google chrome" => "chrome.exe".into(),
        "edge" | "microsoft edge" => "msedge.exe".into(),
        "cursor" => "cursor.exe".into(),
        "spotify" => "spotify.exe".into(),
        "calculator" | "calc" => "calc.exe".into(),
        other if other.ends_with(".exe") || other.contains('\\') || other.contains('/') => {
            query.trim().to_string()
        }
        other => format!("{other}.exe"),
    }
}
