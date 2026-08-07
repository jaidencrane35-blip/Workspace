//! Production file logging with size-based rotation (Gate B1).
//! Privacy: logs stay local under the app data directory — no network upload.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const MAX_LOG_BYTES: u64 = 5_000_000;
const KEEP_ROTATED: usize = 3;

static FILE_LOGGER: Mutex<Option<File>> = Mutex::new(None);

pub fn logs_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("logs")
}

pub fn active_log_path(app_data_dir: &Path) -> PathBuf {
    logs_dir(app_data_dir).join("workspace.log")
}

/// Attach a rotating file sink. Safe to call once during setup after app_data_dir exists.
pub fn attach_file_logger(app_data_dir: &Path) -> io::Result<PathBuf> {
    let dir = logs_dir(app_data_dir);
    fs::create_dir_all(&dir)?;
    let path = active_log_path(app_data_dir);
    rotate_if_needed(&path)?;
    let file = OpenOptions::new().create(true).append(true).open(&path)?;
    let mut slot = FILE_LOGGER
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *slot = Some(file);
    Ok(path)
}

fn rotate_if_needed(path: &Path) -> io::Result<()> {
    let Ok(meta) = fs::metadata(path) else {
        return Ok(());
    };
    if meta.len() < MAX_LOG_BYTES {
        return Ok(());
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    // workspace.log.(KEEP) → delete; shift others up.
    let oldest = parent.join(format!("workspace.log.{}", KEEP_ROTATED));
    let _ = fs::remove_file(&oldest);
    for i in (1..KEEP_ROTATED).rev() {
        let from = parent.join(format!("workspace.log.{}", i));
        let to = parent.join(format!("workspace.log.{}", i + 1));
        if from.exists() {
            let _ = fs::rename(&from, &to);
        }
    }
    let first = parent.join("workspace.log.1");
    fs::rename(path, first)?;
    Ok(())
}

pub fn write_line(line: &str) {
    let Ok(mut slot) = FILE_LOGGER.lock() else {
        return;
    };
    if let Some(file) = slot.as_mut() {
        let _ = writeln!(file, "{line}");
        let _ = file.flush();
    }
}

/// List log files for support-bundle inclusion (active + rotated).
pub fn list_log_files(app_data_dir: &Path) -> Vec<PathBuf> {
    let dir = logs_dir(app_data_dir);
    let mut out = Vec::new();
    let active = active_log_path(app_data_dir);
    if active.is_file() {
        out.push(active);
    }
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with("workspace.log.") && p.is_file() {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}
