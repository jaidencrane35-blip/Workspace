use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WorkspaceStatus {
    pub status: String,
    pub version: String,
}

/// Sprint 01 IPC validation — returns shell runtime status.
#[tauri::command]
pub fn get_workspace_status() -> WorkspaceStatus {
    WorkspaceStatus {
        status: "running".to_string(),
        version: "0.1.0".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected_status() {
        let status = get_workspace_status();
        assert_eq!(status.status, "running");
        assert_eq!(status.version, "0.1.0");
    }
}
