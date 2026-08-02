# 27 — Workspace Session Persistence

**Implementation authority** for durable workspace session lifecycle.  
Experience chrome is frozen — persistence must not alter UI, composition, or interaction.

Live runtime: `architecture/26_Workspace_Runtime_State.md`.

---

## Field classification

| Field (`WorkspaceRuntimeState`) | Class | Notes |
| --- | --- | --- |
| `active_workspace_id` | **Persistent** | Also mirrored in settings; session keeps last known |
| `capture_timestamp` | **Persistent** | Last observation time |
| `confidence_band` | **Persistent** | Last restore compatibility band |
| `restore_history` | **Persistent** | Bounded ring (≤20) |
| last observation pass id | **Persistent** | Via `observation.pass_id` → session |
| last saved context id | **Persistent** | Checkpoint after Save |
| `last_active_monitor_index` | **Persistent** | Last known focused monitor |
| `desktop` | **Derived** | Rebuilt from observation buffer |
| `observation` (status/freshness) | **Derived** | Recomputed from DB + clocks |
| `active_monitor_index` | **Derived** | From current desktop focus |
| `cache_phase` | **Ephemeral** | Never serialized |
| `execution_phase` | **Ephemeral** | Never serialized |
| `generation` | **Ephemeral** | Process-local |
| `authority_effect` | **Ephemeral** | Constant / non-durable |

Durable type: **`PersistentWorkspaceSession`** (`packages/domain/src/persistent_workspace_session.rs`).  
Do not persist full `WorkspaceRuntimeState`.

---

## Storage format

Table `workspace_persistent_session` (migration `045_workspace_persistent_session.sql`):

- Singleton row `id = 'singleton'`
- Scalar columns for persistent fields
- `restore_history_json` TEXT
- `payload_checksum` FNV-1a over durable material
- `schema_version` INTEGER (current: **1**)

Atomic write: `BEGIN IMMEDIATE` transaction + `INSERT … ON CONFLICT DO UPDATE`.

---

## WorkspaceSessionStore

| API | Behaviour |
| --- | --- |
| `load_recovered` | Load + migrate; corrupt/missing/incompatible → empty (never crash) |
| `save` | Atomic replace; refuses non-current schema |
| `checkpoint_runtime` / `checkpoint_current` | Success-path only; skips in-flight cache/execution phases |
| `hydrate_on_startup` | Load → seed owner → rebuild derived desktop |

---

## Checkpoints (automatic)

| Event | When |
| --- | --- |
| Successful observation | After `CaptureCoordinator` completed |
| Successful Save Moment | After `SavedContextRepository::create` |
| Restore planning | After `resolve_resume_plan` (Idle + compatibility) |
| Restore execution / completion | After `execute_resume_plan` returns |

Never checkpoint while `RefreshInProgress` / `Executing` (partial-write guard).

---

## Startup lifecycle

```text
InitializeWorkspace (DB + migrations)
        ↓
hydrate_on_startup
  load_recovered → migrate → seed RuntimeOwner
  current() rebuilds derived WorkspaceState
        ↓
Ready (Experience unchanged)
```

Ambient observation refresh is **not** fired at startup (Product Proof). Persisted pass may be stale; next Manual / Save / `ensure_observation_freshness` refreshes. If live observation differs from persisted pass id, live projection wins without UI disruption.

---

## Recovery

| Condition | Behaviour |
| --- | --- |
| Missing session | Empty session; continue |
| Corrupt JSON / checksum | Log + empty session; SavedContexts untouched |
| Incompatible future schema | Log + empty session |
| Failed migration | Log + empty session |
| Partial write | Transaction rollback; prior row retained |

---

## Migrations

Explicit only: `PersistentWorkspaceSession::migrate`.

```text
v0 → v1   (identity / bound history)
v1        (current)
```

No implicit upgrades. Every step unit-tested.

---

## Consumers

| Consumer | Role |
| --- | --- |
| Kernel startup | Hydrate |
| Capture / Save / Resume | Checkpoint |
| Experience | Unchanged — uses existing IPC; session is invisible chrome |
