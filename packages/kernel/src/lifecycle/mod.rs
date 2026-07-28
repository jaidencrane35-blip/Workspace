use serde::{Deserialize, Serialize};

/// Unified workspace lifecycle states (Sprint 03).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    Starting,
    Initializing,
    Ready,
    Error,
    ShuttingDown,
}

impl LifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Initializing => "initializing",
            Self::Ready => "ready",
            Self::Error => "error",
            Self::ShuttingDown => "shutting_down",
        }
    }

    pub fn is_initialized(self) -> bool {
        self == Self::Ready
    }

    pub fn allows_transition(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Starting, Self::Initializing)
                | (Self::Starting, Self::Error)
                | (Self::Initializing, Self::Ready)
                | (Self::Initializing, Self::Error)
                | (Self::Ready, Self::Error)
                | (Self::Ready, Self::ShuttingDown)
        )
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Error | Self::ShuttingDown)
    }
}

impl Default for LifecycleState {
    fn default() -> Self {
        Self::Starting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_state_is_initialized() {
        assert!(LifecycleState::Ready.is_initialized());
        assert!(!LifecycleState::Initializing.is_initialized());
    }

    #[test]
    fn lifecycle_matrix_keeps_terminal_states_terminal() {
        assert!(LifecycleState::Starting.allows_transition(LifecycleState::Initializing));
        assert!(LifecycleState::Initializing.allows_transition(LifecycleState::Ready));
        assert!(LifecycleState::Ready.allows_transition(LifecycleState::ShuttingDown));
        assert!(!LifecycleState::Starting.allows_transition(LifecycleState::Ready));
        assert!(!LifecycleState::Ready.allows_transition(LifecycleState::Initializing));
        assert!(!LifecycleState::Error.allows_transition(LifecycleState::Ready));
        assert!(!LifecycleState::ShuttingDown.allows_transition(LifecycleState::Ready));
        assert!(LifecycleState::Error.is_terminal());
        assert!(LifecycleState::ShuttingDown.is_terminal());
    }
}
