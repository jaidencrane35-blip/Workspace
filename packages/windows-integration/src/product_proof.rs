//! Live Windows product-proof harness (Win32 only).
//!
//! Collects production evidence against the real desktop session. Never uses
//! stub fixtures. Writes `architecture/evidence/windows-product-proof.json`.

#![cfg(windows)]

use std::fs;
use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::thread;
use std::time::Duration;

use serde::Serialize;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DestroyWindow, RegisterClassW, ShowWindow, CS_HREDRAW, CS_VREDRAW,
    CW_USEDEFAULT, SW_SHOW, WINDOW_EX_STYLE, WNDCLASSW, WS_OVERLAPPEDWINDOW,
};

use crate::mutator::{MutatorEffectOutcome, WindowMutator, WindowPlacementRequest};
use crate::win32::Win32WindowEnumerator;
use crate::DesktopCapturer;

#[derive(Debug, Serialize)]
struct EvidenceRow {
    requested_action: String,
    win32_result: String,
    os_refusal: bool,
    notes: String,
}

#[derive(Debug, Serialize)]
struct ProductProofEvidence {
    collected_at: String,
    os: String,
    desktop_session_id: String,
    monitor_count: usize,
    monitors: Vec<MonitorEvidence>,
    window_count: usize,
    minimized_windows_observed: usize,
    maximized_heuristic_windows: usize,
    mixed_geometry_monitors: bool,
    monitor_disconnect_reconnect_exercised: bool,
    behaviour_matrix: Vec<EvidenceRow>,
    proof_hwnd: Option<String>,
}

#[derive(Debug, Serialize)]
struct MonitorEvidence {
    index: i32,
    name: String,
    width: i32,
    height: i32,
    is_primary: bool,
    x: i32,
    y: i32,
}

fn evidence_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../architecture/evidence/windows-product-proof.json")
}

fn outcome_label(outcome: MutatorEffectOutcome) -> String {
    match outcome {
        MutatorEffectOutcome::Committed => "committed".into(),
        MutatorEffectOutcome::RefusedByEnvironment => "refused_by_environment".into(),
        MutatorEffectOutcome::OutcomeUnknown => "outcome_unknown".into(),
    }
}

fn hwnd_to_hex(hwnd: HWND) -> String {
    format!("0x{:016X}", hwnd.0 as usize)
}

unsafe extern "system" fn proof_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    windows::Win32::UI::WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam)
}

/// Create a real top-level window owned by this process for mutation proof.
fn create_proof_window() -> HWND {
    static CLASS_ATOM: AtomicIsize = AtomicIsize::new(0);

    let class_name: Vec<u16> = "WorkspaceProductProofWindow\0".encode_utf16().collect();
    let window_name: Vec<u16> = "Workspace Product Proof\0".encode_utf16().collect();

    unsafe {
        let module = GetModuleHandleW(None).expect("GetModuleHandleW");
        if CLASS_ATOM.load(Ordering::SeqCst) == 0 {
            let class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(proof_wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: module.into(),
                hIcon: Default::default(),
                hCursor: Default::default(),
                hbrBackground: Default::default(),
                lpszMenuName: PCWSTR::null(),
                lpszClassName: PCWSTR(class_name.as_ptr()),
            };
            let atom = RegisterClassW(&class);
            assert!(atom != 0, "RegisterClassW failed");
            CLASS_ATOM.store(atom as isize, Ordering::SeqCst);
        }

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(window_name.as_ptr()),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            640,
            480,
            None,
            None,
            module,
            Some(ptr::null()),
        )
        .expect("CreateWindowExW");
        let _ = ShowWindow(hwnd, SW_SHOW);
        // Allow the window to enter the enumeration set.
        thread::sleep(Duration::from_millis(200));
        hwnd
    }
}

#[test]
fn live_windows_product_proof_capture_and_behaviour_matrix() {
    let capturer = Win32WindowEnumerator;
    let mutator = Win32WindowEnumerator;

    let baseline = capturer.capture_desktop().expect("live capture");
    assert!(
        !baseline.monitors.is_empty(),
        "production evidence requires at least one monitor"
    );

    let monitors: Vec<MonitorEvidence> = baseline
        .monitors
        .iter()
        .map(|m| MonitorEvidence {
            index: m.index,
            name: m.name.clone(),
            width: m.width,
            height: m.height,
            is_primary: m.is_primary,
            x: m.x,
            y: m.y,
        })
        .collect();

    let minimized_windows_observed = baseline.windows.iter().filter(|w| w.minimized).count();
    let maximized_heuristic_windows = baseline
        .windows
        .iter()
        .filter(|w| {
            let Some(idx) = w.monitor_index else {
                return false;
            };
            let Some(mon) = baseline.monitors.iter().find(|m| m.index == idx) else {
                return false;
            };
            w.width >= (mon.work_width as f64 * 0.95) as i32
                && w.height >= (mon.work_height as f64 * 0.95) as i32
        })
        .count();

    let mixed_geometry_monitors = monitors.len() > 1
        && monitors
            .windows(2)
            .any(|pair| pair[0].width != pair[1].width || pair[0].height != pair[1].height);

    let proof_hwnd = create_proof_window();
    let hwnd = hwnd_to_hex(proof_hwnd);

    // Confirm capture sees our proof window.
    let after_create = capturer.capture_desktop().expect("capture after create");
    assert!(
        after_create.windows.iter().any(|w| w.hwnd.eq_ignore_ascii_case(&hwnd)),
        "proof window must be visible to observation capture"
    );

    let mut matrix = Vec::new();

    let place = mutator
        .place_window(
            &hwnd,
            &WindowPlacementRequest {
                x: 120,
                y: 140,
                width: 640,
                height: 480,
                minimized: false,
            },
        )
        .expect("place_window");
    matrix.push(EvidenceRow {
        requested_action: "window.place".into(),
        win32_result: outcome_label(place),
        os_refusal: matches!(place, MutatorEffectOutcome::RefusedByEnvironment),
        notes: "SetWindowPos with SWP_NOACTIVATE|SWP_NOZORDER|SWP_SHOWWINDOW".into(),
    });

    let minimize = mutator
        .place_window(
            &hwnd,
            &WindowPlacementRequest {
                x: 120,
                y: 140,
                width: 640,
                height: 480,
                minimized: true,
            },
        )
        .expect("minimize");
    matrix.push(EvidenceRow {
        requested_action: "window.minimize".into(),
        win32_result: outcome_label(minimize),
        os_refusal: matches!(minimize, MutatorEffectOutcome::RefusedByEnvironment),
        notes: "ShowWindow(SW_MINIMIZE)".into(),
    });

    let restore = mutator
        .place_window(
            &hwnd,
            &WindowPlacementRequest {
                x: 200,
                y: 180,
                width: 700,
                height: 500,
                minimized: false,
            },
        )
        .expect("restore place");
    matrix.push(EvidenceRow {
        requested_action: "window.restore_minimized_then_place".into(),
        win32_result: outcome_label(restore),
        os_refusal: matches!(restore, MutatorEffectOutcome::RefusedByEnvironment),
        notes: "ShowWindow(SW_RESTORE) then SetWindowPos NOZORDER".into(),
    });

    let focus = mutator.focus_window(&hwnd).expect("focus_window");
    matrix.push(EvidenceRow {
        requested_action: "window.focus".into(),
        win32_result: outcome_label(focus),
        os_refusal: matches!(focus, MutatorEffectOutcome::RefusedByEnvironment),
        notes: "SetForegroundWindow — OS may refuse without input attachment".into(),
    });

    let primary = baseline
        .monitors
        .iter()
        .find(|m| m.is_primary)
        .expect("primary");
    let monitor_place = mutator
        .place_window(
            &hwnd,
            &WindowPlacementRequest {
                x: primary.work_x + 40,
                y: primary.work_y + 40,
                width: 720,
                height: 520,
                minimized: false,
            },
        )
        .expect("monitor place");
    matrix.push(EvidenceRow {
        requested_action: "window.monitor_placement_primary".into(),
        win32_result: outcome_label(monitor_place),
        os_refusal: matches!(monitor_place, MutatorEffectOutcome::RefusedByEnvironment),
        notes: format!("primary monitor index {}", primary.index),
    });

    if let Some(secondary) = baseline.monitors.iter().find(|m| !m.is_primary) {
        let dual = mutator
            .place_window(
                &hwnd,
                &WindowPlacementRequest {
                    x: secondary.work_x + 40,
                    y: secondary.work_y + 40,
                    width: 720,
                    height: 520,
                    minimized: false,
                },
            )
            .expect("secondary place");
        matrix.push(EvidenceRow {
            requested_action: "window.monitor_placement_secondary".into(),
            win32_result: outcome_label(dual),
            os_refusal: matches!(dual, MutatorEffectOutcome::RefusedByEnvironment),
            notes: format!("secondary monitor index {}", secondary.index),
        });
    } else {
        matrix.push(EvidenceRow {
            requested_action: "window.monitor_placement_secondary".into(),
            win32_result: "not_available".into(),
            os_refusal: false,
            notes: "single-monitor host — secondary placement not exercised".into(),
        });
    }

    // Z-order: place uses SWP_NOZORDER — capture z_order before/after place must
    // not be treated as an Action-owned reorder (Action skips z_order effects).
    matrix.push(EvidenceRow {
        requested_action: "window.z_order_preservation".into(),
        win32_result: "committed_by_contract".into(),
        os_refusal: false,
        notes: "place path sets SWP_NOZORDER; Action marks window.z_order unsupported".into(),
    });

    let still = mutator.window_by_hwnd(&hwnd).expect("lookup").is_some();
    matrix.push(EvidenceRow {
        requested_action: "window.identity_lookup".into(),
        win32_result: if still {
            "committed".into()
        } else {
            "missing".into()
        },
        os_refusal: !still,
        notes: "IsWindow after mutations".into(),
    });

    let evidence = ProductProofEvidence {
        collected_at: format!("{:?}", std::time::SystemTime::now()),
        os: format!(
            "Windows {}",
            std::env::var("OS").unwrap_or_else(|_| "unknown".into())
        ),
        desktop_session_id: baseline.desktop_session_id.clone(),
        monitor_count: monitors.len(),
        monitors,
        window_count: baseline.windows.len(),
        minimized_windows_observed,
        maximized_heuristic_windows,
        mixed_geometry_monitors,
        monitor_disconnect_reconnect_exercised: false,
        behaviour_matrix: matrix,
        proof_hwnd: Some(hwnd),
    };

    let path = evidence_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(
        &path,
        serde_json::to_string_pretty(&evidence).expect("serialize evidence"),
    )
    .expect("write evidence file");

    unsafe {
        let _ = DestroyWindow(proof_hwnd);
    }

    assert!(
        evidence
            .behaviour_matrix
            .iter()
            .any(|r| r.requested_action == "window.place" && r.win32_result == "committed"),
        "place must commit on live Windows"
    );
    assert!(
        evidence.behaviour_matrix.iter().any(|r| {
            r.requested_action == "window.restore_minimized_then_place"
                && r.win32_result == "committed"
        }),
        "restore-minimized must commit on live Windows"
    );
}
