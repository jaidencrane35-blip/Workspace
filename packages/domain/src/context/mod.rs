//! Deterministic workspace context boundary (Sprint 19; enriched Sprint 26).
//!
//! `WorkspaceContext` assembles the existing derived read layers — workspace
//! state (projection), activity history (observations), analytics (metrics),
//! authority (capability discovery), and recent execution outcomes — into a
//! single, read-only structure. It is the "Context" stage: a deterministic
//! composition, not intelligence. No AI, memory, learning, or persistence.
//!
//! Pure domain type: no database, IO, or UI dependencies.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::analytics::WorkspaceMetrics;
use crate::discovery::CapabilityDiscovery;
use crate::execution_context::ExecutionContextSummary;
use crate::observation::Observation;
use crate::projection::WorkspaceSnapshot;
use crate::resource::ResourceRef;

/// Context-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ContextError {
    #[error("Context timestamp must not be empty")]
    EmptyTimestamp,

    #[error("Context workspace reference does not match snapshot")]
    WorkspaceRefMismatch,

    #[error("Invalid context component: {0}")]
    InvalidComponent(String),
}

/// A deterministic, read-only composition of the derived workspace read layers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceContext {
    /// RFC 3339 timestamp of when the context was assembled.
    pub generated_at: String,
    /// Canonical identity of the workspace this context describes.
    pub workspace: ResourceRef,
    /// Current workspace state (Sprint 14 projection).
    pub snapshot: WorkspaceSnapshot,
    /// Recent activity history (Sprint 17 observations).
    pub observations: Vec<Observation>,
    /// Deterministic analytics (Sprint 18 metrics).
    pub metrics: WorkspaceMetrics,
    /// Derived actor authority (Sprint 16 capability discovery).
    pub capabilities: CapabilityDiscovery,
    /// Recent execution outcome summary (Sprint 26).
    pub execution_context: ExecutionContextSummary,
}

impl WorkspaceContext {
    pub fn validate(&self) -> Result<(), ContextError> {
        if self.generated_at.trim().is_empty() {
            return Err(ContextError::EmptyTimestamp);
        }

        // ResourceRef identity must be preserved across the composition.
        if self.workspace != self.snapshot.workspace {
            return Err(ContextError::WorkspaceRefMismatch);
        }

        self.snapshot
            .validate()
            .map_err(|error| ContextError::InvalidComponent(format!("snapshot: {error}")))?;

        for observation in &self.observations {
            observation
                .validate()
                .map_err(|error| ContextError::InvalidComponent(format!("observation: {error}")))?;
        }

        self.metrics
            .validate()
            .map_err(|error| ContextError::InvalidComponent(format!("metrics: {error}")))?;

        self.capabilities
            .validate()
            .map_err(|error| ContextError::InvalidComponent(format!("capabilities: {error}")))?;

        self.execution_context.validate().map_err(|error| {
            ContextError::InvalidComponent(format!("execution_context: {error}"))
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::ActorType;
    use crate::capability::Capability;
    use crate::ids::ActorId;
    use crate::projection::WorkspaceSnapshot;
    use crate::resource::{ResourceId, ResourceKind, ResourceRef};

    fn workspace_ref() -> ResourceRef {
        ResourceRef::new(ResourceKind::Workspace, ResourceId::new("ws-1").unwrap())
    }

    fn snapshot(workspace: ResourceRef) -> WorkspaceSnapshot {
        WorkspaceSnapshot {
            workspace,
            workspace_name: "Context WS".into(),
            zones: vec![],
            applications: vec![],
            widgets: vec![],
            layout_id: None,
            layout_placements: vec![],
            relationships: vec![],
            generated_at: "2026-07-24T00:00:00Z".into(),
        }
    }

    fn metrics() -> WorkspaceMetrics {
        WorkspaceMetrics::from_observations(&[], "2026-07-24T00:00:00Z")
    }

    fn capabilities() -> CapabilityDiscovery {
        CapabilityDiscovery {
            actor_id: ActorId::new("local-user").unwrap(),
            actor_type: ActorType::LocalUser,
            capabilities: vec![Capability::workspace_read()],
            available_intents: vec![],
            generated_at: "2026-07-24T00:00:00Z".into(),
        }
    }

    fn context(workspace: ResourceRef) -> WorkspaceContext {
        WorkspaceContext {
            generated_at: "2026-07-24T00:00:00Z".into(),
            workspace: workspace.clone(),
            snapshot: snapshot(workspace),
            observations: vec![],
            metrics: metrics(),
            capabilities: capabilities(),
            execution_context: ExecutionContextSummary::empty(),
        }
    }

    #[test]
    fn validates_complete_context() {
        assert!(context(workspace_ref()).validate().is_ok());
    }

    #[test]
    fn rejects_empty_timestamp() {
        let mut ctx = context(workspace_ref());
        ctx.generated_at = String::new();
        assert_eq!(ctx.validate(), Err(ContextError::EmptyTimestamp));
    }

    #[test]
    fn rejects_workspace_ref_mismatch() {
        let mut ctx = context(workspace_ref());
        ctx.workspace =
            ResourceRef::new(ResourceKind::Workspace, ResourceId::new("ws-other").unwrap());
        assert_eq!(ctx.validate(), Err(ContextError::WorkspaceRefMismatch));
    }

    #[test]
    fn rejects_invalid_execution_context() {
        let mut ctx = context(workspace_ref());
        ctx.execution_context.recent_completed_count = 1;
        assert!(matches!(
            ctx.validate(),
            Err(ContextError::InvalidComponent(_))
        ));
    }

    #[test]
    fn serializes_round_trip() {
        let ctx = context(workspace_ref());
        let json = serde_json::to_string(&ctx).unwrap();
        let restored: WorkspaceContext = serde_json::from_str(&json).unwrap();
        assert_eq!(ctx, restored);
    }

    #[test]
    fn preserves_workspace_resource_ref_identity() {
        let ctx = context(workspace_ref());
        assert_eq!(ctx.workspace, ctx.snapshot.workspace);
        assert_eq!(ctx.workspace.canonical(), "workspace:ws-1");
    }
}
