# Browser Provider
## Product Implementation Program P14 — Levels 1–2

| Field | Value |
| --- | --- |
| **Status** | **Permanently closed** — Product Complete (P14.5) |
| **Domain** | `browser` |
| **Adoption** | WRAP `webbrowser` |
| **Research** | `research/BROWSER_RESEARCH.md` |
| **Conversation IPC** | `execute_capability_intent` |
| **Permissions** | `browser.read`, `browser.open` |

---

## Operations

| Level | Operation | Effect |
| --- | --- | --- |
| 1 | `status` | Availability, default handler, installed browsers |
| 2 | `open` | Open URL / site alias in default browser |
| 2 | `focus` | Compose to Window focus (providers stay independent) |
| 2 | `open_beside` | Operator composition: open URL + Window snap |

---

## Ownership

| Layer | Location |
| --- | --- |
| Port | `packages/windows-integration/src/browser.rs` |
| Provider | `packages/kernel/src/capability_runtime/browser_provider.rs` |
| Commands | `BrowserStatus` / `OpenBrowserUrl` |
| Intent | `browserStatus` / `browserOpen` / `browserOpenBeside` |

---

## Out of scope

Tab management · CDP automation · downloads · web intelligence · memory restore of browser sessions
