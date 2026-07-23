use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{ActorType, AuditEvent, AuditEventId, IntentType};

/// Persistence for durable audit records.
pub struct AuditRepository<'a> {
    db: &'a Database,
}

impl<'a> AuditRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn append(&self, event: &AuditEvent) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO audit_events (id, timestamp, event_type, actor_type, actor_id, command_name, intent_type, capability, resource_ref, success, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            (
                event.id.as_str(),
                &event.timestamp,
                &event.event_type,
                actor_type_to_str(event.actor_type),
                &event.actor_id,
                &event.command_name,
                event.intent_type.map(intent_type_to_str),
                &event.capability,
                &event.resource_ref,
                i32::from(event.success),
                &event.metadata,
            ),
        )?;
        Ok(())
    }

    pub fn list_recent(&self, limit: usize) -> Result<Vec<AuditEvent>> {
        let limit = limit.clamp(1, 500) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT id, timestamp, event_type, actor_type, actor_id, command_name, intent_type, capability, resource_ref, success, metadata
             FROM audit_events
             ORDER BY timestamp DESC
             LIMIT ?1",
        )?;

        let rows = stmt.query_map([limit], map_audit_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn actor_type_to_str(actor_type: ActorType) -> &'static str {
    match actor_type {
        ActorType::LocalUser => "local_user",
        ActorType::System => "system",
        ActorType::AIAssistant => "ai_assistant",
        ActorType::Automation => "automation",
        ActorType::Plugin => "plugin",
        ActorType::RemoteSession => "remote_session",
    }
}

fn intent_type_to_str(intent_type: IntentType) -> &'static str {
    match intent_type {
        IntentType::UserRequest => "user_request",
        IntentType::AISuggestion => "ai_suggestion",
        IntentType::Automation => "automation",
        IntentType::PluginRequest => "plugin_request",
        IntentType::SystemStartup => "system_startup",
        IntentType::SystemShutdown => "system_shutdown",
        IntentType::ScheduledTask => "scheduled_task",
        IntentType::ExternalIntegration => "external_integration",
    }
}

fn parse_actor_type(value: &str) -> rusqlite::Result<ActorType> {
    match value {
        "local_user" | "user" => Ok(ActorType::LocalUser),
        "system" | "service" => Ok(ActorType::System),
        "ai_assistant" => Ok(ActorType::AIAssistant),
        "automation" => Ok(ActorType::Automation),
        "plugin" => Ok(ActorType::Plugin),
        "remote_session" => Ok(ActorType::RemoteSession),
        _ => Err(rusqlite::Error::InvalidColumnType(
            3,
            "actor_type".into(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn parse_intent_type(value: &str) -> rusqlite::Result<IntentType> {
    match value {
        "user_request" => Ok(IntentType::UserRequest),
        "ai_suggestion" => Ok(IntentType::AISuggestion),
        "automation" => Ok(IntentType::Automation),
        "plugin_request" => Ok(IntentType::PluginRequest),
        "system_startup" => Ok(IntentType::SystemStartup),
        "system_shutdown" => Ok(IntentType::SystemShutdown),
        "scheduled_task" => Ok(IntentType::ScheduledTask),
        "external_integration" => Ok(IntentType::ExternalIntegration),
        _ => Err(rusqlite::Error::InvalidColumnType(
            6,
            "intent_type".into(),
            rusqlite::types::Type::Text,
        )),
    }
}

fn map_audit_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuditEvent> {
    let intent_type_raw: Option<String> = row.get(6)?;
    Ok(AuditEvent {
        id: AuditEventId::new(row.get::<_, String>(0)?).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text)
        })?,
        timestamp: row.get(1)?,
        event_type: row.get(2)?,
        actor_type: parse_actor_type(&row.get::<_, String>(3)?)?,
        actor_id: row.get(4)?,
        command_name: row.get(5)?,
        intent_type: intent_type_raw
            .map(|value| parse_intent_type(&value))
            .transpose()?,
        capability: row.get(7)?,
        resource_ref: row.get(8)?,
        success: row.get::<_, i32>(9)? != 0,
        metadata: row.get(10)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use workspace_domain::{Actor, Capability, IntentType};

    #[test]
    fn appends_and_queries_audit_events_with_intent_and_capability() {
        let dir = tempfile::tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();

        let repo = AuditRepository::new(&db);
        let event = AuditEvent::from_actor("command.executed", &Actor::local_user(), true)
            .with_command_name("CreateWorkspace")
            .with_intent_type(IntentType::UserRequest)
            .with_capability(&Capability::workspace_write());

        repo.append(&event).unwrap();
        let records = repo.list_recent(10).unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].actor_type, ActorType::LocalUser);
        assert_eq!(records[0].intent_type, Some(IntentType::UserRequest));
        assert_eq!(records[0].capability.as_deref(), Some("workspace.write"));
    }
}
