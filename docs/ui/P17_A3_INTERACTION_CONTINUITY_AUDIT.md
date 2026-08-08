# P17.A3 — Interaction Continuity Audit

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Interaction Continuity Audit |
| **ID** | P17.A3 *(audit naming only — not File Provider P17)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Read-only product quality audit — **no implementation** |
| **Prior** | P17.S2 unified Moments Save/Restore **success acknowledgment** |
| **Sequence** | A1→S1 failure communication · A2→S2 success communication · **A3→(future S3) interaction behaviour** |
| **Authority inputs** | Spec v2.1 · EES v1 · Product Quality Standard · Interaction Language · Product Gravity |
| **Method** | Repository evidence only |

---

## 1. Executive Summary

Premium desktop products feel predictable because **similar interactions behave similarly** — Escape, Enter, focus return, busy ownership, cancel, back, and recovery. Workspace already has a clear Interaction Language for **Conversation and voice**. Moments agency cards (Approve / Delete / dismiss) and dual busy channels do not yet share that contract.

| Domain | Continuity today |
| --- | --- |
| Voice capture Stop / Cancel | Strong (PX1) — Enter/Space stop; Esc cancel |
| Composer Esc / Enter | Strong — clear draft; send (Shift+Enter newline) |
| Soft Send after voice | Strong (PX3) |
| Moments Approve / Delete / Not now | **Pointer-only** — Esc and Enter do nothing |
| Busy ownership | **Split** — App `busy` vs OperatorRoot `busy` |
| Mid-IPC cancel (Save/Restore) | None — wait until `finally` |
| Focus on agency open / dismiss | Weak — no move into card; no return to composer |

**Worst fracture:** When a restore preview or delete confirmation is on screen, the Owner’s mental model of Esc (“cancel / dismiss”) and Enter (“commit primary”) — established by Conversation and voice — **stops applying**. Agency cards look like dialogs but lack dialog continuity.

**Single highest-value next slice (not implemented here):**  
**Moments agency keyboard continuity** — Esc dismisses preview/delete like Not now / Cancel; Enter commits the primary when the card owns the step; focus enters the card on open and returns to Conversation on dismiss. No new modal framework. No busy unification in this slice.

---

## 2. Interaction Continuity Inventory

### 2.1 Escape

| Surface | Behaviour | Evidence |
| --- | --- | --- |
| Conversation composer | Esc clears draft + voice-ready cue | `OperatorRoot.tsx` ~497–502 |
| Voice capturing | Esc cancels listen (`stopPropagation`, capture phase) | `VoiceMicButton.tsx` ~182–211 |
| Moments preview (“Not now”) | **No Esc** — click only | `ContinuePreviewObject.tsx` ~163–170; `ResumeContextPanel.tsx` |
| Delete confirm | **No Esc** — Cancel button only | `ResumeContextPanel.tsx` ~414–438 |
| Inspect / done / Save cards | **No Esc** | Resume / Save panels |
| App toasts | No Esc dismiss | `App.tsx` banners |
| Conversation `busy` | Composer disabled → Esc handler does not run | `OperatorRoot.tsx` `disabled={busy}` |

### 2.2 Enter / Space

| Surface | Behaviour | Evidence |
| --- | --- | --- |
| Conversation | Enter sends; Shift+Enter newline | `OperatorRoot.tsx` ~504–507 |
| Voice capturing | Enter / Space = **stop** (F10 review preserved) | `VoiceMicButton.tsx` ~204–208 |
| After voice transcript | Enter sends; Soft Send cue | `OperatorRoot.tsx` + Interaction Language |
| Save name / handoff | Enter = newline in textarea; save is button-only | `SaveContextPanel.tsx` |
| Approve / Delete | **Click only** — no Enter binding | `ContinuePreviewObject.tsx`; Resume delete card |
| Moment object (shell path) | Enter/Space activates object | `WorkspaceObject.tsx` (legacy shell) |

### 2.3 Focus movement

| Event | Behaviour |
| --- | --- |
| Expand Conversation | Focus composer (~40ms) |
| Voice → transcript | Focus + select composer |
| Save “Touch to leave a note” | Focus handoff textarea |
| Open preview / delete / inspect | **No autofocus / trap** |
| Toast / error | Focus unchanged |
| Dismiss agency → browse | **No return focus to composer** |

### 2.4 Keyboard ownership & busy

| Channel | Locks | Gap |
| --- | --- | --- |
| OperatorRoot `busy` | Composer + mic during Conversation turn | Moments dock buttons still clickable if open |
| App `busy` | Moments / Home / Pilot controls during IPC | Composer + mic **still live** — Esc can clear draft / voice can start |
| Voice capture | Window capture-phase keys while listening | Correct local ownership |
| Dual `useState` | Independent | Owner has no single “Workspace is working” model |

### 2.5 Dialog / card dismissal

| Card | Dismiss | Labels |
| --- | --- | --- |
| Restore preview | “Not now” → `backToBrowse` | Dismiss class |
| Delete confirm | “Cancel” → inspect | Same class, different word |
| Inspect | “Back” → browse | Navigation |
| You’re back | “Back to saved moments” | Navigation |
| Save success | “Save another moment” | Restart |
| Shared modal layer | **Absent** — step UI on `WorkspaceSurface` | Correct architecture; missing keyboard contract |

### 2.6 Loading interruption / cancellation

| Path | Owner-cancellable? |
| --- | --- |
| Voice listen | Yes — Esc / mic / Enter-stop |
| Conversation stream | Implicit abort only on next turn; **no Cancel control** |
| Save / preview / restore / delete IPC | **No** — disabled until `finally` |
| AssistantPanel Cancel | Exists off default App gravity path — not Owner default |

### 2.7 Retry after failure

| Surface | Affordance |
| --- | --- |
| Voice | Soft retry chrome; Settings after repeated fails |
| Conversation capability | Retype / new utterance (S1 compose) |
| Moments IPC fail | Sticky error banner; re-click same button when unlocked — **no Retry label** |

### 2.8 Back navigation & interruption recovery

| Flow | Continuity |
| --- | --- |
| Resume steps | browse → inspect → confirm_delete; browse → preview → done — Back / Not now / Cancel exist as clicks |
| Save steps | naming → saved; Clear / Save another |
| Leave Moments mid-flow (view switch) | Local step/draft **lost on unmount** |
| Conversation-named restore | `focusContextId` can reopen preview — good when path works |
| Expand host / ActiveMoment primary | Provider path weak on default App tree — structural follow-up, not S3 |

---

## 3. Paired Inconsistencies (same intent, different behaviour)

| Intent | Surface A | Surface B |
| --- | --- | --- |
| Abort / dismiss in-progress UI | Voice Esc cancels; composer Esc clears | Approve / Delete: Esc **silent** |
| Commit primary action | Conversation Enter sends | Approve / Delete: Enter **silent** |
| UI locked while working | Conversation busy disables composer | App busy leaves composer live (and vice versa) |
| Stop what I started | Voice interruptible | Moments IPC: wait only |
| Leave confirmation | Preview “Not now” vs Delete “Cancel” | Same dismiss class, different vocabulary |
| Try again after failure | Voice named retry | Moments: silent re-enable + sticky toast |
| Keys know where to go next | Voice → composer focus+select | Agency open: focus stays wherever it was |

---

## 4. Cognitive Load Analysis

| Workflow | Continuity cost | Removable? |
| --- | --- | --- |
| Voice → review → Send | Low — language already taught | Preserve |
| Capability ask | Low | Preserve |
| Restore Approve (pointer) | Medium — correct agency, wrong keyboard | **Keyboard contract** |
| Delete confirm | Medium — same | **Keyboard contract** |
| Restore while App busy | Medium — Conversation still accepts keys | Later (busy unify) |
| Mid-save cancel expectation | Medium — desktop habit unmet | Later (cancel) |
| Toast stuck after error | Low–Med | Later |

**Product Gravity risk:** Confirmations that protect the desktop **should** earn attention — but once they own attention, Esc/Enter must match the language Conversation already taught. Otherwise every Moments confirmation is a small relearning tax.

---

## 5. Product Cohesion Assessment

| Dimension | Rating | Notes |
| --- | --- | --- |
| Interaction Language (voice/composer) | Strong | PX1–PX3 |
| Interaction Language (Moments agency) | Weak | Pointer dialogs without Esc/Enter |
| Predictability across surfaces | Weak | Same intent ≠ same keys |
| Busy / interruption model | Weak | Dual channels; asymmetric cancel |
| Focus continuity | Weak | Agency open/dismiss ignore gravity return |
| Success / failure communication | Improved | S1 + S2 |
| Premium benchmark | Behind on “one keyboard language” | Raycast / system dialogs / ChatGPT confirmations are keyboard-consistent |

**Verdict:** Workspace does **not** yet behave as one continuous application for interrupt / confirm / dismiss. Capture and Conversation are coherent; Moments agency is the clearest remaining interaction fracture.

---

## 6. Root Cause Analysis

| ID | Root cause | Severity |
| --- | --- | --- |
| **RC1** | Interaction Language Esc/Enter defined for Conversation/voice only; Moments agency never adopted it | **Critical** |
| **RC2** | Agency cards are semantic dialogs without dialog keyboard/focus semantics | **Critical** |
| **RC3** | App `busy` and OperatorRoot `busy` are independent locks | High |
| **RC4** | Moments IPC has no Owner cancel; Conversation stream has no Cancel control | High |
| **RC5** | Default App tree underuses ActiveMoment expand/focus host — Continue continuity gaps | High (structural; separate program) |
| **RC6** | Retry / toast dismiss vocabulary uneven vs voice | Medium |
| **RC7** | Voice Esc/Enter, Approve agency, Soft Send, S2 success cohesion — preserve | Already Excellent |

---

## 7. Ranked Improvement Opportunities

| Rank | Opportunity | Class | Lift |
| --- | --- | --- | --- |
| 1 | **Moments agency keyboard continuity** — Esc / Enter / focus return for preview + delete | **Implement Now (next)** | Highest |
| 2 | Align dismiss vocabulary (Not now vs Cancel) into one Interaction Language line | With #1 | Med |
| 3 | Document or unify App vs OperatorRoot busy | Track A | High |
| 4 | Owner-visible cancel for long Moments IPC (optional) | Track A | Med |
| 5 | Focus return + Esc dismiss for sticky error banners | Track A | Med |
| 6 | Named Retry on Moments failure | Track A | Low–Med |
| 7 | Repair ActiveMoment / expandHost on OperatorRoot path | Structural program | High (larger) |
| 8 | Full modal framework / a11y WCAG sweep / hold-to-talk | — | **Reject for S3** |

---

## 8. Estimated Engineering Effort

| Slice | Effort | Risk | Scope |
| --- | --- | --- | --- |
| Moments agency keyboard continuity (#1) | **S** | Low | `ContinuePreviewObject`, `ResumeContextPanel`; small verifier; Interaction Language note |
| Busy unification | M | Med | `App.tsx` + `OperatorRoot` coordination |
| ActiveMoment on App path | M–L | Med–High | Provider mount / expand host |
| Mid-IPC cancel | M | Med | Abort + UX |
| Full dialog design system | L | High | **Out of scope** |

---

## 9. Single Highest-Value Executable Vertical Slice

### Moments Agency Keyboard Continuity

**Problem:** Approve restore and Delete confirmation require pointer clicks for dismiss and commit, while Conversation and voice have already taught Esc = cancel/dismiss and Enter = primary commit. Focus does not enter the card or return to Conversation. That relearning tax hits the core trust workflow (Moments).

**Outcome:** When preview or delete confirm is the active agency step, Esc and Enter match Conversation-grade continuity; focus is predictable; no change to voice F10, Soft Send, or execution semantics.

**In scope:**
1. Restore **preview**: Esc = same as “Not now” (dismiss, no restore).  
2. **Delete confirm**: Esc = same as “Cancel” (return to inspect, no delete).  
3. Enter activates the **primary** control (Approve and restore / Delete permanently) when the agency card owns the step (focus inside card or documented default focus on open).  
4. On open of preview/delete: move focus into the card (primary or safe Cancel control). On dismiss / back-to-browse: return focus to Conversation composer.  
5. Do **not** change voice capture Esc/Enter ownership, F10, Soft Send, S2 success surfaces, failure banners, or mid-IPC cancel.  
6. Add a focused verifier (e.g. `scripts/verify-moments-agency-keyboard.mjs`) asserting agency Esc/Enter handlers exist.  
7. Optionally one Interaction Language line: agency confirmations share Esc/Enter with Conversation.

**Out of scope:** New modal framework, busy unification, ActiveMoment remount, File Provider P17, Spec reopen, toast redesign, WCAG sweep.

**Why this over busy unification or ActiveMoment repair:** Highest day-to-day cognitive load is “keys mean different things for the same class of intent” between Conversation and Moments confirmations. Scope stays presentation/interaction; builds on S2 (Moments cards as Owner acknowledgment surface); smallest complete vertical slice.

**Success test:** With preview or delete confirm open, Esc dismisses without side effects; Enter commits primary when appropriate; after dismiss, typing resumes in Conversation without a mouse hunt. Voice capture Esc/Enter unchanged.

---

## 10. Preserve / Reject

| Preserve | Why |
| --- | --- |
| F10 — voice never auto-submits | Product Proof / Interaction Language |
| Voice: Enter/Space stop; Esc cancel during capture | PX1 |
| Soft Send after voice | PX3 |
| Composer Esc clears draft when focused & idle | Documented cancel language |
| Approve-before-restore agency | Constitutional trust |
| Delete requires confirm step | Destructive safety |
| S2: no ok-banner duplex on Save/Restore success | Just shipped |
| Operator Authority / Presentation purity | No provider orchestration in UI |

| Reject for next slice | Why |
| --- | --- |
| New status / modal framework | Overbuild |
| Full a11y / shortcut matrix program | Wrong altitude for S3 |
| Spec / governance rewrite | Not required |
| File Provider P17 | Blocked |

---

## Explicit answers

| Question | Answer |
| --- | --- |
| One coherent interaction language across workflows? | **Not yet** — voice/composer yes; Moments agency no |
| Highest continuity fracture? | Esc/Enter/focus on Approve + Delete confirmations |
| Highest-leverage next slice? | **Moments Agency Keyboard Continuity** (§9) |
| Should busy unification be S3? | No — high value, larger; Track A after keyboard continuity |
| Accessibility-only audit? | No — continuity / cognitive load; a11y may benefit as side effect |

---

## Stop

Audit complete. **No code implemented.**  
Await Product Owner review before **P17.S3** implementation.
