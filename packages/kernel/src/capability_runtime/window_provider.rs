//! Window Provider (P12) — owns window geometry and state operations.
//!
//! Adoption: **ADAPT** existing Win32 ports. FancyZones-class UX = STUDY only.
//! Independently testable — never calls ApplicationProvider.

use std::sync::Arc;

use workspace_windows_integration::{
    CapturedDesktopMonitor, CapturedDesktopWindow, DesktopCapturer, DesktopWindowSnapshot,
    MutatorEffectOutcome, UiAutomationPort, WindowEnumerator, WindowMutator,
    WindowPlacementRequest,
};

use super::registry::CapabilityProvider;
use super::types::{
    text_preview, ApplicationWindowItem, CapabilityDomainId, CapabilityOperation, MonitorItem,
    ProviderDescriptor, ProviderInvokeRequest, ProviderInvokeResponse,
};
use crate::error::{KernelError, Result};

pub struct WindowPorts {
    pub enumerator: Arc<dyn WindowEnumerator>,
    pub mutator: Arc<dyn WindowMutator>,
    pub capturer: Arc<dyn DesktopCapturer>,
    pub ui_automation: Arc<dyn UiAutomationPort>,
}

/// Window Capability Provider — Discovery, Focus, State, Placement, Information (L1–L2).
pub struct WindowProvider {
    ports: WindowPorts,
}

impl WindowProvider {
    pub fn new(ports: WindowPorts) -> Self {
        Self { ports }
    }

    fn item(window: &DesktopWindowSnapshot) -> ApplicationWindowItem {
        ApplicationWindowItem {
            hwnd: window.hwnd.clone(),
            title: window.title.clone(),
            process_id: window.process_id,
            process_name: window.process_name.clone(),
            minimized: window.minimized,
            focused: window.focused,
            x: window.x,
            y: window.y,
            width: window.width,
            height: window.height,
            monitor_index: window.monitor_index,
        }
    }

    fn monitor_item(monitor: &CapturedDesktopMonitor) -> MonitorItem {
        MonitorItem {
            index: monitor.index,
            name: monitor.name.clone(),
            x: monitor.x,
            y: monitor.y,
            width: monitor.width,
            height: monitor.height,
            work_x: monitor.work_x,
            work_y: monitor.work_y,
            work_width: monitor.work_width,
            work_height: monitor.work_height,
            is_primary: monitor.is_primary,
        }
    }

    fn windows(&self) -> Result<Vec<DesktopWindowSnapshot>> {
        self.ports
            .enumerator
            .enumerate_windows()
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })
    }

    fn monitors(&self) -> Result<Vec<CapturedDesktopMonitor>> {
        let capture = self
            .ports
            .capturer
            .capture_desktop()
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })?;
        Ok(capture.monitors)
    }

    fn is_active_deixis(query: &str) -> bool {
        matches!(
            query.trim().to_ascii_lowercase().as_str(),
            "this"
                | "it"
                | "this window"
                | "the window"
                | "the active window"
                | "active"
                | "active window"
                | "current"
                | "current window"
                | "foreground"
                | "foreground window"
        )
    }

    fn foreground_window(&self) -> Result<Option<DesktopWindowSnapshot>> {
        let capture =
            self.ports
                .capturer
                .capture_desktop()
                .map_err(|error| KernelError::WindowsIntegration {
                    message: error.to_string(),
                })?;
        let Some(hwnd) = capture.foreground_hwnd.as_ref() else {
            return Ok(None);
        };
        Ok(capture
            .windows
            .iter()
            .find(|w| &w.hwnd == hwnd)
            .map(CapturedDesktopWindow::to_legacy_snapshot))
    }

    fn match_windows(&self, request: &ProviderInvokeRequest) -> Result<Vec<DesktopWindowSnapshot>> {
        let windows = self.windows()?;
        if let Some(hwnd) = request.hwnd.as_deref() {
            return Ok(windows.into_iter().filter(|w| w.hwnd == hwnd).collect());
        }
        if let Some(pid) = request.pid {
            return Ok(windows.into_iter().filter(|w| w.process_id == pid).collect());
        }
        let query = request
            .query
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_ascii_lowercase());
        if let Some(ref needle) = query {
            if Self::is_active_deixis(needle) {
                return Ok(self.foreground_window()?.into_iter().collect());
            }
        }
        let path = request
            .path
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_ascii_lowercase());
        if query.is_none() && path.is_none() {
            return Ok(windows);
        }
        Ok(windows
            .into_iter()
            .filter(|window| {
                let title = window.title.to_ascii_lowercase();
                let query_hit = query
                    .as_ref()
                    .map(|needle| title.contains(needle))
                    .unwrap_or(false);
                let path_hit = path
                    .as_ref()
                    .map(|needle| title.contains(needle) || needle.contains(&title))
                    .unwrap_or(false);
                query_hit || path_hit
            })
            .collect())
    }

    fn resolve_one(
        &self,
        request: &ProviderInvokeRequest,
    ) -> Result<Option<DesktopWindowSnapshot>> {
        let query = request.query.as_deref().map(str::trim).unwrap_or("");
        if query.is_empty() {
            // Bare operate verbs (“maximize”, “center”) target the active window.
            return self.foreground_window();
        }
        Ok(self.match_windows(request)?.into_iter().next())
    }

    fn control_name(request: &ProviderInvokeRequest) -> Option<&str> {
        request
            .text
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .or_else(|| {
                request
                    .title
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
            })
    }

    /// For typing: control name lives in `title` only (value is `text`).
    fn control_name_for_type(request: &ProviderInvokeRequest) -> Option<&str> {
        request
            .title
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
    }

    /// C-VER-003 — bounded timeout (default 2s, max 8s). Never indefinite.
    fn wait_timeout_ms(request: &ProviderInvokeRequest) -> u64 {
        const DEFAULT_MS: u64 = 2_000;
        const MAX_MS: u64 = 8_000;
        let raw = request
            .duration
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("");
        if raw.is_empty() {
            return DEFAULT_MS;
        }
        let lower = raw.to_ascii_lowercase();
        let parsed = match lower.as_str() {
            "short" => Some(2_000),
            "long" => Some(5_000),
            s if s.ends_with("ms") => s.trim_end_matches("ms").parse::<u64>().ok(),
            s if s.ends_with('s') => s
                .trim_end_matches('s')
                .parse::<u64>()
                .ok()
                .map(|secs| secs.saturating_mul(1_000)),
            s => s.parse::<u64>().ok(),
        };
        parsed.unwrap_or(DEFAULT_MS).clamp(1, MAX_MS)
    }

    fn wait_kind(request: &ProviderInvokeRequest) -> &'static str {
        let raw = request
            .category
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .to_ascii_lowercase();
        match raw.as_str() {
            "control_gone" | "gone" | "disappear" | "disappears" => "control_gone",
            "window_available" | "window" => "window_available",
            "window_active" | "active" | "foreground" => "window_active",
            "control_available" | "available" | "appear" | "appears" | "" => {
                if Self::control_name(request).is_some() {
                    "control_available"
                } else {
                    "window_available"
                }
            }
            _ => {
                if Self::control_name(request).is_some() {
                    "control_available"
                } else {
                    "window_available"
                }
            }
        }
    }

    fn wait_condition_met(
        &self,
        request: &ProviderInvokeRequest,
        kind: &str,
    ) -> Result<bool> {
        match kind {
            "control_available" => {
                let Some(control_query) = Self::control_name(request) else {
                    return Ok(false);
                };
                let Some(window) = self.resolve_one(request)? else {
                    return Ok(false);
                };
                Ok(self
                    .ports
                    .ui_automation
                    .find_control(&window.hwnd, control_query)
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?
                    .is_some())
            }
            "control_gone" => {
                let Some(control_query) = Self::control_name(request) else {
                    return Ok(false);
                };
                let Some(window) = self.resolve_one(request)? else {
                    // Window gone ⇒ control gone for this wait.
                    return Ok(true);
                };
                Ok(self
                    .ports
                    .ui_automation
                    .find_control(&window.hwnd, control_query)
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?
                    .is_none())
            }
            "window_available" => Ok(self.resolve_one(request)?.is_some()),
            "window_active" => {
                let Some(wanted) = self.resolve_one(request)? else {
                    return Ok(false);
                };
                let fg = self.foreground_window()?;
                Ok(fg
                    .as_ref()
                    .is_some_and(|active| active.hwnd == wanted.hwnd || active.focused))
            }
            _ => Ok(false),
        }
    }

    fn effect(
        operation: CapabilityOperation,
        status: &str,
        window: &DesktopWindowSnapshot,
        outcome: MutatorEffectOutcome,
        message: impl Into<String>,
    ) -> ProviderInvokeResponse {
        let ok = matches!(
            outcome,
            MutatorEffectOutcome::Committed | MutatorEffectOutcome::OutcomeUnknown
        );
        ProviderInvokeResponse {
            domain: CapabilityDomainId::window(),
            operation,
            ok,
            format: None,
            bytes: None,
            text: None,
            preview: Some(text_preview(&window.title, 80)),
            message: Some(message.into()),
            status: Some(status.into()),
            target: Some(window.title.clone()),
            items: Some(vec![Self::item(window)]),
            monitors: None,
        }
    }

    fn not_found(operation: CapabilityOperation, request: &ProviderInvokeRequest) -> ProviderInvokeResponse {
        ProviderInvokeResponse {
            domain: CapabilityDomainId::window(),
            operation,
            ok: false,
            format: None,
            bytes: None,
            text: None,
            preview: request.query.clone(),
            message: Some("No matching window.".into()),
            status: Some("not_found".into()),
            target: request.query.clone().or_else(|| request.hwnd.clone()),
            items: None,
            monitors: None,
        }
    }

    fn place(
        &self,
        hwnd: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<MutatorEffectOutcome> {
        self.ports
            .mutator
            .place_window(
                hwnd,
                &WindowPlacementRequest {
                    x,
                    y,
                    width,
                    height,
                    minimized: false,
                },
            )
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })
    }

    fn monitor_for(
        monitors: &[CapturedDesktopMonitor],
        index: Option<i32>,
        fallback: Option<i32>,
    ) -> Option<&CapturedDesktopMonitor> {
        if let Some(index) = index {
            if let Some(monitor) = monitors.iter().find(|m| m.index == index) {
                return Some(monitor);
            }
        }
        if let Some(index) = fallback {
            if let Some(monitor) = monitors.iter().find(|m| m.index == index) {
                return Some(monitor);
            }
        }
        monitors
            .iter()
            .find(|m| m.is_primary)
            .or_else(|| monitors.first())
    }
}

impl CapabilityProvider for WindowProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            name: "WindowProvider".into(),
            domain: CapabilityDomainId::window(),
            purpose: "Discover, focus, place, and inspect native windows under Workspace governance."
                .into(),
            operations: vec![
                "enumerate",
                "find",
                "active",
                "bounds",
                "monitors",
                "enumerate_controls",
                "find_control",
                "invoke_control",
                "set_control_value",
                "wait_condition",
                "focus",
                "minimize",
                "restore",
                "maximize",
                "move",
                "resize",
                "center",
                "snap",
            ],
            adoption: "ADAPT",
        }
    }

    fn invoke(&self, request: ProviderInvokeRequest) -> Result<ProviderInvokeResponse> {
        if request.domain != CapabilityDomainId::window() {
            return Err(KernelError::CapabilityRuntime {
                message: format!(
                    "WindowProvider cannot serve domain '{}'",
                    request.domain.as_str()
                ),
            });
        }

        match request.operation {
            CapabilityOperation::EnumerateControls => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(
                        CapabilityOperation::EnumerateControls,
                        &request,
                    ));
                };
                let controls = self
                    .ports
                    .ui_automation
                    .enumerate_controls(&window.hwnd, 40)
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?;
                let lines: Vec<String> = controls
                    .iter()
                    .map(|c| {
                        if c.automation_id.is_empty() {
                            format!("• {} ({})", c.name, c.control_type)
                        } else {
                            format!(
                                "• {} ({}) id={}",
                                c.name, c.control_type, c.automation_id
                            )
                        }
                    })
                    .collect();
                let body = if lines.is_empty() {
                    "I didn’t find named controls in that window (custom-drawn UI may not expose them)."
                        .to_string()
                } else {
                    format!(
                        "Controls in “{}” ({} shown):\n{}",
                        window.title,
                        lines.len(),
                        lines.join("\n")
                    )
                };
                let item = Self::item(&window);
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::EnumerateControls,
                    ok: true,
                    format: Some("control_list".into()),
                    bytes: Some(controls.len()),
                    text: Some(lines.join("\n")),
                    preview: Some(text_preview(&window.title, 80)),
                    message: Some(body),
                    status: Some("enumerated_controls".into()),
                    target: Some(window.title),
                    items: Some(vec![item]),
                    monitors: None,
                })
            }
            CapabilityOperation::FindControl => {
                let control_query = request
                    .text
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .or_else(|| {
                        request
                            .title
                            .as_deref()
                            .map(str::trim)
                            .filter(|s| !s.is_empty())
                    });
                let Some(control_query) = control_query else {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::FindControl,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: None,
                        message: Some(
                            "Name the control to look for (for example Save or Edit).".into(),
                        ),
                        status: Some("need_control_name".into()),
                        target: request.query.clone(),
                        items: None,
                        monitors: None,
                    });
                };
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::FindControl, &request));
                };
                let found = self
                    .ports
                    .ui_automation
                    .find_control(&window.hwnd, control_query)
                    .map_err(|error| KernelError::WindowsIntegration {
                        message: error.to_string(),
                    })?;
                let item = Self::item(&window);
                match found {
                    Some(control) => {
                        let detail = if control.automation_id.is_empty() {
                            format!("{} ({})", control.name, control.control_type)
                        } else {
                            format!(
                                "{} ({}) id={}",
                                control.name, control.control_type, control.automation_id
                            )
                        };
                        Ok(ProviderInvokeResponse {
                            domain: CapabilityDomainId::window(),
                            operation: CapabilityOperation::FindControl,
                            ok: true,
                            format: Some("control_match".into()),
                            bytes: Some(1),
                            text: Some(detail.clone()),
                            preview: Some(text_preview(&control.name, 80)),
                            message: Some(format!(
                                "Found “{}” in “{}” — {}.",
                                control.name, window.title, detail
                            )),
                            status: Some("control_found".into()),
                            target: Some(window.title),
                            items: Some(vec![item]),
                            monitors: None,
                        })
                    }
                    None => Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::FindControl,
                        ok: false,
                        format: Some("control_match".into()),
                        bytes: Some(0),
                        text: None,
                        preview: Some(text_preview(control_query, 80)),
                        message: Some(format!(
                            "I couldn’t find a control named “{control_query}” in “{}”.",
                            window.title
                        )),
                        status: Some("control_not_found".into()),
                        target: Some(window.title),
                        items: Some(vec![item]),
                        monitors: None,
                    }),
                }
            }
            CapabilityOperation::InvokeControl => {
                let Some(control_query) = Self::control_name(&request) else {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::InvokeControl,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: None,
                        message: Some(
                            "Name the control to click (for example Save).".into(),
                        ),
                        status: Some("need_control_name".into()),
                        target: request.query.clone(),
                        items: None,
                        monitors: None,
                    });
                };
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::InvokeControl, &request));
                };
                match self
                    .ports
                    .ui_automation
                    .invoke_control(&window.hwnd, control_query)
                {
                    Ok(outcome) => {
                        let item = Self::item(&window);
                        Ok(ProviderInvokeResponse {
                            domain: CapabilityDomainId::window(),
                            operation: CapabilityOperation::InvokeControl,
                            ok: outcome.verified,
                            format: Some("control_invoke".into()),
                            bytes: Some(1),
                            text: Some(outcome.control.name.clone()),
                            preview: Some(text_preview(&outcome.control.name, 80)),
                            message: Some(if outcome.verified {
                                format!(
                                    "Clicked “{}” in “{}”.",
                                    outcome.control.name, window.title
                                )
                            } else {
                                format!(
                                    "{} (in “{}”).",
                                    outcome.detail, window.title
                                )
                            }),
                            status: Some(if outcome.verified {
                                "control_clicked".into()
                            } else {
                                "control_click_unverified".into()
                            }),
                            target: Some(window.title),
                            items: Some(vec![item]),
                            monitors: None,
                        })
                    }
                    Err(error) => {
                        let item = Self::item(&window);
                        Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::InvokeControl,
                        ok: false,
                        format: Some("control_invoke".into()),
                        bytes: None,
                        text: None,
                        preview: Some(text_preview(control_query, 80)),
                        message: Some(error.to_string()),
                        status: Some("control_click_failed".into()),
                        target: Some(window.title),
                        items: Some(vec![item]),
                        monitors: None,
                    })
                    }
                }
            }
            CapabilityOperation::SetControlValue => {
                let Some(control_query) = Self::control_name_for_type(&request) else {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::SetControlValue,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: None,
                        message: Some(
                            "Name the field to type into (for example Edit).".into(),
                        ),
                        status: Some("need_control_name".into()),
                        target: request.query.clone(),
                        items: None,
                        monitors: None,
                    });
                };
                let value = request.text.as_deref().unwrap_or("").to_string();
                if value.is_empty() {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::SetControlValue,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: None,
                        message: Some("What text should I type?".into()),
                        status: Some("need_text".into()),
                        target: request.query.clone(),
                        items: None,
                        monitors: None,
                    });
                }
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(
                        CapabilityOperation::SetControlValue,
                        &request,
                    ));
                };
                match self.ports.ui_automation.set_control_value(
                    &window.hwnd,
                    control_query,
                    &value,
                ) {
                    Ok(outcome) => {
                        let item = Self::item(&window);
                        Ok(ProviderInvokeResponse {
                            domain: CapabilityDomainId::window(),
                            operation: CapabilityOperation::SetControlValue,
                            ok: outcome.verified,
                            format: Some("control_type".into()),
                            bytes: Some(value.len()),
                            text: Some(value.clone()),
                            preview: Some(text_preview(&outcome.control.name, 80)),
                            message: Some(if outcome.verified {
                                format!(
                                    "Typed into “{}” in “{}”.",
                                    outcome.control.name, window.title
                                )
                            } else {
                                format!("{} (in “{}”).", outcome.detail, window.title)
                            }),
                            status: Some(if outcome.verified {
                                "control_typed".into()
                            } else {
                                "control_type_unverified".into()
                            }),
                            target: Some(window.title),
                            items: Some(vec![item]),
                            monitors: None,
                        })
                    }
                    Err(error) => {
                        let item = Self::item(&window);
                        Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::SetControlValue,
                        ok: false,
                        format: Some("control_type".into()),
                        bytes: None,
                        text: None,
                        preview: Some(text_preview(control_query, 80)),
                        message: Some(error.to_string()),
                        status: Some("control_type_failed".into()),
                        target: Some(window.title),
                        items: Some(vec![item]),
                        monitors: None,
                    })
                    }
                }
            }
            CapabilityOperation::WaitCondition => {
                let kind = Self::wait_kind(&request);
                if matches!(kind, "control_available" | "control_gone")
                    && Self::control_name(&request).is_none()
                {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::WaitCondition,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: None,
                        message: Some(
                            "Name the control to wait for (for example Save).".into(),
                        ),
                        status: Some("need_control_name".into()),
                        target: request.query.clone(),
                        items: None,
                        monitors: None,
                    });
                }
                if matches!(kind, "window_available" | "window_active")
                    && request
                        .query
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .is_none()
                {
                    return Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::WaitCondition,
                        ok: false,
                        format: None,
                        bytes: None,
                        text: None,
                        preview: None,
                        message: Some("Which window should I wait for?".into()),
                        status: Some("need_window".into()),
                        target: None,
                        items: None,
                        monitors: None,
                    });
                }

                let timeout_ms = Self::wait_timeout_ms(&request);
                const POLL_MS: u64 = 50;
                let started = std::time::Instant::now();
                let deadline = started + std::time::Duration::from_millis(timeout_ms);
                let mut polls: u32 = 0;
                let mut met = false;
                loop {
                    polls = polls.saturating_add(1);
                    if self.wait_condition_met(&request, kind)? {
                        met = true;
                        break;
                    }
                    if std::time::Instant::now() >= deadline {
                        break;
                    }
                    let remaining = deadline.saturating_duration_since(std::time::Instant::now());
                    let sleep_for = std::time::Duration::from_millis(POLL_MS).min(remaining);
                    if sleep_for.is_zero() {
                        break;
                    }
                    std::thread::sleep(sleep_for);
                }
                let _elapsed_ms = started.elapsed().as_millis() as u64;

                let subject = Self::control_name(&request)
                    .map(|c| c.to_string())
                    .or_else(|| request.query.clone())
                    .unwrap_or_else(|| "condition".into());
                let resolved = self.resolve_one(&request)?;
                let target = resolved
                    .as_ref()
                    .map(|w| w.title.clone())
                    .or_else(|| request.query.clone());
                let items = resolved.as_ref().map(|w| vec![Self::item(w)]);

                if met {
                    Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::WaitCondition,
                        ok: true,
                        format: Some("wait_condition".into()),
                        bytes: Some(polls as usize),
                        text: Some(kind.into()),
                        preview: Some(text_preview(&subject, 80)),
                        message: Some(match kind {
                            "control_available" => {
                                format!("“{subject}” is available.")
                            }
                            "control_gone" => format!("“{subject}” is no longer available."),
                            "window_available" => format!("“{subject}” is available."),
                            "window_active" => format!("“{subject}” is active."),
                            _ => format!("Condition met for “{subject}”."),
                        }),
                        status: Some("condition_met".into()),
                        target,
                        items,
                        monitors: None,
                    })
                } else {
                    Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::WaitCondition,
                        ok: false,
                        format: Some("wait_condition".into()),
                        bytes: Some(polls as usize),
                        text: Some(kind.into()),
                        preview: Some(text_preview(&subject, 80)),
                        message: Some(format!(
                            "I waited {timeout_ms} ms, but “{subject}” never reached the expected state."
                        )),
                        status: Some("condition_timeout".into()),
                        target,
                        items,
                        monitors: None,
                    })
                }
            }
            CapabilityOperation::Enumerate => {
                let items: Vec<_> = self.windows()?.iter().map(Self::item).collect();
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::Enumerate,
                    ok: true,
                    format: Some("window_list".into()),
                    bytes: Some(items.len()),
                    text: None,
                    preview: Some(text_preview(
                        &items
                            .iter()
                            .take(8)
                            .map(|item| item.title.clone())
                            .collect::<Vec<_>>()
                            .join(", "),
                        160,
                    )),
                    message: Some(format!("{} window(s).", items.len())),
                    status: Some("enumerated".into()),
                    target: None,
                    items: Some(items),
                    monitors: None,
                })
            }
            CapabilityOperation::Find => {
                let matches = self.match_windows(&request)?;
                let items: Vec<_> = matches.iter().map(Self::item).collect();
                let status = if items.is_empty() {
                    "not_found"
                } else {
                    "found"
                };
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::Find,
                    ok: !items.is_empty(),
                    format: Some("window_list".into()),
                    bytes: Some(items.len()),
                    text: None,
                    preview: request.query.clone(),
                    message: Some(format!("{status}: {} match(es).", items.len())),
                    status: Some(status.into()),
                    target: request.query.clone(),
                    items: Some(items),
                    monitors: None,
                })
            }
            CapabilityOperation::Active => {
                let capture = self.ports.capturer.capture_desktop().map_err(|error| {
                    KernelError::WindowsIntegration {
                        message: error.to_string(),
                    }
                })?;
                let hwnd = capture.foreground_hwnd.clone();
                let window = hwnd.as_ref().and_then(|hwnd| {
                    capture
                        .windows
                        .iter()
                        .find(|w| &w.hwnd == hwnd)
                        .map(CapturedDesktopWindow::to_legacy_snapshot)
                });
                match window {
                    Some(window) => Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::Active,
                        ok: true,
                        format: Some("window".into()),
                        bytes: Some(1),
                        text: None,
                        preview: Some(text_preview(&window.title, 80)),
                        message: Some(format!("Active window: “{}”.", window.title)),
                        status: Some("active".into()),
                        target: Some(window.title.clone()),
                        items: Some(vec![Self::item(&window)]),
                        monitors: None,
                    }),
                    None => Ok(ProviderInvokeResponse {
                        domain: CapabilityDomainId::window(),
                        operation: CapabilityOperation::Active,
                        ok: false,
                        format: None,
                        bytes: Some(0),
                        text: None,
                        preview: None,
                        message: Some("No active window.".into()),
                        status: Some("not_found".into()),
                        target: None,
                        items: None,
                        monitors: None,
                    }),
                }
            }
            CapabilityOperation::Monitors => {
                let monitors: Vec<_> = self.monitors()?.iter().map(Self::monitor_item).collect();
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::Monitors,
                    ok: true,
                    format: Some("monitor_list".into()),
                    bytes: Some(monitors.len()),
                    text: None,
                    preview: Some(format!("{} monitor(s)", monitors.len())),
                    message: Some(format!("{} monitor(s) attached.", monitors.len())),
                    status: Some("enumerated".into()),
                    target: None,
                    items: None,
                    monitors: Some(monitors),
                })
            }
            CapabilityOperation::Bounds => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::Bounds, &request));
                };
                Ok(ProviderInvokeResponse {
                    domain: CapabilityDomainId::window(),
                    operation: CapabilityOperation::Bounds,
                    ok: true,
                    format: Some("bounds".into()),
                    bytes: Some(1),
                    text: Some(format!(
                        "{}x{} @ ({},{}) monitor={:?}",
                        window.width, window.height, window.x, window.y, window.monitor_index
                    )),
                    preview: Some(text_preview(&window.title, 80)),
                    message: Some(format!(
                        "“{}” — {}×{} at ({}, {}), monitor {:?}.",
                        window.title,
                        window.width,
                        window.height,
                        window.x,
                        window.y,
                        window.monitor_index
                    )),
                    status: Some("bounds".into()),
                    target: Some(window.title.clone()),
                    items: Some(vec![Self::item(&window)]),
                    monitors: None,
                })
            }
            CapabilityOperation::Focus => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::Focus, &request));
                };
                let outcome = self.ports.mutator.focus_window(&window.hwnd).map_err(|e| {
                    KernelError::WindowsIntegration {
                        message: e.to_string(),
                    }
                })?;
                Ok(Self::effect(
                    CapabilityOperation::Focus,
                    "focused",
                    &window,
                    outcome,
                    format!("Focused “{}”.", window.title),
                ))
            }
            CapabilityOperation::Minimize => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::Minimize, &request));
                };
                let outcome = self
                    .ports
                    .mutator
                    .minimize_window(&window.hwnd)
                    .map_err(|e| KernelError::WindowsIntegration {
                        message: e.to_string(),
                    })?;
                Ok(Self::effect(
                    CapabilityOperation::Minimize,
                    "minimized",
                    &window,
                    outcome,
                    format!("Minimized “{}”.", window.title),
                ))
            }
            CapabilityOperation::Restore => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::Restore, &request));
                };
                let outcome = self
                    .ports
                    .mutator
                    .restore_window(&window.hwnd)
                    .map_err(|e| KernelError::WindowsIntegration {
                        message: e.to_string(),
                    })?;
                Ok(Self::effect(
                    CapabilityOperation::Restore,
                    "restored",
                    &window,
                    outcome,
                    format!("Restored “{}”.", window.title),
                ))
            }
            CapabilityOperation::Maximize => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::Maximize, &request));
                };
                let outcome = self
                    .ports
                    .mutator
                    .maximize_window(&window.hwnd)
                    .map_err(|e| KernelError::WindowsIntegration {
                        message: e.to_string(),
                    })?;
                Ok(Self::effect(
                    CapabilityOperation::Maximize,
                    "maximized",
                    &window,
                    outcome,
                    format!("Maximized “{}”.", window.title),
                ))
            }
            CapabilityOperation::Move => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::Move, &request));
                };
                let monitors = self.monitors()?;
                let target_monitor = Self::monitor_for(
                    &monitors,
                    request.monitor_index,
                    window.monitor_index,
                );
                let (x, y) = if let (Some(x), Some(y)) = (request.x, request.y) {
                    (x, y)
                } else if let Some(monitor) = target_monitor {
                    if request.monitor_index.is_some() {
                        (monitor.work_x + 40, monitor.work_y + 40)
                    } else {
                        return Err(KernelError::CapabilityRuntime {
                            message: "window move requires x/y or monitorIndex".into(),
                        });
                    }
                } else {
                    return Err(KernelError::CapabilityRuntime {
                        message: "window move requires x/y or monitorIndex".into(),
                    });
                };
                let outcome = self.place(&window.hwnd, x, y, window.width, window.height)?;
                Ok(Self::effect(
                    CapabilityOperation::Move,
                    "moved",
                    &window,
                    outcome,
                    format!("Moved “{}” to ({x}, {y}).", window.title),
                ))
            }
            CapabilityOperation::Resize => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::Resize, &request));
                };
                let width = request.width.ok_or_else(|| KernelError::CapabilityRuntime {
                    message: "window resize requires width".into(),
                })?;
                let height = request.height.ok_or_else(|| KernelError::CapabilityRuntime {
                    message: "window resize requires height".into(),
                })?;
                if width < 100 || height < 80 {
                    return Err(KernelError::CapabilityRuntime {
                        message: "window resize minimum is 100×80".into(),
                    });
                }
                let outcome = self.place(&window.hwnd, window.x, window.y, width, height)?;
                Ok(Self::effect(
                    CapabilityOperation::Resize,
                    "resized",
                    &window,
                    outcome,
                    format!("Resized “{}” to {width}×{height}.", window.title),
                ))
            }
            CapabilityOperation::Center => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::Center, &request));
                };
                let monitors = self.monitors()?;
                let monitor = Self::monitor_for(&monitors, request.monitor_index, window.monitor_index)
                    .ok_or_else(|| KernelError::CapabilityRuntime {
                        message: "no monitor available for center".into(),
                    })?;
                let x = monitor.work_x + ((monitor.work_width - window.width) / 2).max(0);
                let y = monitor.work_y + ((monitor.work_height - window.height) / 2).max(0);
                let outcome = self.place(&window.hwnd, x, y, window.width, window.height)?;
                Ok(Self::effect(
                    CapabilityOperation::Center,
                    "centered",
                    &window,
                    outcome,
                    format!("Centered “{}” on monitor {}.", window.title, monitor.index),
                ))
            }
            CapabilityOperation::Snap => {
                let Some(window) = self.resolve_one(&request)? else {
                    return Ok(Self::not_found(CapabilityOperation::Snap, &request));
                };
                let edge = request
                    .snap
                    .as_deref()
                    .unwrap_or("left")
                    .trim()
                    .to_ascii_lowercase();
                let monitors = self.monitors()?;
                let monitor = Self::monitor_for(&monitors, request.monitor_index, window.monitor_index)
                    .ok_or_else(|| KernelError::CapabilityRuntime {
                        message: "no monitor available for snap".into(),
                    })?;
                let (x, y, width, height) = match edge.as_str() {
                    "right" => (
                        monitor.work_x + monitor.work_width / 2,
                        monitor.work_y,
                        monitor.work_width / 2,
                        monitor.work_height,
                    ),
                    "top" => (
                        monitor.work_x,
                        monitor.work_y,
                        monitor.work_width,
                        monitor.work_height / 2,
                    ),
                    "bottom" => (
                        monitor.work_x,
                        monitor.work_y + monitor.work_height / 2,
                        monitor.work_width,
                        monitor.work_height / 2,
                    ),
                    _ => (
                        monitor.work_x,
                        monitor.work_y,
                        monitor.work_width / 2,
                        monitor.work_height,
                    ),
                };
                let outcome = self.place(&window.hwnd, x, y, width.max(100), height.max(80))?;
                Ok(Self::effect(
                    CapabilityOperation::Snap,
                    "snapped",
                    &window,
                    outcome,
                    format!("Snapped “{}” {edge} on monitor {}.", window.title, monitor.index),
                ))
            }
            other => Err(KernelError::CapabilityRuntime {
                message: format!(
                    "WindowProvider does not own operation '{}'",
                    other.as_str()
                ),
            }),
        }
    }
}

#[cfg(test)]
mod wait_condition_tests {
    use super::*;
    use super::super::registry::CapabilityProvider;
    use std::sync::Arc;
    use std::time::Instant;
    use workspace_windows_integration::{
        FixtureWindowEnumerator, MemoryUiAutomationPort, StubDesktopCapturer, StubWindowMutator,
        UiControlSnapshot,
    };

    fn provider_with(ui: Arc<MemoryUiAutomationPort>) -> WindowProvider {
        WindowProvider::new(WindowPorts {
            enumerator: Arc::new(FixtureWindowEnumerator),
            mutator: Arc::new(StubWindowMutator::fixture_dual_monitor()),
            capturer: Arc::new(StubDesktopCapturer::fixture_dual_monitor()),
            ui_automation: ui,
        })
    }

    #[test]
    fn wait_condition_already_true() {
        let ui = Arc::new(MemoryUiAutomationPort::fixture());
        let provider = provider_with(ui);
        let started = Instant::now();
        let response = provider
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::WaitCondition,
                query: Some("Fixture Focus".into()),
                text: Some("Save".into()),
                category: Some("control_available".into()),
                duration: Some("500ms".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(response.ok);
        assert_eq!(response.status.as_deref(), Some("condition_met"));
        assert!(started.elapsed().as_millis() < 400);
    }

    #[test]
    fn wait_condition_becomes_true_after_delay() {
        let ui = Arc::new(MemoryUiAutomationPort::fixture());
        ui.remove_control_named("Later");
        ui.schedule_appear(
            UiControlSnapshot {
                name: "Later".into(),
                control_type: "Button".into(),
                automation_id: "Later".into(),
            },
            120,
        );
        let provider = provider_with(ui);
        let started = Instant::now();
        let response = provider
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::WaitCondition,
                query: Some("Fixture Focus".into()),
                text: Some("Later".into()),
                category: Some("control_available".into()),
                duration: Some("1000ms".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(response.ok);
        assert_eq!(response.status.as_deref(), Some("condition_met"));
        let elapsed = started.elapsed().as_millis();
        assert!(elapsed >= 100, "elapsed={elapsed}");
        assert!(elapsed < 900, "elapsed={elapsed}");
    }

    #[test]
    fn wait_condition_timeout_never_true() {
        let ui = Arc::new(MemoryUiAutomationPort::fixture());
        let provider = provider_with(ui);
        let started = Instant::now();
        let response = provider
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::WaitCondition,
                query: Some("Fixture Focus".into()),
                text: Some("NoSuchControlZZZ".into()),
                category: Some("control_available".into()),
                duration: Some("200ms".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(!response.ok);
        assert_eq!(response.status.as_deref(), Some("condition_timeout"));
        let elapsed = started.elapsed().as_millis();
        assert!(elapsed >= 180, "elapsed={elapsed}");
        assert!(elapsed < 600, "elapsed={elapsed} — no indefinite poll");
    }

    #[test]
    fn wait_condition_control_gone() {
        let ui = Arc::new(MemoryUiAutomationPort::fixture());
        ui.schedule_disappear("Save", 80);
        let provider = provider_with(ui);
        let response = provider
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::WaitCondition,
                query: Some("Fixture Focus".into()),
                text: Some("Save".into()),
                category: Some("control_gone".into()),
                duration: Some("800ms".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(response.ok);
        assert_eq!(response.status.as_deref(), Some("condition_met"));
    }

    #[test]
    fn wait_condition_window_available() {
        let ui = Arc::new(MemoryUiAutomationPort::fixture());
        let provider = provider_with(ui);
        let response = provider
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::WaitCondition,
                query: Some("Fixture Focus".into()),
                category: Some("window_available".into()),
                duration: Some("300ms".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(response.ok);
        assert_eq!(response.status.as_deref(), Some("condition_met"));
    }
}

/// P23.S6 — the owning application travels with the window it was observed on.
///
/// Windows reports the executable image basename during the same capture that
/// produces the title. These prove it survives the whole provider path instead
/// of being dropped on the way out, and that an unreported application stays
/// unreported — the only honest alternative is guessing from the title.
#[cfg(test)]
mod application_identity_tests {
    use super::super::registry::CapabilityProvider;
    use super::*;
    use std::sync::Arc;
    use workspace_windows_integration::{
        dual_monitor_fixture, DesktopObservationCapture, MemoryUiAutomationPort,
        StubDesktopCapturer, StubWindowMutator,
    };

    /// Enumerates whatever the capture holds, so both operations observe one desktop.
    struct CaptureEnumerator(DesktopObservationCapture);

    impl WindowEnumerator for CaptureEnumerator {
        fn enumerate_windows(
            &self,
        ) -> std::result::Result<
            Vec<DesktopWindowSnapshot>,
            workspace_windows_integration::WindowsIntegrationError,
        > {
            Ok(self.0.legacy_window_snapshots())
        }
    }

    /// One synthetic desktop: titles and applications that exist nowhere else.
    fn desktop(applications: [Option<&str>; 2]) -> DesktopObservationCapture {
        let mut capture = dual_monitor_fixture();
        let titles = ["Quokka Ledger", "Quokka Ledger"];
        for (index, window) in capture.windows.iter_mut().take(2).enumerate() {
            window.title = titles[index].into();
            window.process_name = applications[index].map(str::to_string);
        }
        capture
    }

    fn provider_observing(capture: DesktopObservationCapture) -> WindowProvider {
        WindowProvider::new(WindowPorts {
            enumerator: Arc::new(CaptureEnumerator(capture.clone())),
            mutator: Arc::new(StubWindowMutator::fixture_dual_monitor()),
            capturer: Arc::new(StubDesktopCapturer::new(capture)),
            ui_automation: Arc::new(MemoryUiAutomationPort::fixture()),
        })
    }

    fn invoke(provider: &WindowProvider, operation: CapabilityOperation) -> ProviderInvokeResponse {
        provider
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation,
                ..Default::default()
            })
            .unwrap()
    }

    #[test]
    fn active_window_carries_the_observed_application() {
        let provider =
            provider_observing(desktop([Some("quokka-editor.exe"), Some("zarnak.exe")]));
        let response = invoke(&provider, CapabilityOperation::Active);

        let items = response.items.expect("active window is an item");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].process_name.as_deref(), Some("quokka-editor.exe"));
        assert_eq!(items[0].title, "Quokka Ledger");
    }

    #[test]
    fn enumerated_windows_carry_the_observed_application() {
        let provider =
            provider_observing(desktop([Some("quokka-editor.exe"), Some("zarnak.exe")]));
        let response = invoke(&provider, CapabilityOperation::Enumerate);

        let items = response.items.expect("enumeration returns items");
        let observed: Vec<_> = items
            .iter()
            .take(2)
            .map(|item| item.process_name.as_deref())
            .collect();
        assert_eq!(observed, vec![Some("quokka-editor.exe"), Some("zarnak.exe")]);
    }

    #[test]
    fn identical_titles_stay_distinguishable_by_application() {
        let provider =
            provider_observing(desktop([Some("quokka-editor.exe"), Some("zarnak.exe")]));
        let items = invoke(&provider, CapabilityOperation::Enumerate)
            .items
            .expect("enumeration returns items");

        assert_eq!(items[0].title, items[1].title);
        assert_ne!(items[0].process_name, items[1].process_name);
    }

    #[test]
    fn an_unreported_application_is_never_filled_in_from_the_title() {
        let provider = provider_observing(desktop([None, None]));

        for operation in [CapabilityOperation::Active, CapabilityOperation::Enumerate] {
            let items = invoke(&provider, operation).items.expect("items");
            assert!(
                items.iter().all(|item| item.process_name.is_none()),
                "{operation:?} invented an application from the window title"
            );
        }
    }
}
