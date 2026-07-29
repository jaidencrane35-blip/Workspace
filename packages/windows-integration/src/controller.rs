//! Window control boundary (DAF-1a).
//!
//! Why: arrange real OS windows for Desktop Arrangement Foundation.
//! Owner: this crate only — kernel/UI must not call Win32.
//! Non-goals: permissions, persistence, IPC, Assistant, audio, grouping.

use std::sync::Mutex;

use crate::error::{Result, WindowsIntegrationError};

/// Screen-space window rectangle (pixels).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl WindowBounds {
    pub fn validate(&self) -> Result<()> {
        if self.width <= 0 || self.height <= 0 {
            return Err(WindowsIntegrationError::InvalidWindowBounds(format!(
                "width and height must be positive (got {}×{})",
                self.width, self.height
            )));
        }
        Ok(())
    }
}

/// Request to move/resize a top-level window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetWindowBoundsRequest {
    /// HWND identity string from capture (`0x` + 16 hex digits).
    pub hwnd: String,
    pub bounds: WindowBounds,
}

/// Request to bring a window to the foreground.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusWindowRequest {
    pub hwnd: String,
}

/// Outcome of a window control attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowControlOutcome {
    pub hwnd: String,
    /// True when the operation was recorded by the stub (non-Windows / tests).
    pub simulated: bool,
}

/// Operations recorded by [`StubWindowController`] for tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordedWindowControl {
    SetBounds(SetWindowBoundsRequest),
    Focus(FocusWindowRequest),
}

/// Platform window controller — Win32 on Windows, stub elsewhere.
///
/// Deliberately does **not** authorize calls. Permission Gateway belongs in kernel.
pub trait WindowController: Send + Sync {
    fn set_bounds(&self, request: &SetWindowBoundsRequest) -> Result<WindowControlOutcome>;
    fn focus(&self, request: &FocusWindowRequest) -> Result<WindowControlOutcome>;
}

/// Parses capture-format HWND strings (`0x00000000000ABCDE`).
pub fn parse_hwnd_value(hwnd: &str) -> Result<usize> {
    let trimmed = hwnd.trim();
    if trimmed.is_empty() {
        return Err(WindowsIntegrationError::InvalidWindowHandle(
            "hwnd is required".into(),
        ));
    }
    let hex = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);
    usize::from_str_radix(hex, 16).map_err(|_| {
        WindowsIntegrationError::InvalidWindowHandle(format!(
            "hwnd must be a hex value (got '{trimmed}')"
        ))
    })
}

/// Non-Windows / test controller — records operations; does not touch the OS.
#[derive(Debug, Default)]
pub struct StubWindowController {
    recorded: Mutex<Vec<RecordedWindowControl>>,
}

impl StubWindowController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recorded(&self) -> Vec<RecordedWindowControl> {
        self.recorded
            .lock()
            .expect("stub window controller lock")
            .clone()
    }

    pub fn clear_recorded(&self) {
        self.recorded
            .lock()
            .expect("stub window controller lock")
            .clear();
    }
}

impl WindowController for StubWindowController {
    fn set_bounds(&self, request: &SetWindowBoundsRequest) -> Result<WindowControlOutcome> {
        request.bounds.validate()?;
        parse_hwnd_value(&request.hwnd)?;
        self.recorded
            .lock()
            .expect("stub window controller lock")
            .push(RecordedWindowControl::SetBounds(request.clone()));
        Ok(WindowControlOutcome {
            hwnd: request.hwnd.clone(),
            simulated: true,
        })
    }

    fn focus(&self, request: &FocusWindowRequest) -> Result<WindowControlOutcome> {
        parse_hwnd_value(&request.hwnd)?;
        self.recorded
            .lock()
            .expect("stub window controller lock")
            .push(RecordedWindowControl::Focus(request.clone()));
        Ok(WindowControlOutcome {
            hwnd: request.hwnd.clone(),
            simulated: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hwnd_accepts_capture_format() {
        assert_eq!(parse_hwnd_value("0x00000000000000AA").unwrap(), 0xAA);
        assert_eq!(parse_hwnd_value("0X10").unwrap(), 0x10);
    }

    #[test]
    fn parse_hwnd_rejects_empty() {
        assert!(matches!(
            parse_hwnd_value("  "),
            Err(WindowsIntegrationError::InvalidWindowHandle(_))
        ));
    }

    #[test]
    fn stub_set_bounds_records_and_simulates() {
        let controller = StubWindowController::new();
        let request = SetWindowBoundsRequest {
            hwnd: "0x00000000000000AA".into(),
            bounds: WindowBounds {
                x: 10,
                y: 20,
                width: 800,
                height: 600,
            },
        };
        let outcome = controller.set_bounds(&request).unwrap();
        assert!(outcome.simulated);
        assert_eq!(controller.recorded().len(), 1);
    }

    #[test]
    fn stub_rejects_non_positive_bounds() {
        let controller = StubWindowController::new();
        let err = controller
            .set_bounds(&SetWindowBoundsRequest {
                hwnd: "0x1".into(),
                bounds: WindowBounds {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 100,
                },
            })
            .unwrap_err();
        assert!(matches!(
            err,
            WindowsIntegrationError::InvalidWindowBounds(_)
        ));
        assert!(controller.recorded().is_empty());
    }

    #[test]
    fn stub_focus_records() {
        let controller = StubWindowController::new();
        controller
            .focus(&FocusWindowRequest {
                hwnd: "0x00000000000000BB".into(),
            })
            .unwrap();
        assert_eq!(
            controller.recorded(),
            vec![RecordedWindowControl::Focus(FocusWindowRequest {
                hwnd: "0x00000000000000BB".into(),
            })]
        );
    }
}
