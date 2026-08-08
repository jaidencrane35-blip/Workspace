# P17.A4 — Responsive Interaction Audit

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Responsive Interaction Audit |
| **ID** | P17.A4 *(audit naming only — not File Provider P17)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Read-only product quality audit — **no implementation** |
| **Prior** | P17.S3 shipped Moments agency Esc/Enter/focus continuity |
| **Sequence** | A1→S1 failures · A2→S2 success · A3→S3 interaction keys · **A4→(future S4) perceived responsiveness** |
| **Authority inputs** | Spec v2.1 · EES v1 · Product Quality Standard · Interaction Language · Capability Integration Standard |
| **Method** | Repository evidence only |
| **Not in scope** | Runtime performance profiling · architecture redesign · new frameworks · File Provider P17 |

---

## 1. Executive Summary

This audit asks whether Workspace consistently communicates **ownership, progress, interruption, completion, and readiness** during interactive work — not whether code is fast.

| Domain | Perceived responsiveness |
| --- | --- |
| Voice prepare / listen / stop / Soft Send | **Strong** — immediate phase ownership |
| Moments success (S2) / agency keys (S3) | **Strong** for completion & keyboard; weak for mid-IPC language |
| Conversation send → capability / support | **Weak** — received, then silence during real work |
| Busy ownership | **Split** — App vs OperatorRoot; wrong-timed on Conversation |
| Tray / Exit / collapse | **Correct** whisper / explicit Exit |

**Worst fracture:** Conversation turn timing is **inverted**. Owner sees their message (received), then silence while `handleOperatorUtterance` runs capability/support IPC with `busy === false` and composer still live; only after the answer exists does `pushWorkspace` set `busy` and run cosmetic `streamText`. Interaction Language “thinking/working” and Product Quality “Thinking/executing — Already Excellent” are **stale vs code**.

**Single highest-value next slice (not implemented here):**  
**Conversation working-state continuity** — busy + honest working acknowledgment from Send through IPC to idle; light bridge so App Moments busy also locks Conversation. No new status framework.

---

## 2. Workflow Inventory

Legend — Owner knows: **R** received · **W** still working · **F** finished · **I** needs input · **X** can interrupt · **Idle** idle again.

### 2.1 Conversation send

| Dimension | Evidence | Assessment |
| --- | --- | --- |
| Owner action | Enter / Send → `submitUtterance` `OperatorRoot.tsx` | Clear |
| Immediate ack | User bubble appended; draft cleared | **Good (R)** |
| Busy | `setBusy(true)` only inside `pushWorkspace` — **after** `await handleOperatorUtterance` | **Missing during real work** |
| Progress | Empty bubble + `streamText` after reply exists | Cosmetic reveal, not live work |
| Completion | `streaming: false`; busy cleared in `finally` | Good after reveal |
| Failure | S1 Conversation compose | Good |
| Cancel | AbortController aborts **reveal** only on next turn | Weak (X) |
| Owner knows | R yes · **W no during IPC** · F after stream · I via reply · X no · Idle after stream | **Critical** |

### 2.2 Voice start / stop / dictation

| Dimension | Evidence | Assessment |
| --- | --- | --- |
| Start | Preparing… phase; click registers immediately | **Excellent (R/W)** |
| Busy / progress | Mic phases + `aria-busy` | **Excellent** |
| Stop / cancel | Enter/Space stop; Esc cancel (PX1) | **Excellent (X)** |
| Completion | Transcript → composer; Soft Send | **Excellent (F→I)** |
| Failure | Soft retry / Settings path | Good |
| Idle | Phase → idle | Yes |

### 2.3 Moments Save

| Dimension | Evidence | Assessment |
| --- | --- | --- |
| Scope wait | “Checking capture scope…” | Honest (W) |
| Save IPC | `onBusy(true)` → disable fields/buttons | Received via disable |
| Working language | No “Saving…” label change | Weak (W) |
| Progress | None | Missing |
| Completion | “Saved into this place” (S2) | **Good (F)** |
| Failure | Sticky `error.banner` | Chrome, not Conversation |
| Cancel | None mid-IPC | Missing (X) |
| Dual busy | App busy; Conversation still live | **High** |

### 2.4 Moments Restore

| Dimension | Evidence | Assessment |
| --- | --- | --- |
| Preview / approve IPC | `onBusy` around resolve/execute | Disable-only |
| Agency | Approve / Not now; S3 Esc/Enter | Continuity fixed |
| Ambient “Restoring” | `setPresence("restoring")` | **No-op on default App path** (ActiveMoment stub) |
| Completion | “You’re back” (S2) | Good (F) |
| Cancel mid-restore | None | Missing (X) |
| Conversation during App busy | Composer/mic still live | Concurrent surprise |

### 2.5 Capability execution

| Dimension | Evidence | Assessment |
| --- | --- | --- |
| Path | `handleOperatorUtterance` → capability IPC | Single Operator path |
| Ack during IPC | None — busy false; no working bubble | **Critical** |
| Progress events | None in presentation | N/A |
| Completion / failure | Conversation reply then `streamText` | Good content, late feel |
| Interrupt | None | Missing |

### 2.6 Tool / planner execution

| Surface | Feel | Notes |
| --- | --- | --- |
| Default Conversation | Opaque Kernel work | OK if Conversation shows working (it doesn’t today) |
| `OperatorConsole` | “Working…” when busy | Off default gravity path |
| `AssistantPanel` Cancel | Exists off App path | Dual standard vs Moments |
| Pilot | Local “Loading…” | Acceptable satellite |

### 2.7 Long-running / background

| Path | Feel |
| --- | --- |
| Capability / restore / support | Silent wait or disable-only |
| Tray background | Presence only — correct whisper |
| False progress motion | Correctly avoided — but **truthful working text** also missing on gravity path |

### 2.8 Startup

| Path | Evidence | Feel |
| --- | --- | --- |
| Shell bootstrap | Conversation-first | Good |
| Tool bootstrap | “Opening…” until `bootstrapped` | Soft; Conversation already interactive |
| First-run tour | Absent | Low debt |

### 2.9 Shutdown / tray

| Path | Evidence | Feel |
| --- | --- | --- |
| Close window | Collapse to Operator — never Exit | Predictable |
| Exit | Explicit Exit / tray `exit_workspace` | Immediate; no farewell (acceptable) |
| Tray Show | Focus Conversation | Instant — good |

### 2.10 Diagnostics / support bundle

| Dimension | Evidence | Assessment |
| --- | --- | --- |
| Prepared working copy | Intent `reply: "Creating a local support package…"` `intentBridge.ts` | **Discarded** |
| Handler | Awaits `export_support_bundle` then returns final message `intelligence.ts` | Silent during create |
| Completion | “Support package saved… Path: …” | Good (F) |
| Failure | Soft Conversation line | Good |

### 2.11 Installer

| Path | Feel |
| --- | --- |
| NSIS install/uninstall | OS-native; uninstall data prompt — outside in-app busy model |

---

## 3. Responsiveness Assessment

| Dimension | Rating | Notes |
| --- | --- | --- |
| Immediate acknowledgment | Partial | Voice excellent; Conversation user bubble yes; capability work silent |
| Honest wait states | Weak on gravity path | Mic phases honest; Conversation busy wrong-timed |
| Completion clarity | Strong (post S1/S2) | Failures Conversation; Moments success single surface |
| Interruption | Asymmetric | Voice yes; Moments/capability no |
| Idle transition | Partial | Clear after stream/App finally; Owner unsure mid-turn |
| Premium desktop feel | Blocked by Conversation silence | Feels stalled despite functioning |
| Product Gravity | Risk | Dual busy lets Moments and Conversation race |

**Verdict:** Workspace does **not** yet feel consistently responsive. Capture is the gold standard. Conversation capability turns are the clearest remaining fracture.

---

## 4. Busy-State Authority Mapping

```text
SHOULD BE authoritative
─────────────────────────────────────
Conversation busy     → entire Operator turn (send → IPC → settle → idle)
Composer / mic        → locked for that turn
Voice phase chrome    → capture only (local)
Moments App busy      → specialized IPC; should also quiet Conversation
Tray                  → never busy theatre

CURRENTLY COMPETING / WRONG-TIMED
─────────────────────────────────────
OperatorRoot busy  ↔  bound to streamText reveal, not utterance start
App busy           ↔  Moments disable only; Conversation unlocked
Voice aria-busy    ↔  correct local ownership
OperatorConsole Working…  ↔  exists off-path only
```

| Channel | When set | Owner-visible | Locks |
| --- | --- | --- | --- |
| App `busy` | Moments / Home / Pilot IPC | Disable controls — no “Working…” | Moments/Home/Pilot |
| OperatorRoot `busy` | Inside `pushWorkspace` only | Composer + mic disabled; no Working label | Conversation |
| Voice `aria-busy` | Capture phases | Preparing… / Listening… | Mic (correct) |
| `streamText` | After reply string exists | Progressive reveal | Occupies OperatorRoot busy |
| Ambient Restoring | `setPresence` | Dead on default App tree | None |

---

## 5. Progress & Completion Analysis

| Cluster | Progress | Completion | Dup / conflict |
| --- | --- | --- | --- |
| Voice | Phase chrome | Transcript + Soft Send | Cohesive |
| Conversation capability | **None during IPC** | Streamed reply | Ownership without progress → silence |
| Support bundle | Prepared “Creating…” unused | Final path reply | Progress without ownership |
| Moments Save/Restore | Disable only | Inline cards (S2) | Ownership without language |
| Cosmetic stream | Typing animation after fact | Marks “done streaming” | Progress after work — High confusion |
| `data-streaming` | DOM attr | — | Cosmetic (unused CSS) |

---

## 6. Cognitive Load Analysis

| Workflow | Decisions / attention | Removable load? |
| --- | --- | --- |
| Voice → Send | One next action (Soft Send) | Already minimized |
| Capability ask | “Did it hear me? Is it stuck?” during silence | **Remove silence** |
| Support package | Same + discarded Creating… | **Surface working line** |
| Restore approve | Agency correct; busy unclear | Label / dual-busy bridge |
| Double-send risk | Composer live during IPC | **Lock busy earlier** |

**Product Gravity risk:** When Conversation looks idle during real Operator work, Owners re-send, switch tools, or assume failure — trust tax every session.

---

## 7. Root Cause Analysis

| ID | Root cause | Severity |
| --- | --- | --- |
| **RC1** | OperatorRoot `busy` / working bubble bound to **reply reveal**, not **utterance start → IPC → settle** | **Critical** |
| **RC2** | App `busy` and OperatorRoot `busy` never coordinated (A3 Track A, still open) | **High** |
| **RC3** | Prepared working copy (support “Creating…”) unused; no generic Conversation working line | **High** |
| **RC4** | Moments progress = `disabled` only; ambient Restoring dead without provider | **High** |
| **RC5** | Mid-op cancel absent outside voice | **High** (larger) |
| **RC6** | `streamText` implies liveness after work is done | **Medium** |
| **RC7** | PQ / Interaction Language overclaim “Thinking/executing Already Excellent” | **Medium** (docs drift) |
| **RC8** | Voice / S2 / S3 / tray / collapse≠Exit | — | **Preserve** |

---

## 8. Ranked Improvement Opportunities

| Rank | Opportunity | Class | Lift |
| --- | --- | --- | --- |
| 1 | **Conversation working-state continuity** — busy + working ack from Send through IPC to idle; light App→Conversation busy bridge | **Implement Now (next)** | Highest |
| 2 | Surface support “Creating…” before `export_support_bundle` | With #1 | High |
| 3 | Moments “Saving…” / “Restoring…” label while App busy | Track A | Med |
| 4 | Soften or shorten post-hoc `streamText` once busy owns real work | Track A | Med |
| 5 | Owner Cancel for Conversation turn / Moments IPC | Track A | High effort |
| 6 | Remount ActiveMoment ambient restoring on App path | Structural | High |
| 7 | Align PQ “Thinking/executing” row with reality | Docs with #1 | Low |
| 8 | New busy framework / progress % / toast redesign | — | **Reject** |

---

## 9. Estimated Engineering Effort

| Slice | Effort | Risk | Scope |
| --- | --- | --- | --- |
| Conversation working-state continuity (#1+#2) | **S–M** | Low–Med | `OperatorRoot`, optional App busy prop; support path; small verifier |
| Moments busy labels | S | Low | Save/Resume panels |
| streamText tempering | S | Taste | `intentBridge` / OperatorRoot |
| Mid-op cancel | M | Med | Abort plumbing + UX |
| Full busy design system | L | High | **Out of scope** |

---

## 10. Single Highest-Value Executable Vertical Slice

### Conversation Working-State Continuity

**Problem:** After Send, Owner knows the message was received but not that Workspace is still working. `busy` and streaming start only when the answer is already known. Dual busy leaves Moments and Conversation on separate clocks. Support’s honest “Creating…” line never appears.

**Outcome:** From Send until idle, Owner always knows: received → still working → finished / needs input → idle. Composer/mic locked for the **whole** turn. Moments App busy no longer leaves Conversation fully live.

**In scope:**
1. Set OperatorRoot `busy=true` at start of `submitUtterance` / `handleIntent` (before `await handleOperatorUtterance`).  
2. Immediate honest working surface in Conversation (working bubble and/or short truthful line; for support, surface existing “Creating a local support package…” before IPC).  
3. Keep busy through capability/support IPC and reply settle; clear only in final `finally`.  
4. Light dual-busy bridge: when App `setBusy(true)`, also disable Conversation composer/mic (shared prop/callback) — no new status framework.  
5. Focused verifier: busy asserted around capability/support path; support “Creating…” not discarded; voice F10 / Soft Send / S3 keys / S2 success cards unchanged.  
6. Optional Interaction Language / PQ one-liner: busy owns the full Conversation turn, not only reveal.

**Out of scope:** Modal framework; mid-IPC abort plumbing; ActiveMoment remount; toast redesign; planner UI; Spec reopen; fake %-progress bars.

**Why this over Moments “Saving…” labels alone:** Fixes the gravity path every capability uses; repairs inverted acknowledgment; absorbs A3 dual-busy for the Conversation side; smallest complete feel win without new frameworks.

**Success test:** Ask for a capability or support package — composer locks immediately; Conversation shows working; no second Send until idle; reply then settles. Approve restore while typing should not race a live composer (bridge).

---

## Explicit answers

| Question | Answer |
| --- | --- |
| Consistently communicates ownership/progress/idle? | **Not yet** — voice yes; Conversation capability turns no |
| Highest fracture after S3? | **Inverted Conversation busy** + dual busy channels |
| Highest-leverage next slice? | **Conversation Working-State Continuity** (§10) |
| Performance audit? | **No** — perceived responsiveness only |
| New frameworks? | **Reject** |

---

## Stop

Audit complete. **No code implemented.**  
Await Product Owner review before **P17.S4**.
