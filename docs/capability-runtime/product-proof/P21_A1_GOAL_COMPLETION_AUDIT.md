# P21.A1 — Goal Completion Audit

| Field | Value |
| --- | --- |
| **Program** | P21.A1 — Goal Completion Audit |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Engineering audit — **no implementation** |
| **Perspective** | Can Workspace **finish** what the user is trying to accomplish? |
| **Out of scope** | UI polish, visual design, wording, greetings, release engineering |
| **Posture** | Release Hold remains; architecture stable |

**Evaluation shift:** not “Can it execute commands?” — **“Can it complete user goals?”**

---

## 1. Executive Summary

Workspace often **starts** helping and then **abandons** the goal after the first provider action (or after opening a surface). Intent Layer and documentation already describe multi-step desktop outcomes; Kernel Operator frequently plans or executes only the first step — and Conversation sometimes reports the finished goal anyway.

**Verdict:** Capability *coverage* is ahead of capability *completion*. The product can map many requests to an opening move; it rarely owns a goal through observe → act → verify → finish.

**Dominant failure mode:** first-step completion treated as goal completion.

**Single recommended engineering slice (not a new capability):**  
**Finish `browser.open_beside` Kernel composition** — Open → locate beside target → Window Snap — matching Intent, docs, and Owner-facing success language already in tree.

---

## 2. Goal Completion Assessment

| Question | Finding |
| --- | --- |
| Does Conversation resolve user goals or only commands? | Mix: Situation Goals / Goal Resolution name outcomes; execution is usually one `CapabilityIntent` |
| Do multi-step Intent plans reach the Kernel? | **No** — `executionPlanner.ts` is declarative / evidence-only |
| Does Kernel compose finish claimed layouts? | **Partial** — `open_foreground`, `capture_and_copy`, `focus_minimize`, `app.open_maximize` compose; **`open_beside` does not** |
| After an action, does Workspace observe the desktop? | **Almost never** — no re-enumerate / verify-layout loop |
| On partial failure, does it recover toward the goal? | **Stops** on first `!ok` (`execute_turn_inner`); Goal Resolution candidates are not executed |
| Where goals are honestly deferred (Moments)? | Work-mode / multi-app prep opens Continue — correct non-invention, but goal still unfinished until Owner restore |

**Overall:** Workspace is a strong **command starter** and a weak **goal finisher**.

---

## 3. Capability Completion Matrix

Completion % = estimated share of ordinary Owner goals in that domain that end in a finished desktop outcome (not “opened a helper surface”).

| Capability / surface | User goal (typical) | Current behaviour | Completion % | Why execution stops | Missing orchestration | Missing observation | Missing recovery |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **Application open / focus** | Get app ready | Find→Focus\|Launch; optional Maximize | **70–85%** | Bare launch has no “ready” check | Rarely needed | No post-launch verify | No alternate app match |
| **Application open+maximize** | App full-screen ready | Composed Find→Focus/Launch→Maximize | **75–90%** | Stops if maximize fails after open | — | No verify maximized | Partial success narrated from last step |
| **Window focus / state** | Find / snap / move window | Single Window op (mostly) | **60–80%** | One verb; no “then …” | Focus-then-maximize/close not composed like minimize | No re-list after act | Clarify only |
| **Window locate+minimize** | Find then minimize | `focus_minimize` 2-step | **80–90%** | — | Pattern exists | No verify minimized | Break on first fail |
| **Browser open** | Site open | Browser Open | **75–85%** | Tab/window identity weak | — | No confirm title | — |
| **Browser open+foreground** | Site open and in front | Open + Window Focus | **70–85%** | Focus query may miss | — | No verify frontmost | — |
| **Browser open beside** | Site beside named window | **Open only**; reply “Opened beside …” | **15–30%** | Layout steps never planned | Open→locate→Snap (Window Provider exists) | No layout check | Overclaims success |
| **App beside window** | App beside Cursor etc. | Often collapses to bare `appOpen` | **10–25%** | Beside dropped in Intent | Same snap compose after open/focus | — | — |
| **Open X and Y** | Two things open | Unmatched / rejected compound | **0–10%** | Grammar/safety reject `\band\` | Sequential open_or_focus | — | Unknown / clarify |
| **Screenshot capture** | Capture screen/window | Single capture | **85–95%** | — | — | Path not always Owner-visible | — |
| **Screenshot + copy** | Capture on clipboard | Capture + CopyClipboard | **70–85%** | Break if copy fails after capture | Pass capture path into copy | No clipboard verify-read | Partial capture orphaned |
| **Find yesterday’s screenshot** | Retrieve specific shot | Opens Pictures; admits no filter | **10–20%** | Folder open ≠ find | Cannot finish with current providers | No file listing | Honest abandon |
| **Organize Downloads** | Sort/clean folder | Files near-miss → unknown | **0%** | No file ops | Needs File capability (out of slice) | — | Guidance only |
| **Compare documents** | Side-by-side docs | Files near-miss → unknown | **0%** | No file open/compare | Partial: open two apps/windows if named; true compare needs files | — | — |
| **Prepare coding workspace** | Multi-app desktop ready | Navigate Continue / Moments | **20–40%*** | Deliberate: no invented app set | Moments restore is the finish path | — | Owner must Save/approve |
| **Clipboard read/write** | Text on clipboard | Single op | **80–90%** | — | — | No verify-read after write | — |
| **Notifications show** | Toast appears | Single Show | **85–95%** | — | — | — | — |
| **Notify when X finishes** | Deferred watch | Clarified unsupported | **0%** | Out of scope | Not orchestration of existing | Needs watch | — |
| **Save / Continue** | Persist / restore layout | Opens tool surface | **Surface 100% / Goal 0–50%*** | Restore requires Owner approval | Not Kernel compose | — | By design |
| **Voice** | Speak a goal | Transcript → same Intent path | **n/a (input)** | Inherits goal abandonment of resolved intent | — | — | — |

\*Moments path can finish a prior Save; it does not finish an unsaved “prepare workspace” ask.

---

## 4. Goal Abandonment Map

```text
Owner goal
    │
    ▼
Intent Layer (Situation / Grammar / Semantic / Goal Resolution)
    │  builds declarative multi-step plan (evidence only)
    │  emits ONE CapabilityIntent
    ▼
Kernel Operator plan_capability_intent
    │  sometimes multi-step (foreground, capture_and_copy, focus_minimize, open_maximize)
    │  often single-step despite composition_id / docs (open_beside)
    ▼
execute_turn_inner
    │  runs steps; STOP on first !ok
    │  no desktop re-observation
    ▼
compose_user_reply
    │  narrates last result — may claim finished layout (open_beside)
    ▼
GOAL ABANDONED or FALSE-COMPLETE
```

### Concrete abandonments (evidence)

| Owner ask | What happens | Abandoned outcome | Evidence |
| --- | --- | --- | --- |
| “I want yesterday’s screenshot” | Opens Pictures | Find/filter yesterday | `situationGoals.ts` → `shell:My Pictures` |
| “Organize my downloads” | Unknown / files near-miss | Any organize | `conversationGuidance` near-miss |
| “Compare these documents” | Unknown / files near-miss | Side-by-side docs | No Intent kind |
| “Prepare today’s coding workspace” | Continue surface | Apps arranged | `continueForActivity` — never invents layout |
| “Open ChatGPT beside Cursor” | Browser Open only | Side-by-side layout | `plan.rs` `open_beside` 1 step; `compose.rs` “Opened beside” |
| “Open Notepad beside Cursor” | Often bare app open | Beside layout | Semantic beside mainly site/browser |
| “Open X and Y” | Rejected / unmatched | Both open | Intent `\band\` safety; grammar skips `and` |
| “Take a screenshot and copy it” | Usually completes | Partial if copy fails | `capture_and_copy` + break on `!ok` |

---

## 5. Execution Lifecycle Assessment

| Lifecycle stage | Status |
| --- | --- |
| **Goal recognition** | Partial — Situation Goals + Goal Resolution name outcomes |
| **Planning** | Split brain — TS plan multi-step; Kernel plan may disagree |
| **Orchestration** | Incomplete — few true compositions; beside is the flagship miss |
| **Execution** | Strong for single ops and a handful of composed paths |
| **Observation** | Missing — no post-condition check against the Owner goal |
| **Recovery** | Weak — stop + honest/clarify reply; ranked candidates unused at runtime |
| **Completion reporting** | Unsafe when reply asserts layout/result Kernel did not finish |

**Lifecycle gap in one line:** Workspace plans goals in Intent, executes commands in Kernel, and reports goals in Conversation — those three layers are not bound by a single completion contract.

---

## 6. Root Cause Analysis

Fewest systemic causes:

### RC-1 — Composition debt on claimed multi-step goals (Critical)

Docs, Intent kinds, and Conversation success copy assume Operator compositions that Kernel does not fully plan/execute. Flagship: `browser.open_beside` (comment: “open URL then Window snap”; plan: Open only; reply: “Opened beside”).

**Explains:** beside abandonments; false completion; Owner sense that Workspace “starts then stops.”

### RC-2 — One utterance → one CapabilityIntent → no goal runtime (Critical)

There is no Kernel/Intent loop that owns “finish the Owner’s outcome” across observe/act/recover. TS `buildExecutionPlan` and Goal Resolution candidates do not drive execution.

**Explains:** compounds like “open X and Y”; focus-then-act gaps; recovery that never retries toward the goal.

### RC-3 — Folder/file goals have no finishing path with current providers (High, bounded)

Opening Explorer/shell folders is the only move; organize/compare/filter cannot complete without File capability (explicitly deferred). Honest abandon is correct — still a completion cliff.

**Explains:** yesterday’s screenshot, organize downloads, compare documents.

### RC-4 — Multi-app preparation intentionally defers to Moments (Medium, by design)

Situation Goals refuse invented layouts; Continue is the product finish path. Goal completion depends on prior Save + Owner approval — not a Kernel bug, but still “started / didn’t finish” for first-time asks.

**Explains:** prepare coding workspace.

**Not root causes for this audit:** greetings, invent/pretend wording, visual polish, unsigned release.

---

## 7. Ranked Engineering Opportunities

Prefer orchestration of **existing** capabilities.

| Rank | Opportunity | User value | Effort | Risk |
| --- | --- | --- | --- | --- |
| **1** | **Complete `browser.open_beside` Kernel composition** (Open → locate beside → Snap) + truthful failure if snap impossible | **Critical** — flagship Product Proof path; stops false “Opened beside” | S–M | Med (window match / timing) |
| 2 | Intent: app-beside must not collapse to bare `appOpen`; feed same compose path | High | S | Low–Med |
| 3 | Mirror `focus_minimize` for focus→maximize / focus→close compounds | Med–High | S | Low |
| 4 | Sequential compose for “open X and Y” (two `open_or_focus` / browser opens) | High | M | Med (ordering) |
| 5 | Post-step observation hook (enumerate/focus check) before success copy | High | M | Med (determinism) |
| 6 | File Provider (filter/organize/compare) | High for folder goals | L | High — **new capability; blocked until Owner Accept** |

---

## 8. Recommend — ONE engineering slice

### P21.S1 — Beside Layout Completion (Kernel composition)

**Not a new capability.** Window Snap and Browser Open already exist. Intent already emits `open_beside` with beside target. Docs already define Level 2 `open_beside` as open + Window snap. Conversation already claims “Opened beside …”.

**Problem:** Kernel `plan_capability_intent` returns a single Browser Open step under `composition_id: "browser.open_beside"`, so the Owner’s layout goal is abandoned while Conversation reports success.

**Outcome:** “Open ChatGPT beside Cursor” finishes as side-by-side desktop placement when the beside window is findable; otherwise truthful failure (opened but could not place / beside not found) — never false layout success.

**In scope:**
1. `packages/kernel/src/operator/plan.rs` — plan Open + Window Focus/locate + Snap (reuse Window Provider).
2. `compose.rs` — success only when layout steps succeed; partial truth when open works but snap does not.
3. Intent wiring check: `toCapabilityIntent` / semantic beside for sites continues to supply beside query; do not expand File Provider.
4. Verifier: plan step count ≥ 2 for `open_beside`; compose must not claim beside on Open-only results.

**Out of scope:** File Provider; Moments invent; UI polish; new domains; “organize downloads.”

**Success test:** Owner asks to open a known site beside an open window → desktop shows both; Conversation matches reality.

---

## Explicit answers

| Audit question | Answer |
| --- | --- |
| Why start-but-not-finish? | First-step execution + incomplete Kernel compose + no observation loop (RC-1, RC-2) |
| Orchestrate existing capabilities? | **Yes** for beside, compound open, focus-then-act |
| New capabilities required? | Only for true file organize/compare/filter (RC-3) — not the recommended slice |
| Highest leverage completion improvement | **Beside Layout Completion** |

| Constraint | Posture |
| --- | --- |
| No implementation | Observed |
| No architecture redesign | Observed |
| No new frameworks | Observed |
| Release Hold | Remains |
| Stop after audit | Observed |

---

## Stop

P21.A1 Goal Completion Audit complete. **Do not implement** until Owner authorizes **P21.S1 — Beside Layout Completion** (or another ranked opportunity).
