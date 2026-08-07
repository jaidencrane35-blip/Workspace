//! Privacy-preserving support package export (Gate B1).
//! Does NOT include Moments DB contents, transcripts, or network upload.

use std::fs;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde::Serialize;
use tauri::{AppHandle, Manager, State};
use workspace_kernel::WorkspaceKernel;

use crate::file_log;
use super::error::CommandError;
use super::response::IpcResponse;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportBundleResult {
    pub path: String,
    pub message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SupportManifest {
    product: &'static str,
    identifier: &'static str,
    version: String,
    exported_at: String,
    slice: &'static str,
    privacy: &'static str,
    health_status: Option<String>,
    log_files_included: usize,
    database_present: bool,
    database_bytes: Option<u64>,
    /// Explicitly never ship Moments payload in support bundles.
    database_contents_included: bool,
}

/// Create a local support package directory (logs + redacted metadata).
#[tauri::command]
pub fn export_support_bundle(
    app: AppHandle,
    kernel: State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> IpcResponse<SupportBundleResult> {
    match export_inner(&app, &kernel) {
        Ok(result) => IpcResponse::success(result),
        Err(error) => IpcResponse::failure(CommandError::new("support_bundle_failed", error)),
    }
}

fn export_inner(
    app: &AppHandle,
    kernel: &State<'_, Arc<Mutex<WorkspaceKernel>>>,
) -> Result<SupportBundleResult, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Could not resolve app data directory: {e}"))?;

    let stamp = Utc::now().format("%Y%m%d-%H%M%S");
    let bundle_dir = app_data
        .join("support-bundles")
        .join(format!("workspace-support-{stamp}"));
    fs::create_dir_all(&bundle_dir).map_err(|e| format!("Could not create support folder: {e}"))?;

    let logs_out = bundle_dir.join("logs");
    fs::create_dir_all(&logs_out).map_err(|e| format!("Could not create logs folder: {e}"))?;

    let log_files = file_log::list_log_files(&app_data);
    let mut copied = 0usize;
    for src in &log_files {
        if let Some(name) = src.file_name() {
            let dest = logs_out.join(name);
            if fs::copy(src, &dest).is_ok() {
                copied += 1;
            }
        }
    }

    let db_path = app_data.join("workspace.db");
    let (database_present, database_bytes) = if db_path.is_file() {
        (
            true,
            fs::metadata(&db_path).ok().map(|m| m.len()),
        )
    } else {
        (false, None)
    };

    let health_status = kernel
        .lock()
        .ok()
        .map(|k| k.health().status);

    let version = app.package_info().version.to_string();
    let manifest = SupportManifest {
        product: "Workspace",
        identifier: "com.workspace.app",
        version,
        exported_at: Utc::now().to_rfc3339(),
        slice: "P16.PI3.B1",
        privacy: "Local only. No Moments/database contents. No network upload.",
        health_status,
        log_files_included: copied,
        database_present,
        database_bytes,
        database_contents_included: false,
    };

    let manifest_path = bundle_dir.join("manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Could not serialize manifest: {e}"))?;
    fs::write(&manifest_path, manifest_json)
        .map_err(|e| format!("Could not write manifest: {e}"))?;

    // Install / build hints (no secrets).
    let mut notes = String::from("# Workspace support package\n\n");
    notes.push_str("- This folder is for local diagnosis only.\n");
    notes.push_str("- Moments and database contents are intentionally omitted.\n");
    notes.push_str("- Share only with people you trust.\n");
    fs::write(bundle_dir.join("README.txt"), notes)
        .map_err(|e| format!("Could not write README: {e}"))?;

    // Optional install-manifest from install dir (beside exe).
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let install_manifest = dir.join("install-manifest.json");
            if install_manifest.is_file() {
                let _ = fs::copy(
                    &install_manifest,
                    bundle_dir.join("install-manifest.json"),
                );
            }
        }
    }

    let path = bundle_dir
        .to_str()
        .ok_or_else(|| "Support path is not valid UTF-8.".to_string())?
        .to_string();

    log::info!("support_bundle: exported to {path}");

    Ok(SupportBundleResult {
        path: path.clone(),
        message: format!(
            "Support package saved. It includes logs and version info — not your Moments. Path: {path}"
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_never_includes_database_contents_flag_true() {
        let m = SupportManifest {
            product: "Workspace",
            identifier: "com.workspace.app",
            version: "0.1.0".into(),
            exported_at: "t".into(),
            slice: "test",
            privacy: "x",
            health_status: None,
            log_files_included: 0,
            database_present: true,
            database_bytes: Some(1),
            database_contents_included: false,
        };
        assert!(!m.database_contents_included);
        let json = serde_json::to_string(&m).unwrap();
        assert!(json.contains("\"databaseContentsIncluded\":false"));
    }
}
