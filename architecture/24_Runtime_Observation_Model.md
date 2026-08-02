# 24 — Runtime Observation Model

**Implementation authority** for the Windows observation → Save → Continue → Check-in pipeline.  
Experience chrome is frozen; this document describes runtime objects and IPC only.

Canonical lineage (reuse existing types — do not invent parallel snapshots):

```text
DesktopObservationCapture          (windows-integration)
        ↓ WorkspaceObservationService::capture
WorkspaceObservationSnapshot       (ephemeral observation buffer)
        ↓ SavedContextService::assemble
SavedContext / WorkspaceMoment     (user-owned durable Moment)
        ↓ action_request_from_saved_context
ActionRequest → ActionPlan         (RestorePlan)
        ↓ RestoreExecutor::execute
ActionOperationResult              (RestoreExecution)
  (+ RestoreExecutionSummary — see architecture/25_Restore_Execution_Model.md)
```

Naming note: domain `WorkspaceSnapshot` (projection zones/apps) is **not** desktop capture. Do not conflate.

---

## Capability inventory → Tauri / kernel

| Capability | Status | Production path |
| --- | --- | --- |
| Foreground window | yes (Win32) | `capture_workspace_observation` / internal Save capture |
| Open windows | yes | same |
| Process id | yes | `ObservedWindow.process_id` → `SavedContextWindow.process_id` |
| Process image basename | yes (Win32 observation only) | `CapturedDesktopWindow.process_name` → observation buffer; **excluded** from SavedContext scope |
| Executable full path | no (scope exclusion) | — |
| Window title | yes | SavedContext + restore identity fingerprint |
| Monitor layout | yes | `SavedContextMonitor` |
| Bounds / minimized / focused | yes | `SavedContextWindow` |
| Z-order (capture) | yes | stored; restore of z-order is skip-unsupported |
| Virtual desktop | no | — |
| Browser tabs | no | — |
| Desktop session id | yes (WTS / stub) | restore identity `desktop_session_id` |
| Timestamps | yes | `captured_at` / `created_at` |
| Confidence / eligibility | yes | `RestoreCompatibilitySummary` on `ResumePlanPreview` |

### Experience IPC (frozen screens)

| Command | Role |
| --- | --- |
| `get_saved_context_capture_scope` | Consent copy (static scope id) |
| `save_workspace_context` | Capture once → assemble → persist SavedContext |
| `list_saved_contexts` / `get_saved_context` | Home / Inspect / Continue load |
| `resolve_resume_plan` | Load SavedContext → resolve ActionPlan + compatibility |
| `execute_resume_plan` | Approved digest → mutate desktop |
| Pilot measurement commands | Check-in self-report (not ambient sensors) |

Observation commands (`capture_workspace_observation`, `get_latest_workspace_observation`, …) exist for operator / freshness; Save does not require a separate Experience call — capture is inside `save_workspace_context`.

---

## Persistence flow (Save)

1. Validate name, handoff, scope consent (`SAVED_CONTEXT_SCOPE_ID` / v2).
2. Refuse unknown workspace without observing.
3. `CaptureCoordinator` → `platform_desktop_capturer()`:
   - Windows: `Win32WindowEnumerator` (`source=win32`)
   - elsewhere: empty stub (`source=stub`) → **refused** as `DesktopObservationUnavailable` when windows and monitors are both empty
4. Map capture → `WorkspaceObservationSnapshot` (includes optional `process_name` in the rolling buffer).
5. Assemble durable `SavedContext` (titles, PIDs, geometry, monitors, restore identities). Program names/paths are never copied into SavedContext.
6. Persist via `SavedContextRepository`. No demo fallback when Tauri production IPC is active.

---

## Restore flow (Continue)

1. Load `SavedContext` by id.
2. Companion builds `ActionRequest` (`action_request_from_saved_context`) — Action never receives the saved-context id.
3. `DesktopActionService::resolve_plan` against live `WindowMutator` (Win32 or stub fixture).
4. Exact-session match → `will_attempt`; closed / confidence / session mismatch → `will_skip_unresolvable` (codes such as `ACTION_TARGET_NOT_FOUND`); unsupported effects (e.g. z-order) → `will_skip_unsupported`.
5. `RestoreCompatibilitySummary::from_plan` attaches confidence band (`high` / `steady` / `limited` / `empty`) and `missing_window_count`. Frozen UI may still derive quality from disposition ratios — bands match.
6. Execute only after digest approval via `RestoreExecutor` (see doc 25); no `application.launch` for closed apps (restore-limits).

---

## Check-in

Pilot measurement persists consented return-time / interview fields only. Ambient workspace age, uninterrupted focus duration, and restore history are **not** in the pilot snapshot today; Check-in keeps current behaviour until those exist as real kernel data.

---

## Tests

- Kernel: `packages/kernel/src/commands/observation_pipeline_tests.rs` (capture → persist → reload → plan → missing window → empty stub refuse).
- Resume acceptance: `resume_acceptance_tests.rs`.
- Vitest: catalog / demo parity + observation pipeline shape regression.
