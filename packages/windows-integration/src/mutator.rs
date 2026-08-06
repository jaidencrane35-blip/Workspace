//! Bounded window mutation and live identity lookup (DEC-008).
//!
//! Action consumes this trait for exact-session matching and declared effects.
//! It must not expose candidate lists or environment models.

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Live facts Action may compare for one declared hwnd.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveWindowView {
    pub hwnd: String,
    pub process_id: u32,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowPlacementRequest {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub minimized: bool,
}

/// Outcome of one OS effect attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutatorEffectOutcome {
    Committed,
    RefusedByEnvironment,
    OutcomeUnknown,
}

/// OS adapter for desktop-session identity, bounded hwnd lookup, and effects.
pub trait WindowMutator: Send + Sync {
    fn current_desktop_session_id(&self) -> Result<String>;

    /// Returns the live window for exactly this hwnd, or None if absent.
    fn window_by_hwnd(&self, hwnd: &str) -> Result<Option<LiveWindowView>>;

    /// Attached monitor indices for placement satisfiability checks.
    fn attached_monitor_indices(&self) -> Result<Vec<i32>>;

    fn place_window(&self, hwnd: &str, placement: &WindowPlacementRequest) -> Result<MutatorEffectOutcome>;

    fn focus_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome>;

    /// Minimize a top-level window (Application Provider operation).
    fn minimize_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome>;

    /// Restore a minimized top-level window without relocating it.
    fn restore_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome>;

    /// Request close via WM_CLOSE (graceful). Does not force-kill.
    fn close_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome>;
}
