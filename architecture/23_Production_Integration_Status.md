# 23 — Production Integration Status

**Authority for the production integration phase.**  
Experience chrome is frozen (`architecture/research/experience/experience-freeze.md`, commit `e208283` lineage).  
UI must not change to accommodate runtime gaps — runtime adapts to the frozen contract.

Demo mode is a **permanent adapter** (`app/src/demo/`) for development, screenshots, visual regression, and onboarding. It is not the production runtime.

Observation model authority: `architecture/24_Runtime_Observation_Model.md`.

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
| `execute_resume_plan` | `execute_resume_plan` | resume / Action | yes | Approve restore |
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
| **Bootstrap** (shell load) | yes | **implemented** | yes | `experience-demo`, `production-integration-parity` | None for command surface |
| **Home** | yes (frozen) | **implemented** | yes | parity lists contexts | Empty until Moments saved (by design) |
| **Save** | yes (frozen) | **implemented** (Windows) | yes | observation pipeline + consent | Non-Windows: `desktop_observation_unavailable`. Virtual desktop / browser tabs / exe path out of scope |
| **Continue** | yes (frozen) | **implemented** (exact-session) | yes | pipeline + resume acceptance | Closed apps not relaunched (restore-limits). Live match needs same desktop session |
| **Check-in** | yes (frozen) | **implemented** (pilot self-report) | yes | parity snapshot/record | No ambient age/focus/restore-history sensors yet — keep current UI behaviour |
| **Guide** | yes (frozen) | **n/a** | n/a | freeze screenshot anchors | No backend |

---

## Observation pipeline status (this sprint)

| Step | Status |
| --- | --- |
| Win32 capture (windows, monitors, bounds, z-order, session, process basename) | implemented |
| Persist SavedContext via production IPC | implemented |
| Refuse empty non-Windows stub as production success | implemented |
| Continue resolve + compatibility summary | implemented |
| Fixture integration tests (capture→persist→reload→plan→missing) | implemented |
| Demo ↔ real equivalent visual contract | frozen PNGs unchanged; demo adapter still shapes screenshots |

---

## Adapter policy

1. Prefer production whenever Tauri internals are available (`shouldUseExperienceDemo` false).
2. Force adapter with `VITE_FORCE_EXPERIENCE_DEMO=1` for screenshots / visual regression.
3. Disable adapter with `VITE_DISABLE_EXPERIENCE_DEMO=1` even in DEV (fail closed if IPC missing).
4. Frozen panels must not branch on demo vs production for layout, motion, or controls. Transport-leak suppression in `App.tsx` remains the only bootstrap exception.
5. Once production capture succeeds, never substitute demo Moments into the production path.

---

## Regression / validation

| Check | Mechanism |
| --- | --- |
| Demo ↔ catalog exhaustiveness | `demoInvoke` + `ExperienceIpcCommand` switch |
| Catalog ↔ Tauri registration | `production-integration-parity.test.ts` scans `lib.rs` |
| Capture → persist → plan | `observation_pipeline_tests.rs` |
| Behavioural shapes | Demo adapter Vitest + `observation-pipeline.test.ts` |
| Visual freeze | Pass-07 PNGs must remain present and sized |
| Accessibility | Existing smoke + frozen chrome tests |

---

## Remaining production blockers

1. Full Windows E2E under Tauri (`pnpm dev`) on a real multi-monitor desktop session.
2. Virtual desktop / browser-tab fidelity (explicitly out of current capture scope).
3. Check-in ambient metrics (workspace age, focus duration, restore history) — only if product later adds kernel sources; do not fake them in UI.
4. Confirm Check-in SQLite round-trip on Windows CI with Tauri packaging.
