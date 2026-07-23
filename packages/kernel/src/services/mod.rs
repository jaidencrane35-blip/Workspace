//! Foundation service ownership for the Platform Kernel.

mod analytics;
mod application;
mod audit;
mod configuration;
mod context;
mod database;
mod discovery;
mod graph;
mod layout;
mod observation;
mod projection;
mod registry;
mod suggestion;
mod widget;
mod workspace;
mod zone;

pub use analytics::WorkspaceAnalyticsService;
pub use application::ApplicationService;
pub use audit::AuditService;
pub use configuration::ConfigurationService;
pub use context::WorkspaceContextService;
pub use discovery::CapabilityResolver;
pub use database::DatabaseServiceHandle;
pub use graph::GraphService;
pub use layout::LayoutService;
pub use observation::ObservationService;
pub use projection::WorkspaceProjectionService;
pub use registry::{RegisteredService, ServiceRegistry, ServiceStatus};
pub use suggestion::SuggestionService;
pub use widget::WidgetService;
pub use workspace::WorkspaceService;
pub use zone::ZoneService;
