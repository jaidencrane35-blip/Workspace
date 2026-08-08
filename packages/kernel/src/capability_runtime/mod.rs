//! Capability Runtime — P10–P15 providers (Clipboard, Application, Window, Notifications, Browser, Screenshot).
//!
//! Permanent pipeline:
//! Conversation → Intent Layer → execute_capability_intent → Kernel Operator →
//! Capability Runtime → Capability Router → Provider Registry →
//! Capability Provider → Desktop Service → Kernel Operator (response) →
//! Conversation Response.
//!
//! Providers own **operations**, not isolated features.
//! Providers never call each other — only the Capability Runtime routes.
//! Conversation never invokes providers — Kernel Authority / Presentation Purity.

mod application_provider;
mod browser_provider;
mod clipboard_provider;
mod notification_provider;
mod registry;
mod router;
mod screenshot_provider;
mod types;
mod window_provider;

pub use application_provider::{
    application_query_is_launchable, ApplicationPorts, ApplicationProvider,
};
pub use browser_provider::BrowserProvider;
pub use clipboard_provider::ClipboardProvider;
pub use notification_provider::NotificationProvider;
pub use registry::ProviderRegistry;
pub use router::CapabilityRouter;
pub use screenshot_provider::ScreenshotProvider;
pub use types::{
    ApplicationWindowItem, CapabilityDomainId, CapabilityOperation, MonitorItem,
    ProviderDescriptor, ProviderInvokeRequest, ProviderInvokeResponse, ProviderResultSummary,
};
pub use window_provider::{WindowPorts, WindowProvider};

use std::sync::{Arc, OnceLock, RwLock};

use workspace_windows_integration::{
    platform_desktop_capturer, platform_process_launcher, platform_ui_automation,
    platform_window_enumerator, platform_window_mutator, BrowserPort, ClipboardPort,
    NotificationPort, ScreenshotPort,
};
#[cfg(test)]
use workspace_windows_integration::{
    FixtureWindowEnumerator, MemoryBrowserPort, MemoryClipboard, MemoryNotificationPort,
    MemoryScreenshotPort, MemoryUiAutomationPort, StubDesktopCapturer, StubProcessLauncher,
    StubWindowMutator,
};

use crate::error::{KernelError, Result};

/// Process-wide Capability Runtime (registry + router).
pub struct CapabilityRuntime {
    registry: RwLock<ProviderRegistry>,
}

impl CapabilityRuntime {
    fn bootstrap() -> Self {
        let mut registry = ProviderRegistry::new();
        registry
            .register(Box::new(ClipboardProvider::new(clipboard_port())))
            .expect("clipboard provider registers once at bootstrap");
        registry
            .register(Box::new(ApplicationProvider::new(application_ports())))
            .expect("application provider registers once at bootstrap");
        registry
            .register(Box::new(WindowProvider::new(window_ports())))
            .expect("window provider registers once at bootstrap");
        registry
            .register(Box::new(NotificationProvider::new(notification_port())))
            .expect("notification provider registers once at bootstrap");
        registry
            .register(Box::new(BrowserProvider::new(browser_port())))
            .expect("browser provider registers once at bootstrap");
        registry
            .register(Box::new(ScreenshotProvider::new(screenshot_port())))
            .expect("screenshot provider registers once at bootstrap");
        Self {
            registry: RwLock::new(registry),
        }
    }

    pub fn invoke(&self, request: ProviderInvokeRequest) -> Result<ProviderInvokeResponse> {
        let registry = self.registry.read().map_err(|_| {
            KernelError::CapabilityRuntime {
                message: "provider registry lock poisoned".into(),
            }
        })?;
        CapabilityRouter::route(&registry, request)
    }

    pub fn list_providers(&self) -> Result<Vec<ProviderDescriptor>> {
        let registry = self.registry.read().map_err(|_| {
            KernelError::CapabilityRuntime {
                message: "provider registry lock poisoned".into(),
            }
        })?;
        Ok(registry.list())
    }
}

fn clipboard_port() -> Arc<dyn ClipboardPort> {
    #[cfg(test)]
    {
        Arc::new(MemoryClipboard::new())
    }
    #[cfg(not(test))]
    {
        workspace_windows_integration::platform_clipboard()
    }
}

fn application_ports() -> ApplicationPorts {
    #[cfg(test)]
    {
        ApplicationPorts {
            launcher: Arc::new(StubProcessLauncher),
            enumerator: Arc::new(FixtureWindowEnumerator),
            mutator: Arc::new(StubWindowMutator::fixture_dual_monitor()),
        }
    }
    #[cfg(not(test))]
    {
        ApplicationPorts {
            launcher: Arc::from(platform_process_launcher()),
            enumerator: Arc::from(platform_window_enumerator()),
            mutator: Arc::from(platform_window_mutator()),
        }
    }
}

fn window_ports() -> WindowPorts {
    #[cfg(test)]
    {
        WindowPorts {
            enumerator: Arc::new(FixtureWindowEnumerator),
            mutator: Arc::new(StubWindowMutator::fixture_dual_monitor()),
            capturer: Arc::new(StubDesktopCapturer::fixture_dual_monitor()),
            ui_automation: Arc::new(MemoryUiAutomationPort::fixture()),
        }
    }
    #[cfg(not(test))]
    {
        WindowPorts {
            enumerator: Arc::from(platform_window_enumerator()),
            mutator: Arc::from(platform_window_mutator()),
            capturer: Arc::from(platform_desktop_capturer()),
            ui_automation: Arc::from(platform_ui_automation()),
        }
    }
}

fn notification_port() -> Arc<dyn NotificationPort> {
    #[cfg(test)]
    {
        Arc::new(MemoryNotificationPort::new())
    }
    #[cfg(not(test))]
    {
        workspace_windows_integration::platform_notification()
    }
}

fn browser_port() -> Arc<dyn BrowserPort> {
    #[cfg(test)]
    {
        Arc::new(MemoryBrowserPort::new())
    }
    #[cfg(not(test))]
    {
        workspace_windows_integration::platform_browser()
    }
}

fn screenshot_port() -> Arc<dyn ScreenshotPort> {
    #[cfg(test)]
    {
        Arc::new(MemoryScreenshotPort::new())
    }
    #[cfg(not(test))]
    {
        workspace_windows_integration::platform_screenshot()
    }
}

static RUNTIME: OnceLock<CapabilityRuntime> = OnceLock::new();

/// Shared Capability Runtime for the process.
pub fn runtime() -> &'static CapabilityRuntime {
    RUNTIME.get_or_init(CapabilityRuntime::bootstrap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_registers_core_providers() {
        let descriptors = runtime().list_providers().unwrap();
        assert!(descriptors.iter().any(|d| d.domain.as_str() == "clipboard"));
        assert!(descriptors
            .iter()
            .any(|d| d.domain.as_str() == "application"));
        assert!(descriptors.iter().any(|d| d.domain.as_str() == "window"));
        assert!(descriptors
            .iter()
            .any(|d| d.domain.as_str() == "notifications"));
        assert!(descriptors.iter().any(|d| d.domain.as_str() == "browser"));
        assert!(descriptors
            .iter()
            .any(|d| d.domain.as_str() == "screenshots"));
    }

    #[test]
    fn clipboard_write_then_read_through_router() {
        let write = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::clipboard(),
                operation: CapabilityOperation::Write,
                text: Some("p10-runtime".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(write.ok);

        let read = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::clipboard(),
                operation: CapabilityOperation::Read,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(read.text.as_deref(), Some("p10-runtime"));
    }

    #[test]
    fn application_launch_uses_alias_through_router() {
        let launched = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::application(),
                operation: CapabilityOperation::Launch,
                query: Some("notepad".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(launched.ok);
        assert_eq!(launched.target.as_deref(), Some("notepad.exe"));
    }

    #[test]
    fn window_enumerate_and_snap_through_router() {
        let listed = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::Enumerate,
                ..Default::default()
            })
            .unwrap();
        assert!(listed.ok);
        assert!(listed.items.as_ref().map(|i| !i.is_empty()).unwrap_or(false));

        let snapped = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::Snap,
                query: Some("Fixture Focus".into()),
                snap: Some("left".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(snapped.ok);
        assert_eq!(snapped.status.as_deref(), Some("snapped"));
    }

    #[test]
    fn window_enumerate_controls_through_router() {
        let listed = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::EnumerateControls,
                query: Some("Fixture Focus".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(listed.ok);
        assert_eq!(listed.status.as_deref(), Some("enumerated_controls"));
        assert_eq!(listed.format.as_deref(), Some("control_list"));
        assert!(listed.bytes.unwrap_or(0) >= 3);
        assert!(listed
            .text
            .as_ref()
            .is_some_and(|t| t.contains("File") && t.contains("Save")));
    }

    #[test]
    fn window_find_control_through_router() {
        let found = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::FindControl,
                query: Some("Fixture Focus".into()),
                text: Some("Save".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(found.ok);
        assert_eq!(found.status.as_deref(), Some("control_found"));
        assert!(found.text.as_ref().is_some_and(|t| t.contains("Save")));

        let missing = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::FindControl,
                query: Some("Fixture Focus".into()),
                text: Some("NoSuchControlZZZ".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(!missing.ok);
        assert_eq!(missing.status.as_deref(), Some("control_not_found"));
    }

    #[test]
    fn window_invoke_and_set_control_through_router() {
        let clicked = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::InvokeControl,
                query: Some("Fixture Focus".into()),
                text: Some("Save".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(clicked.ok);
        assert_eq!(clicked.status.as_deref(), Some("control_clicked"));

        let typed = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::SetControlValue,
                query: Some("Fixture Focus".into()),
                title: Some("Edit".into()),
                text: Some("hello workspace".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(typed.ok);
        assert_eq!(typed.status.as_deref(), Some("control_typed"));
        assert_eq!(typed.text.as_deref(), Some("hello workspace"));

        let bad_type = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::SetControlValue,
                query: Some("Fixture Focus".into()),
                title: Some("Save".into()),
                text: Some("nope".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(!bad_type.ok);
        assert_eq!(bad_type.status.as_deref(), Some("control_type_failed"));
    }

    #[test]
    fn window_wait_condition_through_router() {
        let met = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::WaitCondition,
                query: Some("Fixture Focus".into()),
                text: Some("Save".into()),
                category: Some("control_available".into()),
                duration: Some("400ms".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(met.ok);
        assert_eq!(met.status.as_deref(), Some("condition_met"));

        let timed_out = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::WaitCondition,
                query: Some("Fixture Focus".into()),
                text: Some("NoSuchControlZZZ".into()),
                category: Some("control_available".into()),
                duration: Some("150ms".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(!timed_out.ok);
        assert_eq!(timed_out.status.as_deref(), Some("condition_timeout"));
    }

    #[test]
    fn notifications_status_and_show_through_router() {
        let status = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::notifications(),
                operation: CapabilityOperation::Status,
                ..Default::default()
            })
            .unwrap();
        assert!(status.ok);

        let shown = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::notifications(),
                operation: CapabilityOperation::Show,
                title: Some("Workspace".into()),
                text: Some("P13 proof".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(shown.ok);
        assert_eq!(shown.status.as_deref(), Some("shown"));
    }

    #[test]
    fn window_monitors_level1_through_router() {
        let monitors = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::window(),
                operation: CapabilityOperation::Monitors,
                ..Default::default()
            })
            .unwrap();
        assert!(monitors.ok);
        assert!(monitors
            .monitors
            .as_ref()
            .map(|m| m.len() >= 2)
            .unwrap_or(false));
    }

    #[test]
    fn browser_status_and_open_through_router() {
        let status = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::browser(),
                operation: CapabilityOperation::Status,
                ..Default::default()
            })
            .unwrap();
        assert!(status.ok);
        assert_eq!(status.status.as_deref(), Some("available"));

        let opened = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::browser(),
                operation: CapabilityOperation::Open,
                path: Some("https://example.com".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(opened.ok);
        assert_eq!(opened.status.as_deref(), Some("opened"));
        assert_eq!(opened.target.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn screenshots_status_and_capture_through_router() {
        let status = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::screenshots(),
                operation: CapabilityOperation::Status,
                ..Default::default()
            })
            .unwrap();
        assert!(status.ok);
        assert_eq!(status.status.as_deref(), Some("available"));

        let shot = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::screenshots(),
                operation: CapabilityOperation::CaptureDesktop,
                ..Default::default()
            })
            .unwrap();
        assert!(shot.ok);
        assert!(shot.text.as_ref().is_some_and(|p| !p.is_empty()));

        let copied = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::screenshots(),
                operation: CapabilityOperation::CopyClipboard,
                path: shot.text.clone(),
                ..Default::default()
            })
            .unwrap();
        assert!(copied.ok);
        assert_eq!(copied.status.as_deref(), Some("copied"));
    }
}
