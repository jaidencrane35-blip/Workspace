# 28 — Runtime Recovery Model

**Implementation authority** for crash resilience, transactional checkpoints, and internal runtime health.  
Experience chrome is frozen — recovery must not alter UI, composition, or interaction.

Related: `26_Workspace_Runtime_State.md`, `27_Workspace_Session_Persistence.md`.

---

## Recovery lifecycle

```text
begin_operation(fence)     ← durable marker before mutation
        ↓
mutate (observe / save / plan / execute)
        ↓
success → checkpoint (clears fence, atomic replace)
failure → clear_operation_fence (intentional; not a crash)
crash   → fence remains
        ↓
startup hydrate_on_startup
  load_outcome → acknowledge fence → last_recovery
  seed RuntimeHealth → publish WorkspaceRuntimeState
```

Interrupted operations are never silently discarded: fence → `RecoveryRecord` → `last_recovery` + process `RuntimeHealth.pending_recovery`.

---

## Recovery matrix

| Mutation | Atomic boundary | Rollback on failure | Crash failure mode | Recovery path |
| --- | --- | --- | --- | --- |
| Observation | Capture flight + session txn after complete | Prior session retained; cache → Failed | Uncleared `Observation` fence | `NeedsRefresh`; next Manual/Save refreshes |
| Session persistence | SQLite `BEGIN IMMEDIATE` upsert | Prior row retained | Uncleared `Persistence` fence | `Recovered`; prior session kept |
| Restore planning | Resolve then checkpoint | Phase → Failed; fence cleared | Uncleared `RestorePlanning` fence | `Incomplete` |
| Restore execution | Execute then checkpoint | Phase → Failed; fence cleared | Uncleared `RestoreExecution` fence | `Incomplete`; history may lack final entry |
| Startup hydration | Best-effort acknowledge write | Empty session on corrupt/migrate fail | N/A (startup) | Never panics; integrity flag set |
| IPC handlers | Command pipeline Result | No partial Experience publish | Same as underlying mutation | Existing error surfaces only |

---

## Checkpoint strategy

1. Never checkpoint while `RefreshRequested` / `RefreshInProgress` / `Planning` / `Validating` / `Executing`.
2. Success checkpoint clears `pending_operation`.
3. Preserve `last_saved_context_id` and `last_recovery` across checkpoints.
4. SQLite singleton upsert is the sole durable commit; no partial JSON rows.
5. If mutation fails (non-crash), clear fence without rewriting durable fields to empty.

---

## Health model (`RuntimeHealth`)

Process-local, published only via `WorkspaceRuntimeState` / `get_workspace_runtime_state` (operator). **Not** an Experience surface.

| Field | Meaning |
| --- | --- |
| `last_successful_observation_at` | Last completed capture |
| `last_persistence_at` | Last successful session save |
| `last_restore_at` | Newest restore history entry |
| `session_integrity` | `ok` / missing / corrupt / migration / incompatible recovered |
| `pending_recovery` | Acknowledged interruption record |
| `degraded` | Integrity degraded or Incomplete / NeedsRefresh |
| `recovery_disposition` | `none` / `recovered` / `incomplete` / `needs_refresh` |

Distinct from cognition `WorkspaceRuntimeHealth` and kernel lifecycle `WorkspaceHealth`.

---

## Interruption dispositions

| Fence kind | Disposition | Notes |
| --- | --- | --- |
| Observation | NeedsRefresh | Live desktop may be stale |
| Save | Incomplete | Moment create may be missing |
| RestorePlanning | Incomplete | Plan preview not durable |
| RestoreExecution | Incomplete | Desktop effects may be partial |
| Persistence | Recovered | Prior valid session retained by txn |

---

## Consistency guarantees

After successful completion, these must agree:

- `WorkspaceRuntimeState.observation.pass_id` ↔ `PersistentWorkspaceSession.last_observation_pass_id`
- `confidence_band` ↔ `last_confidence_band`
- bounded `restore_history` ↔ session history
- `ActionOperationResult` summary ↔ newest `RestoreHistoryEntry` (when execution finished)
- success checkpoint ⇒ `pending_operation == None`

`assert_runtime_session_consistent` enforces the session/runtime gate before durable write (warn on mismatch; tests assert strictly).

---

## Recovery guarantees

1. Startup never crashes because a session cannot be restored.
2. Corrupt / incompatible / failed migration → empty session + integrity flag; SavedContexts untouched.
3. Interrupted fences become `last_recovery` (durable) and health `pending_recovery` (process).
4. Failed mutations retain the previous valid `PersistentWorkspaceSession`.
5. No new Experience IPC, chrome, or interaction.

---

## Schema

`PersistentWorkspaceSession` schema **v2**:

- `pending_operation` (fence)
- `last_recovery` (acknowledged interruption)

Migration: `046_workspace_session_recovery_fence.sql` + explicit `migrate` v1→v2.

---

## Known limitations

1. Nested Save→Observation: Observation fence may briefly overwrite Save fence during capture; Save re-fences before durable create.
2. Ambient observation refresh after hydrate remains deferred (Product Proof).
3. Desktop effects already applied before a crash during restore cannot be rolled back by the OS — marked Incomplete for operator awareness only.
4. OS process-kill during mutation is not automated; durable fence + hydrate cover the recovery contract (see `29_Product_Proof.md`).
