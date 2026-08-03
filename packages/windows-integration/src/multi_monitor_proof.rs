//! Multi-monitor topology product-proof harness (Win32 only).
//!
//! Always executable. Writes structured evidence.
//! Does **not** fabricate dual-monitor results when hardware is absent.
//! Optional hot-plug wait: `WORKSPACE_MULTI_MONITOR_HOTPLUG_SECONDS`.

#![cfg(windows)]

use std::fs;
use std::path::PathBuf;
use std::ptr;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

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
struct MonitorTopology {
    index: i32,
    name: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    work_x: i32,
    work_y: i32,
    work_width: i32,
    work_height: i32,
    is_primary: bool,
}

#[derive(Debug, Serialize)]
struct PlacementEvidence {
    action: String,
    monitor_index: i32,
    win32_result: String,
    os_refusal: bool,
}

#[derive(Debug, Serialize)]
struct MultiMonitorEvidence {
    collected_at: String,
    harness_ready: bool,
    hardware_dual_monitor: bool,
    monitor_count: usize,
    topology: Vec<MonitorTopology>,
    topology_fingerprint: String,
    cross_monitor_restore: String,
    placements: Vec<PlacementEvidence>,
    disconnect_reconnect: String,
    hotplug_wait_seconds: u64,
    hotplug_observed_change: bool,
}

fn evidence_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../architecture/evidence/multi-monitor-topology.json")
}

fn hwnd_to_hex(hwnd: HWND) -> String {
    format!("0x{:016X}", hwnd.0 as usize)
}

unsafe extern "system" fn mm_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    windows::Win32::UI::WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam)
}

fn create_proof_window() -> HWND {
    static CLASS_ATOM: AtomicIsize = AtomicIsize::new(0);
    let class_name: Vec<u16> = "WorkspaceMultiMonitorProof\0".encode_utf16().collect();
    let window_name: Vec<u16> = "Workspace Multi-Monitor Proof\0".encode_utf16().collect();
    unsafe {
        let module = GetModuleHandleW(None).expect("GetModuleHandleW");
        if CLASS_ATOM.load(Ordering::SeqCst) == 0 {
            let class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(mm_wnd_proc),
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
            500,
            400,
            None,
            None,
            module,
            Some(ptr::null()),
        )
        .expect("CreateWindowExW");
        let _ = ShowWindow(hwnd, SW_SHOW);
        thread::sleep(Duration::from_millis(150));
        hwnd
    }
}

fn fingerprint(topology: &[MonitorTopology]) -> String {
    let mut parts: Vec<String> = topology
        .iter()
        .map(|m| {
            format!(
                "{}:{}:{}x{}@{},{}:p={}",
                m.index, m.name, m.width, m.height, m.x, m.y, m.is_primary
            )
        })
        .collect();
    parts.sort();
    parts.join("|")
}

fn outcome_str(outcome: MutatorEffectOutcome) -> String {
    match outcome {
        MutatorEffectOutcome::Committed => "committed".into(),
        MutatorEffectOutcome::RefusedByEnvironment => "refused_by_environment".into(),
        MutatorEffectOutcome::OutcomeUnknown => "outcome_unknown".into(),
    }
}

#[test]
fn multi_monitor_topology_harness_writes_evidence() {
    let capturer = Win32WindowEnumerator;
    let mutator = Win32WindowEnumerator;
    let capture = capturer.capture_desktop().expect("live capture");
    assert!(!capture.monitors.is_empty());

    let topology: Vec<MonitorTopology> = capture
        .monitors
        .iter()
        .map(|m| MonitorTopology {
            index: m.index,
            name: m.name.clone(),
            x: m.x,
            y: m.y,
            width: m.width,
            height: m.height,
            work_x: m.work_x,
            work_y: m.work_y,
            work_width: m.work_width,
            work_height: m.work_height,
            is_primary: m.is_primary,
        })
        .collect();

    let hardware_dual_monitor = topology.len() >= 2;
    let topology_fingerprint = fingerprint(&topology);
    let mut placements = Vec::new();
    let mut cross_monitor_restore = if hardware_dual_monitor {
        "pending".into()
    } else {
        "not_executed_insufficient_hardware".into()
    };

    let proof_hwnd = create_proof_window();
    let hwnd = hwnd_to_hex(proof_hwnd);

    if hardware_dual_monitor {
        let primary = topology.iter().find(|m| m.is_primary).expect("primary");
        let secondary = topology.iter().find(|m| !m.is_primary).expect("secondary");

        let to_primary = mutator
            .place_window(
                &hwnd,
                &WindowPlacementRequest {
                    x: primary.work_x + 48,
                    y: primary.work_y + 48,
                    width: 640,
                    height: 480,
                    minimized: false,
                },
            )
            .expect("place primary");
        placements.push(PlacementEvidence {
            action: "place_on_primary".into(),
            monitor_index: primary.index,
            win32_result: outcome_str(to_primary),
            os_refusal: matches!(to_primary, MutatorEffectOutcome::RefusedByEnvironment),
        });

        let to_secondary = mutator
            .place_window(
                &hwnd,
                &WindowPlacementRequest {
                    x: secondary.work_x + 48,
                    y: secondary.work_y + 48,
                    width: 640,
                    height: 480,
                    minimized: false,
                },
            )
            .expect("place secondary");
        placements.push(PlacementEvidence {
            action: "place_on_secondary".into(),
            monitor_index: secondary.index,
            win32_result: outcome_str(to_secondary),
            os_refusal: matches!(to_secondary, MutatorEffectOutcome::RefusedByEnvironment),
        });

        let back = mutator
            .place_window(
                &hwnd,
                &WindowPlacementRequest {
                    x: primary.work_x + 80,
                    y: primary.work_y + 80,
                    width: 700,
                    height: 500,
                    minimized: false,
                },
            )
            .expect("restore primary");
        placements.push(PlacementEvidence {
            action: "restore_to_primary".into(),
            monitor_index: primary.index,
            win32_result: outcome_str(back),
            os_refusal: matches!(back, MutatorEffectOutcome::RefusedByEnvironment),
        });

        let all_committed = placements.iter().all(|p| p.win32_result == "committed");
        cross_monitor_restore = if all_committed {
            "executed_committed".into()
        } else {
            "executed_with_os_refusal".into()
        };
    }

    // Optional hot-plug observation window (manual cable pull during wait).
    let hotplug_wait_seconds = std::env::var("WORKSPACE_MULTI_MONITOR_HOTPLUG_SECONDS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);
    let mut hotplug_observed_change = false;
    let mut disconnect_reconnect = if hotplug_wait_seconds == 0 {
        "not_executed_no_wait_configured".into()
    } else {
        "waiting_for_topology_change".into()
    };

    if hotplug_wait_seconds > 0 {
        let deadline = Instant::now() + Duration::from_secs(hotplug_wait_seconds);
        while Instant::now() < deadline {
            thread::sleep(Duration::from_millis(500));
            let now = capturer.capture_desktop().expect("hotplug poll");
            let now_fp = fingerprint(
                &now.monitors
                    .iter()
                    .map(|m| MonitorTopology {
                        index: m.index,
                        name: m.name.clone(),
                        x: m.x,
                        y: m.y,
                        width: m.width,
                        height: m.height,
                        work_x: m.work_x,
                        work_y: m.work_y,
                        work_width: m.work_width,
                        work_height: m.work_height,
                        is_primary: m.is_primary,
                    })
                    .collect::<Vec<_>>(),
            );
            if now_fp != topology_fingerprint {
                hotplug_observed_change = true;
                disconnect_reconnect = "topology_change_observed".into();
                break;
            }
        }
        if !hotplug_observed_change {
            disconnect_reconnect = "waited_no_topology_change".into();
        }
    }

    unsafe {
        let _ = DestroyWindow(proof_hwnd);
    }

    let evidence = MultiMonitorEvidence {
        collected_at: format!("{:?}", std::time::SystemTime::now()),
        harness_ready: true,
        hardware_dual_monitor,
        monitor_count: topology.len(),
        topology,
        topology_fingerprint,
        cross_monitor_restore,
        placements,
        disconnect_reconnect,
        hotplug_wait_seconds,
        hotplug_observed_change,
    };

    let path = evidence_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(
        &path,
        serde_json::to_string_pretty(&evidence).expect("serialize"),
    )
    .expect("write multi-monitor evidence");

    // Harness always succeeds as a readiness proof. Dual-monitor claims require hardware.
    assert!(evidence.harness_ready);
    assert!(evidence.monitor_count >= 1);
}
