use serde_json::json;

use crate::commands::context::CommandContext;
use crate::commands::resource::ensure_workspace_exists;
use crate::commands::r#trait::MutationCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::security::PermissionSubject;
use crate::services::SuggestionService;
use workspace_domain::{Capability, ResourceRef, Suggestion, SuggestionStatus, WorkspaceId};

/// Records explicit user dismissal of a proposal.
///
/// Decision only: does not execute Automate-stage workspace mutations.
pub struct RejectSuggestion {
    pub workspace_id: WorkspaceId,
    pub suggestion_id: String,
}

impl RejectSuggestion {
    pub fn new(workspace_id: WorkspaceId, suggestion_id: String) -> Self {
        Self {
            workspace_id,
            suggestion_id,
        }
    }
}

impl crate::commands::Command for RejectSuggestion {
    fn name(&self) -> &'static str {
        "RejectSuggestion"
    }
}

impl MutationCommand for RejectSuggestion {
    type Output = Suggestion;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        // Decision mutates governance history (audit), not workspace entities.
        Capability::audit_write()
    }

    fn audit_resource_ref(&self, output: &Self::Output) -> Option<ResourceRef> {
        output.related_resource_ref.clone()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            json!({
                "suggestion_id": output.id,
                "suggestion_type": output.suggestion_type,
                "decision": "rejected",
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<Suggestion> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        ensure_workspace_exists(ctx, &self.workspace_id)?;

        let suggestion = SuggestionService::resolve_pending(
            &ctx.database,
            &ctx.actor_context,
            &ctx.intent_context,
            &ctx.capability_set,
            ctx.permission_policy,
            ctx.permission_gate,
            &self.workspace_id,
            &self.suggestion_id,
        )?;

        let rejected = suggestion
            .reject()
            .map_err(|error| KernelError::SuggestionValidation {
                message: error.to_string(),
            })?;

        assert_eq!(rejected.status, SuggestionStatus::Rejected);
        Ok(rejected)
    }
}
