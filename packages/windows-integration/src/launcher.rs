//! Process launch boundary (Sprint 41).
//!
//! OS process creation lives only in this crate — never in kernel or UI.

use crate::error::Result;

/// Request to start a desktop process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessLaunchRequest {
    pub executable: String,
    pub args: Vec<String>,
}

/// Outcome of a process launch attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessLaunchOutcome {
    pub process_id: Option<u32>,
    /// True when launch was simulated (stub / non-Windows).
    pub simulated: bool,
}

/// Platform process launcher — Win32 on Windows, stub elsewhere.
pub trait ProcessLauncher: Send + Sync {
    fn launch(&self, request: &ProcessLaunchRequest) -> Result<ProcessLaunchOutcome>;
}
