# P18.A1 — Product Proof Regression Audit

| Field | Value |
| --- | --- |
| **Program** | Product Proof Regression Audit |
| **ID** | P18.A1 *(audit naming — not File Provider / Terminal roadmap IDs)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Read-only Product Proof audit — **no implementation** |
| **Prior** | P17 cohesion program complete (S1–S5) |
| **Lens** | Production-candidate Owner experience — ignore implementation history |
| **Authority inputs** | Spec v2.1 · EES v1 · Product Quality Standard · Interaction Language · Capability Integration Standard · Product Gravity · P16.O1 · PR1 |
| **Method** | Repository evidence on default App → OperatorRoot path |

**Naming clarity:** Constitutional **File Provider P17** remains blocked until P16 Owner Accept. The completed **P17.A\*/S\*** series was a separate UI cohesion program. This audit sets **P18 product direction** and must not default to extending P17 polish.

---

## 1. Executive Summary

P17 made Workspace **speak coherently** — failures, success, keys, working state, and Moments status ownership. Judged as a production candidate, the greatest remaining Owner friction is not another notification or Conversation tweak.

**#1 remaining source of distrust:** Moments **browse truth** on the default App path. Continue can show *“Nothing to continue yet”* while Home lists real Moments. That is a structural product lie: the Owner cannot trust what Workspace says about their saved places.

Secondary open gates (Owner-owned, not polish):
- P16 Voice live Product Proof still pending Accept
- Release trust: unsigned installer / no updater (`productionReadyToday: false`)
- Dock geometry still demotes Conversation when Moments is open

**Recommended P18 direction:** Structural Moments truth on the App path — not more cohesion polish, not D1-first by default.

---

## 2. Product Proof Scorecard

| Area | Score | Notes |
| --- | --- | --- |
| Conversation as product (gravity when dock idle) | **Strong** | Transcript / composer hero |
| Capability failure honesty | **Strong** | P17.S1 |
| Working-state continuity | **Strong** | P17.S4 |
| Voice capture → review → Send (engineering) | **Strong** | PX1–PX4; F10 intact |
| Voice live Owner Product Proof | **Weak / Unknown** | Pending Accept |
| Moments agency (Approve / Delete) | **Strong** | P17.S3 |
| Moments success / status ownership | **Strong** | P17.S2 + S5 |
| Moments browse / ambient truth | **Weak** | ActiveMoment stub on App path |
| Dock vs Conversation hierarchy | **Weak** | Satellite column visually primary |
| Discoverability (empty Conversation) | **Weak** | Blank waiting; no first-session cue |
| First launch / onboarding | **Weak** | No tour by design; vacuum remains |
| Tray / single-instance (engineering) | **Strong** | Eng complete; OA pending |
| Installer / checksums | **Partial** | Foundation + A1; unsigned |
| Signing / updater | **Weak** | Externally blocked |
| Diagnostics / support bundle | **Partial → Strong** | Conversation export path |
| Settings / corrupt-session honesty | **Partial** | C1 ReadyNow |
| Keyboard continuity (Moments agency) | **Strong** | P17.S3 |
| Accessibility (beyond agency) | **Partial** | Labels / reduced-motion; no program |
| Responsiveness (Conversation turns) | **Strong** | P17.S4 |
| Attention / sticky chrome (Moments) | **Strong** | P17.S5 |
| Recovery / interruptions | **Partial** | Voice interruptible; Moments IPC wait-only |
| Premium desktop feel (daily) | **Partial** | Calm Conversation; Moments lie + dock + unsigned |
| Production release confidence | **Weak** | Not public-release ready |

---

## 3. Areas That Now Feel Complete

| Program | Owner-visible closure |
| --- | --- |
| **P17.S1** | Capability failures speak one Conversation voice |
| **P17.S2** | Save / Restore success = one Moments acknowledgment |
| **P17.S3** | Approve / Delete share Esc / Enter / focus with Conversation |
| **P17.S4** | Working acknowledgment from Send through real IPC |
| **P17.S5** | Delete / restore status owned by Moments; banners dismissible / ephemeral |
| **Voice eng (P16)** | Capture / Soft Send / F10 ready for live Owner Proof |
| **Production eng slices** | Single-instance, tray Show/Exit, support bundle, installer foundation, checksums |

Status language and interaction continuity on earned Moments paths are largely done. Remaining fractures are **truth**, **layout gravity**, **discoverability**, and **release trust**.

---

## 4. Areas Still Below Premium Quality

1. **Continue browse vs Home list** — Home lists Moments; Continue can deny they exist.  
2. **Dock geometry** — Moments column outranks Conversation when dock is open.  
3. **Empty Conversation** — No calm first-turn cue; discoverability assumes prior knowledge.  
4. **P16 Voice Owner Accept** — Engineering green ≠ Product Complete.  
5. **Unsigned installer / no updater** — Not finished software for distribution.  
6. **IPC surface size** — Security/trust debt; low daily feel.  
7. **Corrupt-session honesty (C1)** — Rare but trust-critical.  
8. **Accessibility program** — Beyond Moments agency keys.  
9. **Mid-operation cancel** outside voice — Asymmetric interruption model.

---

## 5. Root Cause Analysis

| ID | Root cause | Severity |
| --- | --- | --- |
| **RC-A** | Dual presentation trees: App path stubs ActiveMoment (`primary: null`); Resume browse treats stub as empty while Home uses `list_saved_contexts` | **Critical** |
| **RC-B** | Satellite dock layout predates Product Gravity — geometry demotes Conversation | **High** |
| **RC-C** | Empty product surface intentionally under-specified — creates discoverability vacuum | **High** |
| **RC-D** | Production trust unfinished (signing / updater / OA); `productionReadyToday: false` | **High** (release) |
| **RC-E** | P16 Voice Product Proof still Owner-gated — constitutional sequencing unchanged | **Critical** (gate) |
| **RC-F** | Specialized status residual for non-Moments tools — diminished after S5 | Medium |
| **RC-G** | Cohesion of Conversation / Moments status language | **Resolved (P17)** — preserve |

---

## 6. Ranked Engineering Opportunities

| Rank | Opportunity | Class | Lift |
| --- | --- | --- | --- |
| 1 | **Truthful Moments browse on App path** | **P18 Implement Now** | Highest daily trust |
| 2 | Owner live Voice Product Proof (Owner session) | Gate / parallel | Unblocks File Provider sequencing |
| 3 | Conversation-still-primary dock visual contract | Track A | High when Moments earned |
| 4 | Calm empty-Conversation first-session cue | Track A | First-hour discoverability |
| 5 | Signing → updater when certificate exists | Production | Highest *release* trust |
| 6 | C1 corrupt-session honesty | Production ReadyNow | Rare trust |
| 7 | D1 IPC quarantine | Production `nextReadyNow` | Security; low daily feel |
| 8 | Accessibility program / visual Stop-Cancel | Track A | Med |
| 9 | More Conversation / toast / status polish | — | **Reject as P18 default** |

---

## 7. Estimated Engineering Effort

| Slice | Effort | Risk |
| --- | --- | --- |
| Moments browse truth (list ownership or minimal ActiveMoment on App) | **M** | Med — preserve S3 inline preview |
| Dock gravity CSS contract | **S–M** | Taste |
| Empty Conversation calm cue | **S** | Low |
| C1 corrupt-session UX | **S–M** | Low |
| D1 IPC quarantine | **M–L** | Med regression |
| A2 signing / B2 updater | **L + external** | Process / dependency |
| Voice PP session | Owner time | — |

---

## 8. Estimated User Impact

| Opportunity | Trust | Daily usability | Cognitive load | Premium feel |
| --- | --- | --- | --- | --- |
| Moments browse truth | **Critical** | High | Removes contradiction | High |
| Voice Owner Accept | Critical (gate) | Unlocks confidence | Clarifies Product Complete | High |
| Dock gravity | High | Medium | Less attention split | High |
| Empty Conversation cue | Medium–High | First hour | Learnability | Medium |
| Signing / updater | Release-critical | Low daily until install | — | “Finished software” |
| D1 IPC | Security | Low visible | — | Indirect |

---

## 9. Recommended Next Product Program

### P18 — Structural Moments Truth (App-path Continuity)

**Direction:** Make Continue tell the same Moments truth as Home on the shipping App tree.

**Not** more P17-style Conversation / toast / status polish.  
**Not** File Provider (blocked until P16 Accept).  
**Not** D1-first unless Owner prioritizes invisible security over daily trust.

**Why this over production D1 / signing as the product program:**
- Signing is externally blocked; D1 is largely invisible in daily Owner feel.
- False-empty Continue is a **trust** defect every session Moments exist.
- Home already proves the data path; Continue fails presentation ownership — fixable without Spec reopen.
- Completes the product story P17 polished: Moments now speak well but can still **deny their own existence**.

**Parallel Owner track (non-engineering):** Run P16 Voice live Product Proof under the O2 package. That remains the constitutional gate for File Provider and for declaring Voice Product Complete.

**Production track (Owner-scheduled):** Keep A2/B2 release-critical when a certificate exists; D1/C1 as ReadyNow units — alternate with product truth work rather than displacing it by default.

---

## 10. Single Highest-Value Executable Vertical Slice

### P18.S1 — Moments Browse Truth on App Path

**Problem:** On the default App tree, `ResumeContextPanel` treats ActiveMoment stub `primary === null` as “no Moments,” while `HomeWorkspacePanel` lists Moments via `list_saved_contexts`. The Owner can see Moments on Home, open Continue, and be told nothing is there.

**Outcome:** Continue browse shows the same Moments truth as Home (or an honest empty only when the list is empty). No false EmptyStructure theatre.

**In scope:**
1. Authoritative Moments list for Resume browse on the App path (minimal ActiveMoment mount, or local list ownership aligned with Home).  
2. Preserve P17.S3 agency keyboard + inline preview when expand host is absent.  
3. Preserve P17.S2 / S5 success and status ownership.  
4. Focused verifier: non-empty Home list ⇒ Continue must not render “Nothing to continue yet”; empty list ⇒ honest empty.  
5. Do **not** redesign dock grid, toast system, Voice, File Provider, or Spec.

**Out of scope:** Full AmbientLighting / CognitiveEngine productization; dock rewrite; D1; signing; Spec reopen.

**Success test:** Save a Moment → Home shows it → open Continue without a focused id → Moment is visible and restorable; delete/restore agency remains keyboard-continuous.

---

## Explicit answers

| Question | Answer |
| --- | --- |
| Extend P17 polish by default? | **No** |
| Production candidate for public release? | **No** |
| Ready for Owner live Voice PP? | **Yes** (eng package) |
| Greatest Owner friction after P17? | **False-empty Moments Continue** |
| P18 program direction? | **Structural Moments Truth** |
| Highest-value slice? | **P18.S1 Moments Browse Truth** (§10) |
| Constitutional File Provider? | Still blocked on P16 Owner Accept |

---

## Stop

Audit complete. **No code implemented.**  
Await Product Owner direction for **P18.S1** vs Voice Product Proof session vs production gate scheduling.
