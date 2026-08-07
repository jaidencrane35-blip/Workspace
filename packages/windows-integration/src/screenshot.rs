//! Screenshot port — Workspace-owned interface; WRAP `xcap` (P15).
//!
//! Distinct from [`crate::DesktopCapturer`] (observation metadata only).

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use arboard::{Clipboard, ImageData};

use crate::error::{Result, WindowsIntegrationError};

/// Level 1 screenshot capability snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScreenshotCapabilityStatus {
    pub available: bool,
    pub monitor_count: usize,
    pub can_capture_window: bool,
    pub can_copy_clipboard: bool,
    pub message: String,
}

/// Outcome of a capture / copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScreenshotCaptureOutcome {
    pub ok: bool,
    pub path: Option<String>,
    pub width: u32,
    pub height: u32,
    pub target: String,
    pub message: String,
    pub copied: bool,
}

/// Desktop screenshot access behind a Workspace-owned port.
pub trait ScreenshotPort: Send + Sync {
    fn status(&self) -> Result<ScreenshotCapabilityStatus>;
    fn capture_desktop(&self) -> Result<ScreenshotCaptureOutcome>;
    fn capture_monitor(&self, monitor_index: i32) -> Result<ScreenshotCaptureOutcome>;
    fn capture_window(&self, query: &str) -> Result<ScreenshotCaptureOutcome>;
    fn copy_path_to_clipboard(&self, path: &str) -> Result<ScreenshotCaptureOutcome>;
}

fn captures_dir() -> PathBuf {
    let base = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join("com.workspace.app").join("captures")
}

fn next_capture_path(label: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let safe: String = label
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else {
                '_'
            }
        })
        .take(40)
        .collect();
    captures_dir().join(format!("shot-{safe}-{stamp}.png"))
}

fn ensure_captures_dir() -> Result<()> {
    std::fs::create_dir_all(captures_dir()).map_err(|error| {
        WindowsIntegrationError::ScreenshotFailed(format!("create captures dir: {error}"))
    })
}

fn copy_png_file_to_clipboard(path: &Path) -> Result<()> {
    let bytes = std::fs::read(path).map_err(|error| {
        WindowsIntegrationError::ScreenshotFailed(format!("read capture: {error}"))
    })?;
    let img = image::load_from_memory(&bytes).map_err(|error| {
        WindowsIntegrationError::ScreenshotFailed(format!("decode capture: {error}"))
    })?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let mut clipboard = Clipboard::new().map_err(|error| {
        WindowsIntegrationError::ScreenshotFailed(format!("open clipboard: {error}"))
    })?;
    clipboard
        .set_image(ImageData {
            width: width as usize,
            height: height as usize,
            bytes: rgba.into_raw().into(),
        })
        .map_err(|error| {
            WindowsIntegrationError::ScreenshotFailed(format!("clipboard image: {error}"))
        })?;
    Ok(())
}

/// In-process screenshot port for tests.
#[derive(Debug, Default)]
pub struct MemoryScreenshotPort {
    last_path: Mutex<Option<String>>,
    captures: Mutex<Vec<String>>,
}

impl MemoryScreenshotPort {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ScreenshotPort for MemoryScreenshotPort {
    fn status(&self) -> Result<ScreenshotCapabilityStatus> {
        Ok(ScreenshotCapabilityStatus {
            available: true,
            monitor_count: 2,
            can_capture_window: true,
            can_copy_clipboard: true,
            message: "Screenshot support is available.".into(),
        })
    }

    fn capture_desktop(&self) -> Result<ScreenshotCaptureOutcome> {
        let path = format!("memory://desktop-{}.png", self.captures.lock().map(|c| c.len()).unwrap_or(0));
        self.captures
            .lock()
            .map_err(|_| WindowsIntegrationError::ScreenshotFailed("lock poisoned".into()))?
            .push(path.clone());
        *self
            .last_path
            .lock()
            .map_err(|_| WindowsIntegrationError::ScreenshotFailed("lock poisoned".into()))? =
            Some(path.clone());
        Ok(ScreenshotCaptureOutcome {
            ok: true,
            path: Some(path),
            width: 1920,
            height: 1080,
            target: "desktop".into(),
            message: "Captured your desktop.".into(),
            copied: false,
        })
    }

    fn capture_monitor(&self, monitor_index: i32) -> Result<ScreenshotCaptureOutcome> {
        if monitor_index < 1 || monitor_index > 2 {
            return Ok(ScreenshotCaptureOutcome {
                ok: false,
                path: None,
                width: 0,
                height: 0,
                target: format!("monitor {monitor_index}"),
                message: format!(
                    "I couldn’t find monitor {monitor_index}. Try monitor 1 or monitor 2."
                ),
                copied: false,
            });
        }
        let path = format!("memory://monitor-{monitor_index}.png");
        *self
            .last_path
            .lock()
            .map_err(|_| WindowsIntegrationError::ScreenshotFailed("lock poisoned".into()))? =
            Some(path.clone());
        Ok(ScreenshotCaptureOutcome {
            ok: true,
            path: Some(path),
            width: 1920,
            height: 1080,
            target: format!("monitor {monitor_index}"),
            message: format!("Captured monitor {monitor_index}."),
            copied: false,
        })
    }

    fn capture_window(&self, query: &str) -> Result<ScreenshotCaptureOutcome> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(ScreenshotCaptureOutcome {
                ok: false,
                path: None,
                width: 0,
                height: 0,
                target: String::new(),
                message: "Which window should I capture?".into(),
                copied: false,
            });
        }
        let active = matches!(
            q.to_ascii_lowercase().as_str(),
            "this" | "it" | "active" | "current" | "foreground" | "this window" | "active window"
        );
        let target = if active {
            "active window".into()
        } else {
            q.to_string()
        };
        let path = format!("memory://window-{}.png", target.replace(' ', "_"));
        *self
            .last_path
            .lock()
            .map_err(|_| WindowsIntegrationError::ScreenshotFailed("lock poisoned".into()))? =
            Some(path.clone());
        Ok(ScreenshotCaptureOutcome {
            ok: true,
            path: Some(path),
            width: 1280,
            height: 800,
            target: target.clone(),
            message: format!("Captured “{target}”."),
            copied: false,
        })
    }

    fn copy_path_to_clipboard(&self, path: &str) -> Result<ScreenshotCaptureOutcome> {
        let path = if path.trim().is_empty() {
            self.last_path
                .lock()
                .map_err(|_| WindowsIntegrationError::ScreenshotFailed("lock poisoned".into()))?
                .clone()
                .unwrap_or_default()
        } else {
            path.to_string()
        };
        if path.is_empty() {
            return Ok(ScreenshotCaptureOutcome {
                ok: false,
                path: None,
                width: 0,
                height: 0,
                target: String::new(),
                message: "There’s no screenshot to copy yet. Take one first.".into(),
                copied: false,
            });
        }
        Ok(ScreenshotCaptureOutcome {
            ok: true,
            path: Some(path),
            width: 0,
            height: 0,
            target: "clipboard".into(),
            message: "Copied the screenshot to the clipboard.".into(),
            copied: true,
        })
    }
}

/// Production WRAP of `xcap` (+ arboard for image clipboard).
pub struct SystemScreenshotPort {
    last_path: Mutex<Option<String>>,
}

impl SystemScreenshotPort {
    pub fn new() -> Self {
        Self {
            last_path: Mutex::new(None),
        }
    }

    fn remember(&self, path: &str) -> Result<()> {
        *self
            .last_path
            .lock()
            .map_err(|_| WindowsIntegrationError::ScreenshotFailed("lock poisoned".into()))? =
            Some(path.to_string());
        Ok(())
    }
}

impl Default for SystemScreenshotPort {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenshotPort for SystemScreenshotPort {
    fn status(&self) -> Result<ScreenshotCapabilityStatus> {
        #[cfg(windows)]
        {
            let monitors = xcap::Monitor::all().map_err(|error| {
                WindowsIntegrationError::ScreenshotFailed(format!("list monitors: {error}"))
            })?;
            Ok(ScreenshotCapabilityStatus {
                available: true,
                monitor_count: monitors.len(),
                can_capture_window: true,
                can_copy_clipboard: true,
                message: format!(
                    "Screenshot support is available ({} monitor{}).",
                    monitors.len(),
                    if monitors.len() == 1 { "" } else { "s" }
                ),
            })
        }
        #[cfg(not(windows))]
        {
            Ok(ScreenshotCapabilityStatus {
                available: false,
                monitor_count: 0,
                can_capture_window: false,
                can_copy_clipboard: false,
                message: "Screenshot support isn’t available on this system.".into(),
            })
        }
    }

    fn capture_desktop(&self) -> Result<ScreenshotCaptureOutcome> {
        self.capture_monitor(1)
    }

    fn capture_monitor(&self, monitor_index: i32) -> Result<ScreenshotCaptureOutcome> {
        #[cfg(windows)]
        {
            ensure_captures_dir()?;
            let monitors = xcap::Monitor::all().map_err(|error| {
                WindowsIntegrationError::ScreenshotFailed(format!("list monitors: {error}"))
            })?;
            if monitors.is_empty() {
                return Ok(ScreenshotCaptureOutcome {
                    ok: false,
                    path: None,
                    width: 0,
                    height: 0,
                    target: "desktop".into(),
                    message: "I couldn’t find a monitor to capture.".into(),
                    copied: false,
                });
            }
            let idx = if monitor_index < 1 {
                0usize
            } else {
                (monitor_index as usize).saturating_sub(1)
            };
            let Some(monitor) = monitors.get(idx) else {
                return Ok(ScreenshotCaptureOutcome {
                    ok: false,
                    path: None,
                    width: 0,
                    height: 0,
                    target: format!("monitor {monitor_index}"),
                    message: format!(
                        "I couldn’t find monitor {monitor_index}. This PC has {}.",
                        monitors.len()
                    ),
                    copied: false,
                });
            };
            let image = monitor.capture_image().map_err(|error| {
                WindowsIntegrationError::ScreenshotFailed(format!("capture monitor: {error}"))
            })?;
            let path = next_capture_path(&format!("monitor-{}", idx + 1));
            image.save(&path).map_err(|error| {
                WindowsIntegrationError::ScreenshotFailed(format!("save PNG: {error}"))
            })?;
            let path_str = path.display().to_string();
            self.remember(&path_str)?;
            Ok(ScreenshotCaptureOutcome {
                ok: true,
                path: Some(path_str),
                width: image.width(),
                height: image.height(),
                target: format!("monitor {}", idx + 1),
                message: format!("Captured monitor {} and saved a PNG.", idx + 1),
                copied: false,
            })
        }
        #[cfg(not(windows))]
        {
            let _ = monitor_index;
            Err(WindowsIntegrationError::ScreenshotFailed(
                "screenshot support isn’t available on this system".into(),
            ))
        }
    }

    fn capture_window(&self, query: &str) -> Result<ScreenshotCaptureOutcome> {
        #[cfg(windows)]
        {
            ensure_captures_dir()?;
            let q = query.trim();
            let active = q.is_empty()
                || matches!(
                    q.to_ascii_lowercase().as_str(),
                    "this"
                        | "it"
                        | "active"
                        | "current"
                        | "foreground"
                        | "this window"
                        | "active window"
                        | "current window"
                );
            let windows = xcap::Window::all().map_err(|error| {
                WindowsIntegrationError::ScreenshotFailed(format!("list windows: {error}"))
            })?;
            let needle = q.to_ascii_lowercase();
            let selected = if active {
                windows.into_iter().find(|w| {
                    w.is_focused().unwrap_or(false) && !w.is_minimized().unwrap_or(false)
                })
            } else {
                windows.into_iter().find(|w| {
                    let title = w.title().unwrap_or_default().to_ascii_lowercase();
                    !w.is_minimized().unwrap_or(false) && title.contains(&needle)
                })
            };
            let Some(window) = selected else {
                return Ok(ScreenshotCaptureOutcome {
                    ok: false,
                    path: None,
                    width: 0,
                    height: 0,
                    target: q.to_string(),
                    message: if active {
                        "I couldn’t find an active window to capture.".into()
                    } else {
                        format!("I couldn’t find a window matching “{q}”.")
                    },
                    copied: false,
                });
            };
            let title = window.title().unwrap_or_else(|_| "window".into());
            let image = window.capture_image().map_err(|error| {
                WindowsIntegrationError::ScreenshotFailed(format!("capture window: {error}"))
            })?;
            let path = next_capture_path(&title);
            image.save(&path).map_err(|error| {
                WindowsIntegrationError::ScreenshotFailed(format!("save PNG: {error}"))
            })?;
            let path_str = path.display().to_string();
            self.remember(&path_str)?;
            Ok(ScreenshotCaptureOutcome {
                ok: true,
                path: Some(path_str),
                width: image.width(),
                height: image.height(),
                target: title.clone(),
                message: format!("Captured “{title}” and saved a PNG."),
                copied: false,
            })
        }
        #[cfg(not(windows))]
        {
            let _ = query;
            Err(WindowsIntegrationError::ScreenshotFailed(
                "screenshot support isn’t available on this system".into(),
            ))
        }
    }

    fn copy_path_to_clipboard(&self, path: &str) -> Result<ScreenshotCaptureOutcome> {
        let path = {
            let trimmed = path.trim();
            if trimmed.is_empty() {
                self.last_path
                    .lock()
                    .map_err(|_| WindowsIntegrationError::ScreenshotFailed("lock poisoned".into()))?
                    .clone()
                    .unwrap_or_default()
            } else {
                trimmed.to_string()
            }
        };
        if path.is_empty() {
            return Ok(ScreenshotCaptureOutcome {
                ok: false,
                path: None,
                width: 0,
                height: 0,
                target: String::new(),
                message: "There’s no screenshot to copy yet. Take one first.".into(),
                copied: false,
            });
        }
        let file = Path::new(&path);
        if !file.is_file() {
            return Ok(ScreenshotCaptureOutcome {
                ok: false,
                path: Some(path.clone()),
                width: 0,
                height: 0,
                target: path,
                message: "I couldn’t find that screenshot file to copy.".into(),
                copied: false,
            });
        }
        copy_png_file_to_clipboard(file)?;
        Ok(ScreenshotCaptureOutcome {
            ok: true,
            path: Some(path),
            width: 0,
            height: 0,
            target: "clipboard".into(),
            message: "Copied the screenshot to the clipboard.".into(),
            copied: true,
        })
    }
}

/// Unavailable stub.
#[derive(Debug, Default)]
pub struct UnavailableScreenshotPort;

impl ScreenshotPort for UnavailableScreenshotPort {
    fn status(&self) -> Result<ScreenshotCapabilityStatus> {
        Ok(ScreenshotCapabilityStatus {
            available: false,
            monitor_count: 0,
            can_capture_window: false,
            can_copy_clipboard: false,
            message: "Screenshot support isn’t available on this system.".into(),
        })
    }

    fn capture_desktop(&self) -> Result<ScreenshotCaptureOutcome> {
        Err(WindowsIntegrationError::ScreenshotFailed(
            "screenshot support isn’t available on this system".into(),
        ))
    }

    fn capture_monitor(&self, _monitor_index: i32) -> Result<ScreenshotCaptureOutcome> {
        self.capture_desktop()
    }

    fn capture_window(&self, _query: &str) -> Result<ScreenshotCaptureOutcome> {
        self.capture_desktop()
    }

    fn copy_path_to_clipboard(&self, _path: &str) -> Result<ScreenshotCaptureOutcome> {
        self.capture_desktop()
    }
}

pub fn platform_screenshot() -> std::sync::Arc<dyn ScreenshotPort> {
    std::sync::Arc::new(SystemScreenshotPort::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_screenshot_desktop_and_copy() {
        let port = MemoryScreenshotPort::new();
        assert!(port.status().unwrap().available);
        let shot = port.capture_desktop().unwrap();
        assert!(shot.ok);
        let copied = port
            .copy_path_to_clipboard(shot.path.as_deref().unwrap_or(""))
            .unwrap();
        assert!(copied.copied);
    }

    #[test]
    fn memory_missing_monitor_clarifies() {
        let port = MemoryScreenshotPort::new();
        let miss = port.capture_monitor(9).unwrap();
        assert!(!miss.ok);
        assert!(miss.message.contains("monitor 9"));
    }
}
