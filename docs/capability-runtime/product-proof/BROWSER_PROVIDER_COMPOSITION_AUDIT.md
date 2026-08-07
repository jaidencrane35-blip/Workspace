# Provider Composition Audit — Browser (P14 / P14.5)

| Field | Value |
| --- | --- |
| **Provider** | Browser |
| **Program** | P14.5 Product Proof Remediation |
| **Status** | Complete — re-confirmed |

---

## 1. Independently useful?

**Yes.** Conversation opens URLs, reports browser availability, explains browser help, and refuses invalid sites without other providers.

Evidence: `browser-provider.proof.json` (P14.5) + Intent/Operator path only.

---

## 2. Composes with

| Partner | Today | Notes |
| --- | --- | --- |
| **Window Provider** | **Available** | `open_beside` snaps via Operator; focus (“bring chrome forward”) → Window focus |
| **Application Provider** | Adjacent | “Open Notepad” / “launch chrome” stay Application; sites use Browser |
| **Notifications Provider** | Future | Finish-watching not in L1–2 |
| **Conversation / Operator / Runtime** | **Available** | Sole governed path |

---

## 3. New user-visible capabilities

| Capability | Status |
| --- | --- |
| Open site / URL via Conversation | **Available** |
| NL aliases (chat gpt, git hub, latest chat) | **Available** (P14.5 Intent) |
| Invalid URL refusal | **Available** (P14.5) |
| Browser capability explanation | **Available** (P14.5 — not Guide) |
| Browser + Window — beside / focus | **Available** |
| Browser + Notifications | Future |
| Browser + Memory | Future |

---

## 4. Coupling?

**No.** BrowserPort only opens URLs / reports status. Window effects and focus go through Kernel Operator → Window Provider. P14.5 changed Intent + Operator compose/validate only — no provider-to-provider calls, no registry/runtime redesign.
