//! Capability Runtime — P10–P14 providers (Clipboard, Application, Window, Notifications, Browser).
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
mod types;
mod window_provider;

pub use application_provider::{ApplicationPorts, ApplicationProvider};
pub use browser_provider::BrowserProvider;
pub use clipboard_provider::ClipboardProvider;
pub use notification_provider::NotificationProvider;
pub use registry::ProviderRegistry;
pub use router::CapabilityRouter;
pub use types::{
    ApplicationWindowItem, CapabilityDomainId, CapabilityOperation, MonitorItem,
    ProviderDescriptor, ProviderInvokeRequest, ProviderInvokeResponse, ProviderResultSummary,
};
pub use window_provider::{WindowPorts, WindowProvider};

use std::sync::{Arc, OnceLock, RwLock};

use workspace_windows_integration::{
    platform_desktop_capturer, platform_process_launcher, platform_window_enumerator,
    platform_window_mutator, BrowserPort, ClipboardPort, NotificationPort,
};
#[cfg(test)]
use workspace_windows_integration::{
    FixtureWindowEnumerator, MemoryBrowserPort, MemoryClipboard, MemoryNotificationPort,
    StubDesktopCapturer, StubProcessLauncher, StubWindowMutator,
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
        }
    }
    #[cfg(not(test))]
    {
        WindowPorts {
            enumerator: Arc::from(platform_window_enumerator()),
            mutator: Arc::from(platform_window_mutator()),
            capturer: Arc::from(platform_desktop_capturer()),
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
}
