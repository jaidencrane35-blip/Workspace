use super::enumerator::{DesktopWindowSnapshot, WindowEnumerator};
use crate::error::Result;

/// Test / non-Windows enumerator — returns no windows.
#[derive(Debug, Default, Clone, Copy)]
pub struct StubWindowEnumerator;

impl WindowEnumerator for StubWindowEnumerator {
    fn enumerate_windows(&self) -> Result<Vec<DesktopWindowSnapshot>> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_returns_empty() {
        let windows = StubWindowEnumerator.enumerate_windows().unwrap();
        assert!(windows.is_empty());
    }
}
