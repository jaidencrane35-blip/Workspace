//! Windows Integration Layer (DEC-008).
//!
//! Only this crate talks to OS window and process APIs. Kernel and UI consume
//! traits — never Win32 directly.

mod enumerator;
mod error;
mod launcher;
#[cfg(windows)]
mod win32;
mod stub;

pub use enumerator::{DesktopWindowSnapshot, WindowEnumerator};
pub use error::{WindowsIntegrationError, Result};
pub use launcher::{ProcessLaunchOutcome, ProcessLaunchRequest, ProcessLauncher};
pub use stub::{StubProcessLauncher, StubWindowEnumerator};
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
