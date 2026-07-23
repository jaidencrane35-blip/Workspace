use workspace_database::Database;

use crate::events::EventBus;
use crate::security::PermissionGate;
use crate::state::WorkspaceState;

/// Runtime dependencies available to every command.
pub struct CommandContext<'a> {
    pub state: &'a WorkspaceState,
    pub database: &'a Database,
    pub event_bus: &'a EventBus,
    pub permission_gate: &'a dyn PermissionGate,
}
