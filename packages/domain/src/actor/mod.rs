use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::ids::ActorId;

/// Classification of execution identity (local-first; not authentication).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    LocalUser,
    System,
    AIAssistant,
    Automation,
    Plugin,
    RemoteSession,
}

/// Lightweight, optional metadata describing an actor (no secrets).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ActorMetadata {
    pub label: Option<String>,
}

/// Immutable execution identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Actor {
    pub id: ActorId,
    pub actor_type: ActorType,
    pub metadata: ActorMetadata,
}

/// Execution identity attached to a command or event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorContext {
    pub actor: Actor,
}

/// Well-known identifier for the single local human operator (no accounts yet).
pub const LOCAL_USER_ACTOR_ID: &str = "local-user";

/// Well-known identifier for kernel/system operations.
pub const SYSTEM_ACTOR_ID: &str = "system";

impl Actor {
    pub fn new(id: ActorId, actor_type: ActorType, metadata: ActorMetadata) -> Self {
        Self {
            id,
            actor_type,
            metadata,
        }
    }

    pub fn system() -> Self {
        Self {
            id: ActorId::new(SYSTEM_ACTOR_ID).expect("system actor id is valid"),
            actor_type: ActorType::System,
            metadata: ActorMetadata {
                label: Some("Workspace Kernel".into()),
            },
        }
    }

    pub fn local_user() -> Self {
        Self {
            id: ActorId::new(LOCAL_USER_ACTOR_ID).expect("local user actor id is valid"),
            actor_type: ActorType::LocalUser,
            metadata: ActorMetadata {
                label: Some("Local User".into()),
            },
        }
    }

    /// AI participant identity — starts with zero capabilities (no authority).
    pub fn ai_assistant(id: impl Into<String>) -> Result<Self> {
        Ok(Self {
            id: ActorId::new(id)?,
            actor_type: ActorType::AIAssistant,
            metadata: ActorMetadata::default(),
        })
    }

    pub fn automation(id: impl Into<String>) -> Result<Self> {
        Ok(Self {
            id: ActorId::new(id)?,
            actor_type: ActorType::Automation,
            metadata: ActorMetadata::default(),
        })
    }

    pub fn plugin(id: impl Into<String>) -> Result<Self> {
        Ok(Self {
            id: ActorId::new(id)?,
            actor_type: ActorType::Plugin,
            metadata: ActorMetadata::default(),
        })
    }

    pub fn remote_session(id: impl Into<String>) -> Result<Self> {
        Ok(Self {
            id: ActorId::new(id)?,
            actor_type: ActorType::RemoteSession,
            metadata: ActorMetadata::default(),
        })
    }
}

impl ActorContext {
    pub fn new(actor: Actor) -> Self {
        Self { actor }
    }

    pub fn system() -> Self {
        Self::new(Actor::system())
    }

    pub fn local_user() -> Self {
        Self::new(Actor::local_user())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_system_and_local_user_actors() {
        let system = Actor::system();
        let user = Actor::local_user();

        assert_eq!(system.actor_type, ActorType::System);
        assert_eq!(user.actor_type, ActorType::LocalUser);
        assert_eq!(user.id.as_str(), LOCAL_USER_ACTOR_ID);
    }

    #[test]
    fn actor_context_round_trips_through_serialization() {
        let context = ActorContext::local_user();
        let json = serde_json::to_string(&context).unwrap();
        let restored: ActorContext = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, context);
    }

    #[test]
    fn actors_are_comparable() {
        assert_eq!(Actor::local_user(), Actor::local_user());
        assert_ne!(Actor::local_user(), Actor::system());
    }

    #[test]
    fn ai_assistant_is_first_class_actor() {
        assert_eq!(
            Actor::ai_assistant("ai-1").unwrap().actor_type,
            ActorType::AIAssistant
        );
        assert_eq!(
            Actor::automation("auto-1").unwrap().actor_type,
            ActorType::Automation
        );
        assert_eq!(
            Actor::plugin("plugin-1").unwrap().actor_type,
            ActorType::Plugin
        );
        assert_eq!(
            Actor::remote_session("remote-1").unwrap().actor_type,
            ActorType::RemoteSession
        );
    }
}
