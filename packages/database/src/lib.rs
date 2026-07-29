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
pub use encryption::{EncryptionProvider, EncryptionTier, NoOpEncryptionProvider};
pub use error::DatabaseError;
pub use init::{bundled_migrations_dir, DatabaseService};
pub use migration::MigrationRunner;
pub use repositories::{
    AiMemoryRepository, ApplicationRepository, AuditRepository, AutomationContractRepository,
    AutomationTriggerRepository, CognitiveAgentCastRepository, CognitiveAutonomyRepository,
    CognitiveGraphRepository, CognitiveModelRepository, CognitiveOrchestrationRepository,
    DecisionEngineRepository, DecisionQueueRepository, ExecutionLifecycleRepository,
    GraphRepository, HistoricalReconstructionRepository, InsightCoordinationRepository,
    LayoutRepository, LearningAdaptationRepository, ObservationMonitorRepository,
    ObservationPassRepository, ObservationWindowIdentityRepository, ObservationWindowRepository,
    PermissionApprovalRepository, PolicyGovernanceRepository, ReasoningMemoryRepository,
    RecommendationLifecycleRepository, TaskGraphRepository, TemporalIntelligenceRepository,
    UserPreferenceRepository, WidgetRepository, WorkspaceAssistantContextRepository,
    WorkspaceAssistantRetrievalRepository,
    WorkspaceAssistantSurfaceRepository,
    WorkspaceContextualUnderstandingRepository, WorkspaceCrossIntelligenceRepository,
    WorkspaceDecisionSupportRepository, WorkspaceEvidenceCompletenessRepository,
    WorkspaceEvidenceConsistencyRepository, WorkspaceEvidenceCoverageRepository,
    WorkspaceEvidenceDependencyRepository, WorkspaceEvidenceFreshnessRepository,
    WorkspaceEvidenceNavigationRepository, WorkspaceEvidenceReliabilityRepository,
    WorkspaceEvidenceTraceRepository, WorkspaceExplanationRepository,
    WorkspaceIntelligenceHubRepository, WorkspaceIntentRepository,
    WorkspaceKnowledgeIntegrationRepository, WorkspaceKnowledgeSynthesisRepository,
    WorkspacePlanningRepository, WorkspaceProfileRepository, WorkspaceRepository,
    WorkspaceSemanticQueryRepository, WorkspaceStateEnvelopeRepository, ZoneRepository,
};
pub use settings::SettingsRepository;
pub use transaction::Transaction;
