# Application Provider — P11

| Field | Value |
| --- | --- |
| **Name** | ApplicationProvider |
| **Domain** | `application` |
| **Purpose** | Launch, find, focus, and control desktop applications under Workspace governance |
| **Adoption** | **ADAPT** Workspace contracts + **WRAP/ADAPT** Win32 via existing ports |
| **Rule** | Providers own **operations**, not isolated features |

---

## Operations owned

| Operation | Capability | Conversation examples |
| --- | --- | --- |
| `launch` | `application.launch` | “Launch notepad” / “Start chrome” |
| `enumerate` | `application.read` | “List apps” / “What’s running?” |
| `find` | `application.read` | (internal; used by Open) |
| `focus` | `application.focus` | “Switch to Chrome” / “Focus Slack” |
| `close` | `application.close` | “Close Spotify” |
| `minimize` | `application.minimize` | “Minimize Notepad” |
| `restore` | `application.restore` | “Unminimize Notepad” / “Restore window X” |

**Open** (Intent): find → focus if running, else launch. Not a separate provider operation — Intent composes owned operations.

---

## Dependencies

- `ProcessLauncher` / `Win32ProcessLauncher`
- `WindowEnumerator` / platform enumerator
- `WindowMutator` (+ `minimize_window`, `restore_window`, `close_window`)

No AutoHotkey / PowerToys identity.

---

## Arguments / returns

| Field | Use |
| --- | --- |
| `query` | App / title fragment |
| `path` | Explicit executable |
| `hwnd` | Already-resolved window |

Returns: `{ operation, ok, status, target, message, preview, items? }`  
Statuses: `launched`, `launched_simulated`, `focused`, `closed`, `minimized`, `restored`, `enumerated`, `found`, `not_found`

---

## Failure modes

- Not found (focus/close/minimize/restore)
- Launch refused / invalid target
- Focus refused by Windows foreground rules
- Permission denied

Close uses graceful `WM_CLOSE` — never force-kill in P11.

---

## Audit

Operation + status + target title/path + item counts. No keystroke payloads.

---

## Ownership

| Layer | Location |
| --- | --- |
| Provider | `packages/kernel/src/capability_runtime/application_provider.rs` |
| Command | `ExecuteApplicationOperation` |
| IPC | `execute_application_operation` |
| Intent | `appOpen` / `appLaunch` / `appFocus` / `appClose` / `appMinimize` / `appRestore` / `appEnumerate` |

---

## Future extensions

- Ambiguous-match disambiguation satellite  
- ApprovalRequired for close of elevated / unsaved apps  
- Install detection / recent-apps ranking (Intelligence)  
