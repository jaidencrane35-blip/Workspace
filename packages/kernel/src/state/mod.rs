use serde::{Deserialize, Serialize};

/// Runtime lifecycle status owned by the Platform Kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeStatus {
    Starting,
    Running,
    ShuttingDown,
}

impl RuntimeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Running => "running",
            Self::ShuttingDown => "shutting_down",
        }
    }
}

/// Initialization progress for the workspace core.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InitializationState {
    Uninitialized,
    Initializing,
    Ready,
    Failed,
}

impl InitializationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Uninitialized => "uninitialized",
            Self::Initializing => "initializing",
            Self::Ready => "ready",
            Self::Failed => "failed",
        }
    }
}

/// Authoritative workspace runtime state (Rust-owned).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub version: String,
    pub runtime_status: RuntimeStatus,
    pub initialization: InitializationState,
}

impl WorkspaceState {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            runtime_status: RuntimeStatus::Starting,
            initialization: InitializationState::Uninitialized,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.initialization == InitializationState::Ready
            && self.runtime_status == RuntimeStatus::Running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_with_starting_state() {
        let state = WorkspaceState::new("0.1.0");
        assert_eq!(state.version, "0.1.0");
        assert_eq!(state.runtime_status, RuntimeStatus::Starting);
        assert_eq!(state.initialization, InitializationState::Uninitialized);
        assert!(!state.is_ready());
    }

    #[test]
    fn ready_when_initialized_and_running() {
        let state = WorkspaceState {
            version: "0.1.0".into(),
            runtime_status: RuntimeStatus::Running,
            initialization: InitializationState::Ready,
        };
        assert!(state.is_ready());
    }
}
