use super::capture::{
    CaptureMetadata, CapturedDesktopMonitor, CapturedDesktopWindow, DesktopCapturer,
    DesktopObservationCapture,
};
use super::enumerator::{DesktopWindowSnapshot, WindowEnumerator};
use super::launcher::{ProcessLaunchOutcome, ProcessLaunchRequest, ProcessLauncher};
use crate::error::Result;

/// Test / non-Windows capturer with injectable deterministic fixtures.
#[derive(Debug, Clone)]
pub struct StubDesktopCapturer {
    capture: DesktopObservationCapture,
}

impl StubDesktopCapturer {
    pub fn new(capture: DesktopObservationCapture) -> Self {
        Self { capture }
    }

    pub fn empty() -> Self {
        Self {
            capture: DesktopObservationCapture::empty_stub(),
        }
    }

    /// Deterministic two-monitor fixture with one focused window.
    pub fn fixture_dual_monitor() -> Self {
        Self::new(dual_monitor_fixture())
    }
}

impl Default for StubDesktopCapturer {
    fn default() -> Self {
        Self::empty()
    }
}

impl DesktopCapturer for StubDesktopCapturer {
    fn capture_desktop(&self) -> Result<DesktopObservationCapture> {
        Ok(self.capture.clone())
    }
}

/// Test / non-Windows enumerator — returns legacy snapshots from the default empty capture.
#[derive(Debug, Default, Clone, Copy)]
pub struct StubWindowEnumerator;

impl StubWindowEnumerator {
    pub fn with_capture(capture: DesktopObservationCapture) -> StubDesktopCapturer {
        StubDesktopCapturer::new(capture)
    }
}

impl WindowEnumerator for StubWindowEnumerator {
    fn enumerate_windows(&self) -> Result<Vec<DesktopWindowSnapshot>> {
        Ok(Vec::new())
    }
}

/// Non-Windows / test launcher — does not spawn a process.
#[derive(Debug, Default, Clone, Copy)]
pub struct StubProcessLauncher;

impl ProcessLauncher for StubProcessLauncher {
    fn launch(&self, request: &ProcessLaunchRequest) -> Result<ProcessLaunchOutcome> {
        if request.executable.trim().is_empty() {
            return Err(crate::error::WindowsIntegrationError::InvalidLaunchTarget(
                "executable path is required".into(),
            ));
        }
        Ok(ProcessLaunchOutcome {
            process_id: None,
            simulated: true,
        })
    }
}

pub fn dual_monitor_fixture() -> DesktopObservationCapture {
    let foreground = "0x00000000000000AA".to_string();
    DesktopObservationCapture {
        foreground_hwnd: Some(foreground.clone()),
        monitors: vec![
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
        ],
        windows: vec![
            CapturedDesktopWindow {
                hwnd: foreground,
                title: "Fixture Focus".into(),
                process_id: 100,
                process_name: Some("fixture-focus.exe".into()),
                                visible: true,
                minimized: false,
                focused: true,
                x: 100,
                y: 100,
                width: 800,
                height: 600,
                monitor_index: Some(0),
                z_order: Some(0),
            },
            CapturedDesktopWindow {
                hwnd: "0x00000000000000BB".into(),
                title: "Fixture Secondary".into(),
                process_id: 101,
                process_name: Some("fixture-secondary.exe".into()),
                                visible: true,
                minimized: false,
                focused: false,
                x: 2000,
                y: 120,
                width: 1024,
                height: 768,
                monitor_index: Some(1),
                z_order: Some(1),
            },
            CapturedDesktopWindow {
                hwnd: "0x00000000000000CC".into(),
                title: "Fixture Minimized".into(),
                process_id: 102,
                process_name: Some("fixture-minimized.exe".into()),
                                visible: false,
                minimized: true,
                focused: false,
                x: -32000,
                y: -32000,
                width: 160,
                height: 28,
                monitor_index: None,
                z_order: Some(2),
            },
            CapturedDesktopWindow {
                hwnd: "0x00000000000000DD".into(),
                title: "Fixture Unknown Monitor".into(),
                process_id: 103,
                process_name: Some("fixture-unknown.exe".into()),
                                visible: true,
                minimized: false,
                focused: false,
                x: 5000,
                y: 5000,
                width: 400,
                height: 300,
                monitor_index: None,
                z_order: Some(3),
            },
        ],
        metadata: CaptureMetadata {
                source: "stub".into(),
                duration_ms: Some(0),
                real_os_observation: false,
                process_names_available: true,
            },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::StubWindowController;

    #[test]
    fn stub_fixture_preserves_process_identity_metadata() {
        let capture = StubDesktopCapturer::fixture_dual_monitor()
            .capture_desktop()
            .unwrap();
        assert!(capture.metadata.process_names_available);
        assert!(!capture.metadata.real_os_observation);
        for window in &capture.windows {
            assert!(window.process_name.is_some());
            assert!(window.hwnd.starts_with("0x"));
        }
        assert!(capture.contains_hwnd("0x00000000000000AA"));
        assert!(!capture.contains_hwnd("0xDEAD"));
    }

    #[test]
    fn stub_observation_does_not_imply_control_surface() {
        // Capture is observation-only; control lives on WindowController.
        let _capture = StubDesktopCapturer::fixture_dual_monitor()
            .capture_desktop()
            .unwrap();
        let controller = StubWindowController::new();
        assert!(controller.recorded().is_empty());
    }

    #[test]
    fn stub_returns_empty_legacy_enumeration() {
        let windows = StubWindowEnumerator.enumerate_windows().unwrap();
        assert!(windows.is_empty());
    }

    #[test]
    fn stub_fixture_is_deterministic() {
        let first = StubDesktopCapturer::fixture_dual_monitor().capture_desktop().unwrap();
        let second = StubDesktopCapturer::fixture_dual_monitor().capture_desktop().unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn stub_fixture_captures_foreground_hwnd() {
        let capture = StubDesktopCapturer::fixture_dual_monitor()
            .capture_desktop()
            .unwrap();
        assert_eq!(
            capture.foreground_hwnd.as_deref(),
            Some("0x00000000000000AA")
        );
    }

    #[test]
    fn stub_fixture_marks_focused_window() {
        let capture = StubDesktopCapturer::fixture_dual_monitor()
            .capture_desktop()
            .unwrap();
        let focused = capture
            .windows
            .iter()
            .find(|window| window.focused)
            .expect("focused window");
        assert_eq!(focused.title, "Fixture Focus");
    }

    #[test]
    fn stub_fixture_assigns_monitors_and_unknowns() {
        let capture = StubDesktopCapturer::fixture_dual_monitor()
            .capture_desktop()
            .unwrap();
        assert_eq!(capture.monitors.len(), 2);
        assert_eq!(
            capture
                .windows
                .iter()
                .find(|window| window.title == "Fixture Secondary")
                .and_then(|window| window.monitor_index),
            Some(1)
        );
        assert_eq!(
            capture
                .windows
                .iter()
                .find(|window| window.title == "Fixture Unknown Monitor")
                .and_then(|window| window.monitor_index),
            None
        );
    }

    #[test]
    fn stub_fixture_includes_minimized_window() {
        let capture = StubDesktopCapturer::fixture_dual_monitor()
            .capture_desktop()
            .unwrap();
        assert!(capture
            .windows
            .iter()
            .any(|window| window.title == "Fixture Minimized" && window.minimized));
    }

    #[test]
    fn stub_empty_capture_works() {
        let capture = StubDesktopCapturer::empty().capture_desktop().unwrap();
        assert!(capture.windows.is_empty());
        assert!(capture.monitors.is_empty());
    }

    #[test]
    fn stub_launcher_simulates_success() {
        let outcome = StubProcessLauncher
            .launch(&ProcessLaunchRequest {
                executable: "notepad.exe".into(),
                args: Vec::new(),
            })
            .unwrap();
        assert!(outcome.simulated);
        assert!(outcome.process_id.is_none());
    }
}
