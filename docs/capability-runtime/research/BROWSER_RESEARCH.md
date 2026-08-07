# Browser Provider Research (P14)

| Field | Value |
| --- | --- |
| **Program** | P14 Browser Provider |
| **Decision** | **WRAP** `webbrowser` for URL open; detect installed browsers via Windows paths |
| **Port** | `BrowserPort` in `packages/windows-integration` |

---

## Options evaluated

| Option | Verdict | Notes |
| --- | --- | --- |
| `webbrowser` crate | **ADOPT (WRAP)** | MIT/Apache; opens default handler; stable |
| Win32 `ShellExecuteW` direct | REJECT as primary | Prefer WRAP; same effect behind port if needed |
| CDP / `chromiumoxide` | **STUDY** (later) | Tab control / automation — not L1–2 |
| Embed WebView2 as browser | REJECT | Not “browser provider”; different product surface |

---

## Workspace ownership

Contracts (`browser.read` / `browser.open`), permissions, audit, truthfulness, desktop authority.  
Commodity never appears in Conversation.

---

## Levels shipped

| Level | Operations |
| --- | --- |
| 1 | `status` — availability / default handler / installed browsers |
| 2 | `open` — URL / site alias; `focus` — focus browser window |

Tab management, downloads, CDP automation — out of scope.
