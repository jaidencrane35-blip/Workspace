# Provider Composition Audit Draft — Screenshot (P15 Prep)

| Field | Value |
| --- | --- |
| **Provider** | Screenshot (proposed) |
| **Program** | P15 |
| **Status** | **Draft** — complete during implementation |

---

## 1. Independently useful?

**Expected: Yes.** Conversation can capture a monitor or window still without other providers once Status + Capture ship. Window naming benefits from Window Provider composition but is not required for “capture primary monitor.”

---

## 2. Which providers compose?

| Partner | Expected | Notes |
| --- | --- | --- |
| **Window** | P15 | Resolve “this” / titled window before capture |
| **Application** | P15 (via Window) | “Screenshot Notepad.” |
| **Browser** | Later | Capture after open site |
| **Clipboard** | Future | Copy image bytes |
| **Notifications** | Future | Confirm long-running jobs (not needed for stills) |
| **Memory** | Future | Attach to Moments |
| **Conversation / Operator / Runtime** | Required | Sole governed path |

---

## 3. New user-visible capabilities

| Capability | Status |
| --- | --- |
| Capture monitor / window via Conversation | Planned P15 |
| Screenshot + Window deixis | Planned P15 |
| Screenshot + Clipboard | Future |
| Screenshot + OCR | Future program |
| Screenshot + Memory | Future |

---

## 4. Coupling?

**Required answer at implementation:** No provider-to-provider calls. Window resolution via Kernel Operator only. `ScreenshotPort` must not import Window Provider types.

---

## Completion gate

Fill final audit (evidence + Available vs Future) during P15 engineering — same structure as Notifications / Browser audits.
