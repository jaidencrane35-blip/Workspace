//! UI Automation observation port (C-OBS-003 Desktop UI Tree, C-OBS-004 Window Control Discovery).
//!
//! WRAP Windows UI Automation for control list + locate-by-name.
//! No click/type here — Interaction capabilities are separate Atlas items.

use serde::{Deserialize, Serialize};

use crate::error::{Result, WindowsIntegrationError};

/// One discoverable control inside a top-level window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiControlSnapshot {
    pub name: String,
    pub control_type: String,
    pub automation_id: String,
}

/// Bounded UI tree observation for a host window (hwnd string).
pub trait UiAutomationPort: Send + Sync {
    fn enumerate_controls(&self, hwnd: &str, limit: usize) -> Result<Vec<UiControlSnapshot>>;

    /// Locate a named control inside a window (C-OBS-004). Default: scan enumerate results.
    fn find_control(&self, hwnd: &str, control_query: &str) -> Result<Option<UiControlSnapshot>> {
        let needle = control_query.trim();
        if needle.is_empty() {
            return Ok(None);
        }
        let controls = self.enumerate_controls(hwnd, 80)?;
        Ok(match_control(&controls, needle).cloned())
    }
}

/// Prefer exact name, then case-insensitive equality, then starts-with, then contains.
pub fn match_control<'a>(
    controls: &'a [UiControlSnapshot],
    needle: &str,
) -> Option<&'a UiControlSnapshot> {
    let n = needle.trim();
    if n.is_empty() {
        return None;
    }
    let n_lower = n.to_ascii_lowercase();
    controls
        .iter()
        .find(|c| c.name == n)
        .or_else(|| {
            controls
                .iter()
                .find(|c| c.name.eq_ignore_ascii_case(n))
        })
        .or_else(|| {
            controls.iter().find(|c| {
                c.name.to_ascii_lowercase().starts_with(&n_lower)
                    || c.automation_id.eq_ignore_ascii_case(n)
            })
        })
        .or_else(|| {
            controls
                .iter()
                .find(|c| c.name.to_ascii_lowercase().contains(&n_lower))
        })
}

/// Non-Windows / unavailable stub.
pub struct StubUiAutomationPort;

impl UiAutomationPort for StubUiAutomationPort {
    fn enumerate_controls(&self, _hwnd: &str, _limit: usize) -> Result<Vec<UiControlSnapshot>> {
        Err(WindowsIntegrationError::UiAutomationFailed(
            "UI Automation control discovery is only available on Windows.".into(),
        ))
    }
}

/// In-memory fixture for Kernel unit tests.
pub struct MemoryUiAutomationPort {
    pub controls: Vec<UiControlSnapshot>,
}

impl MemoryUiAutomationPort {
    pub fn fixture() -> Self {
        Self {
            controls: vec![
                UiControlSnapshot {
                    name: "File".into(),
                    control_type: "MenuItem".into(),
                    automation_id: "File".into(),
                },
                UiControlSnapshot {
                    name: "Edit".into(),
                    control_type: "Edit".into(),
                    automation_id: "15".into(),
                },
                UiControlSnapshot {
                    name: "Save".into(),
                    control_type: "Button".into(),
                    automation_id: "SaveBtn".into(),
                },
            ],
        }
    }
}

impl UiAutomationPort for MemoryUiAutomationPort {
    fn enumerate_controls(&self, _hwnd: &str, limit: usize) -> Result<Vec<UiControlSnapshot>> {
        Ok(self.controls.iter().take(limit.max(1)).cloned().collect())
    }
}

pub fn platform_ui_automation() -> Box<dyn UiAutomationPort> {
    #[cfg(windows)]
    {
        Box::new(Win32UiAutomationPort)
    }
    #[cfg(not(windows))]
    {
        Box::new(StubUiAutomationPort)
    }
}

#[cfg(windows)]
pub struct Win32UiAutomationPort;

#[cfg(windows)]
impl UiAutomationPort for Win32UiAutomationPort {
    fn enumerate_controls(&self, hwnd: &str, limit: usize) -> Result<Vec<UiControlSnapshot>> {
        enumerate_controls_win32(hwnd, limit)
    }
}

#[cfg(windows)]
fn enumerate_controls_win32(hwnd_raw: &str, limit: usize) -> Result<Vec<UiControlSnapshot>> {
    use std::ffi::c_void;

    use windows::Win32::Foundation::HWND;
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
    };
    use windows::Win32::UI::Accessibility::{
        CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationElementArray,
        TreeScope_Descendants,
    };

    let hwnd_val: isize = hwnd_raw.trim().parse().map_err(|_| {
        WindowsIntegrationError::UiAutomationFailed("Invalid window handle.".into())
    })?;
    if hwnd_val == 0 {
        return Err(WindowsIntegrationError::UiAutomationFailed(
            "Invalid window handle.".into(),
        ));
    }

    let limit = limit.clamp(1, 80);

    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)
            .map_err(|e| {
                WindowsIntegrationError::UiAutomationFailed(format!(
                    "UI Automation is unavailable ({e})."
                ))
            })?;

        let root: IUIAutomationElement = automation
            .ElementFromHandle(HWND(hwnd_val as *mut c_void))
            .map_err(|e| {
                WindowsIntegrationError::UiAutomationFailed(format!(
                    "I couldn’t read controls for that window ({e})."
                ))
            })?;

        let condition = automation.CreateTrueCondition().map_err(|e| {
            WindowsIntegrationError::UiAutomationFailed(format!(
                "UI Automation condition failed ({e})."
            ))
        })?;

        let found: IUIAutomationElementArray = root
            .FindAll(TreeScope_Descendants, &condition)
            .map_err(|e| {
                WindowsIntegrationError::UiAutomationFailed(format!(
                    "I couldn’t walk the control tree ({e})."
                ))
            })?;

        let count = found.Length().unwrap_or(0).max(0);
        let mut out = Vec::new();
        for i in 0..count {
            if out.len() >= limit {
                break;
            }
            let Ok(el) = found.GetElement(i) else {
                continue;
            };
            let name = el
                .CurrentName()
                .ok()
                .map(|b| b.to_string())
                .unwrap_or_default()
                .trim()
                .to_string();
            if name.is_empty() {
                continue;
            }
            let control_type = el
                .CurrentControlType()
                .ok()
                .map(|id| control_type_label(id.0))
                .unwrap_or_else(|| "Control".into());
            let automation_id = el
                .CurrentAutomationId()
                .ok()
                .map(|b| b.to_string())
                .unwrap_or_default()
                .trim()
                .to_string();

            // Prefer control / content elements with useful names.
            let is_control = el
                .CurrentIsControlElement()
                .map(|b| b.as_bool())
                .unwrap_or(true);
            if !is_control && control_type == "Pane" {
                continue;
            }

            out.push(UiControlSnapshot {
                name,
                control_type,
                automation_id,
            });
        }

        Ok(out)
    }
}

#[cfg(windows)]
fn control_type_label(id: i32) -> String {
    use windows::Win32::UI::Accessibility::{
        UIA_ButtonControlTypeId, UIA_CheckBoxControlTypeId, UIA_ComboBoxControlTypeId,
        UIA_EditControlTypeId, UIA_HyperlinkControlTypeId, UIA_ListControlTypeId,
        UIA_ListItemControlTypeId, UIA_MenuBarControlTypeId, UIA_MenuControlTypeId,
        UIA_MenuItemControlTypeId, UIA_PaneControlTypeId, UIA_TabItemControlTypeId,
        UIA_TextControlTypeId, UIA_ToolBarControlTypeId, UIA_TreeItemControlTypeId,
        UIA_WindowControlTypeId,
    };

    if id == UIA_ButtonControlTypeId.0 {
        "Button".into()
    } else if id == UIA_EditControlTypeId.0 {
        "Edit".into()
    } else if id == UIA_ComboBoxControlTypeId.0 {
        "ComboBox".into()
    } else if id == UIA_TextControlTypeId.0 {
        "Text".into()
    } else if id == UIA_ListItemControlTypeId.0 {
        "ListItem".into()
    } else if id == UIA_ListControlTypeId.0 {
        "List".into()
    } else if id == UIA_MenuItemControlTypeId.0 {
        "MenuItem".into()
    } else if id == UIA_MenuControlTypeId.0 {
        "Menu".into()
    } else if id == UIA_MenuBarControlTypeId.0 {
        "MenuBar".into()
    } else if id == UIA_ToolBarControlTypeId.0 {
        "ToolBar".into()
    } else if id == UIA_TreeItemControlTypeId.0 {
        "TreeItem".into()
    } else if id == UIA_WindowControlTypeId.0 {
        "Window".into()
    } else if id == UIA_PaneControlTypeId.0 {
        "Pane".into()
    } else if id == UIA_CheckBoxControlTypeId.0 {
        "CheckBox".into()
    } else if id == UIA_HyperlinkControlTypeId.0 {
        "Hyperlink".into()
    } else if id == UIA_TabItemControlTypeId.0 {
        "TabItem".into()
    } else {
        format!("Control({id})")
    }
}
