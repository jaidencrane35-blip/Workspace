use crate::commands::context::CommandContext;
use crate::commands::resource::ensure_workspace_exists;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::WorkspaceProjectionService;
use workspace_domain::{Capability, ResourceKind, WorkspaceId, WorkspaceSnapshot};

/// Retrieves a derived read model of current workspace state.
pub struct GetWorkspaceSnapshot {
    pub workspace_id: WorkspaceId,
}

impl crate::commands::Command for GetWorkspaceSnapshot {
    fn name(&self) -> &'static str {
        "GetWorkspaceSnapshot"
    }
}

impl QueryCommand for GetWorkspaceSnapshot {
    type Output = WorkspaceSnapshot;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::workspace_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        // DEC-017: aggregate read of non-sensitive workspace resources; same
        // classification as GetWorkspace / GetLayout for local human actors.
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<WorkspaceSnapshot> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ensure_workspace_exists(ctx, &self.workspace_id)?;
        ctx.with_database(|db| WorkspaceProjectionService::build_snapshot(db, &self.workspace_id))
    }
}

impl GetWorkspaceSnapshot {
    pub fn new(workspace_id: WorkspaceId) -> Self {
        Self { workspace_id }
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
    use workspace_domain::{
        ActorContext, ActorType, CapabilitySet, IntentContext,
    };

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
    fn pipeline_returns_workspace_snapshot() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let workspace = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_mutation(CreateWorkspace::new("Snapshot WS".into()))
            .unwrap();
        CommandPipeline::new(ready_ctx(&init, &bus)).execute_mutation(
            CreateZone::new(workspace.id.clone(), "Zone A".into(), None),
        ).unwrap();

        let snapshot = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetWorkspaceSnapshot::new(workspace.id))
            .unwrap();

        assert_eq!(snapshot.workspace_name, "Snapshot WS");
        assert_eq!(snapshot.zones.len(), 1);
        assert!(!snapshot.generated_at.is_empty());
    }

    #[test]
    fn local_human_read_is_ungoverned() {
        let command = GetWorkspaceSnapshot::new(
            WorkspaceId::new("ws-gov").unwrap(),
        );
        assert_eq!(command.governance_class(), GovernanceClass::Ungoverned);
        assert!(!read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
    }

    #[test]
    fn non_human_read_is_governed() {
        let command = GetWorkspaceSnapshot::new(
            WorkspaceId::new("ws-gov").unwrap(),
        );
        assert!(read_is_governed(
            ActorType::System,
            command.governance_class(),
        ));
    }

    #[test]
    fn rejects_missing_workspace() {
        let bus = EventBus::new();
        let init = InitializeWorkspace::in_memory().execute(&bus).unwrap();
        let error = CommandPipeline::new(ready_ctx(&init, &bus))
            .execute_query(GetWorkspaceSnapshot::new(
                WorkspaceId::new("missing-ws").unwrap(),
            ))
            .unwrap_err();

        assert!(matches!(error, KernelError::WorkspaceNotFound));
    }
}
