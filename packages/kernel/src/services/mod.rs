//! Foundation service ownership for the Platform Kernel.

mod application;
mod audit;
mod configuration;
mod database;
mod graph;
mod layout;
mod registry;
mod widget;
mod workspace;
mod zone;

pub use application::ApplicationService;
pub use audit::AuditService;
pub use configuration::ConfigurationService;
pub use database::DatabaseServiceHandle;
pub use graph::GraphService;
pub use layout::LayoutService;
pub use registry::{RegisteredService, ServiceRegistry, ServiceStatus};
pub use widget::WidgetService;
pub use workspace::WorkspaceService;
pub use zone::ZoneService;
