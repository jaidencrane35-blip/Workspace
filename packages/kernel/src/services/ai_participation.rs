//! AI participation boundary (Sprint 46).
//!
//! Builds [`AiActionRequest`] proposals. Submission always happens through
//! [`crate::commands::CommandHandler`] → CommandPipeline → Permission Gateway.
//! This module never calls privileged launch or approval mutators.

use workspace_domain::{
    ActorType, AiActionRequest, ApplicationId, IntentType,
};

use crate::commands::CommandContext;
use crate::error::{KernelError, Result};

/// AI proposal helpers — no execution authority.
pub(crate) struct AiParticipationService;

impl AiParticipationService {
    /// Builds a launch proposal ("what AI wants") without executing anything.
    pub(crate) fn propose_application_launch(
        actor_id: impl Into<String>,
        application_id: &ApplicationId,
        reason: Option<String>,
    ) -> Result<AiActionRequest> {
        AiActionRequest::propose_application_launch(actor_id, application_id, reason)
            .map_err(KernelError::from)
    }

    /// Validates that a command context is a legitimate AI submission for `request`.
    pub(crate) fn ensure_ai_submission_context(
        ctx: &CommandContext<'_>,
        request: &AiActionRequest,
    ) -> Result<()> {
        request.validate().map_err(KernelError::from)?;

        if request.command_name != "LaunchApplication" {
            return Err(KernelError::AiRequestValidation {
                message: format!(
                    "unsupported AI command '{}'; only LaunchApplication is wired",
                    request.command_name
                ),
            });
        }
        if ctx.actor_context.actor.actor_type != ActorType::AIAssistant {
            return Err(KernelError::AiRequestValidation {
                message: "AI submissions require an AIAssistant actor context".into(),
            });
        }
        if ctx.actor_context.actor.id.as_str() != request.requesting_actor_id.as_str() {
            return Err(KernelError::AiRequestValidation {
                message: "command context actor must match the AI action request".into(),
            });
        }
        if ctx.intent_context.intent.intent_type != IntentType::AISuggestion {
            return Err(KernelError::AiRequestValidation {
                message: "AI submissions must use IntentType::AISuggestion".into(),
            });
        }
        Ok(())
    }
}
