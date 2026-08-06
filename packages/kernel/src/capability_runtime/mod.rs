//! Capability Runtime Foundation (P10).
//!
//! Permanent pipeline:
//! Conversation → Intent Layer → Capability Router → Provider Registry →
//! Capability Provider → Desktop Service → Conversation Response.
//!
//! No capability may bypass this pipeline. Providers never become product identity.

mod clipboard_provider;
mod registry;
mod router;
mod types;

pub use clipboard_provider::ClipboardProvider;
pub use registry::ProviderRegistry;
pub use router::CapabilityRouter;
pub use types::{
    CapabilityDomainId, CapabilityOperation, ProviderDescriptor, ProviderInvokeRequest,
    ProviderInvokeResponse, ProviderResultSummary,
};

use std::sync::{Arc, OnceLock, RwLock};

use workspace_windows_integration::{ClipboardPort, MemoryClipboard};

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

static RUNTIME: OnceLock<CapabilityRuntime> = OnceLock::new();

/// Shared Capability Runtime for the process.
pub fn runtime() -> &'static CapabilityRuntime {
    RUNTIME.get_or_init(CapabilityRuntime::bootstrap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_registers_clipboard_provider() {
        let descriptors = runtime().list_providers().unwrap();
        assert!(descriptors.iter().any(|d| d.domain.as_str() == "clipboard"));
    }

    #[test]
    fn clipboard_write_then_read_through_router() {
        let write = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::clipboard(),
                operation: CapabilityOperation::Write,
                text: Some("p10-runtime".into()),
            })
            .unwrap();
        assert!(write.ok);
        assert_eq!(write.bytes, Some(11));

        let read = runtime()
            .invoke(ProviderInvokeRequest {
                domain: CapabilityDomainId::clipboard(),
                operation: CapabilityOperation::Read,
                text: None,
            })
            .unwrap();
        assert!(read.ok);
        assert_eq!(read.text.as_deref(), Some("p10-runtime"));
        assert_eq!(read.format.as_deref(), Some("text"));
    }
}
