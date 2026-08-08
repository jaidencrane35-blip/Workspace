//! Deterministic window enumeration fixtures for Application Provider tests.

use super::enumerator::{DesktopWindowSnapshot, WindowEnumerator};
use super::stub::dual_monitor_fixture;
use crate::error::Result;

/// Enumerator backed by the dual-monitor stub fixture (shared with StubWindowMutator).
#[derive(Debug, Default, Clone, Copy)]
pub struct FixtureWindowEnumerator;

impl WindowEnumerator for FixtureWindowEnumerator {
    fn enumerate_windows(&self) -> Result<Vec<DesktopWindowSnapshot>> {
        let capture = dual_monitor_fixture();
        Ok(capture
            .windows
            .into_iter()
            .map(|window| DesktopWindowSnapshot {
                hwnd: window.hwnd,
                title: window.title,
                process_id: window.process_id,
                process_name: window.process_name,
                visible: window.visible,
                focused: window.focused,
                minimized: window.minimized,
                x: window.x,
                y: window.y,
                width: window.width,
                height: window.height,
                monitor_index: window.monitor_index,
                monitor_name: None,
            })
            .collect())
    }
}
