# Workspace Visual Reference Specification

**Program:** P24 — Personal AI + Intelligence Experience Architecture Reset
**Date:** 2026-08-08
**Status:** Experience architecture. **Nothing here is authorised for implementation.**
**Companion authority:** [`WORKSPACE_INTELLIGENCE_EXPERIENCE_CONTRACT.md`](WORKSPACE_INTELLIGENCE_EXPERIENCE_CONTRACT.md)
**Canonical Nova art:** [`assets/nova-character-sheet.png`](assets/nova-character-sheet.png) — **character #8 only**

---

## How to read this document

This specification defines the **visual truth** of Workspace: what the Owner sees in each meaningful state. Every frame maps to a real runtime state. Concept art that cannot map to a capability is labelled accordingly and must not be treated as shipping claim.

| Classification | Meaning |
| --- | --- |
| **EXISTING** | Visible in the product path today |
| **PARTIAL** | Substrate or host exists; the depicted experience does not |
| **MISSING** | Not built |
| **PLANNED** | Sequenced in the Intelligence Experience Contract §35 |
| **BLOCKED** | Named dependency not cleared |

**Governing UI law** remains `docs/ui/PRODUCT_GRAVITY_RULE.md` and the two-form shell. Nova is the identity of Form A — not a third shell form.

---

## 0. Visual design language

### 0.1 Target

Premium · dark · calm · intelligent · spatial · lightweight · modern · readable · free-floating · personal.

### 0.2 Palette (current tokens — retain)

| Role | Token | Value | Use |
| --- | --- | --- | --- |
| Ink | `--ws-color-ink` | `#05070c` | Deep host |
| Background | `--ws-color-bg` | `#070b12` | Conversation shell |
| Text | `--ws-color-text` | `#f2f6fb` | Primary copy |
| Muted | `--ws-color-muted` | `#a3b6c9` | Secondary |
| Interface accent | `--ws-color-accent` | `#5fd0d8` | Chrome, focus rings, Send |
| Strong accent | `--ws-color-accent-strong` | `#8eecf0` | Caret, emphasis |
| **Nova / AI identity** | `--ws-color-accent-warm` | `#e8b87a` | Nova body light, AI presence |
| OK | `--ws-color-ok` | `#86efac` | Success |
| Danger | `--ws-color-danger` | `#f87171` | Failure |

**Rule:** Teal is interface. Amber is Nova. They must not compete for the same meaning.

### 0.3 Material

- Conversation: opaque deep gradient, thin white edge, soft drop shadow — **EXISTING**
- Nova: translucent amber dome, soft tentacles, unattached floating sphere — **MISSING** (art exists; code does not)
- Settings perimeter: tentacles as luminous edge — **MISSING**
- No glass blur on conversation shell (explicit P16.15 choice) — **EXISTING**
- No coloured bloom, no neon glow, no purple-on-white AI slop — law

### 0.4 Typography

Segoe UI Variable Text / Display stacks — **EXISTING**. Conversation type stays quiet; Nova never carries typography (no labels on Nova).

### 0.5 Motion laws

1. Motion depicts **runtime truth**, never decoration.
2. Idle motion decays toward stillness.
3. Prefer reduced motion when `prefers-reduced-motion` is set — **EXISTING** for mic CSS.
4. No looping “busy” animation without a real `acting` / `waiting` event.
5. Nova never moves solely for animation’s sake.

### 0.6 Forbidden visual patterns

Command launcher chrome · excessive cards · dashboard clutter · giant borders · fake activity · dense settings grids · giant scrollbars · multiple personality cast · progress bars with invented percentages · provider/engineering vocabulary in UI copy.

---

## 1. Canonical frames

Each frame specifies: visible UI, Nova position/state, Owner action, Workspace state, conversation state, controls, animations, runtime events, forbidden behaviour, accessibility, and status.

---

### Frame 1 — Expanded Workspace (Conversation Form B)

| Field | Spec |
| --- | --- |
| **Visible UI** | Undecorated floating conversation panel on the desktop: transcript, composer, quiet chrome |
| **Nova** | Not required on screen; may rest at edge as Form A if previously collapsed — or absent if only Form B is visible |
| **Nova state** | Idle / rest (if present) |
| **Owner action** | Typing or reading |
| **Workspace state** | ShellMode `1`; `main` window visible |
| **Conversation** | Thread above input; empty cue if first session |
| **Controls** | Brand, Collapse, Exit; mic; Send |
| **Control labels** | “Workspace”, Collapse, Exit; mic aria; Send `↵` |
| **Animations** | None required; mic idle |
| **Runtime events** | None |
| **Forbidden** | Dashboard widgets; satellite dock unless earned; Settings catalogue chrome |
| **Accessibility** | `role="log"` transcript; labelled composer; keyboard Send |
| **Status** | **EXISTING** (`OperatorRoot`, `tauri.conf.json` main window) |

---

### Frame 2 — Docked Nova (Form A idle)

| Field | Spec |
| --- | --- |
| **Visible UI** | Only Nova: 44×44 transparent always-on-top window at last resting position |
| **Nova** | Character #8 at rest — subtle, nearly unnoticed |
| **Nova state** | `idle` — breathing-scale at most; sphere gently present |
| **Owner action** | Working in other apps; Nova ignored |
| **Workspace state** | ShellMode `0`; `operator` visible; `main` hidden |
| **Conversation** | Not visible |
| **Controls** | Click Nova → restore Conversation; drag → reposition |
| **Animations** | Idle only; never “working” while idle |
| **Runtime events** | None |
| **Forbidden** | Wandering; following cursor; covering taskbar; appearing over exclusive fullscreen |
| **Accessibility** | Nova window has accessible name “Workspace”; click restores conversation |
| **Status** | **PARTIAL** — host window **EXISTING** (`DesktopOperator`); Nova character **MISSING** (today: abstract operator glyph) |

---

### Frame 3 — Nova moving on desktop

| Field | Spec |
| --- | --- |
| **Visible UI** | Nova in transit between rest positions or monitors |
| **Nova** | Mid-motion with weight; tentacles trail slightly |
| **Nova state** | `relocating` (Owner drag) or `settling` (gravity after release) |
| **Owner action** | Drag, or release after drag |
| **Workspace state** | Form A; geometry updating |
| **Conversation** | Closed |
| **Controls** | Drag handle = whole Nova |
| **Animations** | Settling to nearest rest with ease-out; DPI-aware monitor cross |
| **Runtime events** | Pointer drag; geometry persist |
| **Forbidden** | Autonomous roaming; bounce for entertainment; leaving work area |
| **Accessibility** | Drag announced only if AT requests; position not required to be spoken continuously |
| **Status** | **PARTIAL** — drag **EXISTING**; gravity / rest snap / monitor courtesy **MISSING** |

---

### Frame 4 — Floating conversation

| Field | Spec |
| --- | --- |
| **Visible UI** | Conversation alone; free-floating; resizable |
| **Nova** | Optional at edge (not inside the panel) |
| **Nova state** | Idle if present |
| **Owner action** | Converse |
| **Workspace state** | Form B |
| **Conversation** | Premium chat: multiline, calm messages, no cards |
| **Controls** | Floating / whispering chrome preferred over title-bar chrome |
| **Animations** | Soft message appear; no fake streaming of invented tokens |
| **Runtime events** | Utterance submit / reply reveal |
| **Forbidden** | Console aesthetic; giant chrome; command catalogue |
| **Accessibility** | Focus trap in composer when typing; Esc clears draft |
| **Status** | **EXISTING** with **PARTIAL** chrome quality (header bar still reads as app chrome; Shift+Enter newline **MISSING**) |

---

### Frame 5 — Nova + conversation

| Field | Spec |
| --- | --- |
| **Visible UI** | Conversation panel + Nova resting nearby (same monitor or adjacent) |
| **Nova** | Outside the conversation window — never embedded as a mascot corner |
| **Nova state** | `attentive` while Owner types or after a turn; returns to idle |
| **Owner action** | Using both presence and dialogue |
| **Workspace state** | Form B (+ Form A visible if dual-window presentation; or Nova as separate entity when implemented) |
| **Conversation** | Active thread |
| **Controls** | Standard conversation controls |
| **Animations** | Nova attention subtle; conversation unchanged |
| **Runtime events** | Composer focus / turn complete |
| **Forbidden** | Nova inside the transcript; avatar bubbles; second personality |
| **Accessibility** | Two windows: clear names; focus moves predictably |
| **Status** | **MISSING** as designed (today only one of Form A/B is the primary visible product presence) |

**Note on Product Gravity:** Default launch attention stays Conversation. Nova presence must not outrank the transcript. Frame 5 is optional coexistence, not a new default launch.

---

### Frame 6 — Nova executing real work

| Field | Spec |
| --- | --- |
| **Visible UI** | Desktop with target apps; Nova expressive; conversation may show truthful status |
| **Nova** | Directed toward the work; sphere forward |
| **Nova state** | `acting` — one gesture per real Kernel step |
| **Owner action** | Issued a desktop goal |
| **Workspace state** | Capability intent in flight |
| **Conversation** | Truthful working copy or step-aligned status — never fake progress % |
| **Controls** | Interrupt / stop when available |
| **Animations** | Map 1:1 to `acting{step}` events |
| **Runtime events** | Kernel step started / completed |
| **Forbidden** | Fake action; decorative busy loops; inventing steps |
| **Accessibility** | Live region announces real phase changes sparingly |
| **Status** | **MISSING** — needs execution event stream (Contract §21) |

---

### Frame 7 — Multi-step execution

| Field | Spec |
| --- | --- |
| **Visible UI** | Nova sequences states: observe → act → wait → verify → next |
| **Nova** | Distinct poses per phase; never blends wait into act |
| **Nova state** | Ordered: `observing` → `acting` → `waiting` → `verifying` → … |
| **Owner action** | Multi-step goal (“Open Chrome and Notepad”, prepare coding workspace, etc.) |
| **Workspace state** | Fixed composition or future generative bind |
| **Conversation** | Goal retained until completed / partial / failed |
| **Controls** | Interrupt |
| **Animations** | Phase changes only on events; pause on `waiting` |
| **Runtime events** | Composition step stream + Completion Contract facts |
| **Forbidden** | Skipping verify; celebrating mid-goal; “100%” bars |
| **Accessibility** | Announce phase only when Owner-facing and useful |
| **Status** | **PARTIAL** — compositions **EXISTING**; visual stream **MISSING** |

---

### Frame 8 — Settings transformation

| Field | Spec |
| --- | --- |
| **Visible UI** | Nova centres; tentacles extend to form a luminous perimeter; settings UI appears **inside** |
| **Nova** | Body size ≈ constant; tentacles become the frame |
| **Nova state** | `opening_settings` → `settings_host` |
| **Owner action** | Opens Settings (future surface) |
| **Workspace state** | Overlay / settings session |
| **Conversation** | May dim or yield focus; not required on screen |
| **Controls** | Permission toggles; Personal Context; Nova prefs; Close |
| **Control labels** | Plain language (mic, memory, patterns, suggestions…) — Contract §23 |
| **Animations** | Tentacles extend with weight; reverse on close |
| **Runtime events** | Settings open / close; permission change |
| **Forbidden** | Nova body scaling into a giant window; modal that ignores Nova; feature catalogue |
| **Accessibility** | Focus moves into first control; Esc closes and reverses transformation |
| **Status** | **MISSING** — Settings UI **MISSING**; transformation **MISSING** |

---

### Frame 9 — Voice

| Field | Spec |
| --- | --- |
| **Visible UI** | Conversation + mic in capturing phase; optional Nova `listening` |
| **Nova** | Eyes widen slightly; stillness otherwise |
| **Nova state** | `listening` while capturing; returns on end |
| **Owner action** | Mic click / stop / cancel |
| **Workspace state** | WinRT continuous recognition |
| **Conversation** | Composer receives transcript for review (F10 — no auto-send) |
| **Controls** | Mic (start/stop); Esc cancel; Enter/Space stop while capturing |
| **Animations** | Mic pulse/wave — **EXISTING**; Nova listening — **MISSING** |
| **Runtime events** | Capturing / ready / transcript / cancel / failure |
| **Forbidden** | Fake listening animation when not capturing; silent upload |
| **Accessibility** | Mic state announced; failure message clear |
| **Status** | **PARTIAL** — mic UI **EXISTING**; repeat-activation defect open; Nova listening **MISSING** |

---

### Frame 10 — Ambient listening

| Field | Spec |
| --- | --- |
| **Visible UI** | Persistent, unambiguous state indicator: LISTENING / NOT LISTENING / PROCESSING LOCALLY / CONTEXT CAPTURED / CONTEXT DISCARDED |
| **Nova** | Distinct quiet `ambient` presence — never urgent |
| **Nova state** | `ambient_on` / `ambient_off` |
| **Owner action** | Explicitly enabled ambient mode |
| **Workspace state** | Local rolling buffer; default OFF |
| **Conversation** | May show captured context when promoted |
| **Controls** | Clear toggle; never suggestion-enabled |
| **Animations** | Minimal; state colour change only |
| **Runtime events** | Buffer rotate; capture; discard |
| **Forbidden** | Silent recording; hidden upload; default ON |
| **Accessibility** | State always readable by screen reader |
| **Status** | **MISSING** — blocked until voice Owner Accept |

---

### Frame 11 — Proactive suggestion

| Field | Spec |
| --- | --- |
| **Visible UI** | Single quiet line (conversation or Nova whisper); dismissable |
| **Nova** | Soft `suggest` pulse once; then idle |
| **Nova state** | `suggesting` → idle |
| **Owner action** | Ignore, accept, or decline |
| **Workspace state** | Pattern recognised; gates passed (Contract §15) |
| **Conversation** | Optional one-line suggestion with evidence (“seen N times”) |
| **Controls** | Accept / Not now / Never for this |
| **Animations** | One-shot; no nag loop |
| **Runtime events** | Suggestion formed / accepted / declined / suppressed |
| **Forbidden** | Modal; focus steal; mid-task interrupt; autonomous execute |
| **Accessibility** | Polite aria live (polite, not assertive) |
| **Status** | **MISSING** on conversation path; kernel suggestion lifecycle **PARTIAL (orphaned)** |

---

### Frame 12 — Success

| Field | Spec |
| --- | --- |
| **Visible UI** | Truthful completion message; desktop shows the result |
| **Nova** | Brief settle; return to rest |
| **Nova state** | `completed` → `idle` |
| **Owner action** | Receives outcome |
| **Workspace state** | Goal completed under Completion Contract |
| **Conversation** | What was done, in Owner language |
| **Controls** | None required |
| **Animations** | Short; no confetti; no fake celebration |
| **Runtime events** | `completed` |
| **Forbidden** | Celebrating a step as a goal; inventing success |
| **Accessibility** | Completion announced once |
| **Status** | **PARTIAL** — reply copy **EXISTING**; Nova success state **MISSING** |

---

### Frame 13 — Partial

| Field | Spec |
| --- | --- |
| **Visible UI** | Clear completed vs remaining; next options |
| **Nova** | Settles unresolved — visibly not `completed` |
| **Nova state** | `partial` |
| **Owner action** | Decide next step |
| **Workspace state** | Partial under C-CMP-001 (within composition) |
| **Conversation** | Names what finished and what did not |
| **Controls** | Continue / stop / clarify as applicable |
| **Animations** | Hold; no happy settle |
| **Runtime events** | `partial` with step facts |
| **Forbidden** | Softening partial into success; hiding unfinished steps |
| **Accessibility** | Partial outcome explicit |
| **Status** | **PARTIAL** — text **EXISTING** for compositions; goal-scoped partial + Nova **MISSING** |

---

### Frame 14 — Failure

| Field | Spec |
| --- | --- |
| **Visible UI** | Honest failure; real next step if any |
| **Nova** | Withdraws; does not hide |
| **Nova state** | `failed` → idle |
| **Owner action** | Retry / grant permission / rephrase goal |
| **Workspace state** | Failed / permission denied / missing target |
| **Conversation** | Owner language; no provider jargon |
| **Controls** | Permission guidance when relevant |
| **Animations** | Withdraw; no shake-as-comedy |
| **Runtime events** | `failed` / `awaiting_permission` |
| **Forbidden** | Fake recovery animation; dead-end without alternative |
| **Accessibility** | Error announced; focus to recovery control if present |
| **Status** | **PARTIAL** — messages **EXISTING**; Nova failure **MISSING** |

---

### Frame 15 — Multi-monitor

| Field | Spec |
| --- | --- |
| **Visible UI** | Conversation and/or Nova on correct work areas |
| **Nova** | Crosses monitors coherently; DPI-aware; never off-screen |
| **Nova state** | `relocating` / `idle` |
| **Owner action** | Drag across displays or restore on another monitor |
| **Workspace state** | Monitor topology from observation — **EXISTING** |
| **Conversation** | Geometry persisted per window |
| **Controls** | Drag |
| **Animations** | Smooth monitor handoff; no teleport |
| **Runtime events** | Display change; geometry persist |
| **Forbidden** | Landing under taskbar; straddling bezels permanently |
| **Accessibility** | Window positions remain reachable |
| **Status** | **PARTIAL** — topology + window geometry **EXISTING**; Nova monitor physics **MISSING** |

---

### Frame 16 — Personal AI context

| Field | Spec |
| --- | --- |
| **Visible UI** | Personal Context view: registers in plain language with source and age |
| **Nova** | Hosts or accompanies Settings (Frame 8) |
| **Nova state** | `settings_host` or attentive |
| **Owner action** | Inspect / edit / delete memory or preferences |
| **Workspace state** | Registers R1–R7 (Contract §3) |
| **Conversation** | May answer “what do you know about me?” from the same data |
| **Controls** | Edit, delete, reset category, reset all |
| **Animations** | None required |
| **Runtime events** | Register read/write/delete |
| **Forbidden** | Hidden fields; inferred sensitive traits; undeletable rows |
| **Accessibility** | Full keyboard; clear delete confirmation |
| **Status** | **MISSING** — substrate orphaned; no Owner surface |

---

### Frame 17 — Capability suggestion

| Field | Spec |
| --- | --- |
| **Visible UI** | Proposal card/line: gap in Owner language; what it would enable; what it would touch |
| **Nova** | Quiet `suggest` |
| **Nova state** | `suggesting` |
| **Owner action** | Accept proposal into backlog / decline |
| **Workspace state** | Gap detection + C-INT-003 proposal pipeline |
| **Conversation** | Truthful: “I don’t have this yet — here’s what it would take” |
| **Controls** | Add to proposals / Not now |
| **Animations** | One-shot |
| **Runtime events** | Gap identified; proposal recorded |
| **Forbidden** | Claiming the capability exists; silent acquisition |
| **Accessibility** | Proposal readable and dismissable |
| **Status** | **PARTIAL** — proposal backlog **EXISTING**; detection + surface **MISSING** |

---

### Frame 18 — Authorized self-improvement proposal

| Field | Spec |
| --- | --- |
| **Visible UI** | Full governance view: change summary, impact, tests/verifiers required, rollback promise |
| **Nova** | Still; waiting — never “coding” animation without real work |
| **Nova state** | `awaiting_authorization` → (if approved) `acting` only on real pipeline events |
| **Owner action** | Authorise creation; later authorise activation |
| **Workspace state** | Contract §18 lifecycle |
| **Conversation** | Explains gates; never hides risk |
| **Controls** | Authorise / Reject / Defer; later Activate / Roll back |
| **Animations** | Only on real stage transitions |
| **Runtime events** | Propose → analyse → authorise → create → test → verify → PP → accept → activate |
| **Forbidden** | Autonomous rewrite; recursive loops; skipping gates; fake “compiling” |
| **Accessibility** | Two distinct authorisation steps; no single-click silent apply |
| **Status** | **MISSING** — last in roadmap (P5) |

---

## 2. Nova state machine (visual)

```text
idle ──► attentive ──► listening / ambient_on
  │              └──► understanding ──► observing ──► awaiting_permission
  │                                              └──► acting ──► waiting ──► verifying
  │                                                         └──► completed / partial / failed
  ├──► suggesting ──► idle
  ├──► relocating / settling ──► idle
  └──► opening_settings ──► settings_host ──► idle
```

**Invariant:** every non-idle state (except Owner drag) requires a runtime event. No event → return to idle.

| State | Event source | Status |
| --- | --- | --- |
| `idle` | default | **MISSING** (character) |
| `attentive` | composer focus / turn | **MISSING** |
| `listening` | voice capturing | **MISSING** |
| `ambient_on` | ambient mode | **MISSING** |
| `understanding` | intent/comprehension start | **MISSING** |
| `observing` | observation IPC | **MISSING** |
| `awaiting_permission` | permission gateway | **MISSING** |
| `acting` | provider step | **MISSING** |
| `waiting` | wait condition | **MISSING** |
| `verifying` | verification | **MISSING** |
| `completed` / `partial` / `failed` | completion contract | **MISSING** |
| `suggesting` | proactive / gap proposal | **MISSING** |
| `relocating` / `settling` | drag / gravity | **PARTIAL** / **MISSING** |
| `opening_settings` / `settings_host` | settings open | **MISSING** |

---

## 3. Visual truth registry

| Visual concept | Maps to capability / substrate | Classification |
| --- | --- | --- |
| Conversation panel | C-CON-001, Form B | **EXISTING** |
| Collapse / tray / Form A host | C-PROC-004, `operator` window | **EXISTING** |
| Nova character | C-NOV-001 (proposed) | **MISSING** |
| Nova physics | C-NOV-001 + monitor observation | **PARTIAL** |
| Execution visualisation | C-NOV-002 + Kernel step stream | **MISSING** |
| Mic listening UI | C-CON-002 | **EXISTING** |
| Ambient listening UI | (future) | **MISSING** / **BLOCKED** |
| Proactive whisper | C-PRO-001 (proposed) | **MISSING** (kernel orphan **PARTIAL**) |
| Settings + tentacle perimeter | C-NOV-003 (proposed) | **MISSING** |
| Personal Context view | C-MEM-002 (proposed) | **MISSING** |
| Capability gap proposal UI | C-INT-003 | **PARTIAL** |
| Self-improvement authorisation UI | §18 | **MISSING** |
| Multi-monitor awareness | C-OBS-006 | **EXISTING** (data); Nova use **MISSING** |
| Fake progress / fake success | — | **REJECTED** |

---

## 4. Relationship to Product Gravity

| Gravity rule | Visual implication |
| --- | --- |
| Attention → Conversation | Launch still opens Form B; Nova does not steal first paint |
| Form A/B frozen | Nova **is** Form A’s body — not Form C |
| Chrome whispers | Frame 4 evolves toward less title-bar chrome |
| No competing docks | Proactive suggestions are whispers, not panels |
| Trust is the outcome | Frames 12–14 never lie; Frame 6 never fakes work |

---

## 5. Accessibility baseline

- All Nova states that matter to outcome have a conversation or live-region equivalent.
- Reduced motion: Nova uses opacity/pose holds instead of travel animations.
- Contrast: amber Nova on dark desktop must meet visible-focus standards for the clickable Form A hit target (minimum 44×44 — **EXISTING**).
- Settings transformation: full keyboard path; Esc reverses.

---

## 6. Implementation non-claim

This document does **not** authorise art production pipelines, new Tauri windows, animation engines, or roadmap execution. It is the visual half of the P24 architecture reset. Implementation begins only when the Owner selects a roadmap item (typically P3 after P3.1 event stream) under a constitutional execution program.

---

## Stop

Visual reference is complete. Companion contract: `WORKSPACE_INTELLIGENCE_EXPERIENCE_CONTRACT.md`.
