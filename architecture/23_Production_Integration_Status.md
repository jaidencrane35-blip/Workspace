# 23 — Production Integration Status

**Authority for the production integration phase.**  
Experience chrome is frozen (`architecture/research/experience/experience-freeze.md`, commit `e208283` lineage).  
UI must not change to accommodate runtime gaps — runtime adapts to the frozen contract.

Demo mode is a **permanent adapter** (`app/src/demo/`) for development, screenshots, visual regression, and onboarding. It is not the production runtime.

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
| `save_workspace_context` | `save_workspace_context` | saved_context | yes | Requires observation / capture |
| `resolve_resume_plan` | `resolve_resume_plan` | resume / kernel | yes | Continue preview |
| `execute_resume_plan` | `execute_resume_plan` | resume / kernel | yes | Approve restore |
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
| **Bootstrap** (shell load) | yes | **implemented** | yes | `experience-demo`, `production-integration-parity` | None for command surface. First-run empty workspace is correct product behaviour. |
| **Home** | yes (frozen) | **partial** | yes (populated fixtures) | parity lists contexts | Production starts empty until Moments are saved; no fixture seed in prod (by design). Windows observation quality affects later Saves. |
| **Save** | yes (frozen) | **partial** | yes | consent tests + parity save path | Real capture needs desktop observation (`save_workspace_context` / windows integration). Stubbed / limited off Windows. Scope mismatch refuses save (correct). |
| **Continue** | yes (frozen) | **partial** | yes | parity resolve/execute | Restore execution depends on live window matching; closed apps not relaunched (restore-limits). |
| **Check-in** | yes (frozen) | **implemented** | yes | parity snapshot/record | SQLite persistence via kernel; verify on Windows CI with Tauri. |
| **Guide** | yes (frozen) | **n/a** | n/a | freeze screenshot anchors | No backend; keep static. |

Status legend:

- **implemented** — Tauri command registered + kernel path exists; UI already calls it via `invokeIpc`.
- **partial** — Command exists but real-world fidelity depends on OS observation / non-empty data.
- **missing** — No production command (none for pilot chrome).
- **blocked** — See blockers column.

---

## Adapter policy

1. Prefer production whenever Tauri internals are available (`shouldUseExperienceDemo` false).
2. Force adapter with `VITE_FORCE_EXPERIENCE_DEMO=1` for screenshots / visual regression.
3. Disable adapter with `VITE_DISABLE_EXPERIENCE_DEMO=1` even in DEV (fail closed if IPC missing).
4. Frozen panels must not branch on demo vs production for layout, motion, or controls. Transport-leak suppression in `App.tsx` remains the only bootstrap exception.

---

## Workflow replacement order

| Order | Workflow | Integration note |
| --- | --- | --- |
| 1 | Home | Uses `list_saved_contexts` → production when Tauri present |
| 2 | Save | Uses capture scope + `save_workspace_context` |
| 3 | Continue | Uses resolve / execute / delete |
| 4 | Check-in | Uses pilot measurement commands |
| 5 | Guide | No change |

This sprint formalizes the adapter + catalog + parity tests. Command handlers were already registered; kernel currently compiles (`cargo check -p workspace-kernel`).

---

## Regression / validation

| Check | Mechanism |
| --- | --- |
| Demo ↔ catalog exhaustiveness | `demoInvoke` + `ExperienceIpcCommand` switch |
| Catalog ↔ Tauri registration | `production-integration-parity.test.ts` scans `lib.rs` |
| Behavioural shapes | Demo adapter Vitest journeys (Home/Save/Continue/Check-in) |
| Visual freeze | Pass-07 PNGs must remain present and sized; do not edit frozen UI to fix data |
| Accessibility | Existing smoke + frozen chrome tests |

---

## Immediate next production work (not UX)

1. Windows end-to-end: `pnpm dev` / Tauri — create workspace → Save Moment → Home → Continue → Check-in.
2. Harden observation capture for Save (real window list fidelity).
3. Confirm Check-in SQLite round-trip under Windows CI.
4. Optional: anonymized local seed only if product requires non-empty first run (must not alter frozen empty-state UI).
