mod ai_memory;
mod application;
mod audit;
mod automation_contract;
mod automation_trigger;
mod cognitive_model;
mod decision_engine;
mod decision_queue;
mod execution_lifecycle;
mod recommendation_lifecycle;
mod task_graph;
mod workspace_planning;
mod reasoning_memory;
mod cognitive_graph;
mod graph;
mod layout;
mod permission_approval;
mod user_preference;
mod widget;
mod workspace;
mod workspace_intent;
mod workspace_profile;
mod observation;
mod zone;
#[cfg(test)]
mod boundary_tests;

pub use ai_memory::AiMemoryRepository;
pub use application::ApplicationRepository;
pub use audit::AuditRepository;
pub use automation_contract::AutomationContractRepository;
pub use automation_trigger::AutomationTriggerRepository;
pub use cognitive_model::CognitiveModelRepository;
pub use decision_engine::DecisionEngineRepository;
pub use decision_queue::DecisionQueueRepository;
pub use execution_lifecycle::ExecutionLifecycleRepository;
pub use recommendation_lifecycle::RecommendationLifecycleRepository;
pub use task_graph::TaskGraphRepository;
pub use workspace_planning::WorkspacePlanningRepository;
pub use reasoning_memory::ReasoningMemoryRepository;
pub use cognitive_graph::CognitiveGraphRepository;
pub use graph::GraphRepository;
pub use layout::LayoutRepository;
pub use permission_approval::PermissionApprovalRepository;
pub use user_preference::UserPreferenceRepository;
pub use widget::WidgetRepository;
pub use workspace::WorkspaceRepository;
pub use workspace_intent::WorkspaceIntentRepository;
pub use workspace_profile::WorkspaceProfileRepository;
pub use observation::{
    ObservationMonitorRepository, ObservationPassRepository, ObservationWindowIdentityRepository,
    ObservationWindowRepository,
};
pub use zone::ZoneRepository;
