use super::registry::ProviderRegistry;
use super::types::{ProviderInvokeRequest, ProviderInvokeResponse};
use crate::error::{KernelError, Result};

/// Routes Intent-derived requests to the registered Capability Provider.
///
/// The router never performs desktop effects itself.
pub struct CapabilityRouter;

impl CapabilityRouter {
    pub fn route(
        registry: &ProviderRegistry,
        request: ProviderInvokeRequest,
    ) -> Result<ProviderInvokeResponse> {
        let domain = request.domain.clone();
        let provider = registry.get(&domain).ok_or_else(|| {
            KernelError::CapabilityRuntime {
                message: format!(
                    "no provider registered for domain '{}'",
                    domain.as_str()
                ),
            }
        })?;
        provider.invoke(request)
    }
}
