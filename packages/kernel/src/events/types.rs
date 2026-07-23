use std::fmt;

use crate::lifecycle::LifecycleState;

/// Metadata for workspace startup beginning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceStarted {
    pub version: String,
}

/// Metadata for workspace reaching ready state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceReady {
    pub version: String,
    pub lifecycle: LifecycleState,
}

/// Metadata for workspace shutdown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceShutdown;

/// Metadata for persisted settings changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsChanged {
    pub theme: String,
    pub first_run: bool,
    pub settings_version: u32,
}

/// Internal domain events for kernel communication (Sprint 04).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainEvent {
    WorkspaceStarted(WorkspaceStarted),
    WorkspaceReady(WorkspaceReady),
    WorkspaceShutdown(WorkspaceShutdown),
    SettingsChanged(SettingsChanged),
}

impl DomainEvent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::WorkspaceStarted(_) => "system.workspace.started",
            Self::WorkspaceReady(_) => "system.workspace.ready",
            Self::WorkspaceShutdown(_) => "system.workspace.shutdown",
            Self::SettingsChanged(_) => "system.settings.changed",
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
