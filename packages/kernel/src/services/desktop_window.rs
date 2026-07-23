//! Desktop window observation — kernel orchestration over the Windows
//! Integration Layer (Sprint 35–36). Kernel never calls Win32 directly.

use workspace_windows_integration::{
    platform_window_enumerator, DesktopWindowSnapshot, WindowEnumerator,
};

use crate::error::{KernelError, Result};

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

/// Lists top-level desktop windows via the platform enumerator.
pub struct DesktopWindowService;

impl DesktopWindowService {
    pub fn list_recent(limit: Option<usize>) -> Result<Vec<DesktopWindowSnapshot>> {
        let enumerator = platform_window_enumerator();
        Self::list_with(&*enumerator, limit)
    }

    pub fn list_with(
        enumerator: &dyn WindowEnumerator,
        limit: Option<usize>,
    ) -> Result<Vec<DesktopWindowSnapshot>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
        let mut windows = enumerator.enumerate_windows().map_err(|error| {
            KernelError::WindowsIntegration {
                message: error.to_string(),
            }
        })?;
        windows.truncate(limit);
        Ok(windows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_windows_integration::StubWindowEnumerator;

    #[test]
    fn stub_enumerator_returns_empty() {
        let windows = DesktopWindowService::list_with(&StubWindowEnumerator, Some(10)).unwrap();
        assert!(windows.is_empty());
    }
}
