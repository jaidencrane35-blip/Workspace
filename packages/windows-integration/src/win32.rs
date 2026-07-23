//! Win32 EnumWindows-backed enumerator (Windows only).

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
};

use super::enumerator::{DesktopWindowSnapshot, WindowEnumerator};
use crate::error::{Result, WindowsIntegrationError};

#[derive(Debug, Default, Clone, Copy)]
pub struct Win32WindowEnumerator;

impl WindowEnumerator for Win32WindowEnumerator {
    fn enumerate_windows(&self) -> Result<Vec<DesktopWindowSnapshot>> {
        let mut collected: Vec<DesktopWindowSnapshot> = Vec::new();
        let ctx = &mut collected as *mut Vec<DesktopWindowSnapshot>;

        // SAFETY: EnumWindows invokes the callback synchronously on this thread.
        // `ctx` remains valid for the duration of the call.
        let ok = unsafe { EnumWindows(Some(enum_proc), LPARAM(ctx as isize)) };

        if ok.is_err() {
            return Err(WindowsIntegrationError::EnumerationFailed(
                "EnumWindows returned an error".into(),
            ));
        }

        Ok(collected)
    }
}

unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let list = &mut *(lparam.0 as *mut Vec<DesktopWindowSnapshot>);

    let visible = unsafe { IsWindowVisible(hwnd) }.as_bool();
    if !visible {
        return BOOL(1);
    }

    let title = unsafe { read_window_title(hwnd) };
    // Skip untitled tool / ghost windows for Phase 1 readability.
    if title.trim().is_empty() {
        return BOOL(1);
    }

    let mut process_id: u32 = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));
    }

    list.push(DesktopWindowSnapshot {
        hwnd: format!("0x{:016X}", hwnd.0 as usize),
        title,
        process_id,
        visible,
    });

    BOOL(1)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerates_without_panic() {
        let result = Win32WindowEnumerator.enumerate_windows();
        assert!(result.is_ok());
        // May be empty in headless CI; when present, titles are non-empty.
        for window in result.unwrap() {
            assert!(!window.title.trim().is_empty());
            assert!(window.hwnd.starts_with("0x"));
        }
    }
}
