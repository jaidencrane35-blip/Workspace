use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::AuditService;
use workspace_domain::{AuditEvent, Capability};

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

    fn permission_subject(&self) -> PermissionSubject {
        // The audit log is a sensitive, system-level resource, not a graph resource.
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        // Sensitive-kind read: governed and audited regardless of actor (DEC-017).
        GovernanceClass::Governed
    }

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
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

    fn test_context<'a>(
        init: &'a crate::commands::initialize::InitializeWorkspaceResult,
        bus: &'a EventBus,
    ) -> CommandContext<'a> {
        CommandContext {
            actor_context: ActorContext::local_user(),
            intent_context: IntentContext::user_request(),
            capability_set: CapabilitySet::local_user_standard(),
            state: &init.state,
            database: init.database.shared(),
            event_bus: bus,
            permission_gate: &AllowAllPermissionGate,
            permission_policy: &AlwaysAllowPolicy,
        }
    }

    #[test]
    fn returns_recent_audit_records() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

        CommandPipeline::new(test_context(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Audit Query".into()))
            .unwrap();

        let history = CommandPipeline::new(test_context(&init, &bus))
            .execute_query(GetAuditHistory::new(Some(10)))
            .unwrap();

        assert!(!history.is_empty());
    }
}
