# Notifications Provider Research (P13)

| Field | Value |
| --- | --- |
| **Program** | P13 Notifications Provider |
| **Decision** | **WRAP** `tauri-winrt-notification` (WinRT toast API) |
| **Port** | `NotificationPort` in `packages/windows-integration` |

---

## Options evaluated

| Option | Verdict | Notes |
| --- | --- | --- |
| WinRT Toast via `tauri-winrt-notification` | **ADOPT (WRAP)** | Stable, used by Tauri ecosystem; title/body/duration |
| `tauri-plugin-notification` | REJECT for Kernel | UI/plugin path; Conversation must not own OS effects |
| Raw Win32 balloon / tray tips | REJECT | Legacy UX; not modern Windows toast |
| Custom overlay UI in Conversation | REJECT | Notifications are a desktop capability, not chrome |

---

## Workspace ownership (unchanged)

Workspace owns identity, contracts (`notify.read` / `notify.show`), permissions, audit, truthfulness, and desktop authority.  
The commodity crate never appears in Conversation replies.

---

## Known limitations (truthful)

1. Unpackaged Tauri builds use PowerShell AppUserModelID so toasts deliver reliably; attribution may show PowerShell until packaging registers `com.workspace.app`.  
2. Programmatic dismiss of an already-shown toast is not reliably available → dismiss reports truthfully when unsupported.  
3. Actionable toast callbacks and event scheduling (“notify me when X finishes”) are **out of Level 1–2 scope**.

---

## Levels shipped

| Level | Operations |
| --- | --- |
| 1 Read | `status` — availability / permission snapshot |
| 2 Operate | `show`, `dismiss` (best-effort) |
