# Provider Composition Audit — Notifications (P13)

| Field | Value |
| --- | --- |
| **Provider** | Notifications |
| **Program** | P13 Product Closure |
| **Date** | 2026-08-07 |
| **Status** | Complete |

---

## 1. Is Notifications independently useful?

**Yes.**

Evidence:
- Conversation can show a desktop toast without any other provider (`notifyShow` → Kernel Operator → NotificationProvider → WinRT).
- Conversation can query availability (`notifyStatus`) and attempt dismiss (`notifyDismiss`).
- Product Proof harness + WinRT toast observed in `winrt_show_desktop_toast_product_proof`.
- User-visible value: sparse OS attention outside Conversation chrome.

---

## 2. Which existing providers compose with Notifications?

| Partner | Composition today | Notes |
| --- | --- | --- |
| **Application Provider** | Future (Operator) | e.g. “Notify me when Notepad launches” — needs watching (not L1–2) |
| **Window Provider** | Future (Operator) | e.g. “Notify when Cursor is maximized” — needs watching |
| **Conversation** | **Available** | Natural language → Intent → Operator → Notifications |
| **Kernel Operator** | **Available** | Sole composition / orchestration authority |
| **Capability Runtime** | **Available** | Routes `notifications` domain; no provider-to-provider calls |

Providers never call each other. Composition authority remains Kernel Operator only.

---

## 3. New user-visible capabilities

| Capability | Status |
| --- | --- |
| Notifications alone — show / status / dismiss via Conversation | **Available today** |
| Notifications + Applications — “tell me when app X launches” | Future (watching / automation) |
| Notifications + Browser — “tell me when the page finishes” | Future |
| Notifications + Memory — “remind me about yesterday’s moment” | Future |
| Notifications + Automation — scheduled / event-driven toasts | Future |
| Notifications + Intelligence — ranked / contextual alerts | Future |

---

## 4. Architectural coupling?

**No coupling introduced.**

- Notifications owns only toast status/show/dismiss via `NotificationPort`.
- No calls into Application, Window, or other providers.
- Conversation uses only `execute_capability_intent`.
- Commodity crate (`tauri-winrt-notification`) stays behind the port.

---

## Gate

Provider Composition Audit complete for P13. Next provider (Browser) may begin after P13 permanent closure.
