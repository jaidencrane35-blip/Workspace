//! Governed application launch — kernel orchestration over the Windows
//! Integration Layer (Sprint 41). Kernel never spawns processes directly.

use workspace_database::Database;
use workspace_domain::{ApplicationId, ApplicationLaunchResult};
use workspace_windows_integration::{
    platform_process_launcher, ProcessLaunchRequest, ProcessLauncher, StubProcessLauncher,
};

use super::ApplicationService;
use crate::error::{KernelError, Result};

/// Launches a registered application after Permission Gateway authorization.
pub(crate) struct ApplicationLaunchService;

impl ApplicationLaunchService {
    /// OS launch — crate-internal only. Callers must go through `LaunchApplication` + gateway.
    pub(crate) fn launch(
        db: &Database,
        application_id: &ApplicationId,
    ) -> Result<ApplicationLaunchResult> {
        let launcher = platform_process_launcher();
        Self::launch_with(db, application_id, &*launcher)
    }

    /// Test helper — uses the stub launcher (no OS spawn).
    pub(crate) fn launch_simulated(
        db: &Database,
        application_id: &ApplicationId,
    ) -> Result<ApplicationLaunchResult> {
        Self::launch_with(db, application_id, &StubProcessLauncher)
    }

    pub(crate) fn launch_with(
        db: &Database,
        application_id: &ApplicationId,
        launcher: &dyn ProcessLauncher,
    ) -> Result<ApplicationLaunchResult> {
        let application = ApplicationService::load(db, application_id)?;
        let executable = application
            .launch_executable()
            .ok_or_else(|| KernelError::InvalidLaunchTarget {
                message: format!(
                    "application '{}' has no executable_path configured",
                    application.name
                ),
            })?
            .to_string();

        let outcome = launcher
            .launch(&ProcessLaunchRequest {
                executable: executable.clone(),
                args: Vec::new(),
            })
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })?;

        Ok(ApplicationLaunchResult {
            application_id: application.id,
            name: application.name,
            executable_path: executable,
            process_id: outcome.process_id,
            simulated: outcome.simulated,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::WorkspaceService;
    use tempfile::tempdir;
    use workspace_database::DatabaseService;

    fn test_db() -> Database {
        let dir = tempdir().unwrap();
        DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database()
    }

    #[test]
    fn launches_registered_application_via_stub() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Launch".into()).unwrap();
        let app = ApplicationService::create(
            &db,
            workspace.id,
            "Notepad".into(),
            None,
            Some("notepad.exe".into()),
        )
        .unwrap();

        let result = ApplicationLaunchService::launch_simulated(&db, &app.id).unwrap();
        assert_eq!(result.name, "Notepad");
        assert!(result.simulated);
        assert_eq!(result.executable_path, "notepad.exe");
    }

    #[test]
    fn rejects_application_without_executable() {
        let db = test_db();
        let workspace = WorkspaceService::create(&db, "Launch".into()).unwrap();
        let app = ApplicationService::create(
            &db,
            workspace.id,
            "NoPath".into(),
            Some("com.example".into()),
            None,
        )
        .unwrap();

        let error = ApplicationLaunchService::launch_simulated(&db, &app.id).unwrap_err();
        assert!(matches!(error, KernelError::InvalidLaunchTarget { .. }));
    }
}
