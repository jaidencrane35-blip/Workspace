# Provider Composition Audit — Browser (P14)

| Field | Value |
| --- | --- |
| **Provider** | Browser |
| **Program** | P14 |
| **Status** | Complete |

---

## 1. Independently useful?

**Yes.** Conversation can open URLs and report browser availability without other providers.

---

## 2. Composes with

| Partner | Today | Notes |
| --- | --- | --- |
| **Applications** | Adjacent | “Open Notepad” stays Application; sites use Browser |
| **Windows** | **Available** | `open_beside` / `focus` compose via Kernel Operator → Window |
| **Notifications** | Future | “Tell me when the browser task finishes” needs watching |
| **Conversation / Operator / Runtime** | **Available** | Sole governed path |

---

## 3. New user-visible capabilities

| Capability | Status |
| --- | --- |
| Open site / URL via Conversation | **Available** |
| Browser + Window — “Open ChatGPT beside Cursor.” | **Available** (Operator composition) |
| Browser + Notifications — finish watching | Future |
| Browser + Applications — layout recipes | Partial via Window |
| Browser + Memory — session restore | Future |

---

## 4. Coupling?

**No.** BrowserPort only opens URLs / reports status. Window effects go through Operator → Window Provider. No provider-to-provider calls.
