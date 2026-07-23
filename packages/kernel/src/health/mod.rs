use serde::{Deserialize, Serialize};

use crate::lifecycle::LifecycleState;
use crate::services::ServiceRegistry;

/// Runtime health snapshot exposed through IPC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceHealth {
    pub status: String,
    pub initialized: bool,
    pub services: Vec<String>,
    pub version: String,
}

impl WorkspaceHealth {
    pub fn from_runtime(
        lifecycle: LifecycleState,
        version: impl Into<String>,
        services: &ServiceRegistry,
    ) -> Self {
        Self {
            status: lifecycle.as_str().to_string(),
            initialized: lifecycle.is_initialized(),
            services: services.healthy_service_names(),
            version: version.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{ServiceRegistry, ServiceStatus};

    #[test]
    fn reports_ready_health() {
        let mut services = ServiceRegistry::new();
        services.register("database", ServiceStatus::Healthy);
        services.register("configuration", ServiceStatus::Healthy);

        let health = WorkspaceHealth::from_runtime(
            LifecycleState::Ready,
            "0.1.0",
            &services,
        );

        assert_eq!(health.status, "ready");
        assert!(health.initialized);
        assert_eq!(health.services, vec!["database", "configuration"]);
    }
}
