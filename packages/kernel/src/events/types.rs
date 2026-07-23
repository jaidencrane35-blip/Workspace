use std::fmt;

use workspace_domain::{ActorContext, Capability, IntentContext, ResourceRef};

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

/// Generic resource lifecycle metadata (Sprint 12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLifecycleEvent {
    pub resource_ref: ResourceRef,
    pub workspace_id: Option<String>,
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
    ResourceCreated(ResourceLifecycleEvent),
    ResourceUpdated(ResourceLifecycleEvent),
    ResourceDeleted(ResourceLifecycleEvent),
    ZoneCreated(ResourceLifecycleEvent),
    ApplicationCreated(ResourceLifecycleEvent),
    WidgetCreated(ResourceLifecycleEvent),
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
            Self::ResourceCreated(_) => "resource.created",
            Self::ResourceUpdated(_) => "resource.updated",
            Self::ResourceDeleted(_) => "resource.deleted",
            Self::ZoneCreated(_) => "zone.created",
            Self::ApplicationCreated(_) => "application.created",
            Self::WidgetCreated(_) => "widget.created",
        }
    }

    pub fn actor(&self) -> Option<&ActorContext> {
        match self {
            Self::WorkspaceShutdown(payload) => payload.actor.as_ref(),
            Self::SettingsChanged(payload) => payload.actor.as_ref(),
            Self::WorkspaceCreated(payload) => payload.actor.as_ref(),
            Self::WorkspaceUpdated(payload) => payload.actor.as_ref(),
            Self::ResourceCreated(payload) => payload.actor.as_ref(),
            Self::ResourceUpdated(payload) => payload.actor.as_ref(),
            Self::ResourceDeleted(payload) => payload.actor.as_ref(),
            Self::ZoneCreated(payload) => payload.actor.as_ref(),
            Self::ApplicationCreated(payload) => payload.actor.as_ref(),
            Self::WidgetCreated(payload) => payload.actor.as_ref(),
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
            Self::ResourceCreated(payload) => payload.intent.as_ref(),
            Self::ResourceUpdated(payload) => payload.intent.as_ref(),
            Self::ResourceDeleted(payload) => payload.intent.as_ref(),
            Self::ZoneCreated(payload) => payload.intent.as_ref(),
            Self::ApplicationCreated(payload) => payload.intent.as_ref(),
            Self::WidgetCreated(payload) => payload.intent.as_ref(),
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
            Self::ResourceCreated(payload) => payload.capability.as_ref(),
            Self::ResourceUpdated(payload) => payload.capability.as_ref(),
            Self::ResourceDeleted(payload) => payload.capability.as_ref(),
            Self::ZoneCreated(payload) => payload.capability.as_ref(),
            Self::ApplicationCreated(payload) => payload.capability.as_ref(),
            Self::WidgetCreated(payload) => payload.capability.as_ref(),
        }
    }

    pub fn resource_ref(&self) -> Option<&ResourceRef> {
        match self {
            Self::ResourceCreated(payload) => Some(&payload.resource_ref),
            Self::ResourceUpdated(payload) => Some(&payload.resource_ref),
            Self::ResourceDeleted(payload) => Some(&payload.resource_ref),
            Self::ZoneCreated(payload) => Some(&payload.resource_ref),
            Self::ApplicationCreated(payload) => Some(&payload.resource_ref),
            Self::WidgetCreated(payload) => Some(&payload.resource_ref),
            _ => None,
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
