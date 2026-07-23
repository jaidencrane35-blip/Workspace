//! Service registration pattern for future Platform Kernel services.
//!
//! Sprint 02: registry placeholder only. Domain services are added in later sprints.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Type-keyed service registry for dependency injection within the kernel.
#[derive(Default)]
pub struct ServiceRegistry {
    services: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T: Send + Sync + 'static>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|service| service.downcast_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ExampleService {
        value: u32,
    }

    #[test]
    fn registers_and_retrieves_services() {
        let mut registry = ServiceRegistry::new();
        registry.register(ExampleService { value: 42 });

        let service = registry.get::<ExampleService>().unwrap();
        assert_eq!(service.value, 42);
    }
}
