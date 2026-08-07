# Workspace Interaction Language

| Field | Value |
| --- | --- |
| **Status** | Canonical product behaviour guide (P16.PX3) |
| **Authority** | Product heuristics under Spec v2 — **not** constitutional law |
| **Peers** | `PRODUCT_GRAVITY_RULE.md` · Product Presentation Spec · Product Proof Rule |
| **Date** | 2026-08-07 |
| **Commit** | `5fef6e3` |

This document defines how Workspace **feels**.  
Future experience and presentation work should implement this language consistently.  
It does not change architecture, capabilities, runtime ownership, or production gates.

---

## Personality

Workspace is a **calm conversational desktop companion**.

Users should reach for words like: effortless · premium · trustworthy · calm · responsive · intentional · conversational.

Users should **not** reach for: AI app · desktop app · voice tool · control panel · agent IDE.

| Pillar | Meaning |
| --- | --- |
| Conversation | The product |
| Desktop operation | What Conversation does |
| Trust | The outcome |

---

## 1. Motion

| Property | Language |
| --- | --- |
| Character | Soft, minimal, intentional |
| Speed | Unhurried breathing for ambient states; quick settle for acknowledgment |
| Elasticity | Slight ease — never bounce or spring spectacle |
| Forbidden | Neon pulses competing with transcript; stacked status animations; motion that implies false progress |
| Accessibility | `prefers-reduced-motion` disables decorative motion; functional state changes remain |

**When motion is forbidden:** error punishment flashes, success fireworks, ambient always-on listen chrome, anything that pulls eyes off Conversation without earning attention.

---

## 2. Feedback

Acknowledge without visual noise.

| State | How Workspace communicates |
| --- | --- |
| Listening | Quiet mic motion (wave/pulse) — no glyph stack |
| Thinking / working | Streaming Conversation reply; honest preparing copy when needed |
| Success | Outcome stated in Conversation — not toast theatre |
| Failure | Plain language in Conversation; recovery path offered |
| Waiting | Stillness + truthful “working” text — not spinners that invent progress |

Never expose Provider / Router / Registry / IPC vocabulary in the default surface.

---

## 3. Attention

| Deserve emphasis | Should disappear / whisper |
| --- | --- |
| Latest Conversation turn | Utility chrome, developer tools |
| The one next action (e.g. Send after voice review) | Secondary status colours |
| Confirmations that protect the desktop | Instructional walls of text |
| Composer when the user is speaking or typing | Competing satellites by default |

Eyes move: **transcript → composer → one obvious control**.

---

## 4. Calmness

- Remove unnecessary movement.
- Remove unnecessary colour (status-board neon is not identity).
- Remove unnecessary words (prefer short conversational lines).
- Reduce cognitive load: one primary focus, one obvious action.

**Calm over clever. Trust over spectacle.**

---

## 5. Voice

| Step | Feel |
| --- | --- |
| Start | One click / clear control — immediate preparing honesty |
| Listening | Soft centered motion; calm fill (PX2) |
| Stop | Click, Enter, or Space (PX1) — no mic hunt required |
| Cancel | Escape during capture; clear draft after review |
| Review | Transcript in composer; F10 — never auto-send |
| Send | Enter or Send — **visually obvious after voice** (PX3) |
| Errors | Soft retry vs Settings path; desktop language |
| Recovery | Retry or Settings once — never punish |

---

## 6. Conversation

Every transition should feel **conversational**, not application-wizard.

- Capabilities appear as natural outcomes inside Conversation.
- Confirmations protect the desktop without breaking the conversational thread.
- Empty states stay quiet (no marketing empty chrome).
- Window behaviour supports Conversation gravity (collapse / expand without OS title-bar theatre).

---

## 7. Quality rules (product heuristics)

Not constitutional laws. Prefer these unless Spec or Product Proof forbids:

1. **One obvious action** at a time.
2. **One primary focus** — Conversation wins.
3. **Never surprise** the user (no silent desktop mutation).
4. **Recover instead of punish.**
5. **Explain instead of expose** implementation.
6. **Reduce clicks** whenever constitutional safety permits.
7. **Calm over clever.**
8. **Trust over spectacle.**
9. **Friction preference:** when choosing between exposing engineering complexity and reducing user friction, prefer reducing friction unless Spec or Product Proof forbids it.

---

## 8. Repository audit snapshot (PX3)

| Surface | Coherent with language? | Notes |
| --- | --- | --- |
| Composer | Mostly | Quiet; voice→Send path strengthened in PX3 |
| Voice / listening | Improved (PX1–PX2) | Keyboard stop; calm wave |
| Thinking / executing | Yes | Streaming + truthful replies |
| Success / failure | Yes | Conversation-native |
| Confirmation | Yes | Restore review honesty |
| Empty / loading | Mostly | Preparing honest; empty still sparse |
| Notifications | Mostly | Keep Conversation-primary |
| Keyboard | Improved | Capture Stop/Cancel; Enter Send |
| Motion | Improved (PX2–PX3) | Soft Send cue after voice |
| Accessibility | Yes | Reduced-motion respected |

---

## 9. PX3 implementation (single improvement)

**Soft Send emphasis after voice review**

- After transcript insert, `data-voice-ready` on Send — calm highlight, brief soft pulse (2 cycles), reduced-motion safe.
- Cleared on Send, Escape/clear, or empty draft.
- F10 preserved — no auto-send.

---

## 10. Future alignment (not shipped here)

| Item | Class |
| --- | --- |
| Shorter reviewing copy | **Done (PQ1)** — `Send when you're ready.` |
| Explicit Stop/Cancel chrome | Track A |
| Composer empty-state calm | Track A |
| Hold-to-talk | Future |
| Production gates / P17 | Out of scope |

---

## Tests for future changes

1. Does this feel conversational rather than application-like?
2. Is there still only one obvious next action?
3. Did we add colour, motion, or words that Conversation did not need?
4. Would an Owner say “calm / trustworthy” — or “AI app / voice tool”?

If (4) trends toward the wrong vocabulary, do not ship in the default surface.
