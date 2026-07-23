# IPC Surface Inventory

| Field | Value |
|-------|-------|
| **Purpose** | Document which Tauri IPC commands the React shell uses vs which remain registered for CommandHandler parity |
| **Owner** | Lead Software Engineer |
| **Dependencies** | `app/src-tauri/src/lib.rs`, `app/src` |
| **Update Process** | Update when React invokes change or IPC is added/removed |

---

## Product-used (Canvas + Diagnostic)

| Command | Consumer |
|---------|----------|
| `get_workspace_status` | App bootstrap |
| `get_workspace_health` | App bootstrap / Diagnostic |
| `get_settings` / `update_settings` | App + Diagnostic |
| `create_workspace` / `get_workspace` | Canvas + Diagnostic |
| `create_zone` | Canvas + Diagnostic |
| `create_layout` / `get_layout` / `update_layout` | Canvas (`layoutPersistence.ts`) |
| `get_workspace_context` | App bootstrap / Diagnostic |
| `get_suggestions` / `accept_suggestion` / `reject_suggestion` | Diagnostic |
| `get_suggestion_lifecycle` | Diagnostic |
| `create_suggestion_intent_request` / `execute_intent_request` | Diagnostic |
| `get_execution_outcomes` / `get_execution_states` | Diagnostic |
| `request_execution_cancellation` | Diagnostic |
| `get_desktop_windows` | Diagnostic |
| `create_application` / `launch_application` | Diagnostic (governed launch) |

---

## Registered, unused by React (quarantined)

Kept for kernel CommandHandler parity and future UI. Not removed during foundation hardening to avoid breaking external/test callers of the Tauri surface.

- `delete_zone`, `get_zone`
- `delete_application`, `get_application`
- `create_widget`, `delete_widget`, `get_widget`
- `delete_layout`, `reset_layout`, `get_layout_snapshot`
- `get_workspace_snapshot`
- `get_actor_capabilities`, `get_audit_history`, `get_observations`, `get_workspace_metrics`
- `get_execution_state` (singular; UI uses list)

---

## Kernel lifecycle (no IPC)

- `initialize_workspace` / `initialize_workspace_in_memory` — Tauri setup
- `shutdown` — not exposed

---

## Rule

React must call IPC only via `app/src/lib/ipc.ts` (`invokeIpc`). Never import `workspace-database` from the app crate.
