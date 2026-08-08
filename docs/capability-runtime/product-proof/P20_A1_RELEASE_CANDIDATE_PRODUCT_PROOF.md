# P20.A1 — Release Candidate Product Proof

| Field | Value |
| --- | --- |
| **Program** | Release Candidate Product Proof |
| **ID** | P20.A1 *(audit naming — not File Provider / Terminal roadmap IDs)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Read-only Release Candidate Product Proof — **no implementation** |
| **Prior product-quality arc** | P17 Cohesion · P18 Runtime Truth · P19 Desktop Lifecycle *(context only — not momentum)* |
| **Lens** | Experienced Windows desktop software reviewer entering RC evaluation |
| **Authority inputs** | Spec v2.1 · EES v1 · Product Quality Standard · Interaction Language · Capability Integration Standard · Product Gravity · Product Proof Rule · PR1 |
| **Method** | Judge the current product only; do not continue the previous subsystem by default |

**Naming clarity:** Constitutional **File Provider P17** remains blocked until P16 Owner Accept. This audit does not authorize capability expansion.

---

## 1. Executive Summary

Judged as a Release Candidate — install, open, use, Collapse, return, leave — Workspace’s **in-session Conversational Desktop Operator is largely coherent**. Conversation speaks honestly, Moments browse and tool sessions hold truth, and tray Show restores Conversation Form rather than an empty shell.

**RC verdict:** **Not Release Candidate ready** for broad Owner distribution. The product core can feel premium inside a known session; the package does not yet feel like finished Windows software end-to-end.

**Single largest remaining engineering-owned obstacle to production-ready feel:**  
**First-session silence** — empty Conversation with a blank waiting surface and empty composer (`placeholder=""`). An experienced reviewer opening Workspace for the first time meets a calm void with no truthful invitation to speak or type. That is not Product Gravity; it is missing learnability. The software asks the Owner to already know how Workspace works.

Secondary engineering-owned gaps (not the recommended program by default):
- Close vs Exit predictability (C2) — behaviour partially exists; Owner-facing policy incomplete
- Earned Moments dock forgotten after Collapse (`toolDock` cleared)
- Conversation demoted when Moments dock is open
- Rare-path corrupt-session honesty (C1)

**Largest non-engineering blockers** (see §11): unsigned installer / no updater; Owner Voice Product Proof Accept.

**Recommended next engineering program (one only):** calm **Empty Conversation First-Session Cue** — a single truthful invitation that preserves Product Gravity and does not invent a catalogue.

---

## 2. Release Candidate Scorecard

Scores = Owner-visible RC feel today.

| Area | Score | Complete? | Trust? | Surprise / contradict / forget? |
| --- | --- | --- | --- | --- |
| Install | **Partial** | No | Low (unsigned) | SmartScreen fear |
| First launch | **Weak** | No | Low | Blank void — “what do I do?” |
| Startup | **Strong** | Yes (eng) | High | Single-instance calm |
| Tray | **Strong** | Mostly | High | Show restores Form (post-P19.S1) |
| Restore (tray / secondary) | **Strong** | Yes (eng) | High | Form returns |
| Shutdown / Exit | **Partial** | No | Med | Close≠Exit; C2 OA open |
| Conversation (known user) | **Strong** | Mostly | High | Honest failures / busy |
| Conversation (first hour) | **Weak** | No | Low | Silence ≠ premium invite |
| Voice (engineering) | **Strong** | Eng yes | High | Review-before-send intact |
| Voice (Owner PP) | **Unknown** | No | — | Pending Accept |
| Moments browse / session | **Strong** | Yes | High | Shared list; session continuity |
| Runtime continuity (in-process) | **Strong** | Mostly | High | toolDock still drops on Collapse |
| Navigation | **Strong** | Yes | High | Home/Continue/Save coherent |
| Keyboard (Moments agency) | **Strong** | Yes | High | Esc/Enter predictable |
| Accessibility | **Partial** | No | Med | Labels present; no a11y program |
| Responsiveness | **Strong** | Yes | High | Working-state continuity |
| Trust (truthfulness) | **Strong** | Mostly | High | Does not invent success |
| Diagnostics / support | **Strong** | Mostly | High | Conversation export |
| Recovery | **Partial** | No | Med | C1 honest corrupt UX open |
| Production polish | **Partial** | No | Med | 0.1.0 / unsigned / no updates |
| Daily workflow (known Owner) | **Strong** | Mostly | High | Operator path works |
| Discoverability | **Weak** | No | Low | NL-only + empty cue absent |
| Cognitive load (first session) | **Weak** | No | — | Owner must invent the first act |
| One premium desktop app? | **Partial** | No | — | Strong core; unfinished edges |

---

## 3. Production-Ready Areas

Surfaces that already feel RC-grade **inside a running, known session**:

| Surface | Why a reviewer would trust it |
| --- | --- |
| Conversation as product | Gravity when dock idle; one voice for outcomes |
| Capability failure honesty | Truthful compose — no invented success |
| Working-state continuity | Busy from Send through real work |
| Moments browse + Save/Restore session | List and flow survive ordinary navigation |
| Tray Show → Conversation Form | Window return matches product return |
| Single-instance focus | Second launch brings Conversation forward |
| Voice capture → review → Send (eng) | F10 intact; calm chrome |
| Support bundle via Conversation | Diagnosable without settings archaeology |
| Closed providers P10–P15 | Clipboard through Screenshot behave as desktop ops |
| Privacy posture | Local-first; no ambient mic; no network telemetry by default |

These areas behave like **one intentional product**, not a kit of panels.

---

## 4. Remaining Production Gaps

| Gap | Owner-visible failure mode |
| --- | --- |
| **Empty first Conversation** | Waiting surface + empty placeholder — no calm invitation |
| **Close vs Exit policy (C2)** | Close collapses; Exit quits; matrix not Owner-accepted as finished |
| **toolDock after Collapse** | Form returns; earned Moments satellite does not |
| **Dock gravity when Moments open** | Conversation narrows; attention can feel split |
| **Corrupt-session honesty (C1)** | Rare empty after recovery can look like amnesia |
| **Unsigned install / no updater** | Not a finished Windows distribution story |
| **Voice live Accept** | Product Complete unknown; File Provider blocked |
| **OA unpaid on production units** | Engineering Complete ≠ Operational Acceptance |
| **Accessibility program** | Labels exist; no RC a11y pass |
| **IPC surface / D1** | Security debt; low daily feel |

---

## 5. Root Cause Analysis

Grouped by root cause — not by feature request.

### RC-A — First-session invitation absent (Critical · engineering-owned · daily first impression)

Product Gravity correctly rejects catalogues and menus. The empty Conversation currently offers **no substitute invitation**. Waiting chrome is decorative silence; composer `placeholder=""` withholds the one line that would teach “talk to Workspace.”

**Owner experience:** “The window opened. The product did not begin.”  
**Contradiction:** A conversational desktop that does not invite conversation.  
**Not:** Architecture debt. **Is:** Learnability / cognitive-load debt.

### RC-B — Shutdown semantics unfinished (High · engineering-owned · daily lifecycle)

Close → Collapse and tray Exit exist, but C2 Owner-facing close/hide/Exit policy is not closed. Reviewers still ask whether X quits, hides, or zombies the app.

**Owner experience:** “I don’t know what leaving means.”  
**Note:** Completing this continues the desktop-lifecycle subsystem; it is ranked, not the default next program (§9).

### RC-C — Earned surface amnesia on Collapse (High · engineering-owned · Moments path)

Collapse intentionally clears `toolDock`. Tray Show restores Form B; the earned Moments satellite does not return. Feels like forgetting work context after a normal desktop hide.

### RC-D — Attention split when Moments earned (Medium · engineering-owned)

Dock-on layout can demote Conversation. Truth is intact; premium cohesion is not.

### RC-E — Rare-path honesty incomplete (Medium · engineering-owned)

C1 corrupt-session UX ReadyNow but unimplemented. Trust fails when rare recovery looks like emptiness.

### RC-F — Distribution trust incomplete (Critical · largely external)

Unsigned binary; no updater. RC distribution confidence fails before the UI is judged.

### RC-G — Owner Product Proof gate open (Critical · Owner-owned)

P16 Voice engineering complete; live Accept pending. Capability expansion remains blocked.

---

## 6. Ranked Engineering Opportunities

| Rank | Opportunity | Class | RC rationale |
| --- | --- | --- | --- |
| **1** | **Calm empty-Conversation first-session cue** | **Implement Now** | Highest engineering-owned first-impression gap; does not continue tray/lifecycle by default |
| 2 | C2 close-vs-hide / Exit predictability | Gate C2 | Finishes desktop organism; lifecycle subsystem |
| 3 | Restore earned toolDock with Conversation after Collapse | Continuity | Remaining “forget” after Form restore |
| 4 | Dock ↔ Conversation attention cohesion | Polish | Premium when Moments open |
| 5 | C1 corrupt-session honesty | Gate ReadyNow | Rare-path trust |
| 6 | D1 IPC quarantine | nextReadyNow | Security; low daily feel |
| — | More tray/ShellMode polish | — | **Reject as default** (P19.S1 closed the Form gap) |
| — | Moments ambient / stage remount | — | **Reject** as next RC slice |
| — | File Provider / new capabilities | — | **Blocked** until Voice Accept |

---

## 7. Estimated User Impact

| Opportunity | First impression | Daily trust | Cognitive load | Premium feel |
| --- | --- | --- | --- | --- |
| First-session Conversation cue | **Critical** | Med | **Highest reduction** | High |
| C2 shutdown clarity | Med | **Critical** | High | High |
| toolDock restore | Low | High (Moments) | Med | Med–High |
| Dock gravity | Low | Med | Med | High polish |
| C1 corrupt honesty | Low | High when hit | Med | Med |
| D1 IPC | None felt | Security | None | Low daily |
| Signing + updater | **Critical** install | High update | Low | Highest release |

---

## 8. Estimated Engineering Effort

| Opportunity | Effort | Risk |
| --- | --- | --- |
| First-session Conversation cue | **S** | Low (copy + Gravity; no catalogue) |
| C2 shutdown policy | **S–M** | Med (OS close semantics / OA) |
| toolDock restore | **S** | Taste / attention |
| Dock gravity cohesion | **S–M** | Layout taste |
| C1 corrupt honesty | **S–M** | Low–Med |
| D1 IPC quarantine | **M** | Regression-sensitive |

---

## 9. Recommended Next Engineering Program

### P20.S1 — Empty Conversation First-Session Cue

**Direction:** When Conversation has no messages, present one calm, truthful invitation to speak or type — without menus, catalogues, or capability lists.

**Why this program (and not defaulting to lifecycle again):**
1. RC evaluation starts at first launch; the blank void is the first Owner-visible failure under engineering control.
2. P19 already closed tray Form restore; continuing Collapse/Exit by default would optimize the previous subsystem rather than the largest remaining Owner gap.
3. In-session operator quality (P17–P18) is strong enough that **learnability**, not cohesion or Moments truth, is now the binding constraint on “feels finished.”
4. Product Gravity forbids a launcher; it does not forbid a single conversational invitation.
5. C2 remains the strongest **runner-up** for Owners who already know how to talk to Workspace and judge Windows quit semantics next.

**Parallel tracks (not this program):**
- Owner: P16 Voice live Product Proof
- Gate: C2 shutdown policy (after Owner prioritizes lifecycle)
- Track A / external: A2 signing → B2 updater

---

## 10. Single Highest-Value Executable Vertical Slice

### Empty Conversation First-Session Cue

**Problem:** First open shows an empty waiting surface and a composer with no placeholder. Reviewers and new Owners confront silence and must invent the first act. Workspace feels unfinished before any capability runs.

**Outcome:** An empty Conversation still feels like a premium conversational desktop — one calm line (and/or quiet empty-state copy) that truthfully invites ordinary speech or typing, then yields completely once the transcript exists.

**In scope (smallest slice):**
1. Empty-state cue when `messages.length === 0` (not a feature tour).
2. Composer invitation that does not become a catalogue (`placeholder` or equivalent calm affordance).
3. Preserve Product Gravity, Interaction Language, Voice F10, and P17–P19 behaviour.
4. Verifier: empty Conversation shows invitation; nonempty hides it; no settings/capability menu introduced.
5. Do not implement C2, toolDock restore, dock CSS rewrite, signing, or File Provider.

**Out of scope:** Onboarding wizard; capability catalogue; tray/ShellMode changes; Spec reopen.

**Success test:** Fresh launch → Owner immediately understands they may talk or type → first Send works unchanged → after first message, silence/cue is gone.

---

## 11. Items blocked by external dependencies

| Item | Blocker | Effect on RC |
| --- | --- | --- |
| **Code signing (A2)** | Authenticode certificate / Track A external | SmartScreen; install trust |
| **Auto-updater (B2)** | Blocked by A2 | No calm update story |
| **Release automation / signed artifacts** | CI release + signing | No RC channel ship |
| **Crash reporter pipeline** | Track A / product choice | Hard-crash diagnosability |
| **Owner Voice Product Proof Accept** | Owner live session | P16 Product Complete; File Provider remains blocked |
| **Operational Acceptance (A0 / B1 / E1)** | Owner checklists | Engineering Complete ≠ Production Ready |
| **D1 IPC quarantine** | Owner-authorized security program | Large surface; low daily feel until done |

These dominate **distribution RC** confidence. They do not replace the engineering-owned first-session cue for **product feel** under engineering control.

---

## Explicit answers

| Question | Answer |
| --- | --- |
| RC ready? | **No** |
| Largest engineering-owned obstacle? | **First-session silence / empty Conversation** |
| Would an experienced desktop user trust it today? | **In-session: mostly. End-to-end RC: no.** |
| Does Workspace contradict / forget? | Rarely on Moments truth; **forgets earned dock** on Collapse; **contradicts “conversational”** at first open |
| Recommended next program? | **P20.S1 Empty Conversation First-Session Cue** (§9–10) |
| Continue tray/lifecycle by default? | **No** |
| Implement now? | **No** — audit only |

---

## Compliance

| Authority | Posture |
| --- | --- |
| Spec v2.1 | Unchanged; no Review Trigger |
| EES v1 | Audit program; documentation max layer |
| Product Quality Standard | Used as excellence lens |
| Interaction Language | Used as feel lens |
| Capability Integration Standard | No provider expansion |
| Production Before Expansion | File Provider remains blocked |
| Artifact obligation | Audit-only — no new verifier for Owner ranking; health + milestone updated |

---

## Stop

Release Candidate Product Proof complete. **Do not implement.** Await Product Owner direction before any engineering program.
