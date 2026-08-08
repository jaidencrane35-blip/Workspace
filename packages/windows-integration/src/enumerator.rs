use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Point-in-time observation of a top-level desktop window.
///
/// Sanitized for IPC: no handles leaked as raw pointers; hwnd as hex string.
/// Populated from persisted observation snapshots — not live OS enumeration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopWindowSnapshot {
    /// Window handle as hex (e.g. `0x00000000001A02B4`).
    pub hwnd: String,
    pub title: String,
    pub process_id: u32,
    /// Executable image basename of the owning process, exactly as Windows
    /// reports it (`Code.exe`). `None` when the OS declines to say — protected
    /// and elevated processes do, and that must stay distinguishable from a
    /// name, because the only alternative is guessing from the title.
    pub process_name: Option<String>,
    pub visible: bool,
    pub focused: bool,
    pub minimized: bool,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub monitor_index: Option<i32>,
    pub monitor_name: Option<String>,
}

impl DesktopWindowSnapshot {
    /// Minimal snapshot for tests and legacy four-field call sites.
    pub fn legacy(
        hwnd: impl Into<String>,
        title: impl Into<String>,
        process_id: u32,
        visible: bool,
    ) -> Self {
        Self {
            hwnd: hwnd.into(),
            title: title.into(),
            process_id,
            process_name: None,
            visible,
            focused: false,
            minimized: false,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            monitor_index: None,
            monitor_name: None,
        }
    }
}

/// Abstracts OS window enumeration for the Platform Kernel.
pub trait WindowEnumerator: Send + Sync {
    fn enumerate_windows(&self) -> Result<Vec<DesktopWindowSnapshot>>;
}
