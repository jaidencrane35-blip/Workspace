use std::fmt;

use serde::Serialize;
use workspace_kernel::KernelError;

/// Safe IPC error returned to the React frontend.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CommandError {
    pub code: String,
    pub message: String,
}

impl CommandError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl std::error::Error for CommandError {}

impl From<KernelError> for CommandError {
    fn from(error: KernelError) -> Self {
        let public = error.to_public();
        Self {
            code: public.code,
            message: public.message,
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for CommandError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::new("internal_error", "Workspace core is temporarily unavailable.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_kernel::KernelError;

    #[test]
    fn maps_kernel_errors_without_internals() {
        let cmd_err = CommandError::from(KernelError::NotReady);
        assert_eq!(cmd_err.code, "not_ready");
        assert!(!cmd_err.message.contains("KernelError"));
    }
}
