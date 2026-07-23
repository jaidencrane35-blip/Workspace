use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::{AuditRepository, Database};
use workspace_domain::{ActorContext, AuditEvent, Capability, IntentContext, ResourceRef};

use crate::error::Result;
use crate::events::types::DomainEvent;

/// Coordinates durable audit persistence — no business logic.
pub struct AuditService;

impl AuditService {
    pub fn append(db: &Arc<Mutex<Database>>, event: AuditEvent) -> Result<()> {
        let db = db.lock().expect("database lock poisoned");
        AuditRepository::new(&db).append(&event)?;
        Ok(())
    }

    pub fn list_recent(db: &Arc<Mutex<Database>>, limit: usize) -> Result<Vec<AuditEvent>> {
        let db = db.lock().expect("database lock poisoned");
        AuditRepository::new(&db).list_recent(limit).map_err(Into::into)
    }

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

        Self::append(db, event)
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

        Self::append(db, audit)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::types::WorkspaceEntityCreated;
    use workspace_database::DatabaseService;
    use workspace_domain::{ActorType, IntentType};
    use tempfile::tempdir;

    fn test_db() -> Arc<Mutex<Database>> {
        let dir = tempdir().unwrap();
        Arc::new(Mutex::new(
            DatabaseService::initialize(dir.path().join("workspace.db"))
                .unwrap()
                .into_database(),
        ))
    }

    #[test]
    fn records_command_audit_with_intent_and_capability() {
        let db = test_db();
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
    fn records_domain_event_with_actor_intent_and_capability() {
        let db = test_db();
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
        let db = test_db();
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
