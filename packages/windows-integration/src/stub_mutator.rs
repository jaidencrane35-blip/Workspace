//! Controllable WindowMutator for tests and non-Windows hosts.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::capture::{CapturedDesktopWindow, DesktopObservationCapture, STUB_DESKTOP_SESSION_ID};
use super::mutator::{
    LiveWindowView, MutatorEffectOutcome, WindowMutator, WindowPlacementRequest,
};
use crate::error::Result;

#[derive(Debug, Clone)]
struct StubDesktopState {
    session_id: String,
    windows: HashMap<String, CapturedDesktopWindow>,
    monitor_indices: Vec<i32>,
    refuse_place: bool,
    refuse_focus: bool,
    unknown_place: bool,
}

/// Injectable mutator used by Action acceptance tests.
#[derive(Debug, Clone)]
pub struct StubWindowMutator {
    state: Arc<Mutex<StubDesktopState>>,
}

impl StubWindowMutator {
    pub fn from_capture(capture: &DesktopObservationCapture) -> Self {
        let mut windows = HashMap::new();
        for window in &capture.windows {
            windows.insert(window.hwnd.clone(), window.clone());
        }
        let monitor_indices = capture.monitors.iter().map(|m| m.index).collect();
        Self {
            state: Arc::new(Mutex::new(StubDesktopState {
                session_id: capture.desktop_session_id.clone(),
                windows,
                monitor_indices,
                refuse_place: false,
                refuse_focus: false,
                unknown_place: false,
            })),
        }
    }

    pub fn fixture_dual_monitor() -> Self {
        Self::from_capture(&super::stub::dual_monitor_fixture())
    }

    pub fn set_session_id(&self, session_id: impl Into<String>) {
        self.state.lock().expect("stub mutator").session_id = session_id.into();
    }

    pub fn remove_window(&self, hwnd: &str) {
        self.state.lock().expect("stub mutator").windows.remove(hwnd);
    }

    pub fn set_window_title(&self, hwnd: &str, title: impl Into<String>) {
        if let Some(window) = self.state.lock().expect("stub mutator").windows.get_mut(hwnd) {
            window.title = title.into();
        }
    }

    pub fn set_window_process_id(&self, hwnd: &str, process_id: u32) {
        if let Some(window) = self.state.lock().expect("stub mutator").windows.get_mut(hwnd) {
            window.process_id = process_id;
        }
    }

    pub fn set_monitor_indices(&self, indices: Vec<i32>) {
        self.state.lock().expect("stub mutator").monitor_indices = indices;
    }

    pub fn refuse_place(&self) {
        self.state.lock().expect("stub mutator").refuse_place = true;
    }

    pub fn refuse_focus(&self) {
        self.state.lock().expect("stub mutator").refuse_focus = true;
    }

    pub fn unknown_place_outcome(&self) {
        self.state.lock().expect("stub mutator").unknown_place = true;
    }

    pub fn window_placement(&self, hwnd: &str) -> Option<(i32, i32, i32, i32, bool)> {
        self.state.lock().ok()?.windows.get(hwnd).map(|w| {
            (w.x, w.y, w.width, w.height, w.minimized)
        })
    }

    pub fn focused_hwnd(&self) -> Option<String> {
        self.state
            .lock()
            .ok()?
            .windows
            .values()
            .find(|w| w.focused)
            .map(|w| w.hwnd.clone())
    }
}

impl Default for StubWindowMutator {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(StubDesktopState {
                session_id: STUB_DESKTOP_SESSION_ID.into(),
                windows: HashMap::new(),
                monitor_indices: vec![0],
                refuse_place: false,
                refuse_focus: false,
                unknown_place: false,
            })),
        }
    }
}

impl WindowMutator for StubWindowMutator {
    fn current_desktop_session_id(&self) -> Result<String> {
        Ok(self.state.lock().expect("stub mutator").session_id.clone())
    }

    fn window_by_hwnd(&self, hwnd: &str) -> Result<Option<LiveWindowView>> {
        Ok(self
            .state
            .lock()
            .expect("stub mutator")
            .windows
            .get(hwnd)
            .map(|window| LiveWindowView {
                hwnd: window.hwnd.clone(),
                process_id: window.process_id,
                title: window.title.clone(),
            }))
    }

    fn attached_monitor_indices(&self) -> Result<Vec<i32>> {
        Ok(self
            .state
            .lock()
            .expect("stub mutator")
            .monitor_indices
            .clone())
    }

    fn place_window(
        &self,
        hwnd: &str,
        placement: &WindowPlacementRequest,
    ) -> Result<MutatorEffectOutcome> {
        let mut state = self.state.lock().expect("stub mutator");
        if state.unknown_place {
            return Ok(MutatorEffectOutcome::OutcomeUnknown);
        }
        if state.refuse_place {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        }
        let Some(window) = state.windows.get_mut(hwnd) else {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        };
        window.x = placement.x;
        window.y = placement.y;
        window.width = placement.width;
        window.height = placement.height;
        window.minimized = placement.minimized;
        Ok(MutatorEffectOutcome::Committed)
    }

    fn focus_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome> {
        let mut state = self.state.lock().expect("stub mutator");
        if state.refuse_focus {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        }
        if !state.windows.contains_key(hwnd) {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        }
        for window in state.windows.values_mut() {
            window.focused = window.hwnd == hwnd;
        }
        Ok(MutatorEffectOutcome::Committed)
    }

    fn minimize_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome> {
        let mut state = self.state.lock().expect("stub mutator");
        let Some(window) = state.windows.get_mut(hwnd) else {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        };
        window.minimized = true;
        window.visible = false;
        window.focused = false;
        Ok(MutatorEffectOutcome::Committed)
    }

    fn restore_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome> {
        let mut state = self.state.lock().expect("stub mutator");
        let Some(window) = state.windows.get_mut(hwnd) else {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        };
        window.minimized = false;
        window.visible = true;
        Ok(MutatorEffectOutcome::Committed)
    }

    fn close_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome> {
        let mut state = self.state.lock().expect("stub mutator");
        if state.windows.remove(hwnd).is_none() {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        }
        Ok(MutatorEffectOutcome::Committed)
    }

    fn maximize_window(&self, hwnd: &str) -> Result<MutatorEffectOutcome> {
        let mut state = self.state.lock().expect("stub mutator");
        let Some(window) = state.windows.get_mut(hwnd) else {
            return Ok(MutatorEffectOutcome::RefusedByEnvironment);
        };
        window.minimized = false;
        window.visible = true;
        window.x = 0;
        window.y = 0;
        window.width = 1920;
        window.height = 1040;
        Ok(MutatorEffectOutcome::Committed)
    }
}
