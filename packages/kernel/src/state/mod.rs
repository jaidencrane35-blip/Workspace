use serde::{Deserialize, Serialize};

use crate::error::{KernelError, Result};
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

    pub fn transition(&mut self, next: LifecycleState) -> Result<()> {
        if !self.lifecycle.allows_transition(next) {
            return Err(KernelError::IntegrityViolation {
                message: format!(
                    "invalid workspace lifecycle transition: {} -> {}",
                    self.lifecycle.as_str(),
                    next.as_str()
                ),
            });
        }
        log::debug!(
            "lifecycle transition: {} -> {}",
            self.lifecycle.as_str(),
            next.as_str()
        );
        self.lifecycle = next;
        Ok(())
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
        state.transition(LifecycleState::Initializing).unwrap();
        state.transition(LifecycleState::Ready).unwrap();
        assert!(state.is_ready());
    }

    #[test]
    fn rejects_skipped_backward_and_terminal_transitions() {
        let mut state = WorkspaceState::new("0.1.0");
        assert!(matches!(
            state.transition(LifecycleState::Ready),
            Err(KernelError::IntegrityViolation { .. })
        ));
        assert_eq!(state.lifecycle, LifecycleState::Starting);
        state.transition(LifecycleState::Initializing).unwrap();
        state.transition(LifecycleState::Error).unwrap();
        assert!(matches!(
            state.transition(LifecycleState::Ready),
            Err(KernelError::IntegrityViolation { .. })
        ));
        assert_eq!(state.lifecycle, LifecycleState::Error);
    }
}
