# P21.A2 — Task Decomposition & Initiative Audit

| Field | Value |
| --- | --- |
| **Program** | P21.A2 — Task Decomposition & Initiative Audit |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Engineering audit — **no implementation** |
| **Depends on** | P21.A1 (accepted), P21.S1 Capability Completion Contract (complete) |
| **Not this audit** | Autonomy · AI planning · new cognitive layers · Spec redesign |
| **Posture** | Release Hold remains; architecture stable |

**Core question:** When does Workspace already know enough to continue — and when must it stop and ask?

**Evaluation principle:** Minimize interaction steps **without** reducing Owner agency. Constitutional authority unchanged.

---

## 1. Executive Summary

Workspace’s remaining friction is less “can’t execute” and more **unnecessary yield**: after resolving a clear Owner goal with named targets, Conversation often returns control instead of taking the next safe composed step already supported by providers.

**P21.S1** closed the flagship false-completion path (`browser.open_beside` layout). The dominant *initiative* gap now is:

> One utterance → one `CapabilityIntent` → stop — even when the utterance already named **two** resolvable opens, or named an **app beside** a window.

Moments handoffs, restore approval, and file-capability walls are **correct stops**. Inventing multi-app layouts or deleting files without approval would violate agency.

**Recommended slice:** **Compound Open Decomposition** — when “open A and B” resolves two known entities, sequentially open both via existing Kernel composition; keep `\band\` safety only for unresolvable / inventable executable names.

---

## 2. Task Decomposition Assessment

| Stage | Status |
| --- | --- |
| Goal recognition | Partial — Situation Goals / Goal Resolution name outcomes |
| Decomposition | Weak — compounds often rejected or collapsed |
| Safe continuation | Strong for pronouns when Context bound; weak for dual-named opens |
| Stop / ask | Correct for Moments restore, unbound pronouns, destructive ops |
| Capability wall | Honest for files/PDF/search contents |

**Philosophy issue:** Safety against inventing `A and B.exe` is over-applied to **two already-known** targets, forcing a second Owner turn that adds no new information.

---

## 3. Initiative Boundary Matrix

| Boundary | Meaning | Correct when |
| --- | --- | --- |
| **READY TO CONTINUE** | Enough context + capability to act without asking | Named resolvable targets; Context-bound pronouns; post-P21.S1 site-beside |
| **NEEDS CLARIFICATION** | Multiple reasonable interpretations | “Open project”; bare “arrange windows”; unbound “it” |
| **OWNER AUTHORIZATION** | Meaningful / irreversible desktop change | Moment restore; mass close; delete/clean |
| **CAPABILITY LIMIT** | Cannot finish with current providers | Organize/compare/PDF/date-filter screenshots |
| **MOMENTS HANDOFF** | Correct transfer into Save/Continue | Prepare coding workspace; continue yesterday |

---

## 4. Goal Continuation Matrix

| Owner goal | Class | Known information | Reasonable next | Proceed? | Safe steps | Stop condition | Approval |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Find yesterday’s screenshot | **CAPABILITY LIMIT** | Want screenshot; folder ≈ Pictures | Open Pictures + honesty | Partial only | 1 | After folder open | No |
| Open ChatGPT beside Cursor | **READY TO CONTINUE** | Site + beside window | `browser.open_beside` (P21.S1) | Yes | ≤5 | Layout verified / partial truth | No |
| Prepare coding workspace | **MOMENTS HANDOFF** | Work-mode phrasing | Open Continue | Surface yes | 1 | Owner picks Moment | Restore yes |
| Clean Downloads | **CAPABILITY LIMIT** | Folder intent | Open Downloads or refuse organize | No finish | 0–1 | File ops absent | Delete would need Owner |
| Compare two documents | **CAPABILITY LIMIT** | Documents/files | Near-miss honesty | No | 0 | No file compare | — |
| Open project | **NEEDS CLARIFICATION** | Vague noun | Ask which app/folder | No | 0 | Unknown | — |
| Continue yesterday’s work | **OWNER AUTHORIZATION** + Moments | Resume / yesterday | Open Continue | Handoff | 1 + Owner | No Moment / reject | **Yes** |
| Launch multiple apps (open X and Y) | **READY TO CONTINUE** *(blocked)* | Two named entities | Sequential open | **Should; currently won’t** | 2 | First fail / unknown entity | No |
| Arrange windows | **NEEDS CLARIFICATION** *(unless named)* | Often bare | Ask window + edge | Only if concrete | 1–2 | Underspecified | No |
| Restore saved Moments | **OWNER AUTHORIZATION** | Moment name or list | Open Continue / named | Surface | 1 + approve | Owner deny | **Yes** |
| Find a PDF | **CAPABILITY LIMIT** | File type | Files near-miss | No | 0 | No file search | — |
| Search Desktop | **CAPABILITY LIMIT** / weak open | Folder vs content search | `shell:Desktop` or honesty | Folder only | 0–1 | Content search absent | No |
| Open Notepad beside Cursor | **READY TO CONTINUE** *(partially blocked)* | App + beside | Open/focus + Snap | Should | 3–5 | App-beside collapses to bare open today | No |
| Close that / again | **READY TO CONTINUE** | Context referent | Replay / close | Yes when bound | 1 | Unbound → clarify | No |

---

## 5. User Agency Assessment

| Pattern | Agency verdict |
| --- | --- |
| Moments restore only after approve | **Preserve** — correct OWNER AUTHORIZATION |
| Never invent multi-app layout for “I’m coding” | **Preserve** — MOMENTS HANDOFF |
| End-session clarify (no close-all invent) | **Preserve** |
| Unbound pronoun clarify | **Preserve** |
| Reject `open A and B` when both known | **Unnecessary yield** — Owner already authorized both opens by naming them |
| Collapse app-beside to bare open | **Incomplete help** — drops stated layout intent |
| Delete/clean Downloads without ask | **Would violate** agency — do not auto-initiate |
| Auto-restore best Moment | **Would violate** — keep Owner approve |

**Principle:** Initiative may complete **already-named, non-destructive** steps. Initiative must **not** invent targets, mutate Moments layouts, or perform destructive file ops.

---

## 6. Existing Capability Reuse Opportunities

| Opportunity | Reuse | New capability? |
| --- | --- | --- |
| Sequential “open A and B” | `app.open_or_focus` / browser `open` ×2 + Completion Contract reporting | No |
| App beside window | Application open/focus + Window Snap (same as site beside) | No |
| Focus then maximize/close | Mirror `window.focus_minimize` | No |
| Pictures / Downloads open | Existing `shell:` launches | No (finish still limited) |
| Continue yesterday | Existing Moments surface | No |
| File organize / PDF find | — | **Yes** — File Provider (blocked; not this slice) |

---

## 7. Root Cause Analysis

Fewest systemic causes:

### RC-1 — One-utterance / one-Intent execution ceiling (Critical)

TS execution plans and Goal Resolution candidates are evidence-only. Kernel runs one composed plan per turn; no safe “continue the named remainder” for dual opens.

**Explains:** open X and Y wait; unused alternative plans.

### RC-2 — Compound safety over-rejection (Critical)

`\band\` in Intent rejects compounds to avoid inventing executable names — correct for garbage strings, incorrect when **both** sides resolve to known apps/sites.

**Explains:** unnecessary “clearer desktop request” after Owner already specified two targets.

### RC-3 — Beside Intent incomplete for applications (High)

Site/browser beside reaches Kernel Completion Contract; application beside often collapses to bare `appOpen`, dropping layout.

**Explains:** “Open Notepad beside Cursor” yields after open.

### RC-4 — Correct intentional stops (Not defects)

Moments approval, unbound pronouns, file capability wall, end-session clarify.

**Explains:** prepare workspace / restore / clean Downloads — not initiative bugs.

---

## 8. Ranked Engineering Opportunities

| Rank | Opportunity | Owner value | Effort | Risk |
| --- | --- | --- | --- | --- |
| **1** | **Compound Open Decomposition** — resolvable “open A and B” → sequential opens + Completion Contract reporting | **Critical** | M | Med |
| 2 | App-beside → Snap compose (don’t drop beside) | High | S–M | Low–Med |
| 3 | Focus→maximize / focus→close compositions | Med–High | S | Low |
| 4 | Generalize post-step observation (beyond beside) | High | M | Med |
| 5 | File Provider (organize / PDF / filter) | High for file goals | L | High — blocked |

---

## 9. Recommend — ONE bounded engineering slice

### P21.S2 — Compound Open Decomposition

**Not autonomy. Not a new planner. Not Moments invent.**

**Problem:** Owner says “Open Notepad and Calculator” (or two known sites/apps). Workspace already knows both targets, but Intent `\band\` safety returns control with a clarify — an unnecessary interaction step.

**Outcome:** When both sides of a compound open resolve to known entities, Kernel executes sequential opens (existing providers). Conversation reports completed / partial under the Capability Completion Contract. Unresolvable compounds still refuse inventing executables.

**In scope:**
1. Intent: detect resolvable dual-open compounds; stop treating them as unknown solely due to `and`.
2. Kernel: composition id for sequential open (reuse `open_or_focus` / browser open).
3. Compose: Completion Contract — both opened / first only / neither.
4. Verifier: “open A and B” with two known apps does not emit unknown “clearer desktop request.”

**Out of scope:** File Provider; auto Moments restore; inventing app sets for “I’m coding”; autonomous agents; Spec changes.

**Success test:** “Open Notepad and Calculator” opens both (or truthful partial) without asking which app — Owner agency preserved because both were named.

---

## Audit area answers

| # | Area | Finding |
| --- | --- | --- |
| 1 | Unnecessary wait | Dual-named opens; app-beside drop |
| 2 | Questions context answers | Rare for pronouns (Context works); common for compound open |
| 3 | Compose without new caps | Dual open; app-beside Snap; focus-then-act |
| 4 | Initiative vs agency | Restore / invent layout / delete = must not; named dual open = may |
| 5 | Moments correct | Prepare workspace; continue yesterday; restore |
| 6 | Intentional stop | File wall; unbound pronouns; Moments approve; `!ok` break |

---

## Compliance

| Constraint | Posture |
| --- | --- |
| No implementation | Observed |
| No architecture redesign | Observed |
| No autonomous agents / AI planning | Observed |
| Preserve constitutional authority | Observed |
| Release Hold | Remains |
| Stop after audit | Observed |

---

## Stop

P21.A2 Task Decomposition & Initiative Audit complete. **Do not implement** until Owner authorizes **P21.S2 — Compound Open Decomposition** (or another ranked opportunity).
