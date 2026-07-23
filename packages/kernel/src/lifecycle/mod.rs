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
}
