use serde::{Deserialize, Serialize};

/// Health of an individual foundation service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStatus {
    Healthy,
    Failed,
}

/// A registered runtime service tracked by the kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredService {
    pub name: &'static str,
    pub status: ServiceStatus,
}

/// Named service registry for foundation services (Sprint 03).
#[derive(Debug, Clone, Default)]
pub struct ServiceRegistry {
    services: Vec<RegisteredService>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: &'static str, status: ServiceStatus) {
        if let Some(existing) = self.services.iter_mut().find(|service| service.name == name) {
            existing.status = status;
            return;
        }

        self.services.push(RegisteredService { name, status });
    }

    pub fn mark_failed(&mut self, name: &'static str) {
        self.register(name, ServiceStatus::Failed);
    }

    pub fn all(&self) -> &[RegisteredService] {
        &self.services
    }

    pub fn healthy_service_names(&self) -> Vec<String> {
        self.services
            .iter()
            .filter(|service| service.status == ServiceStatus::Healthy)
            .map(|service| service.name.to_string())
            .collect()
    }

    pub fn all_healthy(&self) -> bool {
        !self.services.is_empty()
            && self
                .services
                .iter()
                .all(|service| service.status == ServiceStatus::Healthy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_and_tracks_service_health() {
        let mut registry = ServiceRegistry::new();
        registry.register("database", ServiceStatus::Healthy);
        registry.register("configuration", ServiceStatus::Healthy);

        assert!(registry.all_healthy());
        assert_eq!(
            registry.healthy_service_names(),
            vec!["database", "configuration"]
        );

        registry.mark_failed("database");
        assert!(!registry.all_healthy());
        assert_eq!(registry.healthy_service_names(), vec!["configuration"]);
    }
}
