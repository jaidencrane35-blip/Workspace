# 25 — Restore Execution Model

**Implementation authority** for production restore execution.  
Experience chrome is frozen; this document describes runtime execution only.

Observation / plan authority: `architecture/24_Runtime_Observation_Model.md`.

Canonical lineage:

```text
WorkspaceObservationSnapshot / SavedContext
        ↓ resolve
RestoreCompatibilitySummary + ActionPlan   (RestorePlan)
        ↓ RestoreExecutor::execute
ActionOperationResult                      (RestoreExecution)
  + RestoreExecutionSummary
```

---

## Windows integration map (reuse only)

| Capability | Trait / API | Production impl |
| --- | --- | --- |
| HWND lookup | `WindowMutator::window_by_hwnd` | Win32 `IsWindow` + PID/title |
| Session id | `WindowMutator::current_desktop_session_id` | WTS session |
| Monitors | `WindowMutator::attached_monitor_indices` | EnumDisplayMonitors |
| Restore / place / minimise | `WindowMutator::place_window` | `ShowWindow(SW_RESTORE\|SW_MINIMIZE)` + `SetWindowPos` |
| Foreground | `WindowMutator::focus_window` | `SetForegroundWindow` (approved focus items only) |
| Process launch | `ProcessLauncher` | **Not** used by restore execute (restore-limits) |
| Enumeration | `DesktopCapturer` / `WindowEnumerator` | Used at Save/resolve match time, not for candidate search at execute |

Entry: `execute_resume_plan` → `RestoreExecutor` → `DesktopActionService::execute` → `WindowMutator`.

---

## Execution flow

1. Caller presents approved `ActionPlan` + digest proofs.
2. Digest / expiry validated; unknown plan refused wholly.
3. For each item:
   - `will_skip_*` → record skip disposition (no OS call).
   - `will_attempt` → re-check effect permission → rematch exact-session identity → apply place or focus.
4. Rematch failure → `RefusedChanged` (safe; no effect).
5. Environment refuse → `Failed` for that item; remaining eligible items still run unless cancelled / authority revoked.
6. Aggregate `RestoreExecutionSummary` + overall `OperationOutcome`. Partial success is retained.

---

## Matching strategy

Exact-session only (`match_exact_session`):

- same `desktop_session_id`
- same captured `hwnd` still present
- same `process_id`
- same title fingerprint

No fuzzy title search, no candidate list, no relaunch when missing.

---

## Execution lifecycle

| Phase | Object |
| --- | --- |
| Plan | `ActionPlan` + `RestoreCompatibilitySummary` (preview) |
| Approve | User confirms `plan_digest` |
| Execute | `RestoreExecutor::execute` |
| Result | `ActionOperationResult` with per-item outcomes + summary |

Summary fields: `restored_windows`, `skipped_windows`, `missing_applications`, `failed_operations`, `duration_ms`.

---

## Failure handling

| Condition | Disposition | Notes |
| --- | --- | --- |
| Closed window | `SkippedUnresolvable` / `ACTION_TARGET_NOT_FOUND` | Counted in `missing_applications` |
| PID/title/session change at execute | `RefusedChanged` | No effect |
| WM refuse | `Failed` / `ACTION_TARGET_REFUSED_BY_ENVIRONMENT` | Other items continue when safe |
| Unsupported type (z-order, launch) | `SkippedUnsupported` | Never forced |
| Cancel mid-flight | `NotAttempted` + outcome `Cancelled` when applicable | |

---

## Safety guarantees

- Never terminates processes.
- Never relaunches applications on the restore path.
- Place uses `SWP_NOACTIVATE | SWP_NOZORDER` — no unexpected foreground/z-order steal on place-only.
- Focus runs only for windows saved as focused, after rematch + proof.
- Insufficient confidence → skip or refuse-changed; never invent a target.

---

## Production limitations

- Non-Windows hosts use `StubWindowMutator` fixtures (not live desktop).
- Virtual desktop / browser tabs unsupported.
- Closed apps stay skipped (product restore-limits).
- `SetForegroundWindow` may still be refused by the OS; reported as environment refuse.
