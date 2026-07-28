//! Workspace SQLite persistence foundation.

pub mod connection;
pub mod encryption;
pub mod error;
pub mod init;
pub mod migration;
pub mod repositories;
pub mod settings;
pub mod transaction;

pub use connection::Database;
pub use transaction::Transaction;
pub use encryption::{EncryptionProvider, EncryptionTier, NoOpEncryptionProvider};
pub use error::DatabaseError;
pub use init::{bundled_migrations_dir, DatabaseService};
pub use migration::MigrationRunner;
pub use repositories::{
    AiMemoryRepository, ApplicationRepository, AuditRepository, AutomationContractRepository,
    AutomationTriggerRepository, CognitiveModelRepository, DecisionEngineRepository,
    DecisionQueueRepository, GraphRepository, ExecutionLifecycleRepository, LayoutRepository,
    PermissionApprovalRepository, RecommendationLifecycleRepository, TaskGraphRepository,
    UserPreferenceRepository, WidgetRepository, WorkspaceProfileRepository,
    ObservationMonitorRepository, ObservationPassRepository, ObservationWindowIdentityRepository,
    ObservationWindowRepository, WorkspacePlanningRepository, ReasoningMemoryRepository,
    CognitiveGraphRepository, CognitiveOrchestrationRepository, LearningAdaptationRepository,
    CognitiveAgentCastRepository, CognitiveAutonomyRepository,
    WorkspaceRepository, WorkspaceIntentRepository, ZoneRepository,
};
pub use settings::SettingsRepository;
