//! Foundation service ownership for the Platform Kernel.

mod configuration;
mod database;
mod registry;

pub use configuration::ConfigurationService;
pub use database::DatabaseServiceHandle;
pub use registry::{RegisteredService, ServiceRegistry, ServiceStatus};
