# P19.A1 — Production Product Proof Audit

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Production Product Proof Audit |
| **ID** | P19.A1 *(audit naming — not File Provider / Terminal roadmap IDs)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Read-only Product Proof audit — **no implementation** |
| **Prior** | P17.S1–S5 · P18.S1–S2 · P16 Voice eng-complete · PR1 / PR2 production gates |
| **Lens** | Production-release candidate — Owner experience as a Windows desktop product |
| **Authority inputs** | Spec v2.1 · EES v1 · Product Quality Standard · Interaction Language · Capability Integration Standard · Product Gravity · Product Proof Rule · PR1 readiness |
| **Method** | Repository evidence on current default App → OperatorRoot path; ignore implementation history |

**Naming clarity:** Constitutional **File Provider P17** remains blocked until P16 Owner Accept. Completed **P17.\*** / **P18.\*** series were product-quality programs, not File Provider. This audit must not default to Moments polish.

---

## 1. Executive Summary

P17 made Workspace **speak coherently**. P18 made Moments **list and session truth** hold on the default App path. Judged as a production Windows desktop application — not as a Moments backlog — the largest remaining obstacle to feeling production-ready is no longer Moments interaction amnesia.

**#1 remaining source of distrust:** **Production lifecycle trust** — Workspace still fails the ordinary Windows product test of *install → live on the desktop → Collapse → return → Exit* as one calm, predictable organism. Engineering tray presence exists, but **Show Conversation restores the native window without restoring React ShellMode**, so Collapse → tray Show can leave an empty or wrong Form. Close-vs-hide policy (C2) remains open. Outside daily use, **unsigned installer + no updater** keep release confidence weak (`productionReadyToday: false`).

Secondary gates (different ownership):
- **Owner Voice live Product Proof** — constitutional Product Complete gate for P16; blocks File Provider expansion
- **Empty Conversation discoverability** — first-session vacuum (composer `placeholder=""`)
- **Dock vs Conversation gravity** when Moments is open — attention cohesion, not lifecycle truth

Moments browse + tool session continuity (P18.S1/S2) are **no longer the primary production-feel blocker**. Remaining Moments ambient/stage gaps are secondary.

**Recommended next product program (not implemented):** restore **Tray Show ↔ ShellMode** continuity as the smallest executable vertical slice with the largest daily premium-desktop lift. Parallel Owner track: Voice Product Proof. Parallel Track A: signing (external) → updater.

---

## 2. Product Proof Scorecard

Scores = Owner-visible production feel today (not engineering completeness alone).

| Area | Score | Notes |
| --- | --- | --- |
| First launch / onboarding | **Weak** | No tour by design; empty Conversation vacuum |
| Startup | **Strong** | Single-instance; Conversation gravity |
| Shutdown / Exit | **Partial** | Tray “Exit Workspace”; C2 close-vs-hide open |
| Tray | **Partial** | E1 eng-complete; Show ≠ ShellMode restore; OA unpaid |
| Conversation (idle dock) | **Strong** | Gravity + P17 honesty / working state |
| Conversation (Moments open) | **Partial** | Dock can demote Conversation |
| Voice (engineering path) | **Strong** | PX1–PX4; F10 review-before-send |
| Voice (Owner Product Proof) | **Weak / Unknown** | Pending live Accept |
| Moments browse | **Strong** | P18.S1 shared list |
| Moments tool session (Save/Restore flow) | **Strong** | P18.S2 continuity holder + inline Save |
| Save | **Strong** | Usable with/without expandHost |
| Restore | **Strong** | Session survives remount where appropriate |
| Navigation (Home / Continue / Save) | **Strong** | Focus + pin preserved in Moments family |
| Runtime continuity (in-process Moments) | **Strong** | Post-S2 |
| Runtime continuity (Collapse → tray Show) | **Weak** | Native show without ShellMode 1 |
| Notifications | **Partial** | Provider closed; low daily premium signal |
| Responsiveness | **Strong** | P17.S4 Conversation busy truth |
| Accessibility | **Partial** | Labels / reduced-motion; no a11y program |
| Diagnostics / logging | **Strong** | File logs + rotation |
| Support bundle | **Strong** | Conversation export path (B1) |
| Settings | **Partial** | IPC present; no in-app Settings surface; NL honesty |
| Discoverability | **Weak** | Blank composer; NL-only by Product Gravity |
| Installer | **Partial** | NSIS + checksums; unsigned |
| Updater readiness | **Weak** | Absent; blocked by signing |
| Recovery | **Partial** | Session checksums; C1 corrupt honesty open |
| Production trust (distribution) | **Weak** | SmartScreen / no updates / version 0.1.0 |
| Owner confidence (overall) | **Partial** | Strong operator core; unfinished Windows product shell |

---

## 3. Areas That Feel Production Ready

| Surface | Why it feels finished |
| --- | --- |
| **Conversation as product** | Transcript / composer hero; Product Gravity when dock idle |
| **Capability failure honesty** | One Conversation voice (P17.S1) |
| **Working-state continuity** | Busy from Send through IPC (P17.S4) |
| **Moments agency + status** | Esc/Enter/focus; success/status ownership (P17.S2/S3/S5) |
| **Moments browse + session** | Shared list; Save/Resume survive remount (P18.S1/S2) |
| **Voice engineering path** | Capture → review → Soft Send; F10 intact |
| **Closed providers P10–P15** | Clipboard → Screenshot permanently closed |
| **Single-instance** | Secondary launch focuses Conversation |
| **Support bundle** | Privacy-preserving export via Conversation |
| **Installer foundation + checksums** | NSIS + A1 tooling exist |
| **Privacy posture** | Local-first; no network telemetry by default |

These surfaces already behave like a calm Conversational Desktop Operator **inside a running session**.

---

## 4. Areas That Do Not

| Surface | Owner-visible fracture |
| --- | --- |
| **Collapse → tray Show** | Window may appear without Conversation Form restored |
| **Close vs hide** | Shutdown policy incomplete (C2) |
| **Install / update trust** | Unsigned binary; no calm update story |
| **First session** | Empty Conversation with no gentle cue |
| **Live Voice Product Proof** | Not Owner-accepted; Product Complete unknown |
| **Moments ambient stage** | No PersistentMomentStage on App path (secondary) |
| **toolDock after Collapse** | Earned Moments dock not restored with Conversation |
| **Settings discoverability** | No in-app Settings; Intent tells truth — still a vacuum |
| **Operational Acceptance** | A0 / B1 / E1 OA checklists unpaid |
| **Crash / corrupt honesty** | No crash reporter; C1 ReadyNow unimplemented |
| **IPC / security surface** | Large IPC surface; D1 nextReadyNow (low daily feel) |

---

## 5. Root Cause Analysis

Findings grouped by root cause — not by feature request.

### RC-A — Native lifecycle ≠ product Form (Critical for daily premium feel)

Tray `show_conversation` shows/focuses the webview only (`tray.rs`). It does not advance React `ShellMode` from Collapse (0) back to Conversation (1). Collapse can leave the product Form behind while the OS window returns.

**Surfaces:** tray, Collapse, startup-after-hide, recovery of attention.  
**Owner experience:** “I brought Workspace back and it forgot how to be Conversation.”

### RC-B — Distribution trust incomplete (Critical for release confidence)

Installer foundation exists; Authenticode signing (A2) and updater (B2←A2) do not. `productionReadyToday: false`. SmartScreen fear and “never updates” dominate first impression for non-developer distribution.

**Surfaces:** installer, updater, production trust.  
**Ownership:** largely Track A / external cert — not a pure UX slice.

### RC-C — Shutdown semantics unfinished (High)

E1 tray Exit exists; C2 close-vs-hide policy still open. Window close vs Exit Workspace remains a trust ambiguity for a tray-resident desktop app.

**Surfaces:** shutdown, tray Exit, background lifecycle.

### RC-D — First-session silence (High for learnability)

Product Gravity correctly rejects catalogues. Empty Conversation still offers no calm first cue (`placeholder=""`). Discoverability relies entirely on Owner knowledge or external workbook.

**Surfaces:** onboarding, discoverability, learnability.

### RC-E — Owner Product Proof gate open (Critical for capability sequencing)

P16 Voice is engineering complete and package-ready; live Accept pending. Until Owner Accept, File Provider and permanent P16 close remain blocked. This is **product authority**, not missing polish.

**Surfaces:** Voice Product Complete, P17 File sequencing.

### RC-F — Attention cohesion when Moments earned (Medium)

Dock geometry can still compete with Conversation when Moments is open (P18.A1). Session truth is fixed; visual gravity is not fully premium.

**Surfaces:** navigation, Moments open state, premium feel.

### RC-G — Secondary continuity leftovers (Medium / Low)

toolDock restore after Collapse; Cognitive/composition stubs on App; PersistentMomentStage absent. Real, but no longer the largest production-feel blocker after P18.S2.

### RC-H — Rare-path honesty (Medium)

Corrupt-session UX (C1), crash dumps (B3), release automation — trust under failure, not daily path.

---

## 6. Ranked Engineering Opportunities

| Rank | Opportunity | Class | Why now |
| --- | --- | --- | --- |
| **1** | **Tray Show ↔ React ShellMode restore** | **Implement Now** | Highest daily lifecycle trust; executable; not Moments-scoped |
| 2 | C2 close-vs-hide policy (after #1 coherent) | Gate C2 | Completes shutdown story |
| 3 | Calm empty-Conversation first-session cue | Product quality | Learnability without catalogue |
| 4 | Owner Voice live Product Proof | **Owner** | Constitutional Product Complete |
| 5 | toolDock restore with Conversation after Collapse | Track A | Continuity polish |
| 6 | Dock ↔ Conversation attention cohesion | Product quality | Premium when Moments open |
| 7 | C1 corrupt-session honesty | Gate ReadyNow | Rare-path trust |
| 8 | D1 IPC quarantine | nextReadyNow | Security; low daily feel |
| 9 | A2 signing → B2 updater | Track A / external | Release confidence |
| 10 | Moments ambient / Cognitive on App | Structural | Reject as next slice |
| — | More Moments session polish | — | **Reject** (S2 done) |
| — | File Provider / new capabilities | — | **Blocked** until Voice Accept |

---

## 7. Estimated Engineering Effort

| Opportunity | Effort | Risk |
| --- | --- | --- |
| Tray Show ↔ ShellMode 1 restore | **S–M** | Med (native ↔ React contract) |
| C2 shutdown policy | **S–M** | Med (OS close semantics) |
| Empty Conversation cue | **S** | Low (copy + Product Gravity) |
| toolDock restore | **S** | Taste / attention |
| Dock gravity cohesion | **S–M** | Layout taste |
| C1 corrupt honesty | **S–M** | Low–Med |
| D1 IPC quarantine | **M** | Regression-sensitive |
| A2 signing | **External** | Cert / CI |
| B2 updater | **M** after A2 | Release ops |
| Voice live PP | **Owner session** | Not engineering |

---

## 8. Estimated User Impact

| Opportunity | Trust | Daily usability | Learnability | Premium feel |
| --- | --- | --- | --- | --- |
| Tray ↔ ShellMode | **Critical** | **Highest** after Collapse | Med | High |
| C2 shutdown | High | High | Med | High |
| Empty Conversation cue | Med | Med | **Highest** first hour | Med–High |
| Owner Voice Accept | Critical (gate) | High if pass | Med | High |
| Signing + updater | **Critical** install/update | Med daily | Low | Highest release |
| Dock gravity | Med | Med | Low | High polish |
| D1 IPC | High security | Low felt | Low | Low daily |
| Moments leftovers | Med | Med | Low | Med |

---

## 9. Recommended Next Product Program

### P19.S1 — Tray Show ↔ ShellMode Continuity

**Direction:** Make Collapse → tray Show (and equivalent restore paths) return the Owner to **Conversation as Form**, not merely a visible webview.

**Why this program (not Moments, not Conversation copy, not D1):**
1. P18.S2 closed the largest Moments session fracture; remaining Moments gaps are secondary.
2. Production readiness machine state still says not release-ready — but the **executable** daily fracture engineering owns without a code-signing certificate is lifecycle Form restore.
3. PR2 already established tray language (“Show Conversation”); the missing piece is product Form continuity.
4. C2 becomes clearer once Show/Collapse share one restore contract.
5. Owner Voice Product Proof remains a **parallel Owner track**, not a substitute engineering program.

**Parallel (do not conflate):**
- Owner: P16 Voice live Product Proof
- Track A / external: A2 signing → B2 updater
- Canonical security nextReadyNow: D1 (lower daily feel)

**Not the next program by default:** Moments ambient stage, File Provider, Conversation rewrite, IPC quarantine-as-premium-feel.

---

## 10. Single Highest-Value Executable Vertical Slice

### Tray Show ↔ React ShellMode Continuity

**Problem:** After Collapse, tray **Show Conversation** / left-click shows the native window without restoring ShellMode Conversation Form. The Owner experiences Workspace as present-but-empty or wrong — the opposite of premium desktop trust.

**Outcome:** Every ordinary restore path that means “bring Conversation forward” leaves the Owner in truthful Conversation Form (mode 1), with deterministic behaviour for Collapse, tray Show, and secondary-instance focus.

**In scope (smallest slice):**
1. Unify tray Show / left-click with the same restore intent as Desktop Operator Conversation restore (ShellMode → Conversation).
2. Preserve Product Gravity language: Show Conversation / Exit Workspace.
3. Deterministic behaviour when Collapse hid Form A; no silent empty shell.
4. Verifier: Collapse → Show restores Conversation Form markers; P17/P18 Moments verifiers unchanged.
5. Do not implement C2 full policy, signing, updater, Moments ambient, or File Provider in this slice.

**Out of scope:** Authentode; updater; Voice Product Proof; empty-composer cue (follow-up); dock CSS rewrite; Spec reopen.

**Success test:** Start Conversation → Collapse → tray Show Conversation → Owner sees Conversation Form ready to type/speak → Exit Workspace still explicit and calm.

---

## Explicit answers

| Question | Answer |
| --- | --- |
| Greatest remaining obstacle to production-ready feel? | **Production lifecycle trust** — especially Collapse → tray Show Form restore, then shutdown policy + unsigned/no-updater |
| Is Moments still #1? | **No** — browse + session continuity closed enough; leftovers secondary |
| Is Conversation complete? | **Strong core; not complete** — empty first-session cue + dock gravity remain |
| Highest-value executable slice? | **Tray Show ↔ ShellMode Continuity** (§10) |
| Owner-owned gate? | **P16 Voice live Product Proof** (parallel) |
| Release-confidence blocker? | **Signing → updater** (Track A / external) |
| Implement now? | **No** — audit only |

---

## Compliance

| Authority | Posture |
| --- | --- |
| Spec v2.1 | Unchanged; no Review Trigger |
| EES v1 | Audit program; documentation max layer |
| Product Quality Standard | Used as excellence lens |
| Interaction Language | Used as feel lens |
| Capability Integration Standard | No provider expansion proposed |
| Production Before Expansion | File Provider remains blocked |
| Artifact obligation | Audit-only — no new verifier (no machine check for Owner feel ranking); health + milestone updated |

---

## Stop

Audit complete. **Do not implement.** Await Product Owner direction before P19.S1 or any other program.
