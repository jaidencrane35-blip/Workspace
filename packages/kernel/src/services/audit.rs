use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::{AuditRepository, Database};
use workspace_domain::{ActorContext, AuditEvent, Capability, IntentContext, ResourceRef};

use crate::error::{KernelError, Result};
use crate::events::types::DomainEvent;

/// Coordinates durable audit persistence — no business logic.
pub struct AuditService;

impl AuditService {
    pub fn append(db: &Arc<Mutex<Database>>, event: AuditEvent) -> Result<()> {
        Self::append_at(db, event, "audit.append")
    }

    pub fn list_recent(db: &Arc<Mutex<Database>>, limit: usize) -> Result<Vec<AuditEvent>> {
        let db = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        AuditRepository::new(&db).list_recent(limit).map_err(Into::into)
    }

    /// Records the command authorization evidence required before dispatch.
    ///
    /// This record is fail-closed: [`CommandPipeline`] must not execute the
    /// command if persistence fails.
    pub fn record_command_authorized(
        db: &Arc<Mutex<Database>>,
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        command_name: &str,
        capability: &Capability,
    ) -> Result<()> {
        let event = AuditEvent::from_actor("command.authorized", &actor_context.actor, true)
            .with_command_name(command_name)
            .with_intent_type(intent_context.intent.intent_type)
            .with_capability(capability);
        Self::append_at(db, event, "command.authorization")
    }

    /// Records a post-dispatch command outcome.
    ///
    /// Callers treat this as an explicit best-effort exception: once a command
    /// has produced side effects, generic rollback is unavailable. The durable
    /// pre-dispatch `command.authorized` record remains the execution evidence.
    pub fn record_command(
        db: &Arc<Mutex<Database>>,
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        command_name: &str,
        capability: &Capability,
        resource_ref: Option<&ResourceRef>,
        success: bool,
        metadata: Option<String>,
    ) -> Result<()> {
        let event_type = if success {
            "command.executed"
        } else {
            "command.failed"
        };

        let mut event = AuditEvent::from_actor(event_type, &actor_context.actor, success)
            .with_command_name(command_name)
            .with_intent_type(intent_context.intent.intent_type)
            .with_capability(capability);

        if let Some(resource_ref) = resource_ref {
            event = event.with_resource_ref(resource_ref);
        }

        if let Some(metadata) = metadata {
            event = event.with_metadata(metadata);
        }

        let stage = if success {
            "command.completion"
        } else {
            "command.failure"
        };
        Self::append_at(db, event, stage)
    }

    /// Records a Permission Gateway decision (allow / deny / approval_required).
    pub fn record_permission_decision(
        db: &Arc<Mutex<Database>>,
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        event_type: &str,
        command_name: &str,
        capability: &Capability,
        success: bool,
        metadata: String,
    ) -> Result<()> {
        let event = AuditEvent::from_actor(event_type, &actor_context.actor, success)
            .with_command_name(command_name)
            .with_intent_type(intent_context.intent.intent_type)
            .with_capability(capability)
            .with_metadata(metadata);

        Self::append_at(db, event, "permission.decision")
    }

    /// Records operational AI planning events (goal/proposal ids only — no chain-of-thought).
    pub fn record_ai_planning_event(
        db: &Arc<Mutex<Database>>,
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        event_type: &str,
        success: bool,
        metadata: String,
    ) -> Result<()> {
        let event = AuditEvent::from_actor(event_type, &actor_context.actor, success)
            .with_intent_type(intent_context.intent.intent_type)
            .with_metadata(metadata);
        Self::append_at(db, event, "ai.operational")
    }

    /// Append-only recovery diagnostic evidence (never a lifecycle transition).
    ///
    /// Event types are restricted to `system.recovery.*` constants. No command
    /// name is attached — diagnostics are observational, not mutation authority.
    pub fn record_recovery_diagnostic(
        db: &Arc<Mutex<Database>>,
        event_type: &str,
        success: bool,
        metadata: String,
    ) -> Result<()> {
        if !workspace_domain::recovery_diagnostic_event_type_is_non_commandable(event_type) {
            return Err(KernelError::IntegrityViolation {
                message: format!(
                    "recovery diagnostic event type is not evidence-only: {event_type}"
                ),
            });
        }
        let actor = ActorContext::system();
        let event = AuditEvent::from_actor(event_type, &actor.actor, success)
            .with_intent_type(IntentContext::system_startup().intent.intent_type)
            .with_capability(&Capability::system_startup())
            .with_metadata(metadata);
        Self::append_at(db, event, "recovery.diagnostic")
    }

    pub fn record_domain_event(db: &Arc<Mutex<Database>>, event: &DomainEvent) -> Result<()> {
        let actor = event
            .actor()
            .cloned()
            .unwrap_or_else(ActorContext::system);

        let intent_context = event
            .intent()
            .cloned()
            .unwrap_or_else(IntentContext::system_startup);

        let capability = event
            .capability()
            .cloned()
            .unwrap_or_else(Capability::system_startup);

        let audit = AuditEvent::from_actor(event.name(), &actor.actor, true)
            .with_intent_type(intent_context.intent.intent_type)
            .with_capability(&capability)
            .with_metadata(Self::sanitized_domain_metadata(event)?);

        Self::append_at(db, audit, "domain.event")
    }

    fn sanitized_domain_metadata(event: &DomainEvent) -> Result<String> {
        let value = match event {
            DomainEvent::WorkspaceStarted(payload) => json!({
                "version": payload.version,
            }),
            DomainEvent::WorkspaceReady(payload) => json!({
                "version": payload.version,
                "lifecycle": payload.lifecycle.as_str(),
            }),
            DomainEvent::WorkspaceShutdown(_) => json!({}),
            DomainEvent::SettingsChanged(payload) => json!({
                "settings_version": payload.settings_version,
                "first_run": payload.first_run,
            }),
            DomainEvent::WorkspaceCreated(payload) => json!({
                "workspace_id": payload.workspace_id,
            }),
            DomainEvent::WorkspaceUpdated(payload) => json!({
                "workspace_id": payload.workspace_id,
            }),
            DomainEvent::ResourceCreated(payload)
            | DomainEvent::ResourceUpdated(payload)
            | DomainEvent::ResourceDeleted(payload)
            | DomainEvent::ZoneCreated(payload)
            | DomainEvent::ApplicationCreated(payload)
            | DomainEvent::WidgetCreated(payload) => json!({
                "resource_ref": payload.resource_ref.canonical(),
                "workspace_id": payload.workspace_id,
            }),
            DomainEvent::LayoutCreated(payload)
            | DomainEvent::LayoutUpdated(payload)
            | DomainEvent::LayoutDeleted(payload)
            | DomainEvent::LayoutReset(payload)
            | DomainEvent::LayoutSnapshot(payload) => json!({
                "layout_id": payload.layout_id.to_string(),
                "workspace_id": payload.workspace_id.to_string(),
                "resource_ref": payload.resource_ref.canonical(),
            }),
        };

        Ok(value.to_string())
    }

    fn append_at(
        db: &Arc<Mutex<Database>>,
        event: AuditEvent,
        stage: &'static str,
    ) -> Result<()> {
        let db = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        AuditRepository::new(&db)
            .append(&event)
            .map_err(|source| KernelError::AuditPersistence { stage, source })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::types::WorkspaceEntityCreated;
    use workspace_database::DatabaseService;
    use workspace_domain::{ActorType, IntentType};
    use tempfile::tempdir;

    fn test_db() -> (tempfile::TempDir, Arc<Mutex<Database>>) {
        let dir = tempdir().unwrap();
        let db = Arc::new(Mutex::new(
            DatabaseService::initialize(dir.path().join("workspace.db"))
                .unwrap()
                .into_database(),
        ));
        (dir, db)
    }

    #[test]
    fn records_command_audit_with_intent_and_capability() {
        let (_dir, db) = test_db();
        AuditService::record_command(
            &db,
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            "CreateWorkspace",
            &Capability::workspace_write(),
            None,
            true,
            None,
        )
        .unwrap();

        let records = AuditService::list_recent(&db, 10).unwrap();
        assert_eq!(records[0].actor_type, ActorType::LocalUser);
        assert_eq!(records[0].actor_id.as_deref(), Some("local-user"));
        assert_eq!(records[0].intent_type, Some(IntentType::UserRequest));
        assert_eq!(records[0].capability.as_deref(), Some("workspace.write"));
    }

    #[test]
    fn domain_metadata_omits_sensitive_fields() {
        let metadata = AuditService::sanitized_domain_metadata(&DomainEvent::SettingsChanged(
            crate::events::types::SettingsChanged {
                theme: "secret-theme".into(),
                first_run: false,
                settings_version: 2,
                actor: Some(ActorContext::local_user()),
                intent: Some(IntentContext::user_request()),
                capability: Some(Capability::settings_write()),
            },
        ))
        .unwrap();

        assert!(!metadata.contains("secret-theme"));
        assert!(metadata.contains("settings_version"));
    }

    #[test]
    fn records_command_authorized_event() {
        let (_dir, db) = test_db();
        AuditService::record_command_authorized(
            &db,
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            "CreateWorkspace",
            &Capability::workspace_write(),
        )
        .unwrap();

        let records = AuditService::list_recent(&db, 10).unwrap();
        assert_eq!(records[0].event_type, "command.authorized");
        assert_eq!(records[0].command_name.as_deref(), Some("CreateWorkspace"));
        assert!(records[0].success);
    }

    #[test]
    fn persistence_failure_is_classified() {
        let (_dir, db) = test_db();
        {
            let db = db.lock().unwrap();
            db.connection()
                .execute_batch(
                    "CREATE TRIGGER fail_audit_insert BEFORE INSERT ON audit_events
                     BEGIN SELECT RAISE(ABORT, 'injected audit failure'); END;",
                )
                .unwrap();
        }

        let err = AuditService::record_permission_decision(
            &db,
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            "permission.allowed",
            "CreateWorkspace",
            &Capability::workspace_write(),
            true,
            "{}".into(),
        )
        .expect_err("must fail closed");

        assert_eq!(err.to_public().code, "audit_persistence_error");
        assert!(matches!(
            err,
            KernelError::AuditPersistence {
                stage: "permission.decision",
                ..
            }
        ));
    }

    #[test]
    fn records_domain_event_with_actor_intent_and_capability() {
        let (_dir, db) = test_db();
        AuditService::record_domain_event(
            &db,
            &DomainEvent::WorkspaceCreated(WorkspaceEntityCreated {
                workspace_id: "ws-1".into(),
                name: "Sensitive Name".into(),
                actor: Some(ActorContext::local_user()),
                intent: Some(IntentContext::user_request()),
                capability: Some(Capability::workspace_write()),
            }),
        )
        .unwrap();

        let records = AuditService::list_recent(&db, 10).unwrap();
        assert_eq!(records[0].event_type, "workspace.entity.created");
        assert_eq!(records[0].actor_type, ActorType::LocalUser);
        assert_eq!(records[0].intent_type, Some(IntentType::UserRequest));
        assert_eq!(records[0].capability.as_deref(), Some("workspace.write"));
        assert!(!records[0].metadata.as_ref().unwrap().contains("Sensitive Name"));
    }

    #[test]
    fn system_defaults_used_when_domain_event_has_no_attribution() {
        let (_dir, db) = test_db();
        AuditService::record_domain_event(
            &db,
            &DomainEvent::WorkspaceStarted(crate::events::types::WorkspaceStarted {
                version: "0.1.0".into(),
                intent: Some(IntentContext::system_startup()),
                capability: Some(Capability::system_startup()),
            }),
        )
        .unwrap();

        let records = AuditService::list_recent(&db, 10).unwrap();
        assert_eq!(records[0].intent_type, Some(IntentType::SystemStartup));
        assert_eq!(records[0].capability.as_deref(), Some("system.startup"));
    }
}
