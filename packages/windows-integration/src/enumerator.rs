use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Point-in-time observation of a top-level desktop window.
///
/// Sanitized for IPC: no handles leaked as raw pointers; hwnd as hex string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopWindowSnapshot {
    /// Window handle as hex (e.g. `0x00000000001A02B4`).
    pub hwnd: String,
    pub title: String,
    pub process_id: u32,
    pub visible: bool,
}

/// Abstracts OS window enumeration for the Platform Kernel.
pub trait WindowEnumerator: Send + Sync {
    fn enumerate_windows(&self) -> Result<Vec<DesktopWindowSnapshot>>;
}
