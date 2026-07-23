use std::fmt;

use workspace_domain::{ActorContext, Capability, IntentContext};

use crate::lifecycle::LifecycleState;

/// Metadata for workspace startup beginning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceStarted {
    pub version: String,
    pub intent: Option<IntentContext>,
    pub capability: Option<Capability>,
}

/// Metadata for workspace reaching ready state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceReady {
    pub version: String,
    pub lifecycle: LifecycleState,
    pub intent: Option<IntentContext>,
    pub capability: Option<Capability>,
}

/// Metadata for workspace shutdown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceShutdown {
    pub actor: Option<ActorContext>,
    pub intent: Option<IntentContext>,
    pub capability: Option<Capability>,
}

/// Metadata for persisted settings changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsChanged {
    pub theme: String,
    pub first_run: bool,
    pub settings_version: u32,
    pub actor: Option<ActorContext>,
    pub intent: Option<IntentContext>,
    pub capability: Option<Capability>,
}

/// Metadata when a workspace entity is created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceEntityCreated {
    pub workspace_id: String,
    pub name: String,
    pub actor: Option<ActorContext>,
    pub intent: Option<IntentContext>,
    pub capability: Option<Capability>,
}

/// Metadata when a workspace entity is updated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceEntityUpdated {
    pub workspace_id: String,
    pub name: String,
    pub actor: Option<ActorContext>,
    pub intent: Option<IntentContext>,
    pub capability: Option<Capability>,
}

/// Internal domain events for kernel communication (Sprint 04+).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainEvent {
    WorkspaceStarted(WorkspaceStarted),
    WorkspaceReady(WorkspaceReady),
    WorkspaceShutdown(WorkspaceShutdown),
    SettingsChanged(SettingsChanged),
    WorkspaceCreated(WorkspaceEntityCreated),
    WorkspaceUpdated(WorkspaceEntityUpdated),
}

impl DomainEvent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::WorkspaceStarted(_) => "system.workspace.started",
            Self::WorkspaceReady(_) => "system.workspace.ready",
            Self::WorkspaceShutdown(_) => "system.workspace.shutdown",
            Self::SettingsChanged(_) => "system.settings.changed",
            Self::WorkspaceCreated(_) => "workspace.entity.created",
            Self::WorkspaceUpdated(_) => "workspace.entity.updated",
        }
    }

    pub fn actor(&self) -> Option<&ActorContext> {
        match self {
            Self::WorkspaceShutdown(payload) => payload.actor.as_ref(),
            Self::SettingsChanged(payload) => payload.actor.as_ref(),
            Self::WorkspaceCreated(payload) => payload.actor.as_ref(),
            Self::WorkspaceUpdated(payload) => payload.actor.as_ref(),
            _ => None,
        }
    }

    pub fn intent(&self) -> Option<&IntentContext> {
        match self {
            Self::WorkspaceStarted(payload) => payload.intent.as_ref(),
            Self::WorkspaceReady(payload) => payload.intent.as_ref(),
            Self::WorkspaceShutdown(payload) => payload.intent.as_ref(),
            Self::SettingsChanged(payload) => payload.intent.as_ref(),
            Self::WorkspaceCreated(payload) => payload.intent.as_ref(),
            Self::WorkspaceUpdated(payload) => payload.intent.as_ref(),
        }
    }

    pub fn capability(&self) -> Option<&Capability> {
        match self {
            Self::WorkspaceStarted(payload) => payload.capability.as_ref(),
            Self::WorkspaceReady(payload) => payload.capability.as_ref(),
            Self::WorkspaceShutdown(payload) => payload.capability.as_ref(),
            Self::SettingsChanged(payload) => payload.capability.as_ref(),
            Self::WorkspaceCreated(payload) => payload.capability.as_ref(),
            Self::WorkspaceUpdated(payload) => payload.capability.as_ref(),
        }
    }
}

impl fmt::Display for DomainEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Typed internal event interface.
pub trait Event: Send + Sync {
    fn event_name(&self) -> &'static str;
}

impl Event for DomainEvent {
    fn event_name(&self) -> &'static str {
        self.name()
    }
}
