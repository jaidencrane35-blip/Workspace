use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::services::AuditService;
use workspace_domain::AuditEvent;

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

/// Returns recent audit history entries (read-only).
pub struct GetAuditHistory {
    pub limit: usize,
}

impl crate::commands::Command for GetAuditHistory {
    fn name(&self) -> &'static str {
        "GetAuditHistory"
    }
}

impl QueryCommand for GetAuditHistory {
    type Output = Vec<AuditEvent>;

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<AuditEvent>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        let limit = self.limit.clamp(1, MAX_LIMIT);
        AuditService::list_recent(&ctx.database, limit)
    }
}

impl GetAuditHistory {
    pub fn new(limit: Option<usize>) -> Self {
        Self {
            limit: limit.unwrap_or(DEFAULT_LIMIT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::context::CommandContext;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::events::EventBus;
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::ActorContext;

    #[test]
    fn returns_recent_audit_records() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let ctx = CommandContext {
            actor_context: ActorContext::local_user(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
        };

        CommandPipeline::new(ctx)
            .execute_mutation(CreateWorkspace::new("Audit Query".into()))
            .unwrap();

        let history = CommandPipeline::new(CommandContext {
            actor_context: ActorContext::local_user(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: &bus,
            permission_gate: &AllowAllPermissionGate,
        })
        .execute_query(GetAuditHistory::new(Some(10)))
        .unwrap();

        assert!(!history.is_empty());
    }
}
