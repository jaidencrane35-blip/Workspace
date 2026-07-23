use super::enumerator::{DesktopWindowSnapshot, WindowEnumerator};
use super::launcher::{ProcessLaunchOutcome, ProcessLaunchRequest, ProcessLauncher};
use crate::error::Result;

/// Test / non-Windows enumerator — returns no windows.
#[derive(Debug, Default, Clone, Copy)]
pub struct StubWindowEnumerator;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_returns_empty() {
        let windows = StubWindowEnumerator.enumerate_windows().unwrap();
        assert!(windows.is_empty());
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
