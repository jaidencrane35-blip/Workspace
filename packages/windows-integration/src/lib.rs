//! Windows Integration Layer (DEC-008).
//!
//! Only this crate talks to OS window and process APIs. Kernel and UI consume
//! traits — never Win32 directly.
//!
//! DAF-1a adds [`WindowController`] for move/resize/focus. Permissions stay in
//! the kernel Permission Gateway — this crate executes OS calls only.

mod capture;
mod controller;
mod enumerator;
mod error;
mod launcher;
#[cfg(windows)]
mod win32;
mod stub;

pub use capture::{
    monitor_index_for_point, monitor_index_for_window_bounds, CaptureMetadata,
    CapturedDesktopMonitor, CapturedDesktopWindow, DesktopCapturer, DesktopObservationCapture,
};
pub use controller::{
    parse_hwnd_value, FocusWindowRequest, RecordedWindowControl, SetWindowBoundsRequest,
    StubWindowController, WindowBounds, WindowControlOutcome, WindowController,
};
pub use enumerator::{DesktopWindowSnapshot, WindowEnumerator};
pub use error::{Result, WindowsIntegrationError};
pub use launcher::{ProcessLaunchOutcome, ProcessLaunchRequest, ProcessLauncher};
pub use stub::{
    dual_monitor_fixture, StubDesktopCapturer, StubProcessLauncher, StubWindowEnumerator,
};
#[cfg(windows)]
pub use win32::{Win32ProcessLauncher, Win32WindowController, Win32WindowEnumerator};

/// Returns the platform enumerator: Win32 on Windows, stub elsewhere.
pub fn platform_window_enumerator() -> Box<dyn WindowEnumerator> {
    #[cfg(windows)]
    {
        Box::new(Win32WindowEnumerator)
    }
    #[cfg(not(windows))]
    {
        Box::new(StubWindowEnumerator)
    }
}

/// Returns the platform desktop capturer: Win32 on Windows, stub elsewhere.
pub fn platform_desktop_capturer() -> Box<dyn DesktopCapturer> {
    #[cfg(windows)]
    {
        Box::new(Win32WindowEnumerator)
    }
    #[cfg(not(windows))]
    {
        Box::new(StubDesktopCapturer::default())
    }
}

/// Returns the platform process launcher: Win32 on Windows, stub elsewhere.
pub fn platform_process_launcher() -> Box<dyn ProcessLauncher> {
    #[cfg(windows)]
    {
        Box::new(Win32ProcessLauncher)
    }
    #[cfg(not(windows))]
    {
        Box::new(StubProcessLauncher)
    }
}

/// Returns the platform window controller: Win32 on Windows, stub elsewhere.
pub fn platform_window_controller() -> Box<dyn WindowController> {
    #[cfg(windows)]
    {
        Box::new(Win32WindowController)
    }
    #[cfg(not(windows))]
    {
        Box::new(StubWindowController::new())
    }
}
