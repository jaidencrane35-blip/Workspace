//! Win32 desktop capture and legacy enumeration (Windows only).

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::process::Command;
use std::time::Instant;

use windows::Win32::Foundation::{BOOL, CloseHandle, HANDLE, HWND, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO, MONITORINFOEXW,
};
use windows::Win32::System::RemoteDesktop::ProcessIdToSessionId;
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
    PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, MONITORINFOF_PRIMARY,
    SetForegroundWindow, SetWindowPos, ShowWindow, HWND_TOP, SWP_NOACTIVATE, SWP_NOZORDER,
    SWP_SHOWWINDOW, SW_MINIMIZE, SW_RESTORE,
};

use super::capture::{
    monitor_index_for_window_bounds, CaptureMetadata, CapturedDesktopMonitor,
    CapturedDesktopWindow, DesktopCapturer, DesktopObservationCapture,
};
use super::enumerator::{DesktopWindowSnapshot, WindowEnumerator};
use super::launcher::{ProcessLaunchOutcome, ProcessLaunchRequest, ProcessLauncher};
use super::mutator::{
    LiveWindowView, MutatorEffectOutcome, WindowMutator, WindowPlacementRequest,
};
use crate::error::{Result, WindowsIntegrationError};

#[derive(Debug, Default, Clone, Copy)]
pub struct Win32WindowEnumerator;

impl WindowEnumerator for Win32WindowEnumerator {
    fn enumerate_windows(&self) -> Result<Vec<DesktopWindowSnapshot>> {
        Ok(self.capture_desktop()?.legacy_window_snapshots())
    }
}

impl DesktopCapturer for Win32WindowEnumerator {
    fn capture_desktop(&self) -> Result<DesktopObservationCapture> {
        let started = Instant::now();
        let monitors = enumerate_monitors()?;
        let foreground = unsafe { GetForegroundWindow() };
        let foreground_hwnd = if foreground.0.is_null() {
            None
        } else {
            Some(hwnd_to_string(foreground))
        };

        let mut ctx = CaptureContext {
            monitors: monitors.clone(),
            foreground,
            windows: Vec::new(),
            z_order: 0,
        };

        let ok = unsafe {
            EnumWindows(
                Some(capture_enum_proc),
                LPARAM(&mut ctx as *mut CaptureContext as isize),
            )
        };

        if ok.is_err() {
            return Err(WindowsIntegrationError::EnumerationFailed(
                "EnumWindows returned an error".into(),
            ));
        }

        Ok(DesktopObservationCapture {
            desktop_session_id: current_desktop_session_id()?,
            foreground_hwnd,
            windows: ctx.windows,
            monitors,
            metadata: CaptureMetadata {
                source: "win32".into(),
                duration_ms: Some(started.elapsed().as_millis() as u64),
            },
        })
    }
}

fn current_desktop_session_id() -> Result<String> {
    let pid = std::process::id();
    let mut session_id = 0u32;
    let ok = unsafe { ProcessIdToSessionId(pid, &mut session_id) };
    if ok.is_err() {
        return Err(WindowsIntegrationError::EnumerationFailed(
            "ProcessIdToSessionId failed".into(),
        ));
    }
    Ok(format!("wts-session-{session_id}"))
}

fn parse_hwnd(hwnd: &str) -> Result<HWND> {
    let trimmed = hwnd.trim().trim_start_matches("0x").trim_start_matches("0X");
    let value = usize::from_str_radix(trimmed, 16).map_err(|_| {
        WindowsIntegrationError::EnumerationFailed(format!("invalid hwnd '{hwnd}'"))
    })?;
    Ok(HWND(value as *mut core::ffi::c_void))
}

impl WindowMutator for Win32WindowEnumerator {
    fn current_desktop_session_id(&self) -> Result<String> {
        current_desktop_session_id()
    }

    fn window_by_hwnd(&self, hwnd: &str) -> Result<Option<LiveWindowView>> {
        let handle = parse_hwnd(hwnd)?;
        let exists = unsafe { IsWindow(handle).as_bool() };
        if !exists {
            return Ok(None);
        }
        let mut process_id = 0u32;
        unsafe { GetWindowThreadProcessId(handle, Some(&mut process_id)) };
        let title = unsafe { read_window_title(handle) };
        Ok(Some(LiveWindowView {
            hwnd: hwnd_to_string(handle),
            process_id,
            title,
        }))
    }

    fn attached_monitor_indices(&self) -> Result<Vec<i32>> {
        Ok(enumerate_monitors()?
            .into_iter()
            .map(|monitor| monitor.index)
            .collect())
    }

    fn place_window(
        &self,
        hwnd: &str,
        placement: &WindowPlacementRequest,
    ) -> Result<MutatorEffectOutcome> {
        let handle = parse_hwnd(hwnd)?;
        if !unsafe { IsWindow(handle).as_bool() } {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        }
        if placement.minimized {
            let ok = unsafe { ShowWindow(handle, SW_MINIMIZE) };
            if !ok.as_bool() {
                return Ok(MutatorEffectOutcome::RefusedByEnvironment);
            }
            return Ok(MutatorEffectOutcome::Committed);
        }
        // Restore from minimized without activating; keep existing z-order so
        // place-only restore does not steal stacking relative to the user's work.
        let _ = unsafe { ShowWindow(handle, SW_RESTORE) };
        let ok = unsafe {
            SetWindowPos(
                handle,
                HWND_TOP,
                placement.x,
                placement.y,
                placement.width,
                placement.height,
                SWP_NOACTIVATE | SWP_NOZORDER | SWP_SHOWWINDOW,
            )
        };
        if ok.is_err() {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        }
        Ok(MutatorEffectOutcome::Committed)
    }

    fn focus_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome> {
        let handle = parse_hwnd(hwnd)?;
        if !unsafe { IsWindow(handle).as_bool() } {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        }
        let ok = unsafe { SetForegroundWindow(handle) };
        if !ok.as_bool() {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        }
        Ok(MutatorEffectOutcome::Committed)
    }
}

struct CaptureContext {
    monitors: Vec<CapturedDesktopMonitor>,
    foreground: HWND,
    windows: Vec<CapturedDesktopWindow>,
    z_order: i32,
}

unsafe extern "system" fn capture_enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let ctx = &mut *(lparam.0 as *mut CaptureContext);

    let visible = unsafe { IsWindowVisible(hwnd) }.as_bool();
    let minimized = unsafe { IsIconic(hwnd) }.as_bool();
    if !visible && !minimized {
        return BOOL(1);
    }

    let title = unsafe { read_window_title(hwnd) };
    if title.trim().is_empty() {
        return BOOL(1);
    }

    let mut process_id: u32 = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));
    }

    let rect = match unsafe { read_window_rect(hwnd) } {
        Some(rect) => rect,
        None => return BOOL(1),
    };

    let monitor_index = monitor_index_for_window_bounds(
        rect.x,
        rect.y,
        rect.width,
        rect.height,
        &ctx.monitors,
    );

    let z_order = ctx.z_order;
    ctx.z_order += 1;

    ctx.windows.push(CapturedDesktopWindow {
        hwnd: hwnd_to_string(hwnd),
        title,
        process_id,
        process_name: unsafe { process_image_basename(process_id) },
        visible,
        minimized,
        focused: hwnd == ctx.foreground,
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        monitor_index,
        z_order: Some(z_order),
    });

    BOOL(1)
}

/// Basename only — observation buffer aid. Never stored on SavedContext.
unsafe fn process_image_basename(process_id: u32) -> Option<String> {
    if process_id == 0 {
        return None;
    }
    let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id).ok()?;
    let name = (|| {
        let mut buffer = vec![0u16; 512];
        let mut size = buffer.len() as u32;
        QueryFullProcessImageNameW(
            HANDLE(handle.0),
            PROCESS_NAME_WIN32,
            windows::core::PWSTR(buffer.as_mut_ptr()),
            &mut size,
        )
        .ok()?;
        let path = OsString::from_wide(&buffer[..size as usize]);
        let path = std::path::Path::new(&path);
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    })();
    let _ = CloseHandle(handle);
    name
}

struct WindowRect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

unsafe fn read_window_rect(hwnd: HWND) -> Option<WindowRect> {
    let mut rect = RECT::default();
    unsafe { GetWindowRect(hwnd, &mut rect) }.ok()?;
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;
    Some(WindowRect {
        x: rect.left,
        y: rect.top,
        width,
        height,
    })
}

fn enumerate_monitors() -> Result<Vec<CapturedDesktopMonitor>> {
    let mut collected: Vec<CapturedDesktopMonitor> = Vec::new();
    let ctx = &mut collected as *mut Vec<CapturedDesktopMonitor>;

    let ok = unsafe {
        EnumDisplayMonitors(
            None,
            None,
            Some(monitor_enum_proc),
            LPARAM(ctx as isize),
        )
    };

    if !ok.as_bool() {
        return Err(WindowsIntegrationError::EnumerationFailed(
            "EnumDisplayMonitors returned an error".into(),
        ));
    }

    collected.sort_by_key(|monitor| monitor.index);
    Ok(collected)
}

unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let list = &mut *(lparam.0 as *mut Vec<CapturedDesktopMonitor>);

    let mut info = MONITORINFOEXW::default();
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

    if !unsafe { GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut MONITORINFO) }.as_bool() {
        return BOOL(1);
    }

    let monitor = &info.monitorInfo;
    let name = OsString::from_wide(
        &info.szDevice[..info
            .szDevice
            .iter()
            .position(|&ch| ch == 0)
            .unwrap_or(info.szDevice.len())],
    )
    .to_string_lossy()
    .into_owned();

    let index = list.len() as i32;
    list.push(CapturedDesktopMonitor {
        index,
        name,
        x: monitor.rcMonitor.left,
        y: monitor.rcMonitor.top,
        width: monitor.rcMonitor.right - monitor.rcMonitor.left,
        height: monitor.rcMonitor.bottom - monitor.rcMonitor.top,
        work_x: monitor.rcWork.left,
        work_y: monitor.rcWork.top,
        work_width: monitor.rcWork.right - monitor.rcWork.left,
        work_height: monitor.rcWork.bottom - monitor.rcWork.top,
        is_primary: (monitor.dwFlags & MONITORINFOF_PRIMARY) != 0,
    });

    BOOL(1)
}

fn hwnd_to_string(hwnd: HWND) -> String {
    format!("0x{:016X}", hwnd.0 as usize)
}

unsafe fn read_window_title(hwnd: HWND) -> String {
    let len = GetWindowTextLengthW(hwnd);
    if len <= 0 {
        return String::new();
    }
    let mut buf = vec![0u16; (len as usize) + 1];
    let written = GetWindowTextW(hwnd, &mut buf);
    if written <= 0 {
        return String::new();
    }
    buf.truncate(written as usize);
    OsString::from_wide(&buf).to_string_lossy().into_owned()
}

/// Spawns a process without going through a shell (no injection via `cmd /c`).
#[derive(Debug, Default, Clone, Copy)]
pub struct Win32ProcessLauncher;

impl ProcessLauncher for Win32ProcessLauncher {
    fn launch(&self, request: &ProcessLaunchRequest) -> Result<ProcessLaunchOutcome> {
        let executable = request.executable.trim();
        if executable.is_empty() {
            return Err(WindowsIntegrationError::InvalidLaunchTarget(
                "executable path is required".into(),
            ));
        }
        if executable.contains('\0') {
            return Err(WindowsIntegrationError::InvalidLaunchTarget(
                "executable path contains invalid characters".into(),
            ));
        }

        let mut command = Command::new(executable);
        command.args(&request.args);
        let child = command.spawn().map_err(|error| {
            WindowsIntegrationError::LaunchFailed(format!(
                "failed to spawn '{executable}': {error}"
            ))
        })?;

        Ok(ProcessLaunchOutcome {
            process_id: Some(child.id()),
            simulated: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_includes_foreground_hwnd_when_present() {
        let capture = Win32WindowEnumerator.capture_desktop().unwrap();
        if capture.windows.iter().any(|window| window.focused) {
            assert!(capture.foreground_hwnd.is_some());
            let focused = capture.windows.iter().find(|window| window.focused).unwrap();
            assert_eq!(capture.foreground_hwnd.as_deref(), Some(focused.hwnd.as_str()));
        }
    }

    #[test]
    fn capture_records_window_bounds_when_windows_exist() {
        let capture = Win32WindowEnumerator.capture_desktop().unwrap();
        for window in &capture.windows {
            assert!(!window.title.trim().is_empty());
            assert!(window.hwnd.starts_with("0x"));
            // Some shell / cloaked HWNDs report 0×0 without IsIconic; accept either
            // a non-zero edge or an explicit minimized flag.
            assert!(
                window.minimized || window.width > 0 || window.height > 0,
                "window {} had empty bounds without minimized",
                window.hwnd
            );
        }
        if !capture.windows.is_empty() {
            assert!(
                capture
                    .windows
                    .iter()
                    .any(|window| window.process_name.is_some()),
                "at least one window should resolve a process image basename"
            );
        }
    }

    #[test]
    fn capture_enumerates_monitors_on_windows() {
        let capture = Win32WindowEnumerator.capture_desktop().unwrap();
        assert!(!capture.monitors.is_empty());
        assert!(capture.monitors.iter().any(|monitor| monitor.is_primary));
    }

    #[test]
    fn legacy_enumeration_still_works() {
        let result = Win32WindowEnumerator.enumerate_windows();
        assert!(result.is_ok());
        for window in result.unwrap() {
            assert!(!window.title.trim().is_empty());
            assert!(window.hwnd.starts_with("0x"));
        }
    }

    #[test]
    fn rejects_empty_executable() {
        let error = Win32ProcessLauncher
            .launch(&ProcessLaunchRequest {
                executable: "  ".into(),
                args: Vec::new(),
            })
            .unwrap_err();
        assert!(matches!(
            error,
            WindowsIntegrationError::InvalidLaunchTarget(_)
        ));
    }
}
