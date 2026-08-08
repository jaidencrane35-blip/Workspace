# P17.A2 — Workspace State Surface Audit

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Workspace State Surface Audit |
| **ID** | P17.A2 *(audit naming only — not File Provider P17)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Read-only product quality audit — **no implementation** |
| **Prior** | P17.S1 unified Conversation for **capability failures** |
| **Authority inputs** | Spec v2.1 · EES v1 · Product Quality Standard · Interaction Language · Capability Integration Standard |
| **Method** | Repository evidence only |

---

## 1. Executive Summary

P17.S1 made Conversation the Owner-facing authority for **capability failures**. Transient **state** (progress, success, attention, passive info) is still split across competing surfaces.

| Authority | Role today | Cohesion |
| --- | --- | --- |
| **Conversation** | Capability outcomes, support bundle, voice guidance, shell replies | Gold path |
| **Composer chrome** | Mic phases, Soft Send (`data-voice-ready`) | Calm, cohesive |
| **Specialized `ws-toast`** | Save / Resume / Pilot / Home ok+error banners | **Competes** with product voice |
| **Inline Moments cards** | Preview, Approve, “You’re back”, empty states | Agency-correct; sometimes duplicated by toast |
| **OS** | Desktop notifications, tray (quiet), installer | Dual with Conversation for notify by design |

**Worst fracture:** Moments restore success emits both a calm card (`You’re back`) and a chrome toast (`Resume finished: …` with engineering outcomes). Save success similarly duplexes toast + inline place copy.

**Single highest-value next slice (not implemented here):**  
**Moments success cohesion** — one acknowledgment surface for Save/Restore success; retire ok-banner dual channel on those paths. No architecture redesign.

---

## 2. State Surface Inventory

| ID | Surface | Purpose | Authority | When | Duration | Dismiss | Animation | Interaction | Hierarchy | Language sample | Dup Conversation? | Must ack? | Interrupts? | Severity |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T1 | Conversation stream | Product replies / status | Conversation | Every Operator turn | Sticky in log | New turn / scroll | Progressive reveal (~streamText) | Read | Primary | First-person compose | — | No | Soft (`busy`) | — |
| T2 | Conversation empty | Waiting | Conversation | No messages | Until first turn | — | None | — | Under-specified blank | *(none)* | — | No | No | Medium |
| T3 | Composer `busy` | Lock input during turn | Chrome | During stream | Until settle | Auto | None | Disabled controls | Secondary | — | No | No | Yes | Low |
| T4 | Mic phase chrome | Capture state | Composer | Dictation | Phase-bound | Phase / Esc | Pulse / wave | Mic / keys | Local | Preparing… / Listening… | Intentionally not (PX4) | No | Local | None |
| T5 | Soft Send voice-ready | Next action after voice | Composer | Post-transcript | Until Send/clear | Auto clear | Soft pulse ×2 | Optional click | Local | Send reviewed words | No | No | No | None |
| T6 | `ok.banner` ws-toast | Tool success | Satellite chrome | `onMessage` | Until overwritten | Manual only | Static pill | None | Competes | Saved “…”, Resume finished… | **Yes** (Moments) | No | Soft | **Critical** |
| T7 | `error.banner` ws-toast | Tool failure | Satellite chrome | `onError` | Until overwritten | Manual only | Static | None | Competes | Raw/formatError | Separate (A1/S1) | No | Soft | High* |
| T8 | Resume “You’re back” | Restore done | Moments card | After approve | Until navigate | Navigation | Card | Continue browsing | In-panel | You’re back | Dup’d by T6 | No | No | — |
| T9 | Restore preview / Approve | Agency gate | Moments card | Preview step | Until choose | Not now / Approve | Semantic place | Required for restore | In-panel | Approve and restore | No | **Yes** | Soft | None (correct) |
| T10 | Delete confirm card | Destructive confirm | Moments card | Delete flow | Until choose | Cancel / Delete | Card | Required | In-panel | Delete “…”? | No | **Yes** | Soft | None |
| T11 | Moments empty | Invite save | Moments / Home | No Moments | Sticky | — | Quiet | Speak / UI | In-panel | Nothing to continue yet | Partial | No | No | Low |
| T12 | Save inline place | Place acknowledgment | Save panel | After save | Sticky in panel | — | — | — | In-panel | Saved into this place | Dup’d by T6 | No | No | High |
| T13 | OS desktop notification | External attention | OS + Conversation reply | NL notify | OS lifecycle | OS | OS | Optional click | Outside | Desktop notification shown. | Dual by design | No | OS may | Medium |
| T14 | Tray | Presence | OS tray | Always (E1) | Persistent | Exit | None | Show / Exit | Whisper | Workspace | No | No | No | None |
| T15 | Support bundle reply | Diagnostics complete | Conversation | NL export | Sticky | — | Stream | — | Primary | Support package saved… Path: | No | No | Soft busy | Low–Med |
| T16 | App “Opening…” | Tool bootstrap | Chrome | Pre-mount tools | Until ready | Auto | Muted text | None | Temporary | Opening… | No | No | Soft | Low |
| T17 | Pilot / canvas Loading… | Panel load | Chrome (if mounted) | Load/save | Until settle | Auto | Muted | None | Local | Loading… | No | No | Soft | Low |
| T18 | Ambient Moment presence | Living restore feel | Ambient chrome | Restore presence | Session | — | Lighting | — | Background | — | No | No | No | Low |
| T19 | Installer / uninstall MB | Install lifecycle | OS NSIS | Install/uninstall | Modal | User | Native | Required (uninstall data) | Outside | Remove user data…? | No | Yes (uninstall) | Yes | N/A in-app |
| T20 | Startup / first-run | Welcome | *(absent on default path)* | — | — | — | — | — | — | — | — | — | — | Low debt |
| T21 | Update messages | Updater | *(absent — B2 blocked)* | — | — | — | — | — | — | — | — | — | — | Future |
| T22 | Cancellation confirm | Cancel dialogs | *(no general modal)* | — | — | — | — | Esc clears draft / mic | — | — | — | — | — | None |

\*Error banners: failure authority still chrome for tools; S1 fixed Conversation capability path only.

**Evidence paths:** `App.tsx` (ws-toast), `OperatorRoot.tsx`, `VoiceMicButton.tsx`, `ResumeContextPanel.tsx`, `SaveContextPanel.tsx`, `tray.rs`, `compose.rs`, `intelligence.ts`, `INSTALLER.md` / `hooks.nsh`.

---

## 3. Authority Mapping

```text
SHOULD BE authoritative
─────────────────────────────────────
Conversation     → capability status/success/failure; support; voice guidance
Composer chrome  → capture progress; one next action (Soft Send)
Moments cards    → restore agency (preview / approve / delete) — not toast
OS notify        → true external attention (with calm Conversation ack)
Tray             → presence only (already correct)

CURRENTLY COMPETING
─────────────────────────────────────
ws-toast ok.banner  ↔  Moments inline success  ↔  (sometimes) Conversation
ws-toast            ↔  Product Gravity when specialized dock open
```

| Kind of information | Correct authority | Today |
| --- | --- | --- |
| Capability outcome | Conversation | Conversation (S1 failures ✓) |
| Voice capture progress | Composer chrome | Composer ✓ |
| Restore approval | Moments card | Card ✓ |
| Restore/Save **success** | One surface (card or Conversation) | **Card + toast** ✗ |
| Support package | Conversation | Conversation ✓ |
| Desktop notify effect | OS + Conversation ack | Dual ✓ (wording Med) |
| Tray | Passive | Quiet ✓ |

---

## 4. Duplication Analysis

| Cluster | Surfaces | Conflict |
| --- | --- | --- |
| **D1 Moments restore success** | T8 “You’re back” + T6 “Resume finished: …” | Tone + priority conflict — **Critical** |
| **D2 Moments save success** | T12 inline + T6 ``Saved “…”`` | Dual acknowledgment — **High** |
| **D3 Presentation helpers** | Multiple `formatError` / `onMessage` pipes | Same pattern, no shared status contract — **High** |
| **D4 Notify dual** | OS toast + Conversation “Desktop notification shown.” | Acceptable dual; system-tone reply — **Medium** |
| **D5 Busy locks** | Conversation busy / App tool busy / mic aria-busy | Three locks, no shared progress language — **Medium** |
| **D6 Capability success voice** | “Copied.” vs “Desktop notification shown.” vs first-person fails | Mild Conversation-internal drift — **Low–Med** |

---

## 5. Cognitive Load Analysis

| Workflow | Decisions / attention switches | Removable? |
| --- | --- | --- |
| Voice → Send | Mic chrome + Soft Send | Already minimized (PX1–PX4) |
| Capability ask | One Conversation reply | Good |
| Restore approve | Preview → Approve (correct agency) | Keep |
| Restore **done** | Card narrative **and** toast | **Remove toast** |
| Save done | Inline **and** toast | **Remove one** |
| Notify | OS + Conversation | Keep OS; soften Conversation |

**Product Gravity risk:** When the specialized dock is open, ok-banners pull eyes away from Conversation for information Conversation already (or the Moments card already) owns.

---

## 6. Product Cohesion Assessment

| Dimension | Rating | Notes |
| --- | --- | --- |
| Personality | Partial | Calm operator in Conversation/mic; admin chrome on Moments success |
| Calm computing | Partial | Soft Send / tray quiet; toast permanence is noisy |
| Consistency | Weak on success | Failures improved (S1); successes still duplex |
| Premium feel | Blocked by D1/D2 | Engineering “Resume finished” breaks “You’re back” |
| Interaction Language | Mic/Send yes; toast no | |
| Premium benchmark (Raycast / ChatGPT / Notion) | Behind on single-success acknowledgment | |

**Verdict:** Workspace does **not** yet present one unified product for transient state. Capture and Conversation are close; Moments success chrome is the clearest remaining cohesion break.

---

## 7. Root Cause Analysis

| ID | Root cause | Severity |
| --- | --- | --- |
| **RC1** | Specialized tools predate Product Gravity; status still defaults to `onMessage` → `ok.banner` | Critical |
| **RC2** | Restore done emits narrative card **and** engineering toast | Critical |
| **RC3** | No shared non-error status presentation contract | High |
| **RC4** | Conversation empty / stream under-specified vs rich mic motion | Medium |
| **RC5** | Capability success copy uneven (third-person vs first-person) | Low–Med |
| **RC6** | Tray / Soft Send / Approve cards already correct — preserve | Already Excellent |

---

## 8. Ranked Improvement Opportunities

| Rank | Opportunity | Class | Lift |
| --- | --- | --- | --- |
| 1 | **Moments success cohesion** — single Save/Restore success surface; drop ok-banner on those paths | **Implement Now (next)** | Highest |
| 2 | Conversation-tone restore outcome (never snake_case on Owner surface) | With #1 | High |
| 3 | Soften notification success compose to first-person | Track A | Med |
| 4 | Auto-dismiss or suppress ok.banner when inline ack exists | Track A | Med |
| 5 | Quiet Conversation idle affordance | Track A | Low–Med |
| 6 | Shared muted loading vocabulary | Track A | Low |
| 7 | Leave mic / Soft Send / tray / Approve cards | — | Preserve |
| 8 | New status framework / toast redesign | — | **Reject** |

---

## 9. Estimated Engineering Effort

| Slice | Effort | Risk | Scope |
| --- | --- | --- | --- |
| Moments success cohesion (#1+#2) | **S** | Low | `ResumeContextPanel`, `SaveContextPanel`; optional App toast awareness; small verifier |
| Notify success wording | S | Low | `compose.rs` notifications arm |
| ok.banner auto-dismiss | S | Low | `App.tsx` |
| Conversation idle | S | Taste | `OperatorRoot` empty copy |
| Full status design system | L | High | **Out of scope** |

---

## 10. Single Highest-Value Executable Vertical Slice

### Moments Success Cohesion

**Problem:** Restore/Save success is acknowledged twice with conflicting tone (`You’re back` / place language vs `Resume finished:` / toast chrome), splitting attention when the specialized surface is open.

**Outcome:** One Owner-visible success acknowledgment per Moments save/restore — Conversation-grade calm, no engineering outcome tokens, no competing ok-banner.

**In scope:**
1. Remove `onMessage(\`Resume finished: …\`)` (or equivalent) from restore approve success; keep the `"You’re back"` card as sole specialized ack.  
2. Choose **one** of Save toast vs `"Saved into this place"` (prefer inline place language; drop toast).  
3. Do **not** change Approve/Delete agency cards, error banners, Conversation capability path, tray, or mic.  
4. Add a focused verifier that Save/Restore success paths do not emit `"Resume finished"` / dual-save toast strings.

**Out of scope:** New toast framework, Spec changes, File Provider P17, updater messages, installer UI.

**Why this over idle Conversation copy or notify wording:** Highest Product Gravity collision frequency when Owners use Moments — the core trust workflow — and smallest safe scope.

**Success test:** After Save or Approve restore, Owner sees exactly one calm success signal; no `Resume finished` / snake_case toast.

---

## Explicit answers

| Question | Answer |
| --- | --- |
| One unified product for state? | **Not yet** — capture/Conversation yes; Moments success no |
| Where Conversation should own? | Capability status; support; voice guidance; (optionally) Moments success if not card-owned |
| Where passive UI should own? | Mic progress; Soft Send; tray presence; restore preview/approve |
| Where both compete today? | Moments success (card + ws-toast) |
| Highest-leverage next slice? | **Moments Success Cohesion** (§10) |

---

## Stop

Audit complete. **No code implemented.**  
Await Product Owner review before any implementation program.
