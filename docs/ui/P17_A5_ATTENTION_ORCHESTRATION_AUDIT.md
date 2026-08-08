# P17.A5 — Attention Orchestration Audit

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Attention Orchestration Audit |
| **ID** | P17.A5 *(audit naming only — not File Provider P17)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Read-only product quality audit — **no implementation** |
| **Prior** | P17.S4 Conversation working-state continuity shipped |
| **Sequence** | A1→S1 · A2→S2 · A3→S3 · A4→S4 · **A5→(future S5) attention** |
| **Authority inputs** | Spec v2.1 · EES v1 · Product Quality Standard · Interaction Language · Product Gravity · Capability Integration Standard |
| **Method** | Repository evidence on default App → `OperatorRoot` path |

---

## 1. Executive Summary

S1–S4 improved what Conversation and Moments **say** (failures, success, keys, working). Attention orchestration asks what **pulls the eyes**.

| Domain | Attention cohesion |
| --- | --- |
| Conversation stream + working ack (S4) | Strong primary when dock idle |
| Voice / Soft Send | Strong local cues; no longer race working (S4) |
| Moments Save/Restore **success** cards (S2) | Correct sole success surfaces |
| Agency Approve / Delete (S3) | Earned attention + keyboard continuity |
| Specialized `ws-toast` (ok/error) | **Competes** — sticky, no dismiss, no TTL |
| Delete ok-banner (S2 left open) | **Competes** — can outlive later Moments steps |
| Tool dock layout | **Competes** — satellite column larger than Conversation |
| Tray / Collapse / Exit | Correct whisper |

**Worst fracture:** Sticky specialized status chrome (`ok.banner` / `error.banner`) remains a second primary above Moments, with Delete success still on that channel and never cleared when Resume continues. Conversation is demoted by geometry when the dock is open; leftover toast can sit beside “You’re back.”

**Single highest-value next slice (not implemented here):**  
**Moments status chrome quieting** — inline Delete ack (S2 pattern), clear/auto-dismiss sticky banners, calm Owner-language restore outcome on You’re back. No toast framework redesign. No dock grid rewrite in this slice.

---

## 2. Attention Surface Inventory

Legend: **P** primary · **S** secondary · **W** whisper.

| ID | Surface | Owner | Hierarchy | Motion | Timing | Focus | Dismiss | Persistence | Competes? |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T1 | Conversation stream | Conversation **P** | Hero (dock off) | `streamText` reveal | After IPC | Live region | Next turn | Sticky in log | — |
| T2 | Working ack | Conversation **P** | Correct wait | Streaming attr | Send→idle (S4) | Composer locked | Replaced by reply | Turn | Preserve |
| T3 | Composer | Composer **P** when typing | Transcript→composer→action | Soft Send pulse | Voice review | Focus after voice | Esc clears | Draft | Preserve |
| T4 | Soft Send | One next action | Local | Pulse ×2 | Until Send/clear | — | Auto | Ephemeral | Preserve |
| T5 | Voice mic phases | Capture **P** | Local | Pulse / wave | Phase-bound | Mic keys | Esc/stop | Phase | Preserve |
| T6 | Empty Conversation | Under-owned | Blank | None | Until first turn | Composer | — | Sticky blank | Medium |
| T7 | Tool dock | Moments/Health **P** when open | Dual primary vs Conversation | Panel | Until close/collapse | Agency→composer | Close health | Session | **High** |
| T8 | `ok.banner` ws-toast | Satellite **P** | Pill above tool | Static green | `onMessage` | Unchanged | **None** | Until overwrite | **Critical** |
| T9 | `error.banner` ws-toast | Satellite **P** | Pill; beats ok | Static red | `onError` | Unchanged | Next clear only | Sticky | **High** |
| T10 | Delete ok-toast | Toast **P** | Same T8 | Static | After delete | — | **Never cleared by Resume** | Session-sticky | **Critical** |
| T11 | You’re back | Moments card **P** | Sole success (S2) | Card | Until navigate | Buttons | Back | Panel | Tone leak |
| T12 | Outcome line on done | Engineering | Undermines T11 | Bold | With done | — | — | With card | **High** |
| T13 | Restore preview / Approve | Agency **P** | Correct gate | Staggered panes | Until choose | Card (S3) | Esc / Not now | Step | Medium motion |
| T14 | Delete confirm | Agency **P** | Correct | Card | Until choose | Card (S3) | Esc / Cancel | Step | — |
| T15 | Saved into this place | Save panel **P** | Sole ack (S2) | place pulse | Until Save another | — | Start again | Panel | — |
| T16 | Resume browse empty | Often **false** | Ghost orbit | EmptyStructure motion | When `!primary` | — | — | Sticky wrong | **High** |
| T17 | Home Moments list | Home tool | List in dock | — | When home | — | — | — | Split from Resume |
| T18 | Opening… | Bootstrap **W** | Temporary | Muted | Pre-bootstrap | — | Auto | Until ready | Low |
| T19 | Pilot ok-banners | Toast **P** | Same T8 | Static | Pilot actions | — | None | Sticky | Medium |
| T20 | OS notification | OS **P** | Dual by design | OS | NL notify | OS | OS | OS | Medium |
| T21 | Notify Conversation reply | Conversation | “Desktop notification shown.” | Reveal | After show | — | — | Log | Medium |
| T22 | Tray | Presence **W** | Whisper | None | Always | — | Exit | Persistent | Preserve |
| T23 | Support Creating… / path | Conversation **P** | Working then final | S4 | Turn | Locked | — | Log | Low–Med |
| T24 | Health dock | Dev diagnostics | Satellite | — | Dev / NL health | Close | Close | Until close | Low (gated) |
| T25 | Ambient Moment lighting | Intended **W** | Background | Spot | Restore/write | — | — | Session | Dead on App path |
| T26 | Desktop Operator Form A | Companion **P** | Collapse whisper | — | Mode 0 | Click→Conversation | — | — | Preserve |
| T27 | Chrome Collapse / Exit | Whisper | Chrome | Dev-dot if developer | Always Form B | — | — | — | Low |
| T28 | Create-workspace ok-toast | Toast | Into Save | Static | Create | — | None | Sticky | Medium |

---

## 3. Attention Authority Mapping

```text
SHOULD OWN ATTENTION
─────────────────────────────────────
Conversation transcript     → default primary (Product Gravity)
Composer + one next action  → when Owner is speaking/typing/reviewing
Agency cards (Approve/Delete) → when protecting the desktop (earned)
Working ack                 → during Operator turn (S4)
Voice phase chrome          → during capture only
OS notification             → true external attention (with calm Conversation ack)
Tray / Collapse / Exit      → whisper only

MUST NOT OWN ATTENTION BY DEFAULT
─────────────────────────────────────
Sticky ws-toast ok/error    → ephemeral or inline; never permanent second primary
Engineering outcome tokens  → never on Owner success cards
False empty Moments theatre → never louder than truth
Dock geometry               → satellite must not outshine transcript

CURRENT COMPETITION
─────────────────────────────────────
ws-toast  ↔  Moments cards  ↔  Conversation column
Delete toast (sticky)  ↔  later preview / You’re back
You’re back title  ↔  Outcome: snake_case line
```

| Kind of signal | Correct authority | Today |
| --- | --- | --- |
| Capability working / outcome | Conversation | Conversation (S1/S4) ✓ |
| Voice capture progress | Mic chrome | Mic ✓ |
| Restore agency | Moments card | Card ✓ |
| Save/Restore **success** | One card | Card ✓ (S2) |
| Delete success | Moments place / browse | **Toast** ✗ |
| Tool errors | Calm ephemeral / Conversation tone | Sticky red banner ✗ |
| Tray presence | Whisper | Whisper ✓ |

---

## 4. Visual Priority Analysis

### Hierarchy
- Dock **off**: Conversation owns attention — Product Gravity OK.
- Dock **on**: CSS gives specialized column the larger share (`minmax(280px, 340px) 1fr` pattern) — satellite becomes visual hero; transcript is squeezed.
- Eyes path becomes: **toast → Moments card → squeezed transcript** — violates Interaction Language “transcript → composer → one obvious control.”

### Motion
| Motion | When | Compete? |
| --- | --- | --- |
| Soft Send pulse | Voice review | No (cleared on Send) |
| Mic pulse/wave | Capture | No while `composerBusy` |
| Working ack | Turn | Correct |
| Preview stagger + quality-dot | Agency | Local; heavy |
| Ghost empty orbit | Often false empty | Unnecessary |
| `streamText` chunks | After reply known | Mild post-hoc theatre |
| Ambient lighting | Shell path only | Dead on App |

### Timing / persistence
| Signal | Timing quality | Persistence quality |
| --- | --- | --- |
| Working ack | Correct (S4) | Ephemeral ✓ |
| Soft Send | Correct | Ephemeral ✓ |
| Agency cards | Earned | Step-bound ✓ |
| You’re back / Saved place | Correct | Panel sticky OK |
| ok/error ws-toast | On event | **Sticky; no TTL; no dismiss** ✗ |
| Delete toast | On delete | **Survives later Resume steps** ✗ |

### Focus
- S3 agency focus in/out: good.
- Toasts never steal keyboard focus — good — but still **visual**-steal.
- Composer locked during `composerBusy` — correct.

---

## 5. Cognitive Load Analysis

| Workflow | What Owner must decide | Removable load? |
| --- | --- | --- |
| Voice → Send | One Soft Send cue | Already minimized |
| Capability ask | Working → reply | Good (S4) |
| Delete Moment | Confirm card + green toast | **Remove toast** |
| Approve restore after delete | You’re back + leftover Delete toast? | **Clear toast** |
| Restore done | Calm title + engineering outcome | **Calm outcome language** |
| Open Moments dock | Watch satellite or Conversation? | Later (layout) |
| Resume browse empty vs Home list | Which is truth? | Later (ActiveMoment) |

**Owner knowledge gaps today**

| Question | Answer quality |
| --- | --- |
| What deserves attention now? | Unclear when toast + card + dock compete |
| What can safely be ignored? | Sticky toast implies it cannot |
| What action is expected? | Agency cards clear; leftover toast confuses |
| What is merely informative? | Toast presents as primary status |

---

## 6. Root Cause Analysis

| ID | Root cause | Severity |
| --- | --- | --- |
| **RC1** | Specialized tools still default status to sticky `onMessage`/`onError` → `ws-toast` with no ephemeral contract | **Critical** |
| **RC2** | Delete success left on toast channel after S2; Resume never clears `onMessage` on later steps | **Critical** |
| **RC3** | Restore done exposes raw `OperationOutcome` via `replace(/_/g," ")` instead of Owner language | **High** |
| **RC4** | Earned satellite layout permanently demotes Conversation column | **High** |
| **RC5** | Default App tree omits ActiveMoment/Ambient providers — false empty Moments browse | **High** |
| **RC6** | Cosmetic `streamText` still implies liveness after working ack owns the wait | Medium |
| **RC7** | Capability notify success copy still system-tone | Medium |
| **RC8** | Conversation idle empty under-specified | Medium |
| **RC9** | Preview quality-dot / ghost empty residual neon | Low–Cosmetic |
| **RC10** | Voice / Soft Send / tray / S2 cards / S3 keys / S4 working | **Preserve** |

---

## 7. Ranked Improvement Opportunities

| Rank | Opportunity | Class | Lift |
| --- | --- | --- | --- |
| 1 | **Moments status chrome quieting** — inline Delete ack; clear/auto-dismiss sticky banners; calm You’re back outcome | **Implement Now (next)** | Highest |
| 2 | Clear `onMessage` whenever Resume/Save starts a new action (mirror `onError(null)`) | With #1 | High |
| 3 | Conversation-still-primary dock visual contract (without full layout redesign) | Track A | High |
| 4 | ActiveMoment / truthful Moments browse on App path | Structural | High |
| 5 | Soften/shorten post-hoc `streamText` | Track A | Med |
| 6 | Notify success first-person wording | Track A | Med |
| 7 | Conversation idle affordance | Track A | Low–Med |
| 8 | New toast design system / attention framework | — | **Reject** |

---

## 8. Estimated Engineering Effort

| Slice | Effort | Risk | Scope |
| --- | --- | --- | --- |
| Moments status chrome quieting (#1+#2) | **S** | Low | `ResumeContextPanel`, optional `App.tsx` dismiss/TTL; outcome map; small verifier |
| Dock gravity visual contract | M | Taste | CSS / OperatorRoot |
| ActiveMoment on App path | M–L | Med | Provider mount |
| streamText tempering | S | Taste | OperatorRoot / intentBridge |
| Full attention framework | L | High | **Out of scope** |

---

## 9. Single Highest-Value Executable Vertical Slice

### Moments Status Chrome Quieting

**Problem:** After S2, Save/Restore success no longer duplex toast + card, but Delete still uses sticky `ok.banner`, tool errors stick without dismiss, and You’re back still shows engineering outcome tokens. Sticky chrome becomes a second primary beside Moments cards and Conversation — the highest remaining Product Gravity collision that is still Critical and small enough for one slice.

**Outcome:** One calm acknowledgment per Moments path; specialized status is ephemeral or inline; Owner always knows the card (or Conversation) owns attention — not a permanent green/red pill.

**In scope:**
1. Delete success → **inline Moments ack** (browse/place line); remove `onMessage(\`Deleted…\`)` ok-banner (S2 pattern).  
2. Clear `message` whenever Resume/Save starts a new action (mirror `onError(null)`), and/or auto-dismiss ok.banner (~4–6s) + optional dismiss — no new framework.  
3. Calm restore outcome on You’re back: map outcomes to Owner phrases (`Restored` / `Mostly restored` / `Couldn’t restore`…) — no snake_case tokens.  
4. Focused verifier: no Delete ok-banner string; no sticky dual with done card; outcome mapping covered.  
5. Do **not** change Voice/Soft Send/S4 working, S3 keys, dock grid, ActiveMoment remount, or Spec.

**Out of scope:** Toast design system, dock layout redesign, streamText rewrite, notify OS dual, File Provider P17.

**Why this over dock geometry or ActiveMoment remount:** Highest Critical frequency on earned Moments workflows; completes S2’s unfinished edge; S effort; no architecture redesign. RC4/RC5 remain Track A.

**Success test:** Delete a Moment → one calm inline signal, no green toast. Later Approve restore → You’re back alone with Owner-language outcome, no leftover Delete toast. Error banner clears when the next Moments action starts — never a permanent second primary.

---

## Explicit answers

| Question | Answer |
| --- | --- |
| Consistently directs attention to the most important interaction? | **Not yet** when Moments dock + sticky chrome are active |
| Multiple primary attention owners? | **Yes** — toast + Moments card + Conversation column |
| Soft Send / voice vs working? | **Resolved (S4)** — preserve |
| Highest-leverage next slice? | **Moments Status Chrome Quieting** (§9) |
| New frameworks? | **Reject** |

---

## Stop

Audit complete. **No code implemented.**  
Await Product Owner review before **P17.S5**.
