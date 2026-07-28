//! Workspace Platform Kernel — core runtime boundary.
//!
//! Owns application lifecycle, state, configuration, service registration,
//! internal events, command pipeline, and permission boundaries.

pub mod commands;
pub mod config;
pub mod error;
pub mod events;
pub mod health;
pub mod intent;
pub mod lifecycle;
pub mod policy;
pub mod security;
pub mod services;
pub mod state;

pub use commands::CommandHandler;
pub use config::{ConfigManager, SettingsUpdate, WorkspaceSettings};
pub use error::{KernelError, PublicError, Result};
pub use events::{DomainEvent, EventBus};
pub use health::WorkspaceHealth;
pub use intent::{CommandIntentMapping, ActionIntentValidationService};
pub use lifecycle::LifecycleState;
pub use policy::{
    read_is_governed, AlwaysAllowPolicy, CapabilityBoundPolicy, DefaultPolicyEvaluator,
    GovernanceClass, PermissionPolicy, PolicyContext, PolicyDecision, PolicyEvaluator, PolicyResult,
};
pub use security::{
    AllowAllPermissionGate, GatewayDecision, PermissionGate, PermissionGateway, PermissionRequest,
    PermissionSubject, StandardPermissionGate,
};
pub use services::{
    ActionCatalogService, AuditService, CapabilityResolver, ConfigurationService,
    DatabaseServiceHandle, ExecutionCancellationService,
    ExecutionContextService, ExecutionGuardService, ExecutionOutcomeService,
    ExecutionReconciliationService, GovernedIntentExecutionService, ObservationService,
    ServiceRegistry, ServiceStatus, SuggestionIntentService, SuggestionLifecycleService,
    SuggestionService, WorkspaceAnalyticsService, WorkspaceContextService,
};
pub use state::WorkspaceState;
pub use workspace_domain::{
    Actor, ActorContext, ActorType, Addressable, Capability, CapabilityId, CapabilityScope,
    CapabilitySet, Intent, IntentContext, IntentType, ResourceId, ResourceKind, ResourceRef,
};

use std::path::Path;
use std::sync::{Arc, Mutex};

use commands::CommandContext;
use events::AuditEventSubscriber;
use policy::CapabilityBoundPolicy as DefaultPermissionPolicy;
use security::StandardPermissionGate as DefaultPermissionGate;
use services::{
    AssistantWorkflowStore, ObservationScheduler, ObservationStartupTrigger, OrchestratedPlanStore,
};
use workspace_domain::ObservationScheduleConfig;

/// Kernel crate version aligned with application semver.
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SERVICE_DATABASE: &str = "database";
pub const SERVICE_CONFIGURATION: &str = "configuration";
pub const SERVICE_WORKSPACE: &str = "workspace";

/// Central runtime authority for Workspace.
pub struct WorkspaceKernel {
    state: WorkspaceState,
    database: DatabaseServiceHandle,
    services: ServiceRegistry,
    event_bus: EventBus,
    permission_gate: Arc<dyn PermissionGate>,
    permission_policy: Arc<dyn PermissionPolicy>,
    /// Diagnostic in-memory orchestrated AI plans (not durable authority).
    orchestrated_plans: Arc<Mutex<OrchestratedPlanStore>>,
    /// Diagnostic in-memory assistant workflows (interface only, not authority).
    assistant_workflows: Arc<Mutex<AssistantWorkflowStore>>,
    /// Observation schedule runtime (disabled for in-memory test kernels).
    observation_scheduler: ObservationScheduler,
}

impl WorkspaceKernel {
    /// Runs InitializeWorkspace through the command layer.
    pub fn initialize(db_path: impl AsRef<Path>) -> Result<Self> {
        log::info!("workspace kernel startup beginning");
        let mut kernel = Self::bootstrap_shell(KERNEL_VERSION);
        CommandHandler::initialize_workspace(&mut kernel, db_path)?;
        // First real observation trigger: once after Ready. Soft-fail only.
        ObservationStartupTrigger::fire(&kernel);
        // Schedule runtime ownership — starts only after Ready.
        kernel.start_observation_scheduler(ObservationScheduleConfig::enabled_default());
        log::info!("workspace kernel ready");
        Ok(kernel)
    }

    /// Initializes with an in-memory database (tests).
    ///
    /// Does **not** fire the startup observation trigger or start the observation
    /// scheduler — avoids Win32 capture and keeps fixtures deterministic.
    pub fn initialize_in_memory() -> Result<Self> {
        log::debug!("workspace kernel in-memory initialization");
        let mut kernel = Self::bootstrap_shell(KERNEL_VERSION);
        CommandHandler::initialize_workspace_in_memory(&mut kernel)?;
        Ok(kernel)
    }

    pub fn state(&self) -> &WorkspaceState {
        &self.state
    }

    pub fn services(&self) -> &ServiceRegistry {
        &self.services
    }

    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    pub(crate) fn shared_database(&self) -> Arc<Mutex<workspace_database::Database>> {
        self.database.shared()
    }

    pub(crate) fn orchestrated_plans(&self) -> Arc<Mutex<OrchestratedPlanStore>> {
        Arc::clone(&self.orchestrated_plans)
    }

    pub(crate) fn assistant_workflows(&self) -> Arc<Mutex<AssistantWorkflowStore>> {
        Arc::clone(&self.assistant_workflows)
    }

    pub fn health(&self) -> WorkspaceHealth {
        WorkspaceHealth::from_runtime(
            self.state.lifecycle,
            &self.state.version,
            &self.services,
        )
    }

    pub fn get_settings(&self) -> Result<WorkspaceSettings> {
        CommandHandler::get_settings(
            self,
            ActorContext::local_user(),
            IntentContext::user_request(),
        )
    }

    pub fn update_settings(&self, update: SettingsUpdate) -> Result<WorkspaceSettings> {
        CommandHandler::update_settings(
            self,
            ActorContext::local_user(),
            IntentContext::user_request(),
            update,
        )
    }

    pub fn create_workspace(&self, name: String) -> Result<workspace_domain::Workspace> {
        CommandHandler::create_workspace(
            self,
            ActorContext::local_user(),
            IntentContext::user_request(),
            name,
        )
    }

    pub fn get_workspace(&self, id: String) -> Result<workspace_domain::Workspace> {
        CommandHandler::get_workspace(
            self,
            ActorContext::local_user(),
            IntentContext::user_request(),
            id,
        )
    }

    pub fn get_audit_history(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<workspace_domain::AuditEvent>> {
        CommandHandler::get_audit_history(
            self,
            ActorContext::local_user(),
            IntentContext::user_request(),
            limit,
        )
    }

    pub fn begin_shutdown(&mut self) {
        self.observation_scheduler.stop();
        CommandHandler::shutdown(self);
    }

    pub(crate) fn observation_scheduler(&self) -> &ObservationScheduler {
        &self.observation_scheduler
    }

    pub(crate) fn observation_scheduler_mut(&mut self) -> &mut ObservationScheduler {
        &mut self.observation_scheduler
    }

    pub(crate) fn start_observation_scheduler(&mut self, config: ObservationScheduleConfig) {
        let db = self.database.shared();
        self.observation_scheduler.configure(config);
        self.observation_scheduler.start(db);
    }

    pub(crate) fn apply_runtime(
        &mut self,
        state: WorkspaceState,
        database: DatabaseServiceHandle,
        services: ServiceRegistry,
    ) {
        self.state = state;
        self.database = database;
        self.services = services;
        AuditEventSubscriber::register(&self.event_bus, self.database.shared());
    }

    pub(crate) fn transition_lifecycle(
        &mut self,
        lifecycle: crate::lifecycle::LifecycleState,
    ) -> Result<()> {
        self.state.transition(lifecycle)
    }

    pub(crate) fn command_context(
        &self,
        actor: ActorContext,
        intent: IntentContext,
    ) -> CommandContext<'_> {
        let capability_set = CapabilitySet::for_actor_type(actor.actor.actor_type);

        CommandContext {
            actor_context: actor,
            intent_context: intent,
            capability_set,
            state: &self.state,
            database: self.database.shared(),
            event_bus: &self.event_bus,
            permission_gate: self.permission_gate.as_ref(),
            permission_policy: self.permission_policy.as_ref(),
        }
    }

    pub(crate) fn permission_gate(&self) -> &dyn PermissionGate {
        self.permission_gate.as_ref()
    }

    /// Production policy handle (`CapabilityBoundPolicy`).
    pub(crate) fn permission_policy(&self) -> &dyn PermissionPolicy {
        self.permission_policy.as_ref()
    }

    fn bootstrap_shell(version: &str) -> Self {
        Self {
            state: WorkspaceState::new(version),
            database: DatabaseServiceHandle::new(
                workspace_database::Database::open_in_memory()
                    .expect("bootstrap in-memory database placeholder"),
            ),
            services: ServiceRegistry::new(),
            event_bus: EventBus::new(),
            permission_gate: Arc::new(DefaultPermissionGate),
            permission_policy: Arc::new(DefaultPermissionPolicy),
            orchestrated_plans: Arc::new(Mutex::new(OrchestratedPlanStore::new())),
            assistant_workflows: Arc::new(Mutex::new(AssistantWorkflowStore::new())),
            observation_scheduler: ObservationScheduler::new(ObservationScheduleConfig::disabled()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn kernel_wires_capability_bound_permission_policy() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let allowed = PolicyContext::new(
            Actor::local_user().id.to_string(),
            Intent::user_request(),
            Capability::workspace_write(),
            "CreateWorkspace",
            PermissionSubject::Resource(workspace_domain::ResourceKind::Workspace),
        )
        .with_granted(CapabilitySet::local_user_standard());

        assert!(kernel
            .permission_policy()
            .evaluate(&allowed)
            .unwrap()
            .is_allowed());

        let denied = PolicyContext::new(
            Actor::local_user().id.to_string(),
            Intent::user_request(),
            Capability::workspace_write(),
            "CreateWorkspace",
            PermissionSubject::Resource(workspace_domain::ResourceKind::Workspace),
        );

        assert!(!kernel
            .permission_policy()
            .evaluate(&denied)
            .unwrap()
            .is_allowed());
    }

    #[test]
    fn shutdown_audit_uses_system_actor() {
        use workspace_domain::ActorType;

        let mut kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        kernel.begin_shutdown();

        let history = AuditService::list_recent(&kernel.shared_database(), 20).unwrap();
        assert!(history.iter().any(|record| {
            record.event_type == "system.workspace.shutdown"
                && record.actor_type == ActorType::System
        }));
    }

    #[test]
    fn kernel_initializes_in_memory() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        assert!(kernel.state().is_ready());
        assert_eq!(kernel.state().lifecycle, LifecycleState::Ready);
        assert!(kernel.services().all_healthy());
    }

    #[test]
    fn kernel_emits_lifecycle_events() {
        let received = Arc::new(Mutex::new(Vec::new()));
        let captured = Arc::clone(&received);

        let mut kernel = WorkspaceKernel::bootstrap_shell(KERNEL_VERSION);
        kernel.event_bus.subscribe(move |event| {
            captured.lock().unwrap().push(event.name().to_string());
        });

        CommandHandler::initialize_workspace_in_memory(&mut kernel).unwrap();

        let events = received.lock().unwrap();
        assert!(events.contains(&"system.workspace.started".to_string()));
        assert!(events.contains(&"system.workspace.ready".to_string()));
    }

    #[test]
    fn update_settings_emits_settings_changed() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let received = Arc::new(Mutex::new(String::new()));
        let captured = Arc::clone(&received);

        kernel.event_bus.subscribe(move |event| {
            if event.name() == "system.settings.changed" {
                *captured.lock().unwrap() = event.name().to_string();
            }
        });

        kernel
            .update_settings(SettingsUpdate {
                theme: Some("dark".into()),
                first_run: None,
                active_workspace_id: None,
                personalization_enabled: None,
            })
            .unwrap();

        assert_eq!(*received.lock().unwrap(), "system.settings.changed");
    }

    #[test]
    fn kernel_exposes_default_settings() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let settings = kernel.get_settings().unwrap();
        assert_eq!(settings.theme, "system");
        assert!(settings.first_run);
    }

    #[test]
    fn kernel_reports_health() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let health = kernel.health();

        assert_eq!(health.status, "ready");
        assert!(health.initialized);
        assert!(health.services.contains(&SERVICE_DATABASE.to_string()));
        assert!(health.services.contains(&SERVICE_CONFIGURATION.to_string()));
    }

    #[test]
    fn shutdown_transitions_lifecycle_and_emits_event() {
        let mut kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let received = Arc::new(Mutex::new(false));
        let captured = Arc::clone(&received);

        kernel.event_bus.subscribe(move |event| {
            if event.name() == "system.workspace.shutdown" {
                *captured.lock().unwrap() = true;
            }
        });

        kernel.begin_shutdown();
        assert_eq!(kernel.state().lifecycle, LifecycleState::ShuttingDown);
        assert!(*received.lock().unwrap());
    }

    #[test]
    fn create_workspace_persists_and_emits_event() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let received = Arc::new(Mutex::new(String::new()));
        let captured = Arc::clone(&received);

        kernel.event_bus.subscribe(move |event| {
            if event.name() == "workspace.entity.created" {
                *captured.lock().unwrap() = event.name().to_string();
            }
        });

        let workspace = kernel.create_workspace("Sprint 05".into()).unwrap();
        let loaded = kernel.get_workspace(workspace.id.to_string()).unwrap();

        assert_eq!(loaded.name, "Sprint 05");
        assert_eq!(*received.lock().unwrap(), "workspace.entity.created");
    }

    #[test]
    fn failed_migration_prevents_initialization() {
        use tempfile::tempdir;
        use workspace_database::DatabaseService;

        let dir = tempdir().unwrap();
        let bad_migrations = dir.path().join("bad_migrations");
        std::fs::create_dir_all(&bad_migrations).unwrap();
        std::fs::write(
            bad_migrations.join("001_bad.sql"),
            "CREATE TABLE bad syntax ;",
        )
        .unwrap();

        let db_path = dir.path().join("workspace.db");
        let result =
            DatabaseService::initialize_with_migrations(&db_path, &bad_migrations);

        assert!(result.is_err());
    }

    #[test]
    fn kernel_defaults_block_ai_launch_with_approval_required() {
        use commands::{
            CommandPipeline, CreateApplication, CreateWorkspace, LaunchApplication,
        };

        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let local = ActorContext::local_user();
        let intent = IntentContext::user_request();

        let workspace = CommandPipeline::new(kernel.command_context(local.clone(), intent.clone()))
            .execute_mutation(CreateWorkspace::new("Seal WS".into()))
            .unwrap();

        let app = CommandPipeline::new(kernel.command_context(local, intent.clone()))
            .execute_mutation(CreateApplication::new(
                workspace.id,
                "Notepad".into(),
                None,
                Some("notepad.exe".into()),
            ))
            .unwrap();

        let ai = ActorContext::new(Actor::ai_assistant("ai-kernel-default").unwrap());
        let error = CommandPipeline::new(kernel.command_context(ai, intent))
            .execute_mutation(LaunchApplication::simulated(app.id))
            .unwrap_err();

        assert!(matches!(error, KernelError::ApprovalRequired { .. }));
    }
}
