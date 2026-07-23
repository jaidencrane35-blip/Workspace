use serde::{Deserialize, Serialize};

use crate::lifecycle::LifecycleState;

/// Authoritative workspace runtime state (Rust-owned).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub version: String,
    pub lifecycle: LifecycleState,
}

impl WorkspaceState {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            lifecycle: LifecycleState::Starting,
        }
    }

    pub fn transition(&mut self, next: LifecycleState) {
        log::debug!(
            "lifecycle transition: {} -> {}",
            self.lifecycle.as_str(),
            next.as_str()
        );
        self.lifecycle = next;
    }

    pub fn is_ready(&self) -> bool {
        self.lifecycle == LifecycleState::Ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_in_starting_state() {
        let state = WorkspaceState::new("0.1.0");
        assert_eq!(state.lifecycle, LifecycleState::Starting);
        assert!(!state.is_ready());
    }

    #[test]
    fn transitions_through_lifecycle() {
        let mut state = WorkspaceState::new("0.1.0");
        state.transition(LifecycleState::Initializing);
        state.transition(LifecycleState::Ready);
        assert!(state.is_ready());
    }
}
