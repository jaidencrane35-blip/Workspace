//! Shared helpers for resource commands.

use crate::commands::context::CommandContext;
use crate::error::{KernelError, Result};
use crate::events::types::ResourceLifecycleEvent;
use crate::lifecycle::LifecycleState;
use workspace_domain::{Capability, ResourceRef};

pub fn ensure_ready(ctx: &CommandContext<'_>) -> Result<()> {
    if ctx.state.lifecycle == LifecycleState::Ready {
        Ok(())
    } else {
        Err(KernelError::NotReady)
    }
}

pub fn resource_lifecycle_event(
    resource_ref: ResourceRef,
    workspace_id: Option<String>,
    ctx: &CommandContext<'_>,
    capability: Capability,
) -> ResourceLifecycleEvent {
    ResourceLifecycleEvent {
        resource_ref,
        workspace_id,
        actor: Some(ctx.actor_context.clone()),
        intent: Some(ctx.intent_context.clone()),
        capability: Some(capability),
    }
}

pub fn ensure_workspace_exists(
    ctx: &CommandContext<'_>,
    workspace_id: &workspace_domain::WorkspaceId,
) -> Result<()> {
    let exists = ctx.with_database(|db| crate::services::WorkspaceService::exists(db, workspace_id))?;
    if exists {
        Ok(())
    } else {
        Err(KernelError::WorkspaceNotFound)
    }
}
