//! Foundation service ownership for the Platform Kernel.

mod configuration;
mod database;
mod registry;
mod workspace;

pub use configuration::ConfigurationService;
pub use database::DatabaseServiceHandle;
pub use registry::{RegisteredService, ServiceRegistry, ServiceStatus};
pub use workspace::WorkspaceService;
