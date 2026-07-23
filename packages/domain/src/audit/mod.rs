use serde::{Deserialize, Serialize};

use crate::actor::{Actor, ActorType};
use crate::ids::AuditEventId;

/// Durable record of important system activity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: AuditEventId,
    pub timestamp: String,
    pub event_type: String,
    pub actor_type: ActorType,
    pub actor_id: Option<String>,
    pub command_name: Option<String>,
    pub success: bool,
    pub metadata: Option<String>,
}

impl AuditEvent {
    pub fn new(
        event_type: impl Into<String>,
        actor_type: ActorType,
        success: bool,
    ) -> Self {
        Self {
            id: AuditEventId::generate(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            event_type: event_type.into(),
            actor_type,
            actor_id: None,
            command_name: None,
            success,
            metadata: None,
        }
    }

    pub fn from_actor(
        event_type: impl Into<String>,
        actor: &Actor,
        success: bool,
    ) -> Self {
        Self::new(event_type, actor.actor_type, success).with_actor(actor)
    }

    pub fn with_actor(mut self, actor: &Actor) -> Self {
        self.actor_type = actor.actor_type;
        self.actor_id = Some(actor.id.to_string());
        self
    }

    pub fn with_command_name(mut self, command_name: impl Into<String>) -> Self {
        self.command_name = Some(command_name.into());
        self
    }

    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::Actor;

    #[test]
    fn creates_audit_event_with_actor() {
        let event = AuditEvent::from_actor("command.executed", &Actor::system(), true)
            .with_command_name("CreateWorkspace");

        assert_eq!(event.event_type, "command.executed");
        assert_eq!(event.actor_type, ActorType::System);
        assert_eq!(event.actor_id.as_deref(), Some("system"));
    }
}
