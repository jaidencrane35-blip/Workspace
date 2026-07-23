use std::sync::{Arc, Mutex};

use workspace_database::Database;
use workspace_domain::ActorContext;

use crate::events::EventBus;
use crate::security::PermissionGate;
use crate::state::WorkspaceState;

/// Runtime dependencies available to every command.
pub struct CommandContext<'a> {
    pub actor_context: ActorContext,
    pub state: &'a WorkspaceState,
    pub database: Arc<Mutex<Database>>,
    pub event_bus: &'a EventBus,
    pub permission_gate: &'a dyn PermissionGate,
}

impl CommandContext<'_> {
    pub fn with_database<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Database) -> R,
    {
        let guard = self.database.lock().expect("database lock poisoned");
        f(&guard)
    }
}
