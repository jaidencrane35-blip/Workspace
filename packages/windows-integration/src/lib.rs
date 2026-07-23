//! Windows Integration Layer (DEC-008).
//!
//! Only this crate talks to OS window APIs. Kernel and UI consume the
//! [`WindowEnumerator`] trait — never Win32 directly.

mod enumerator;
mod error;
#[cfg(windows)]
mod win32;
mod stub;

pub use enumerator::{DesktopWindowSnapshot, WindowEnumerator};
pub use error::{WindowsIntegrationError, Result};
pub use stub::StubWindowEnumerator;
#[cfg(windows)]
pub use win32::Win32WindowEnumerator;

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
