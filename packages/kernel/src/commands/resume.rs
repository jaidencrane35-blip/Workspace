//! Deterministic Resume workflow commands (PP-M1-02).
//!
//! Companion sequencing lives here for Product Proof: load a saved context,
//! ask Action to resolve a plan, collect approval, then execute with per-item
//! proofs. Action never receives a saved-context identifier.

use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::{
    action_request_from_saved_context, ActionExecutionControls, DesktopActionService,
    RestoreExecutor, SavedContextService, WorkspaceRuntimeStateService, WorkspaceSessionStore,
};
use serde::{Deserialize, Serialize};
use workspace_domain::{
    ActionOperationResult, ActionPlan, Capability, ItemEffectProof, ResourceKind,
    RestoreCompatibilitySummary, RestoreExecutionPhase, SavedContext, SavedContextId, WorkspaceId,
};
use workspace_windows_integration::{platform_window_mutator, WindowMutator};

/// Lists saved contexts for one workspace. No desktop mutation.
pub struct ListSavedContexts {
    pub workspace_id: WorkspaceId,
}

impl crate::commands::Command for ListSavedContexts {
    fn name(&self) -> &'static str {
        "ListSavedContexts"
    }
}

impl QueryCommand for ListSavedContexts {
    type Output = Vec<SavedContext>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::workspace_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<SavedContext>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        SavedContextService::list_for_workspace(&ctx.database, &self.workspace_id)
    }
}

/// Loads one saved context for inspect/browse.
pub struct GetSavedContext {
    pub saved_context_id: SavedContextId,
}

impl crate::commands::Command for GetSavedContext {
    fn name(&self) -> &'static str {
        "GetSavedContext"
    }
}

impl QueryCommand for GetSavedContext {
    type Output = SavedContext;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::workspace_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<SavedContext> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        SavedContextService::get_by_id(&ctx.database, &self.saved_context_id)?
            .ok_or(KernelError::SavedContextNotFound)
    }
}

/// Deletes one saved context after explicit user confirmation (PP-P01C).
///
/// Workspace Management owns the durable record; this mutation removes it and
/// cascaded restore identities. It does not close windows or mutate the desktop.
pub struct DeleteSavedContext {
    pub saved_context_id: SavedContextId,
}

impl crate::commands::Command for DeleteSavedContext {
    fn name(&self) -> &'static str {
        "DeleteSavedContext"
    }
}

impl MutationCommand for DeleteSavedContext {
    type Output = ();

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::workspace_write()
    }

    fn audit_metadata(&self, _output: &Self::Output) -> Option<String> {
        Some(
            serde_json::json!({
                "saved_context_id": self.saved_context_id.as_str(),
                "deleted": true,
            })
            .to_string(),
        )
    }

    fn audit_failure_metadata(&self) -> Option<String> {
        Some(
            serde_json::json!({
                "saved_context_id": self.saved_context_id.as_str(),
                "deleted": false,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<()> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        // Fail closed: refuse unknown ids rather than reporting success.
        let existing = SavedContextService::get_by_id(&ctx.database, &self.saved_context_id)?
            .ok_or(KernelError::SavedContextNotFound)?;
        let deleted = SavedContextService::delete_by_id(&ctx.database, &existing.id)?;
        if !deleted {
            return Err(KernelError::SavedContextNotFound);
        }
        Ok(())
    }
}

/// Preview payload returned to Experience. Holds the Action plan value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumePlanPreview {
    pub saved_context_id: String,
    pub saved_context_name: String,
    /// User-authored intended next action (PP-P01A). Not an Action effect.
    pub handoff_note: String,
    pub plan: ActionPlan,
    /// Compatibility / confidence derived from plan dispositions (additive; UI may ignore).
    pub compatibility: RestoreCompatibilitySummary,
}

/// Resolves a restore plan for a saved context. Mutates nothing.
pub struct ResolveResumePlan {
    pub saved_context_id: SavedContextId,
}

impl crate::commands::Command for ResolveResumePlan {
    fn name(&self) -> &'static str {
        "ResolveResumePlan"
    }
}

impl QueryCommand for ResolveResumePlan {
    type Output = ResumePlanPreview;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::Resource(ResourceKind::Workspace)
    }

    fn required_capability(&self) -> Capability {
        Capability::action_plan_resolve()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<ResumePlanPreview> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        let context = SavedContextService::get_by_id(&ctx.database, &self.saved_context_id)?
            .ok_or(KernelError::SavedContextNotFound)?;

        WorkspaceRuntimeStateService::note_execution_phase(RestoreExecutionPhase::Planning);
        // Companion copies fields; Action never sees the saved-context id.
        let request = action_request_from_saved_context(&context);
        let mutator = platform_window_mutator();
        let plan =
            DesktopActionService::resolve_plan(&request, &ctx.capability_set, mutator.as_ref())?;

        let compatibility = RestoreCompatibilitySummary::from_plan(&plan);
        WorkspaceRuntimeStateService::note_compatibility(&compatibility);
        WorkspaceRuntimeStateService::note_execution_phase(RestoreExecutionPhase::Idle);
        let _ = WorkspaceSessionStore::checkpoint_current(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
            None,
        );
        Ok(ResumePlanPreview {
            saved_context_id: context.id.to_string(),
            saved_context_name: context.name,
            handoff_note: context.handoff_note,
            plan,
            compatibility,
        })
    }
}

/// Executes an approved resume plan. Requires explicit approval binding.
pub struct ExecuteResumePlan {
    pub plan: ActionPlan,
    /// The user must confirm the plan digest they reviewed.
    pub approved_plan_digest: String,
}

impl crate::commands::Command for ExecuteResumePlan {
    fn name(&self) -> &'static str {
        "ExecuteResumePlan"
    }
}

impl MutationCommand for ExecuteResumePlan {
    type Output = ActionOperationResult;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Point-of-use checks enforce per-item place/focus scopes.
        Capability::action_plan_resolve()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            serde_json::json!({
                "operation_id": output.operation_id,
                "outcome": output.outcome,
                "item_count": output.items.len(),
                "plan_digest": self.plan.plan_digest,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<ActionOperationResult> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        if self.approved_plan_digest != self.plan.plan_digest {
            return Err(KernelError::DesktopAction(
                workspace_domain::DesktopActionError::PlanUnknown,
            ));
        }

        // Approval produces one effect proof per will_attempt item.
        let proofs: Vec<ItemEffectProof> = DesktopActionService::proofs_for_plan(
            &self.plan,
            ctx.actor_context.actor.id.as_str(),
        );

        // Matching authority alone is insufficient — each item still needs its
        // effect scope at point of use (ADM-AC-18).
        let mutator = platform_window_mutator();
        let result = RestoreExecutor::execute(
            &self.plan,
            &proofs,
            &ctx.capability_set,
            mutator.as_ref(),
            &ActionExecutionControls::default(),
        )?;
        let _ = WorkspaceSessionStore::checkpoint_current(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
            None,
        );
        Ok(result)
    }
}

/// Test-only execute path with an injected mutator.
#[cfg(test)]
pub(crate) fn resolve_and_execute_for_tests(
    context: &SavedContext,
    capability_set: &workspace_domain::CapabilitySet,
    mutator: &dyn WindowMutator,
    controls: &ActionExecutionControls,
) -> Result<(ActionPlan, ActionOperationResult)> {
    let request = action_request_from_saved_context(context);
    let plan = DesktopActionService::resolve_plan(&request, capability_set, mutator)?;
    let proofs = DesktopActionService::proofs_for_plan(&plan, "local-user");
    let result = RestoreExecutor::execute(&plan, &proofs, capability_set, mutator, controls)?;
    Ok((plan, result))
}
