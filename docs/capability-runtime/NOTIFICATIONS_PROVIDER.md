# Notifications Provider
## Product Implementation Program P13 — Levels 1–2

| Field | Value |
| --- | --- |
| **Status** | **Permanently closed** — Product Complete |
| **Domain** | `notifications` |
| **Adoption** | WRAP WinRT toast (`tauri-winrt-notification`) |
| **Research** | `research/NOTIFICATIONS_RESEARCH.md` |
| **Conversation IPC** | `execute_capability_intent` |
| **Permissions** | `notify.read`, `notify.show` |

---

## Operations

| Level | Operation | Effect |
| --- | --- | --- |
| 1 | `status` | Availability + permission snapshot |
| 2 | `show` | Desktop toast (`title`, `text`/body, optional category/priority/duration) |
| 2 | `dismiss` | Best-effort dismiss by id (may be unsupported on unpackaged WinRT) |

---

## Ownership

| Layer | Location |
| --- | --- |
| Port | `packages/windows-integration/src/notification.rs` |
| Provider | `packages/kernel/src/capability_runtime/notification_provider.rs` |
| Commands | `NotificationStatus` / `ShowNotification` / `DismissNotification` |
| Conversation IPC | `execute_capability_intent` |
| Intent | `notifyStatus` / `notifyShow` / `notifyDismiss` |

---

## Out of scope (later)

- Scheduled / event-driven “notify me when…” watching  
- Rich actionable toast → Conversation deep links  
- Packaged AUMID identity polish  
