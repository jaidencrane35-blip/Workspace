# 23 — Production Integration Status

**Authority for the production integration phase.**  
Experience chrome is frozen (`architecture/research/experience/experience-freeze.md`, commit `e208283` lineage).  
UI must not change to accommodate runtime gaps — runtime adapts to the frozen contract.

Demo mode is a **permanent adapter** (`app/src/demo/`) for development, screenshots, visual regression, and onboarding. It is not the production runtime.

- Observation model: `architecture/24_Runtime_Observation_Model.md`
- Restore execution: `architecture/25_Restore_Execution_Model.md`
- Live runtime state: `architecture/26_Workspace_Runtime_State.md`
- Session persistence: `architecture/27_Workspace_Session_Persistence.md`

---

## Migration table — Demo API → Production API

| Demo / catalog command | Production Tauri command | Kernel / package | Adapter (`demoIpc`) | Notes |
| --- | --- | --- | --- | --- |
| `get_workspace_status` | `get_workspace_status` | status / kernel | yes | Bootstrap |
| `get_workspace_health` | `get_workspace_health` | health | yes | Bootstrap |
| `get_settings` | `get_settings` | settings | yes | Active workspace id |
| `update_settings` | `update_settings` | settings | yes | Persist active workspace |
| `get_workspace` | `get_workspace` | workspace | yes | |
| `create_workspace` | `create_workspace` | workspace | yes | Empty → invite Home |
| `list_saved_contexts` | `list_saved_contexts` | resume / kernel | yes | Home + Continue |
| `get_saved_context` | `get_saved_context` | resume / kernel | yes | Inspect |
| `get_saved_context_capture_scope` | `get_saved_context_capture_scope` | saved_context | yes | Save consent scope |
| `save_workspace_context` | `save_workspace_context` | saved_context + observation | yes | Live Win32 capture; empty stub refused |
| `resolve_resume_plan` | `resolve_resume_plan` | resume / Action | yes | Plan + `RestoreCompatibilitySummary` |
| `execute_resume_plan` | `execute_resume_plan` | `RestoreExecutor` / Action | yes | Live place/focus + `RestoreExecutionSummary` |
| `delete_saved_context` | `delete_saved_context` | resume / kernel | yes | Inspect delete |
| `get_pilot_measurement_scope` | `get_pilot_measurement_scope` | pilot_measurement | yes | Check-in |
| `get_pilot_measurement_snapshot` | `get_pilot_measurement_snapshot` | pilot_measurement | yes | Check-in |
| `grant_pilot_consent` | `grant_pilot_consent` | pilot_measurement | yes | |
| `withdraw_pilot_consent` | `withdraw_pilot_consent` | pilot_measurement | yes | |
| `record_pilot_baseline` | `record_pilot_baseline` | pilot_measurement | yes | |
| `record_pilot_leave_resume` | `record_pilot_leave_resume` | pilot_measurement | yes | |
| `record_pilot_interview` | `record_pilot_interview` | pilot_measurement | yes | |
| _(Guide)_ | — | — | n/a | Static copy; no IPC |

Catalog source: `app/src/demo/experienceIpcCatalog.ts`.  
Routing: `app/src/lib/ipc.ts` → Tauri `invoke` when not in adapter mode; else `demoInvoke`.

---

## Integration matrix by screen

| Screen | UI complete | Production complete | Demo complete | Tests | Remaining blockers |
| --- | --- | --- | --- | --- | --- |
| **Bootstrap** | yes | **implemented** | yes | parity | None for command surface |
| **Home** | yes (frozen) | **implemented** | yes | parity | Empty until Moments saved |
| **Save** | yes (frozen) | **implemented** (Windows) | yes | observation pipeline | Non-Windows unavailable; tabs/VD out of scope |
| **Continue** | yes (frozen) | **implemented** (plan + execute) | yes | observation + restore execution | Closed apps not relaunched; same desktop session required |
| **Check-in** | yes (frozen) | **implemented** (self-report) | yes | parity | No ambient age/focus sensors |
| **Guide** | yes (frozen) | **n/a** | n/a | freeze PNG anchors | No backend |

---

## Restore execution status (this sprint)

| Step | Status |
| --- | --- |
| `RestoreExecutor` entry on `execute_resume_plan` | implemented |
| Exact-session rematch at execute | implemented |
| Place (incl. unminimise) + focus via Win32 mutator | implemented |
| Place without z-order / activate steal (`SWP_NOZORDER`) | implemented |
| Partial success retained + `RestoreExecutionSummary` | implemented |
| Demo execute summary parity | implemented |
| Fixture integration tests | implemented |

---

## Adapter policy

1. Prefer production whenever Tauri internals are available (`shouldUseExperienceDemo` false).
2. Force adapter with `VITE_FORCE_EXPERIENCE_DEMO=1` for screenshots / visual regression.
3. Disable adapter with `VITE_DISABLE_EXPERIENCE_DEMO=1` even in DEV (fail closed if IPC missing).
4. Frozen panels must not branch on demo vs production for layout, motion, or controls.
5. Once production capture/execute succeeds, never substitute demo Moments or fake OS effects into the production path.

---

## Regression / validation

| Check | Mechanism |
| --- | --- |
| Catalog ↔ Tauri | `production-integration-parity.test.ts` |
| Capture → plan | `observation_pipeline_tests.rs` |
| Plan → execute | `restore_execution_pipeline_tests.rs` + resume acceptance |
| Demo ↔ production shapes | Vitest observation + parity (summary fields) |
| Visual freeze | Pass-07 PNGs present and sized |
| Accessibility | Existing smoke + frozen chrome tests |

---

## Live runtime state status

| Step | Status |
| --- | --- |
| `WorkspaceRuntimeState` domain bundle | implemented |
| Process-local owner + desktop cache by pass id | implemented |
| Observation cache phases published from CaptureCoordinator | implemented |
| Execution phases + restore history from RestoreExecutor | implemented |
| `get_workspace_runtime_state` IPC (operator; not Experience) | implemented |
| `PersistentWorkspaceSession` + `WorkspaceSessionStore` | implemented |
| Startup hydration + success-path checkpoints | implemented |

---

## Remaining production blockers

1. Full Windows E2E under Tauri on a real multi-monitor session (Save → Continue approve → live place/focus).
2. Virtual desktop / browser-tab fidelity (out of scope).
3. Check-in ambient metrics — only when real kernel sources exist (can later read `WorkspaceRuntimeState`).
4. OS may refuse `SetForegroundWindow`; reported per-item, not silently forced.
