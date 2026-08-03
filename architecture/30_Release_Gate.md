# 30 — Release Gate

Each subsystem has exactly one status.  
**PASS** requires production evidence generated on a real Windows host.  
Stub/demo results do not qualify. Status is never upgraded without evidence from this inventory.

Evidence host: Windows build **26200**, single monitor (`\\.\DISPLAY1`), commit lineage through release-gate closure.

---

## Gate table

| Subsystem | Status | Evidence file | Automated tests | Manual / host validation | Remaining OS limitations |
| --- | --- | --- | --- | --- | --- |
| Observation | **PASS** | `evidence/windows-product-proof.json` | `win32::tests::*`, `product_proof::live_windows_product_proof_*` | Live Win32 capture on host | Cloaked/empty-title HWNDs excluded by design |
| Save | **PASS** | `evidence/experience-e2e-behaviour.json` (save step) | `experience_operator_ipc_flow_*`, prior live product-proof | Live `save_workspace_context` via CommandHandler | Tabs / virtual desktops out of scope |
| Runtime State | **PASS** | `evidence/experience-e2e-behaviour.json` (restart_hydrate) | `get_workspace_runtime_state` in E2E; runtime state tests | Restart hydrate on file-backed DB | No ambient observation at startup |
| Restore Planning | **PASS** | `evidence/experience-e2e-behaviour.json` (continue_plan) | `resolve_resume_plan` in E2E | Live plan against saved Moment | Exact-session requires same desktop session |
| Restore Execution | **PASS WITH LIMITATIONS** | `evidence/windows-product-proof.json` + E2E restore step | product-proof matrix; `execute_resume_plan` in E2E | Live place/focus on proof HWND + E2E restore | See limitations below |
| Persistence | **PASS** | `evidence/experience-e2e-behaviour.json` (session after restart) | session store + E2E restart | File-backed session survives kernel drop/re-init | Desktop projection never persisted |
| Recovery | **PASS WITH LIMITATIONS** | `evidence/process-kill-recovery.json` | `os_process_kill_during_restore_fence_*` + recovery suite | `taskkill /F` mid restore-execution fence; hydrate → Incomplete | See limitations below |
| Experience chrome (frozen) | **PASS** | `evidence/experience-e2e-behaviour.json` | Experience IPC behavioural E2E; freeze PNG + a11y Vitest | Production CommandHandler sequence matching Tauri catalog (no WebView pixel asserts — by design) | WebView chrome not driven; behavioural IPC only |
| Multi-monitor / topology change | **FAIL** | `evidence/multi-monitor-topology.json` | `multi_monitor_topology_harness_writes_evidence` | Harness ready; host has **1** monitor — dual restore + hot-plug **not** executed | Dual-monitor HW absent; set `WORKSPACE_MULTI_MONITOR_HOTPLUG_SECONDS` for optional hot-plug wait |

---

## PASS WITH LIMITATIONS — detail

### Restore Execution
- `SetForegroundWindow` may be refused without foreground input permission (refusal path retained).
- `window.z_order` unsupported by Action; place uses `SWP_NOZORDER`.
- Closed applications are never relaunched.
- Secondary-monitor placement not evidenced on this host (see Multi-monitor FAIL).

### Recovery
- OS `taskkill /F` during an uncleared restore-execution fence is evidenced (`process-kill-recovery.json`).
- Desktop effects already applied before kill are not rolled back by the OS.
- Kill during an in-flight Win32 `SetWindowPos` call itself is not separately instrumented.

---

## FAIL — blockers

### Multi-monitor / topology change
Evidence: `architecture/evidence/multi-monitor-topology.json`

| Field | Value on proof host |
| --- | --- |
| `harness_ready` | `true` |
| `hardware_dual_monitor` | `false` |
| `monitor_count` | `1` |
| `cross_monitor_restore` | `not_executed_insufficient_hardware` |
| `disconnect_reconnect` | `not_executed_no_wait_configured` |

Re-run on a dual-monitor machine:

```text
cargo test -p workspace-windows-integration multi_monitor_topology -- --nocapture
# optional hot-plug observation:
$env:WORKSPACE_MULTI_MONITOR_HOTPLUG_SECONDS=60
cargo test -p workspace-windows-integration multi_monitor_topology -- --nocapture
```

Upgrade this row only when evidence shows `hardware_dual_monitor: true` and cross-monitor restore executed.

---

## Validation record (this gate)

| Suite | Result |
| --- | --- |
| `cargo test -p workspace-windows-integration --lib` | includes product proof + multi-monitor harness |
| `cargo test -p workspace-kernel experience_operator_ipc_flow` | 1 passed (evidence written) |
| `cargo test -p workspace-kernel os_process_kill_during_restore` | 1 passed (evidence written) |
| `cargo test -p workspace-kernel workspace_recovery` | recovery suite |
| `cargo test -p workspace-kernel workspace_session_store` | persistence suite |
| `pnpm typecheck` | ok |
| `pnpm test` | freeze anchors + a11y + evidence integrity |

---

## Release recommendation

**Release runtime + frozen Experience behavioural path for single-monitor Windows** with documented Restore/Recovery OS limitations.

**Do not claim** dual-monitor placement or monitor hot-plug until `multi-monitor-topology.json` records hardware dual-monitor execution.

Frozen Experience chrome remains unchanged; operator IPC behavioural proof uses the production `CommandHandler` surface also registered by Tauri.

Engineering baseline: `architecture/32_Version_1_Baseline.md`.
