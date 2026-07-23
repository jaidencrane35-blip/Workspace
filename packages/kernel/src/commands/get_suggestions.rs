use crate::commands::context::CommandContext;
use crate::commands::resource::ensure_workspace_exists;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::SuggestionService;
use workspace_domain::{Capability, Suggestion, WorkspaceId};

const DEFAULT_LIMIT: usize = 200;
const MAX_LIMIT: usize = 500;

/// Returns deterministic, read-only proposals derived from the workspace
/// context. Suggestions are proposals only — never actions.
pub struct GetSuggestions {
    pub workspace_id: WorkspaceId,
    pub limit: usize,
}

impl GetSuggestions {
    pub fn new(workspace_id: WorkspaceId, limit: Option<usize>) -> Self {
        Self {
            workspace_id,
            limit: limit.unwrap_or(DEFAULT_LIMIT),
        }
    }
}

impl crate::commands::Command for GetSuggestions {
    fn name(&self) -> &'static str {
        "GetSuggestions"
    }
}

impl QueryCommand for GetSuggestions {
    type Output = Vec<Suggestion>;

    fn permission_subject(&self) -> PermissionSubject {
        // Suggestions derive from the workspace context, which aggregates
        // sensitive, system-level derived reads.
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Suggestions derive from context, which includes activity history;
        // reuse the audit read capability rather than inventing a new one.
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        // Derived from sensitive context; governed and audited regardless of
        // actor (DEC-017), consistent with GetWorkspaceContext.
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<Suggestion>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ensure_workspace_exists(ctx, &self.workspace_id)?;

        let limit = self.limit.clamp(1, MAX_LIMIT);
        SuggestionService::list(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
            &ctx.capability_set,
            ctx.permission_policy,
            ctx.permission_gate,
            &self.workspace_id,
            limit,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::initialize::InitializeWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::commands::zone::CreateZone;
    use crate::events::EventBus;
    use crate::policy::{read_is_governed, AlwaysAllowPolicy, GovernanceClass};
    use crate::security::AllowAllPermissionGate;
    use workspace_domain::{ActorContext, ActorType, CapabilitySet, IntentContext, SuggestionStatus};

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
    fn pipeline_returns_suggestions() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        crate::events::AuditEventSubscriber::register(&bus, init.database.shared());

        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Suggest WS".into()))
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

        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().all(|s| s.status == SuggestionStatus::Pending));
        assert!(suggestions.iter().all(|s| s.validate().is_ok()));
    }

    #[test]
    fn suggestions_read_is_governed() {
        let command = GetSuggestions::new(WorkspaceId::new("ws-gov").unwrap(), None);
        assert_eq!(command.governance_class(), GovernanceClass::Governed);
        assert!(read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
    }

    #[test]
    fn rejects_missing_workspace() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();

        let error = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetSuggestions::new(
                WorkspaceId::new("missing-ws").unwrap(),
                None,
            ))
            .unwrap_err();

        assert!(matches!(error, KernelError::WorkspaceNotFound));
    }
}
