use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

use crate::events::EventBus;
use crate::policy::PermissionPolicy;
use crate::security::PermissionGate;
use crate::state::WorkspaceState;

/// Runtime dependencies available to every command.
pub struct CommandContext<'a> {
    pub actor_context: ActorContext,
    pub intent_context: IntentContext,
    pub capability_set: CapabilitySet,
    pub state: &'a WorkspaceState,
    pub database: Arc<Mutex<Database>>,
    pub event_bus: &'a EventBus,
    pub permission_gate: &'a dyn PermissionGate,
    pub permission_policy: &'a dyn PermissionPolicy,
}

impl CommandContext<'_> {
    pub fn with_database<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Database) -> R,
    {
        let guard = self.database.lock().expect("database lock poisoned");
        f(&guard)
    }

    /// Creates an equivalent context for nested pipeline execution.
    pub fn fork(&self) -> CommandContext<'_> {
        CommandContext {
            actor_context: self.actor_context.clone(),
            intent_context: self.intent_context.clone(),
            capability_set: self.capability_set.clone(),
            state: self.state,
            database: Arc::clone(&self.database),
            event_bus: self.event_bus,
            permission_gate: self.permission_gate,
            permission_policy: self.permission_policy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventBus;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use crate::state::WorkspaceState;
    use workspace_domain::{CapabilitySet, IntentContext, IntentType};

    #[test]
    fn command_context_propagates_actor_and_intent() {
        let state = WorkspaceState::new("test");
        let bus = EventBus::new();
        let db = Arc::new(Mutex::new(
            workspace_database::Database::open_in_memory().unwrap(),
        ));

        let ctx = CommandContext {
            actor_context: workspace_domain::ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::local_user_standard(),
            state: &state,
            database: db,
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        };

        assert_eq!(
            ctx.intent_context.intent.intent_type,
            IntentType::UserRequest
        );
        assert!(ctx
            .capability_set
            .contains(&workspace_domain::Capability::workspace_write()));
    }
}
