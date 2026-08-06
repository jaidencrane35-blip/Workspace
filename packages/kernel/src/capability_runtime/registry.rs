use super::types::{CapabilityDomainId, ProviderDescriptor, ProviderInvokeRequest, ProviderInvokeResponse};
use crate::error::{KernelError, Result};

/// One capability domain → one provider implementation.
pub trait CapabilityProvider: Send + Sync {
    fn descriptor(&self) -> ProviderDescriptor;
    fn invoke(&self, request: ProviderInvokeRequest) -> Result<ProviderInvokeResponse>;
}

/// Workspace-owned registry of capability providers.
#[derive(Default)]
pub struct ProviderRegistry {
    providers: Vec<Box<dyn CapabilityProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, provider: Box<dyn CapabilityProvider>) -> Result<()> {
        let domain = provider.descriptor().domain;
        if self
            .providers
            .iter()
            .any(|existing| existing.descriptor().domain == domain)
        {
            return Err(KernelError::CapabilityRuntime {
                message: format!("provider already registered for domain '{}'", domain.as_str()),
            });
        }
        self.providers.push(provider);
        Ok(())
    }

    pub fn get(&self, domain: &CapabilityDomainId) -> Option<&dyn CapabilityProvider> {
        self.providers
            .iter()
            .find(|provider| &provider.descriptor().domain == domain)
            .map(|provider| provider.as_ref())
    }

    pub fn list(&self) -> Vec<ProviderDescriptor> {
        self.providers
            .iter()
            .map(|provider| provider.descriptor())
            .collect()
    }
}
