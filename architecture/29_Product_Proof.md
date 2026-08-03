# 29 — Product Proof

**Evidence only.** Collected against a real Windows desktop session.  
No demo adapter results. No stub-fixture claims. No aspirations.

Host evidence file: `architecture/evidence/windows-product-proof.json`  
Host: Windows 10/11 build **26200** (reported ProductName: Windows 10 Home 25H2).  
Desktop session id: `wts-session-1`.  
Monitors at proof time: **1** (`\\.\DISPLAY1`, 1707×1067, primary).  
Proof window HWND: recorded in evidence JSON.

Authority lineage: commit `798469c` + `architecture/28_Runtime_Recovery_Model.md` + frozen Experience contract (unchanged this sprint).

---

## Observation

| Record | Evidence |
| --- | --- |
| Implementation status | Production Win32 capture via `Win32WindowEnumerator` |
| Production evidence | Live capture on host: **9** titled windows, **2** minimized, **5** maximized-heuristic, **1** monitor; proof window appeared in enumeration after `CreateWindowExW` |
| Automated tests | `workspace-windows-integration` Win32 tests (4) + `product_proof::live_windows_product_proof_capture_and_behaviour_matrix`; kernel `live_observation_save_plan_execute_persist_hydrate` |
| Manual Windows validation | Exercised by live harness on this host (not stub) |
| Known OS limitations | Cloaked/empty-title HWNDs excluded by design; 0×0 non-minimized shell windows accepted only with minimized flag |

---

## Save

| Record | Evidence |
| --- | --- |
| Implementation status | `SavedContextService::save` / `save_with` → CaptureCoordinator → persisted `SavedContext` |
| Production evidence | Live Win32 capture saved as Moment with non-empty windows/monitors and restore identities present (`live_observation_save_plan_execute_persist_hydrate`) |
| Automated tests | Kernel live product-proof save; prior observation pipeline (fixture) remains regression-only and is **not** cited as production evidence |
| Manual Windows validation | Live path above |
| Known OS limitations | Empty stub captures refused; browser-tab / virtual-desktop contents are not in scope and were not claimed |

---

## Runtime State

| Record | Evidence |
| --- | --- |
| Implementation status | Canonical `WorkspaceRuntimeState` + process-local owner |
| Production evidence | After live observation/save/execute, state published; after owner reset, `hydrate_on_startup` restored durable fields including `last_saved_context_id` |
| Automated tests | `workspace_runtime_state_tests`, live hydrate in product-proof, recovery health tests |
| Manual Windows validation | Restart simulation via process-local reset + durable session load |
| Known OS limitations | Ambient observation refresh at startup is not performed (Product Proof Manual/Save policy) |

---

## Restore Planning

| Record | Evidence |
| --- | --- |
| Implementation status | `resolve_resume_plan` / `DesktopActionService::resolve_plan` + `RestoreCompatibilitySummary` |
| Production evidence | Live plan produced non-empty items against the just-saved live Moment (`resolve_and_execute_for_tests` on platform mutator) |
| Automated tests | Live product-proof; fixture restore-planning tests are regression-only |
| Manual Windows validation | Live path above |
| Known OS limitations | Plan TTL and exact-session matching require the same interactive desktop session |

---

## Restore Execution

| Record | Evidence |
| --- | --- |
| Implementation status | `RestoreExecutor` → Win32 place/focus; `SWP_NOZORDER` on place |
| Production evidence | Behaviour matrix (live HWND): place **committed**, minimize **committed**, restore-minimized+place **committed**, focus **committed**, primary monitor placement **committed**, identity lookup **committed**. Secondary monitor: **not_available** on this host. Z-order: contract `SWP_NOZORDER` / Action skips `window.z_order` |
| Automated tests | `product_proof` matrix; kernel live execute; Windows integration suite **21** passed |
| Manual Windows validation | Temporary top-level proof window created in-process (Notepad spawn unavailable in this agent session) |
| Known OS limitations | `SetForegroundWindow` may refuse without input attachment (committed on this host, refusal path retained); closed apps not relaunched; dual-monitor placement not exercised on this host; monitor disconnect/reconnect **not** exercised |

---

## Persistence

| Record | Evidence |
| --- | --- |
| Implementation status | `PersistentWorkspaceSession` schema v2 + atomic singleton upsert |
| Production evidence | Checkpoint after live save retained `last_saved_context_id`; restart hydrate reloaded it |
| Automated tests | DB persistent_session (5), session store (7), live product-proof checkpoint |
| Manual Windows validation | In-process durable SQLite session (kernel DB) |
| Known OS limitations | Desktop projection never persisted (derived on read) |

---

## Recovery

| Record | Evidence |
| --- | --- |
| Implementation status | Operation fences + `RuntimeHealth` + startup acknowledgment |
| Production evidence | Deterministic fence matrix on live kernel DB: Observation→`NeedsRefresh`, Save/Planning/Execution→`Incomplete`, Persistence→`Recovered`; corrupt checksum → `CorruptRecovered` without panic |
| Automated tests | `workspace_recovery_tests` (12); `live_crash_fence_matrix_is_deterministic`; `live_corrupt_session_never_crashes_startup` |
| Manual Windows validation | Fence injection + hydrate (simulates crash by leaving uncleared fence). **OS process kill during mutation was not performed** |
| Known OS limitations | Full process-termination injection under Tauri is not automated; desktop effects already applied before crash are not OS-rolled-back |

---

## Experience chrome (frozen)

| Record | Evidence |
| --- | --- |
| Implementation status | Frozen; this sprint did not modify Experience |
| Production evidence | `pnpm test` UI boundary + freeze PNG anchors + accessibility smoke **74** passed |
| Automated tests | `experience-contract`, `ui-experience-boundary`, freeze anchors |
| Manual Windows validation | No Experience redesign; no interaction change |
| Known OS limitations | Full Tauri UI click-through Home→Save→Continue on this host was **not** instrumented |

---

## Host matrix coverage

| Scenario | Result on proof host |
| --- | --- |
| Single monitor | Exercised (1 monitor) |
| Dual monitor | **Not available** on host |
| Monitor disconnect/reconnect | **Not exercised** |
| Minimized windows | Observed (2) + minimize/restore committed on proof HWND |
| Maximized windows | Observed (5 heuristic) |
| Mixed DPI / mixed geometry | **Not available** (single monitor) |
| Restart → hydrate → continue again | Exercised in kernel live product-proof |
| Crash fences | Exercised (deterministic dispositions) |
| OS process kill | **Not exercised** |
