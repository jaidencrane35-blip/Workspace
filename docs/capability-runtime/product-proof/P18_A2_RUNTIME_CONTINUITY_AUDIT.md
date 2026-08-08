# P18.A2 — Runtime Continuity Audit

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Runtime Continuity Audit |
| **ID** | P18.A2 *(audit naming — not roadmap Terminal/Memory IDs)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Read-only Product Quality audit — **no implementation** |
| **Prior** | P18.S1 Moments Browse Truth · P17.S1–S5 cohesion |
| **Lens** | Continuously running desktop app — truthful state across navigation & lifecycle |
| **Authority inputs** | Spec v2.1 · EES v1 · Product Quality Standard · Interaction Language · Capability Integration Standard |
| **Method** | Repository evidence on default App → OperatorRoot path |

---

## 1. Executive Summary

P18.S1 fixed **browse list truth** (Home and Continue share one Moments list). Judged as a continuously running application, the largest remaining continuity fracture is **session truth**.

Conversation remembers its transcript across Collapse within a process. Moments tool flows (preview, approve, done, delete ack, Save draft) live inside view-swapped panels that **unmount** on navigation or Collapse. Selection (focus / pinned primary) is wiped on ordinary view changes. After Moments exist, Save can render a shell with **no write form** because `expandHost` is never registered on the App path (Resume already has an inline fallback; Save does not).

**#1 remaining source of distrust:** Moments tool session is ephemeral UI — and Save becomes non-interactive once ActiveMoment has a `primary` without `PersistentMomentStage`.

**Recommended next slice (not implemented):** **P18.S2 — Moments Tool Session Continuity** (Save inline fallback + flow state that survives remount / honest reset).

---

## 2. Runtime Continuity Scorecard

| Area | Score | Notes |
| --- | --- | --- |
| Moments browse list (post-S1) | **Strong** | Shared `moments` authority |
| Moments interaction / step continuity | **Weak** | Panel-local; dies on unmount |
| Selection continuity (focus / pin) | **Weak** | Cleared on navigate / view effect |
| Conversation messages (same process) | **Strong** | Lives in `OperatorRoot` |
| Conversation across relaunch | **Weak** | In-memory only (expected) |
| Save with nonempty Moments | **Weak** | Portal-only; `expandHost` null → blank |
| Resume preview without stage | **Partial** | Inline fallback exists |
| Ambient / presence / attach host | **Weak** | No `PersistentMomentStage` on App |
| Tray Show after Collapse | **Weak** | Native show ≠ React shell mode 1 |
| Workspace id across relaunch | **Strong** | Settings persistence |
| Moments DB across relaunch | **Strong** | Reloaded on mount |
| `toolBusy` ↔ Conversation | **Strong** | Bridge intact while App mounted |
| Cognitive / composition on App | **Weak** | Stub no-ops |
| Restore done → browse (in panel) | **Partial** | Holds until navigate/collapse |
| Empty → nonempty after first Save | **Weak** | List updates; Save UI can blank |

---

## 3. Runtime Authority Map

```text
DURABLE (process or longer)
─────────────────────────────────────
settings.active_workspace_id     → App bootstrap
Moments DB (list_saved_contexts) → ActiveMoment.moments
Shell mode / geometry            → localStorage + native windows
Conversation messages            → OperatorRoot (process only)

EPHEMERAL (remount = amnesia)
─────────────────────────────────────
Resume step / preview / done / deletedAck  → ResumeContextPanel local state
Save naming / handoff / saved step         → SaveContextPanel local state
toolDock                                   → OperatorRoot (forced false on Collapse)
focusContextId                             → App (cleared on most navigate)
pinned primary                             → ActiveMoment (cleared on every view change)

DEAD / NO-OP ON APP PATH
─────────────────────────────────────
expandHost (PersistentMomentStage)         → never registered
CognitiveEngine / WorkspaceComposition     → stub providers only on WorkspaceShell
```

| Kind of truth | Correct authority | Continuity today |
| --- | --- | --- |
| Which Moments exist | ActiveMoment ← DB | Strong (S1) |
| Which Moment is selected | focus + pin | Weak — wiped often |
| Where Owner is in Restore flow | Resume session | Weak — panel-local |
| Where Owner is in Save flow | Save session | Weak — panel-local + blank attach |
| Conversation thread | OperatorRoot | Strong in-process |
| Shell Form A/B | ShellMode | Partial — tray Show diverges |

---

## 4. Lifecycle Consistency Assessment

### Navigation between views
- Tool tree swaps per `view` (`App.tsx`) → Resume/Save unmount → local flow state lost.  
- Conversation stays mounted → transcript continuous.  
- **Owner sees:** chat remembers; Moments forgets.

### Collapse / tray / reopen
- Collapse → mode `0`, `toolDock` false; React may render empty stub while native main is hidden.  
- Tray **Show Conversation** only `main.show()` / focus — does **not** set shell mode 1 (`tray.rs`).  
- Desktop Operator path correctly restores mode 1.  
- **Owner sees:** possible empty window after tray Show; Moments dock not restored.

### Relaunch
- Workspace id + Moments DB + shell mode persist.  
- Conversation, view, focus, steps, dock do not.  
- Acceptable for cold start if in-process continuity holds.

### Save → Continue → Save again
- After S1, list updates (`reloadMoments` + `selectMoment`).  
- With `primary` set and `expandHost` null, Save returns attach shell with **null portal** — no fields.  
- **Owner sees:** first Save works; second Save can appear broken.

### Sequential Restore
- In-panel done → browse works.  
- Leave Continue mid-preview → return → browse reset; preview gone.  
- No Conversation duplex of restore done (correct per S2) — but silent amnesia of in-progress agency.

### Working-state / busy
- `toolBusy` bridge holds while App mounted.  
- Mid-IPC Collapse can leave Conversation locked with Moments UI gone — Partial honesty.

---

## 5. Root Cause Analysis

| ID | Root cause | Severity |
| --- | --- | --- |
| **RC-1** | Moments tool UI is a remounted satellite; flow state is panel-local | **Critical** |
| **RC-2** | Save attach assumes `expandHost` from `PersistentMomentStage`; App path never registers host (Resume already has inline fallback) | **Critical** |
| **RC-3** | Selection model: navigate clears `focusContextId`; view effect clears `pinned` | **High** |
| **RC-4** | Tray/native show ≠ React `ShellMode` restore | **High** |
| **RC-5** | Collapse always closes `toolDock` with no restore of earned Moments surface | **High** |
| **RC-6** | Cognitive / composition providers absent on App path | **Medium** |
| **RC-7** | NL `listMoments` on-demand vs ActiveMoment cache | **Medium** |
| **RC-8** | Process boundary cold-start amnesia for Conversation | **Expected** |
| **RC-9** | Browse list dual-presentation (P18.A1) | **Resolved (S1)** |

---

## 6. Ranked Engineering Opportunities

| Rank | Opportunity | Class | Lift |
| --- | --- | --- | --- |
| 1 | **Moments tool session continuity** — Save inline when no expandHost; lift/preserve flow state across remount | **Implement Now** | Highest |
| 2 | Tray Show ↔ ShellMode 1 restore | Track A / S3 | High lifecycle |
| 3 | Preserve pin/focus across Home↔Continue↔Save while dock earned | With #1 | High |
| 4 | Restore toolDock after Collapse when Moments was earned | Track A | Med–High |
| 5 | Cognitive/composition on App path (or stop calling stubs) | Structural | Med |
| 6 | Unify Moments list access (NL vs ActiveMoment) | Track A | Low–Med |
| 7 | Full PersistentMomentStage remount | Structural | Reject for next slice |
| 8 | More browse polish | — | **Reject** (S1 done) |

---

## 7. Estimated Engineering Effort

| Slice | Effort | Risk |
| --- | --- | --- |
| Save inline fallback + minimal session lift / honest reset | **S–M** | Low–Med |
| Selection pin policy across Moments views | **S** | Low |
| Tray ↔ ShellMode sync | **S–M** | Med (native/React) |
| toolDock restore after Collapse | **S** | Taste |
| Full ambient stage on App | **M–L** | High |

---

## 8. Estimated User Impact

| Opportunity | Trust | Daily usability | Cognitive load |
| --- | --- | --- | --- |
| Session continuity + Save usable | **Critical** | High | Removes “Workspace forgot” |
| Tray Show restores Conversation | High | High after Collapse | Removes empty-window confusion |
| Pin/focus across Moments views | High | Medium | Less re-selection |
| Ambient stage | Medium | Medium | Presence theatre |

---

## 9. Recommended Next Vertical Slice

### P18.S2 — Moments Tool Session Continuity

**Direction:** Make Moments *work* as continuous as Moments *browse* already is after S1.

**Not** more list polish. **Not** full ambient/Cognitive remount. **Not** dock gravity rewrite.

**Parallel Owner track:** P16 Voice live Product Proof still pending.  
**Parallel lifecycle candidate after S2:** Tray Show ↔ ShellMode restore.

---

## 10. Single Highest-Value Executable Improvement

### Moments Tool Session Continuity (App path)

**Problem:** List truth is shared, but Moments interaction state is disposable. Save becomes blank when `primary` exists without `expandHost`. Conversation feels continuous; Moments feels amnesiac.

**Outcome:** Save remains usable with or without expand host. Resume/Save session state survives ordinary navigation and Collapse→Conversation reopen within one process — or a single explicit honest reset (never silent amnesia).

**In scope:**
1. Save: when `expandHost` is null, render write form inline (mirror Resume preview fallback).  
2. Lift Resume (and Save) step/session fields above the view-swapped panel — or one documented honest reset that Conversation/Moments acknowledges.  
3. Minimal selection policy: preserve pin/focus across Home↔Continue↔Save while Moments dock remains earned.  
4. Verifier: nonempty Moments ⇒ Save shows controls; mid-preview → leave → return retains preview **or** honest reset; P17/S1 preserved.  
5. Do not mount full PersistentMomentStage, redesign tray, or reopen Spec.

**Out of scope:** Cognitive productization; dock CSS; Voice; File Provider; tray/shell sync (follow-up).

**Success test:** Save a Moment → Save another still shows fields → Continue → open preview → Home → Continue still in preview (or clear reset) → Collapse → reopen Conversation → Moments session policy is deterministic.

---

## Explicit answers

| Question | Answer |
| --- | --- |
| Browse truth after S1? | **Strong** |
| Continuous truthful runtime? | **Not yet** — session/lifecycle gaps |
| Greatest friction? | **Ephemeral Moments tool session + Save blank without expandHost** |
| Highest-value slice? | **P18.S2 Moments Tool Session Continuity** (§10) |
| Implement now? | **No** |

---

## Stop

Audit complete. **No code implemented.**  
Await Product Owner review before **P18.S2**.
