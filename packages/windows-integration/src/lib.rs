//! Windows Integration Layer (DEC-008).
//!
//! Only this crate talks to OS window and process APIs. Kernel and UI consume
//! traits — never Win32 directly.

mod capture;
mod enumerator;
mod error;
mod launcher;
mod mutator;
mod stub;
mod stub_mutator;
#[cfg(windows)]
mod win32;

pub use capture::{
    monitor_index_for_point, monitor_index_for_window_bounds, CaptureMetadata,
    CapturedDesktopMonitor, CapturedDesktopWindow, DesktopCapturer, DesktopObservationCapture,
    STUB_DESKTOP_SESSION_ID,
};
pub use enumerator::{DesktopWindowSnapshot, WindowEnumerator};
pub use error::{Result, WindowsIntegrationError};
pub use launcher::{ProcessLaunchOutcome, ProcessLaunchRequest, ProcessLauncher};
pub use mutator::{
    LiveWindowView, MutatorEffectOutcome, WindowMutator, WindowPlacementRequest,
};
pub use stub::{dual_monitor_fixture, StubDesktopCapturer, StubProcessLauncher, StubWindowEnumerator};
pub use stub_mutator::StubWindowMutator;
#[cfg(windows)]
pub use win32::{Win32ProcessLauncher, Win32WindowEnumerator};

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

/// Returns the platform window mutator: Win32 on Windows, stub elsewhere.
pub fn platform_window_mutator() -> Box<dyn WindowMutator> {
    #[cfg(windows)]
    {
        Box::new(Win32WindowEnumerator)
    }
    #[cfg(not(windows))]
    {
        Box::new(StubWindowMutator::fixture_dual_monitor())
    }
}
