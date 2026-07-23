use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::SuggestionLifecycleService;
use workspace_domain::{Capability, SuggestionLifecycleRecord};

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

/// Returns a derived, read-only stream of recent suggestion lifecycle records.
pub struct GetSuggestionLifecycle {
    pub limit: usize,
}

impl GetSuggestionLifecycle {
    pub fn new(limit: Option<usize>) -> Self {
        Self {
            limit: limit.unwrap_or(DEFAULT_LIMIT),
        }
    }
}

impl crate::commands::Command for GetSuggestionLifecycle {
    fn name(&self) -> &'static str {
        "GetSuggestionLifecycle"
    }
}

impl QueryCommand for GetSuggestionLifecycle {
    type Output = Vec<SuggestionLifecycleRecord>;

    fn permission_subject(&self) -> PermissionSubject {
        // Lifecycle derives from the audit trail: sensitive, system-level history.
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<SuggestionLifecycleRecord>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        let limit = self.limit.clamp(1, MAX_LIMIT);
        SuggestionLifecycleService::list_recent(&ctx.database, limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::accept_suggestion::AcceptSuggestion;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::get_suggestions::GetSuggestions;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::commands::zone::CreateZone;
    use crate::commands::CommandContext;
    use crate::events::EventBus;
    use crate::policy::{read_is_governed, AlwaysAllowPolicy, GovernanceClass};
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{ActorContext, ActorType, CapabilitySet, IntentContext, SuggestionLifecycleState};

    fn ready_ctx<'a>(
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
    fn pipeline_returns_lifecycle_records() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Lifecycle WS".into()))
            .unwrap();
        for index in 0..4 {
            CommandPipeline::new(ready_ctx(&init, &bus))
                .execute_mutation(CreateZone::new(
                    workspace.id.clone(),
                    format!("Zone {index}"),
                    None,
                ))
                .unwrap();
        }

        let suggestions = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetSuggestions::new(workspace.id.clone(), Some(200)))
            .unwrap();
        let suggestion_id = suggestions[0].id.clone();

        CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(AcceptSuggestion::new(
                workspace.id.clone(),
                suggestion_id.clone(),
            ))
            .unwrap();

        let records = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetSuggestionLifecycle::new(Some(50)))
            .unwrap();

        assert!(records.iter().any(|record| {
            record.suggestion_id == suggestion_id
                && record.state == SuggestionLifecycleState::Accepted
        }));
        assert!(records.iter().all(|record| record.validate().is_ok()));
    }

    #[test]
    fn lifecycle_read_is_governed() {
        let command = GetSuggestionLifecycle::new(None);
        assert_eq!(command.governance_class(), GovernanceClass::Governed);
        assert!(read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
    }
}
