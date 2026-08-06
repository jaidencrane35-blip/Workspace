//! Capability Runtime Foundation (P10) + Application Provider (P11).
//!
//! Permanent pipeline:
//! Conversation → Intent Layer → Capability Router → Provider Registry →
//! Capability Provider → Desktop Service → Conversation Response.
//!
//! Providers own **operations**, not isolated features.
//! No capability may bypass this pipeline. Providers never become product identity.

mod application_provider;
mod clipboard_provider;
mod registry;
mod router;
mod types;

pub use application_provider::{ApplicationPorts, ApplicationProvider};
pub use clipboard_provider::ClipboardProvider;
pub use registry::ProviderRegistry;
pub use router::CapabilityRouter;
pub use types::{
    ApplicationWindowItem, CapabilityDomainId, CapabilityOperation, ProviderDescriptor,
    ProviderInvokeRequest, ProviderInvokeResponse, ProviderResultSummary,
};

use std::sync::{Arc, OnceLock, RwLock};

use workspace_windows_integration::{
    platform_process_launcher, platform_window_enumerator, platform_window_mutator, ClipboardPort,
};
#[cfg(test)]
use workspace_windows_integration::{
    FixtureWindowEnumerator, MemoryClipboard, StubProcessLauncher, StubWindowMutator,
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

static RUNTIME: OnceLock<CapabilityRuntime> = OnceLock::new();

/// Shared Capability Runtime for the process.
pub fn runtime() -> &'static CapabilityRuntime {
    RUNTIME.get_or_init(CapabilityRuntime::bootstrap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_registers_clipboard_and_application_providers() {
        let descriptors = runtime().list_providers().unwrap();
        assert!(descriptors.iter().any(|d| d.domain.as_str() == "clipboard"));
        assert!(descriptors
            .iter()
            .any(|d| d.domain.as_str() == "application"));
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
        assert_eq!(write.bytes, Some(11));

        let read = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::clipboard(),
                operation: CapabilityOperation::Read,
                ..Default::default()
            })
            .unwrap();
        assert!(read.ok);
        assert_eq!(read.text.as_deref(), Some("p10-runtime"));
    }

    #[test]
    fn application_enumerate_and_focus_through_router() {
        let listed = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::application(),
                operation: CapabilityOperation::Enumerate,
                ..Default::default()
            })
            .unwrap();
        assert!(listed.ok);
        assert!(listed.items.as_ref().map(|items| !items.is_empty()).unwrap_or(false));

        let focused = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::application(),
                operation: CapabilityOperation::Focus,
                query: Some("Fixture Focus".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(focused.ok);
        assert_eq!(focused.status.as_deref(), Some("focused"));
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
        assert_eq!(launched.status.as_deref(), Some("launched_simulated"));
    }
}
