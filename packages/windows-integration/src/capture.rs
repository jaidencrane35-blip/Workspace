//! Rich desktop observation capture (Sprint 104).
//!
//! OS-neutral capture DTOs. No SQLite, workspace, or cognition concepts.

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Metadata for one capture pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureMetadata {
    pub source: String,
    pub duration_ms: Option<u64>,
}

/// One monitor observed during capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapturedDesktopMonitor {
    pub index: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub work_x: i32,
    pub work_y: i32,
    pub work_width: i32,
    pub work_height: i32,
    pub is_primary: bool,
}

/// One top-level window observed during capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapturedDesktopWindow {
    pub hwnd: String,
    pub title: String,
    pub process_id: u32,
    /// Ephemeral observation aid (image basename). Not part of SavedContext scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,
    pub visible: bool,
    pub minimized: bool,
    pub focused: bool,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub monitor_index: Option<i32>,
    pub z_order: Option<i32>,
}

impl CapturedDesktopWindow {
    /// Maps to the legacy four-field snapshot for existing callers.
    pub fn to_legacy_snapshot(&self) -> super::enumerator::DesktopWindowSnapshot {
        super::enumerator::DesktopWindowSnapshot {
            hwnd: self.hwnd.clone(),
            title: self.title.clone(),
            process_id: self.process_id,
            visible: self.visible,
            focused: self.focused,
            minimized: self.minimized,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            monitor_index: self.monitor_index,
            monitor_name: None,
        }
    }
}

/// Opaque identifier for one interactive Windows desktop session.
pub const STUB_DESKTOP_SESSION_ID: &str = "stub-desktop-session-1";

/// Full desktop observation payload from one OS capture pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopObservationCapture {
    /// Stable for one interactive logon desktop; changes after logoff/reboot.
    pub desktop_session_id: String,
    pub foreground_hwnd: Option<String>,
    pub windows: Vec<CapturedDesktopWindow>,
    pub monitors: Vec<CapturedDesktopMonitor>,
    pub metadata: CaptureMetadata,
}

impl DesktopObservationCapture {
    pub fn empty_stub() -> Self {
        Self {
            desktop_session_id: STUB_DESKTOP_SESSION_ID.into(),
            foreground_hwnd: None,
            windows: Vec::new(),
            monitors: Vec::new(),
            metadata: CaptureMetadata {
                source: "stub".into(),
                duration_ms: Some(0),
            },
        }
    }

    /// Legacy window list — visible, non-minimized windows only (Phase 1 behaviour).
    pub fn legacy_window_snapshots(&self) -> Vec<super::enumerator::DesktopWindowSnapshot> {
        self.windows
            .iter()
            .filter(|window| window.visible && !window.minimized)
            .filter(|window| !window.title.trim().is_empty())
            .map(CapturedDesktopWindow::to_legacy_snapshot)
            .collect()
    }
}

/// Abstracts a full desktop observation capture pass.
pub trait DesktopCapturer: Send + Sync {
    fn capture_desktop(&self) -> Result<DesktopObservationCapture>;
}

/// Assigns a monitor index by window center point containment. No guessing beyond geometry.
pub fn monitor_index_for_point(
    center_x: i32,
    center_y: i32,
    monitors: &[CapturedDesktopMonitor],
) -> Option<i32> {
    monitors
        .iter()
        .find(|monitor| {
            center_x >= monitor.x
                && center_x < monitor.x + monitor.width
                && center_y >= monitor.y
                && center_y < monitor.y + monitor.height
        })
        .map(|monitor| monitor.index)
}

pub fn monitor_index_for_window_bounds(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    monitors: &[CapturedDesktopMonitor],
) -> Option<i32> {
    if width <= 0 || height <= 0 {
        return None;
    }
    let center_x = x + width / 2;
    let center_y = y + height / 2;
    monitor_index_for_point(center_x, center_y, monitors)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_monitors() -> Vec<CapturedDesktopMonitor> {
        vec![
            CapturedDesktopMonitor {
                index: 0,
                name: "Primary".into(),
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
                work_x: 0,
                work_y: 0,
                work_width: 1920,
                work_height: 1040,
                is_primary: true,
            },
            CapturedDesktopMonitor {
                index: 1,
                name: "Secondary".into(),
                x: 1920,
                y: 0,
                width: 1280,
                height: 720,
                work_x: 1920,
                work_y: 0,
                work_width: 1280,
                work_height: 720,
                is_primary: false,
            },
        ]
    }

    #[test]
    fn focused_window_marked_correctly() {
        let capture = DesktopObservationCapture {
            desktop_session_id: STUB_DESKTOP_SESSION_ID.into(),
            foreground_hwnd: Some("0x00000000000000AA".into()),
            windows: vec![
                CapturedDesktopWindow {
                    hwnd: "0x00000000000000AA".into(),
                    title: "Focused".into(),
                    process_id: 1,
                    process_name: None,
                    visible: true,
                    minimized: false,
                    focused: true,
                    x: 10,
                    y: 10,
                    width: 800,
                    height: 600,
                    monitor_index: Some(0),
                    z_order: Some(0),
                },
                CapturedDesktopWindow {
                    hwnd: "0x00000000000000BB".into(),
                    title: "Background".into(),
                    process_id: 2,
                    process_name: None,
                    visible: true,
                    minimized: false,
                    focused: false,
                    x: 20,
                    y: 20,
                    width: 400,
                    height: 300,
                    monitor_index: Some(0),
                    z_order: Some(1),
                },
            ],
            monitors: sample_monitors(),
            metadata: CaptureMetadata {
                source: "test".into(),
                duration_ms: Some(1),
            },
        };

        assert_eq!(
            capture.foreground_hwnd.as_deref(),
            Some("0x00000000000000AA")
        );
        assert!(capture.windows[0].focused);
        assert!(!capture.windows[1].focused);
    }

    #[test]
    fn window_bounds_captured() {
        let window = CapturedDesktopWindow {
            hwnd: "0x1".into(),
            title: "Bounds".into(),
            process_id: 9,
            process_name: None,
            visible: true,
            minimized: false,
            focused: false,
            x: 100,
            y: 200,
            width: 640,
            height: 480,
            monitor_index: Some(0),
            z_order: Some(0),
        };
        assert_eq!((window.x, window.y, window.width, window.height), (100, 200, 640, 480));
    }

    #[test]
    fn minimized_state_captured() {
        let window = CapturedDesktopWindow {
            hwnd: "0x2".into(),
            title: "Min".into(),
            process_id: 3,
            process_name: None,
            visible: false,
            minimized: true,
            focused: false,
            x: -32000,
            y: -32000,
            width: 160,
            height: 28,
            monitor_index: None,
            z_order: Some(2),
        };
        assert!(window.minimized);
    }

    #[test]
    fn window_assigned_to_correct_monitor() {
        let monitors = sample_monitors();
        assert_eq!(
            monitor_index_for_window_bounds(100, 100, 800, 600, &monitors),
            Some(0)
        );
        assert_eq!(
            monitor_index_for_window_bounds(2000, 100, 800, 600, &monitors),
            Some(1)
        );
    }

    #[test]
    fn unknown_monitor_remains_null() {
        let monitors = sample_monitors();
        assert_eq!(
            monitor_index_for_window_bounds(5000, 5000, 100, 100, &monitors),
            None
        );
    }

    #[test]
    fn empty_desktop_capture_works() {
        let capture = DesktopObservationCapture::empty_stub();
        assert!(capture.windows.is_empty());
        assert!(capture.monitors.is_empty());
        assert!(capture.foreground_hwnd.is_none());
    }

    #[test]
    fn legacy_snapshots_exclude_minimized_windows() {
        let capture = DesktopObservationCapture {
            desktop_session_id: STUB_DESKTOP_SESSION_ID.into(),
            foreground_hwnd: None,
            windows: vec![
                CapturedDesktopWindow {
                    hwnd: "0x1".into(),
                    title: "Open".into(),
                    process_id: 1,
                    process_name: None,
                    visible: true,
                    minimized: false,
                    focused: false,
                    x: 0,
                    y: 0,
                    width: 100,
                    height: 100,
                    monitor_index: None,
                    z_order: None,
                },
                CapturedDesktopWindow {
                    hwnd: "0x2".into(),
                    title: "Min".into(),
                    process_id: 2,
                    process_name: None,
                    visible: false,
                    minimized: true,
                    focused: false,
                    x: 0,
                    y: 0,
                    width: 100,
                    height: 100,
                    monitor_index: None,
                    z_order: None,
                },
            ],
            monitors: Vec::new(),
            metadata: CaptureMetadata {
                source: "test".into(),
                duration_ms: None,
            },
        };

        let legacy = capture.legacy_window_snapshots();
        assert_eq!(legacy.len(), 1);
        assert_eq!(legacy[0].title, "Open");
    }
}
