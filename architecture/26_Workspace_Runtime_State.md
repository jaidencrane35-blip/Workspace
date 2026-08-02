# 26 — Workspace Runtime State

**Implementation authority** for the canonical live desktop runtime model.  
Experience chrome is frozen — this document describes kernel ownership only.

Related: `24_Runtime_Observation_Model.md`, `25_Restore_Execution_Model.md`.

---

## Ownership

| Concern | Owner |
| --- | --- |
| Live desktop bundle | `WorkspaceRuntimeStateService` (process-local) |
| Desktop projection rows | Domain `WorkspaceState` (embedded in runtime state) |
| Capture single-flight | `CaptureCoordinator` (publishes cache phases) |
| Exact-session mutate | `RestoreExecutor` / `WindowMutator` (publishes execution phases) |
| Cognition overlay | `WorkspaceRuntimeContext` — **does not** own desktop capture |

There is exactly one authoritative live desktop representation: **`WorkspaceRuntimeState`**.

Do not introduce parallel `LiveDesktopState` types. Projection `WorkspaceSnapshot` (zones/apps) remains a separate read model.

---

## Canonical lifecycle

```text
DesktopObservationCapture
        ↓ CaptureCoordinator (cache phase)
WorkspaceObservationSnapshot
        ↓ WorkspaceStateEngine::build (cached by pass id)
WorkspaceState  ⊂  WorkspaceRuntimeState
        ↓ resolve_resume_plan (execution phase: planning)
ActionPlan + RestoreCompatibilitySummary
        ↓ RestoreExecutor (execution phase: validating → executing)
ActionOperationResult + RestoreExecutionSummary
        ↓ published into WorkspaceRuntimeState.restore_history
```

---

## `WorkspaceRuntimeState` fields

| Field | Source |
| --- | --- |
| `desktop` | Cached `WorkspaceState` (windows, processes, focus) |
| `observation` | `WorkspaceObservationStatus` (freshness, age, failure) |
| `cache_phase` (`ObservationCachePhase`) | Idle / RefreshRequested / RefreshInProgress / RefreshCompleted / RefreshFailed |
| `execution_phase` (`RestoreExecutionPhase`) | Idle / Planning / Validating / Executing / Completed / Partial / Failed |
| `active_workspace_id` | Settings |
| `active_monitor_index` | Focused window monitor (else first known) |
| `capture_timestamp` | Latest observation `captured_at` |
| `confidence_band` | Last `RestoreCompatibilitySummary` |
| `restore_history` | Bounded ring of `RestoreHistoryEntry` (newest first) |
| `generation` | Bumped on invalidate / refresh complete|fail |

---

## Cache rules

1. Desktop projection is keyed by observation `pass_id`.
2. Cache hit when pass unchanged — no DB rebuild.
3. Successful capture → `RefreshCompleted`, invalidate desktop cache, bump generation.
4. Failed capture → `RefreshFailed`, invalidate desktop cache.
5. Concurrent capture → `RejectedConcurrent`; cache phase stays `RefreshInProgress` for the winning flight.
6. Explicit `invalidate()` forces rebuild on next read.

Consumers must call `WorkspaceStateEngine::get_current` or `WorkspaceRuntimeStateService::current` — not `ObservationPassRepository::load_latest_snapshot` for interpreted desktop state.

---

## State transitions

### Observation cache

`Idle` → `RefreshRequested` → `RefreshInProgress` → (`RefreshCompleted` | `RefreshFailed`) → (next request or Idle on read alignment)

### Restore execution

`Idle` → `Planning` (resolve) → `Idle` (preview ready)  
`Idle` → `Validating` → `Executing` → (`Completed` | `Partial` | `Failed`)

---

## Publication flow

| Event | Publisher |
| --- | --- |
| Capture requested/started/completed/failed | `CaptureCoordinator` → `WorkspaceRuntimeStateService` |
| Plan resolved | `ResolveResumePlan` → compatibility + phase |
| Plan executed | `RestoreExecutor` → phase + history entry |

---

## Consumers

| Consumer | Reads |
| --- | --- |
| Environment / Intelligence / Attention / Runtime cognition | `WorkspaceStateEngine::get_current` (cached desktop) |
| Operator / diagnostics | `get_workspace_runtime_state` IPC |
| Experience Continue | Existing resolve/execute IPC only (frozen UI; no new Experience commands) |

---

## Invalidation

- New observation pass lands
- Capture fails
- Explicit `WorkspaceRuntimeStateService::invalidate()`

Restore execution does **not** invalidate observation cache (exact-session mutate is separate from observation buffer).

---

## Migrations (duplicated query → owner)

| Former path | Migration |
| --- | --- |
| Direct `ObservationPassRepository::load_latest_snapshot` in `WorkspaceStateEngine::get_current` | Routed through `WorkspaceRuntimeStateService` cache |
| Independent Environment/Intelligence reloads | Unchanged call sites; now share cache via engine |
| Capture lifecycle only in coordinator atomics | Also published as `ObservationCachePhase` |
| Execute outcomes only returned to caller | Also published to `restore_history` + `execution_phase` |

Kept separate (not superseded): `LiveWindowView` for Action rematch; SavedContext durable Moments; projection `WorkspaceSnapshot`.
