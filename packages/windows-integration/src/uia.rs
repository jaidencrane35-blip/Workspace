//! UI Automation port — observation (C-OBS-003/004) + interaction (C-ACT-004/005)
//! + wait fixtures (C-VER-003).
//!
//! WRAP Windows UI Automation. Providers never invent success: locate → act → verify.

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

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

/// Result of a bounded UI interaction with observable verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiInteractionOutcome {
    pub control: UiControlSnapshot,
    /// True when post-condition was observed (not assumed).
    pub verified: bool,
    pub detail: String,
}

/// Bounded UI tree observation + Kernel-gated interaction for a host window.
pub trait UiAutomationPort: Send + Sync {
    fn enumerate_controls(&self, hwnd: &str, limit: usize) -> Result<Vec<UiControlSnapshot>>;

    /// Locate a named control inside a window (C-OBS-004).
    fn find_control(&self, hwnd: &str, control_query: &str) -> Result<Option<UiControlSnapshot>> {
        let needle = control_query.trim();
        if needle.is_empty() {
            return Ok(None);
        }
        let controls = self.enumerate_controls(hwnd, 80)?;
        Ok(match_control(&controls, needle).cloned())
    }

    /// C-ACT-004 — invoke/click a named control; verify control remains discoverable.
    fn invoke_control(&self, hwnd: &str, control_query: &str) -> Result<UiInteractionOutcome>;

    /// C-ACT-005 — set value / type into a named control; verify read-back when possible.
    fn set_control_value(
        &self,
        hwnd: &str,
        control_query: &str,
        value: &str,
    ) -> Result<UiInteractionOutcome>;
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
        .or_else(|| controls.iter().find(|c| c.name.eq_ignore_ascii_case(n)))
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

fn control_key(c: &UiControlSnapshot) -> String {
    if c.automation_id.is_empty() {
        format!("{}|{}", c.name, c.control_type)
    } else {
        c.automation_id.clone()
    }
}

/// Non-Windows / unavailable stub.
pub struct StubUiAutomationPort;

impl UiAutomationPort for StubUiAutomationPort {
    fn enumerate_controls(&self, _hwnd: &str, _limit: usize) -> Result<Vec<UiControlSnapshot>> {
        Err(WindowsIntegrationError::UiAutomationFailed(
            "UI Automation is only available on Windows.".into(),
        ))
    }

    fn invoke_control(&self, _hwnd: &str, _control_query: &str) -> Result<UiInteractionOutcome> {
        Err(WindowsIntegrationError::UiAutomationFailed(
            "UI Automation click is only available on Windows.".into(),
        ))
    }

    fn set_control_value(
        &self,
        _hwnd: &str,
        _control_query: &str,
        _value: &str,
    ) -> Result<UiInteractionOutcome> {
        Err(WindowsIntegrationError::UiAutomationFailed(
            "UI Automation typing is only available on Windows.".into(),
        ))
    }
}

/// In-memory fixture for Kernel unit tests (mutable interaction + wait scheduling).
pub struct MemoryUiAutomationPort {
    controls: Mutex<Vec<UiControlSnapshot>>,
    appear_at: Mutex<Vec<(std::time::Instant, UiControlSnapshot)>>,
    disappear_at: Mutex<Vec<(std::time::Instant, String)>>,
    values: Mutex<HashMap<String, String>>,
    invoked: Mutex<HashSet<String>>,
}

impl MemoryUiAutomationPort {
    pub fn fixture() -> Self {
        Self {
            controls: Mutex::new(vec![
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
            ]),
            appear_at: Mutex::new(Vec::new()),
            disappear_at: Mutex::new(Vec::new()),
            values: Mutex::new(HashMap::new()),
            invoked: Mutex::new(HashSet::new()),
        }
    }

    /// Schedule a control to become discoverable after `delay_ms` (C-VER-003 tests).
    pub fn schedule_appear(&self, control: UiControlSnapshot, delay_ms: u64) {
        let at = std::time::Instant::now() + std::time::Duration::from_millis(delay_ms);
        self.appear_at
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push((at, control));
    }

    /// Schedule a control to leave the live set after `delay_ms`.
    pub fn schedule_disappear(&self, name: &str, delay_ms: u64) {
        let at = std::time::Instant::now() + std::time::Duration::from_millis(delay_ms);
        self.disappear_at
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push((at, name.to_string()));
    }

    /// Remove a control from the live set immediately (for control_gone waits).
    pub fn remove_control_named(&self, name: &str) {
        let mut controls = self.controls.lock().unwrap_or_else(|e| e.into_inner());
        controls.retain(|c| !c.name.eq_ignore_ascii_case(name));
    }

    fn flush_scheduled(&self) {
        let now = std::time::Instant::now();
        {
            let mut pending = self.appear_at.lock().unwrap_or_else(|e| e.into_inner());
            let mut controls = self.controls.lock().unwrap_or_else(|e| e.into_inner());
            let mut remain = Vec::new();
            for (at, control) in pending.drain(..) {
                if at <= now {
                    if !controls
                        .iter()
                        .any(|c| c.name.eq_ignore_ascii_case(&control.name))
                    {
                        controls.push(control);
                    }
                } else {
                    remain.push((at, control));
                }
            }
            *pending = remain;
        }
        {
            let mut pending = self.disappear_at.lock().unwrap_or_else(|e| e.into_inner());
            let mut controls = self.controls.lock().unwrap_or_else(|e| e.into_inner());
            let mut remain = Vec::new();
            for (at, name) in pending.drain(..) {
                if at <= now {
                    controls.retain(|c| !c.name.eq_ignore_ascii_case(&name));
                } else {
                    remain.push((at, name));
                }
            }
            *pending = remain;
        }
    }
}

impl UiAutomationPort for MemoryUiAutomationPort {
    fn enumerate_controls(&self, _hwnd: &str, limit: usize) -> Result<Vec<UiControlSnapshot>> {
        self.flush_scheduled();
        let controls = self.controls.lock().unwrap_or_else(|e| e.into_inner());
        Ok(controls.iter().take(limit.max(1)).cloned().collect())
    }

    fn invoke_control(&self, hwnd: &str, control_query: &str) -> Result<UiInteractionOutcome> {
        let Some(control) = self.find_control(hwnd, control_query)? else {
            return Err(WindowsIntegrationError::UiAutomationFailed(format!(
                "I couldn’t find a control named “{}” to click.",
                control_query.trim()
            )));
        };
        let key = control_key(&control);
        self.invoked
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(key.clone());
        let still = self.find_control(hwnd, &control.name)?;
        let verified = still.is_some()
            && self
                .invoked
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .contains(&key);
        Ok(UiInteractionOutcome {
            detail: if verified {
                format!("Clicked “{}” ({})", control.name, control.control_type)
            } else {
                format!(
                    "Tried to click “{}”, but I couldn’t verify the result.",
                    control.name
                )
            },
            control,
            verified,
        })
    }

    fn set_control_value(
        &self,
        hwnd: &str,
        control_query: &str,
        value: &str,
    ) -> Result<UiInteractionOutcome> {
        let Some(control) = self.find_control(hwnd, control_query)? else {
            return Err(WindowsIntegrationError::UiAutomationFailed(format!(
                "I couldn’t find a control named “{}” to type into.",
                control_query.trim()
            )));
        };
        if !matches!(
            control.control_type.as_str(),
            "Edit" | "ComboBox" | "Document" | "Text"
        ) {
            return Err(WindowsIntegrationError::UiAutomationFailed(format!(
                "“{}” ({}) doesn’t accept typed text.",
                control.name, control.control_type
            )));
        }
        let key = control_key(&control);
        self.values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(key.clone(), value.to_string());
        let read_back = self
            .values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&key)
            .cloned();
        let verified = read_back.as_deref() == Some(value);
        Ok(UiInteractionOutcome {
            detail: if verified {
                format!("Typed into “{}” and confirmed the value.", control.name)
            } else {
                format!(
                    "Typed into “{}”, but I couldn’t confirm the value.",
                    control.name
                )
            },
            control,
            verified,
        })
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
        with_window_elements(hwnd, |elements| {
            let limit = limit.clamp(1, 80);
            let mut out = Vec::new();
            for el in elements {
                if out.len() >= limit {
                    break;
                }
                if let Some(snap) = element_snapshot(&el) {
                    out.push(snap);
                }
            }
            Ok(out)
        })
    }

    fn invoke_control(&self, hwnd: &str, control_query: &str) -> Result<UiInteractionOutcome> {
        use windows::core::Interface;
        use windows::Win32::UI::Accessibility::{
            IUIAutomationInvokePattern, UIA_InvokePatternId,
        };

        let needle = control_query.trim();
        let control = with_window_elements(hwnd, |elements| {
            find_element(&elements, needle)
                .map(|(snap, el)| (snap, el))
                .ok_or_else(|| {
                    WindowsIntegrationError::UiAutomationFailed(format!(
                        "I couldn’t find a control named “{needle}” to click."
                    ))
                })
        })?;

        let (snap, el) = control;
        unsafe {
            let enabled = el.CurrentIsEnabled().map(|b| b.as_bool()).unwrap_or(true);
            if !enabled {
                return Err(WindowsIntegrationError::UiAutomationFailed(format!(
                    "“{}” is disabled, so I can’t click it.",
                    snap.name
                )));
            }
            let pattern = el
                .GetCurrentPattern(UIA_InvokePatternId)
                .map_err(|_| {
                    WindowsIntegrationError::UiAutomationFailed(format!(
                        "“{}” doesn’t support click/invoke.",
                        snap.name
                    ))
                })?;
            let invoke: IUIAutomationInvokePattern = pattern.cast().map_err(|_| {
                WindowsIntegrationError::UiAutomationFailed(format!(
                    "“{}” doesn’t support click/invoke.",
                    snap.name
                ))
            })?;
            invoke.Invoke().map_err(|e| {
                WindowsIntegrationError::UiAutomationFailed(format!(
                    "I couldn’t click “{}” ({e}).",
                    snap.name
                ))
            })?;
        }

        // Observable verify: control still discoverable after invoke.
        let verified = self.find_control(hwnd, &snap.name)?.is_some();
        Ok(UiInteractionOutcome {
            detail: if verified {
                format!("Clicked “{}” ({})", snap.name, snap.control_type)
            } else {
                format!(
                    "Clicked “{}”, but I couldn’t re-find it afterward to confirm.",
                    snap.name
                )
            },
            control: snap,
            verified,
        })
    }

    fn set_control_value(
        &self,
        hwnd: &str,
        control_query: &str,
        value: &str,
    ) -> Result<UiInteractionOutcome> {
        use windows::core::{Interface, BSTR};
        use windows::Win32::UI::Accessibility::{
            IUIAutomationValuePattern, UIA_ValuePatternId,
        };

        let needle = control_query.trim();
        let control = with_window_elements(hwnd, |elements| {
            find_element(&elements, needle)
                .map(|(snap, el)| (snap, el))
                .ok_or_else(|| {
                    WindowsIntegrationError::UiAutomationFailed(format!(
                        "I couldn’t find a control named “{needle}” to type into."
                    ))
                })
        })?;

        let (snap, el) = control;
        let read_back = unsafe {
            let enabled = el.CurrentIsEnabled().map(|b| b.as_bool()).unwrap_or(true);
            if !enabled {
                return Err(WindowsIntegrationError::UiAutomationFailed(format!(
                    "“{}” is disabled, so I can’t type into it.",
                    snap.name
                )));
            }
            let pattern = el.GetCurrentPattern(UIA_ValuePatternId).map_err(|_| {
                WindowsIntegrationError::UiAutomationFailed(format!(
                    "“{}” doesn’t accept typed text.",
                    snap.name
                ))
            })?;
            let value_pattern: IUIAutomationValuePattern = pattern.cast().map_err(|_| {
                WindowsIntegrationError::UiAutomationFailed(format!(
                    "“{}” doesn’t accept typed text.",
                    snap.name
                ))
            })?;
            if value_pattern.CurrentIsReadOnly().map(|b| b.as_bool()).unwrap_or(false) {
                return Err(WindowsIntegrationError::UiAutomationFailed(format!(
                    "“{}” is read-only.",
                    snap.name
                )));
            }
            value_pattern
                .SetValue(&BSTR::from(value))
                .map_err(|e| {
                    WindowsIntegrationError::UiAutomationFailed(format!(
                        "I couldn’t type into “{}” ({e}).",
                        snap.name
                    ))
                })?;
            value_pattern
                .CurrentValue()
                .ok()
                .map(|b| b.to_string())
                .unwrap_or_default()
        };

        let verified = read_back == value;
        Ok(UiInteractionOutcome {
            detail: if verified {
                format!("Typed into “{}” and confirmed the value.", snap.name)
            } else {
                format!(
                    "Typed into “{}”, but the value didn’t match what I set.",
                    snap.name
                )
            },
            control: snap,
            verified,
        })
    }
}

#[cfg(windows)]
fn with_window_elements<T>(
    hwnd_raw: &str,
    f: impl FnOnce(Vec<windows::Win32::UI::Accessibility::IUIAutomationElement>) -> Result<T>,
) -> Result<T> {
    use std::ffi::c_void;

    use windows::Win32::Foundation::HWND;
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
    };
    use windows::Win32::UI::Accessibility::{
        CUIAutomation, IUIAutomation, IUIAutomationElement, TreeScope_Descendants,
    };

    let hwnd_val: isize = hwnd_raw.trim().parse().map_err(|_| {
        WindowsIntegrationError::UiAutomationFailed("Invalid window handle.".into())
    })?;
    if hwnd_val == 0 {
        return Err(WindowsIntegrationError::UiAutomationFailed(
            "Invalid window handle.".into(),
        ));
    }

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
        let found = root.FindAll(TreeScope_Descendants, &condition).map_err(|e| {
            WindowsIntegrationError::UiAutomationFailed(format!(
                "I couldn’t walk the control tree ({e})."
            ))
        })?;
        let count = found.Length().unwrap_or(0).max(0);
        let mut elements = Vec::new();
        for i in 0..count {
            if let Ok(el) = found.GetElement(i) {
                elements.push(el);
            }
        }
        f(elements)
    }
}

#[cfg(windows)]
fn element_snapshot(
    el: &windows::Win32::UI::Accessibility::IUIAutomationElement,
) -> Option<UiControlSnapshot> {
    unsafe {
        let name = el
            .CurrentName()
            .ok()
            .map(|b| b.to_string())
            .unwrap_or_default()
            .trim()
            .to_string();
        if name.is_empty() {
            return None;
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
        let is_control = el
            .CurrentIsControlElement()
            .map(|b| b.as_bool())
            .unwrap_or(true);
        if !is_control && control_type == "Pane" {
            return None;
        }
        Some(UiControlSnapshot {
            name,
            control_type,
            automation_id,
        })
    }
}

#[cfg(windows)]
fn find_element(
    elements: &[windows::Win32::UI::Accessibility::IUIAutomationElement],
    needle: &str,
) -> Option<(
    UiControlSnapshot,
    windows::Win32::UI::Accessibility::IUIAutomationElement,
)> {
    let snaps: Vec<(UiControlSnapshot, usize)> = elements
        .iter()
        .enumerate()
        .filter_map(|(i, el)| element_snapshot(el).map(|s| (s, i)))
        .collect();
    let controls: Vec<UiControlSnapshot> = snaps.iter().map(|(s, _)| s.clone()).collect();
    let matched = match_control(&controls, needle)?;
    let idx = snaps.iter().position(|(s, _)| s == matched)?;
    let (_, el_idx) = snaps[idx];
    Some((matched.clone(), elements[el_idx].clone()))
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
