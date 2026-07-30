# Workspace Product Alignment Audit — Post Governance Baseline

| Field | Value |
|-------|-------|
| **Status** | Historical audit — **partially superseded** by [WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md) (2026-07-30) |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/workspace-product-alignment-audit-34a5` |
| **Governance baseline** | [AI Engineering Governance](../00-Governance/AI_ENGINEERING_GOVERNANCE.md), [AGENTS.md](../../AGENTS.md) |
| **Related** | [WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md), [PRODUCT-VISION-REALIGNMENT-AUDIT.md](PRODUCT-VISION-REALIGNMENT-AUDIT.md), [WORKSPACE-VISUAL-DIRECTION.md](WORKSPACE-VISUAL-DIRECTION.md), [DAF-ARCHITECTURE-AUDIT.md](../03-Engineering/DAF-ARCHITECTURE-AUDIT.md) |
| **Audience** | Product, engineering, commercial review |

**Supersession note:** This document still frames Workspace as an environment the user **organises into**. Human review corrected the model to **spatial representation of the active desktop**. Prefer the Reality Alignment Audit for product decisions.

**This is not a feature batch.**  
**Do not expand Assistant architecture or create new AI engines from this document.**

---

## Mission reminder

Workspace is a **desktop workspace operating environment**.

A user opens Workspace and manages their desktop environment:

- applications can be organised
- applications can be positioned
- layouts / arrangements can be saved
- workspaces can be switched
- applications can be controlled together
- user workflow is improved

The Assistant is a **supporting sidecar**. The Assistant is **not** the main product.

---

## Section 1 — Current Product Reality

### Working capabilities

Features that exist and are functionally usable (on the intended Windows + Tauri path unless noted):

| Capability | Evidence |
|------------|----------|
| Tauri Windows desktop shell | `app/`, `app/src-tauri` |
| Workspace create / get; active workspace in settings | IPC + SQLite; `App.tsx` bootstrap |
| Zones create / place on companion canvas | `CanvasShell.tsx`, layout IPC |
| Canvas layout save / restore (zone board) | `layout` domain, `007_layout.sql`, `layoutPersistence.ts` |
| Manual application registry + governed launch | `application` + `application_launch` + WindowController/launcher |
| Desktop observation (monitors, windows, identity) | `windows-integration` capture; observation migrations; IPC |
| Window control boundary (`set_bounds`, `focus`) | DAF-1a `WindowController` (Win32 / stub) |
| DesktopArrangement persistence (membership + bounds) | DAF-1c/1d migrations `079`–`080` |
| Governed capture / restore pathway | DAF-1d CommandPipeline → PermissionGateway → WindowController |
| Desktop Arrangement UI rail (save / list / select / restore) | DAF-1e Workspace stage panel |
| Permission Gateway + Command Pipeline + audit | kernel security + commands |
| SQLite persistence substrate | `packages/database` |
| Assistant read/compose projections (Programme IV) | assistant IPC + domain packages |
| Diagnostic Operator console | kitchen-sink validation surface |

### Partial capabilities

Foundations exist; user-facing product journey incomplete or environment-limited:

| Capability | Gap |
|------------|-----|
| OS window restore | Works on Win32 path; stub on non-Windows; needs Windows human verification |
| Arrangement UX | Rail exists; empty/runtime states clear; not yet a full “apps as stage” product |
| Application management UX | Register/launch mainly via Diagnostic Operator — no product Apps surface |
| Installed-app discovery | Missing; manual registry only |
| Workspace switching | Active id works; no `list_workspaces` IPC / switcher UI on Workspace tab |
| Multi-monitor awareness | Observed in capture; no mode/manager for multi-monitor arrangements |
| Canvas `locked` flag | Locks canvas drag only — not OS arrangement lock |
| Product chrome | Top tabs (Workspace / Work / Assistant / Diagnostic) — not concept bottom nav |
| Assistant placement | Full **tab**, not persistent side rail |
| Environment “window groups” | Informational only — cannot move/resize groups together |

### Missing capabilities

Required by original vision / concept direction; not implemented:

| Capability | Notes |
|------------|-------|
| Flow ↔ Focus (or equivalent) work modes | Explicit DAF follow-on in architecture audit |
| Density transform with apps staying open | No mode engine |
| Group move / resize of applications | Not implemented |
| Lock OS arrangements | Not implemented |
| Audio mixer / per-app audio | Future `domain-audio`; not started |
| Bottom product navigation (Home / Apps / Layouts / …) | Misaligned chrome |
| Quick-launch app grid from OS discovery | Missing |
| Auto-layout / snapping | Explicit non-goal for DAF-1a–1e; still missing as product feature |
| Phone mirroring | Deferred (acceptable post-MVP) |

---

## Section 2 — Original Vision Alignment

Sources: `docs/01-Product/references/`, [WORKSPACE-VISUAL-DIRECTION.md](WORKSPACE-VISUAL-DIRECTION.md), DAF docs, [PRODUCT-VISION-REALIGNMENT-AUDIT.md](PRODUCT-VISION-REALIGNMENT-AUDIT.md).  
Images are **not** pixel specs.

### User problem

Professionals lose time reconstructing working sets of applications. They need a controllable desktop environment that **remembers and restores** organised setups — not another AI chat product.

### Interaction model (from concept)

| Intent | Concept | Today |
|--------|---------|-------|
| Primary stage | Real desktop apps | Companion canvas zones + arrangement rail over observation/control |
| Mode change | User-driven Flow ↔ Focus | Missing |
| Assistant | Stable right rail (~⅕) | Peer **Assistant** tab (+ Work intelligence tab) |
| Chrome | Workspace-oriented nav | Engineering tabs dominate |
| Save / restore | Named desktop arrangements | **Now present** via DAF-1c–1e (major progress since prior realignment audit) |

### Workflow intent

1. Open Workspace  
2. See / manage the working desktop  
3. Save arrangement  
4. Switch context / mode later  
5. Restore arrangement  
6. Optionally ask Assistant for help  

**Today:** steps 3 and 5 have a governed foundation + UI. Steps 2 (apps as stage), 4 (modes), and 6 (sidecar) remain incomplete or mis-prioritised in chrome.

### Information hierarchy

Concept: **Apps stage → workspace chrome → Assistant sidecar.**  

Today: **Tabs treat Work / Assistant / Diagnostic as peer product modes**, while core desktop management is still thin beside a large intelligence surface. Governance now forbids continuing that drift without product need.

### Alignment note on prior audit

The July realignment audit correctly identified inverted AI priority and missing HWND control. **DAF-1a–1e closed the “observation only / no SetWindowPos / no arrangement persistence/UI” gaps.** Remaining critical gaps are modes, chrome, app discovery/UX, grouping, and Assistant-as-sidecar.

---

## Section 3 — Product Priority Audit

### Core Workspace

| System | Classification | Notes |
|--------|----------------|-------|
| WindowController | Core | DAF-1a |
| Observation / identity | Core (facts) | DAF-1b |
| DesktopArrangement | Core | DAF-1c–1e |
| Canvas Layout / Zones | Core companion shell | Not OS tiling |
| Application registry + launch | Core (partial UX) | Needs product surface |
| Workspace entity / active id | Core (partial switch UX) | |
| Work modes / grouping / audio | Core — **missing** | |

### Supporting Infrastructure

| System | Classification | Notes |
|--------|----------------|-------|
| IPC / Tauri commands | Supporting | Solid; large surface |
| SQLite + migrations | Supporting | Solid |
| Permission Gateway / CommandPipeline | Supporting | Required for desktop control |
| Audit / lifecycle governance | Supporting | Strong |
| Kernel command handler façade | Supporting | Maintainability hotspot (monolith) |

### Assistant Capability

| System | Classification | Notes |
|--------|----------------|-------|
| Assistant projections (surface/context/retrieval/explanation/interaction/personalisation) | Assistant | Working; frozen expansion |
| Assistant / Work UI tabs | Assistant (oversized placement) | Should become sidecar |
| Decision / recommendation / evidence engines | Assistant / future infra | Keep; do not expand |

### Future / Frozen

| System | Classification | Notes |
|--------|----------------|-------|
| New AI intelligence / evidence / assistant engines | Frozen | Engineering Governance |
| Layout recommendation engines | Frozen | |
| Autonomous workspace agents | Frozen | |
| Assistant-executed window movement | Forbidden | Architecture + governance |

---

## Section 4 — AI Balance Review

| Score | Value | Explanation |
|-------|------:|-------------|
| **AI alignment** | **4/10** | Assistant cannot move windows (correct). Programme IV is valuable future capability but still occupies peer navigation and disproportionate LOC/docs relative to unfinished core desktop UX. |
| **Workspace core alignment** | **6/10** | Major jump from pre-DAF: control + persistence + restore UI exist. Still missing modes, chrome, Apps surface, grouping, discovery — the “open and manage my desktop” loop is incomplete. |
| **Human usability** | **5/10** | Workspace tab + arrangement rail is discoverable; Diagnostic remains the path for many app actions; browser Vite cannot exercise restore; concept chrome not present. |
| **Maintainability** | **5/10** | Governance package now defines ≥8 target. Hotspots remain: `domain.ts` (~9k), `OperatorConsole` (~4k), `WorkspaceIntelligencePanel` (~3k), `handler.rs` (~6k), large Programme IV domains. New engineer can find DAF paths with docs; AI surface still hard to hold in head. |
| **Commercial readiness** | **4/10** | Credible technical foundation for a Windows workspace product, but not yet a sellable “manage your desktop” experience (modes, polish, Apps UX, Windows verification, chrome). |

**Balance verdict:** AI development historically **exceeded** near-term product need. DAF-1a–1e and governance baseline **correct the trajectory**. Next investment must stay on **core Workspace**, not Assistant expansion.

---

## Section 5 — Technical Debt Review

Recommendations only — **do not delete or rewrite** from this audit.

### Finding 1 — Peer AI tabs invert product hierarchy

- **Problem:** Work + Assistant are primary navigation peers; concept wants Assistant sidecar.  
- **Impact:** Users and agents treat intelligence as the product.  
- **Recommendation:** DAF-1f chrome realignment — Workspace stage primary; Assistant rail.  
- **Priority:** High  

### Finding 2 — Application management buried in Diagnostic

- **Problem:** Create/launch apps live mainly in OperatorConsole.  
- **Impact:** Core workflow not product-visible.  
- **Recommendation:** Thin product Apps surface using existing IPC; keep Operator for diagnostics.  
- **Priority:** High  

### Finding 3 — No workspace list / switcher

- **Problem:** Active workspace works; browse/switch list missing from product UI/IPC.  
- **Impact:** “Switch workspaces” vision incomplete.  
- **Recommendation:** Add list IPC (if needed) + simple switcher on Workspace stage.  
- **Priority:** Medium  

### Finding 4 — Dual “layout” vocabulary

- **Problem:** Canvas Layout vs Desktop Arrangement both called “layout” in speech.  
- **Impact:** Confusion and accidental merge risk.  
- **Recommendation:** Keep systems separate; enforce naming in UI copy (“Canvas layout” vs “Desktop arrangement”).  
- **Priority:** Medium  

### Finding 5 — Monolithic surfaces

- **Problem:** Giant TS/Rust façades (`domain.ts`, OperatorConsole, intelligence panel, handler).  
- **Impact:** Slow onboarding; risky edits.  
- **Recommendation:** Incremental extraction when touching those areas; no big-bang rewrite.  
- **Priority:** Medium  

### Finding 6 — Oversized frozen AI surface area

- **Problem:** Evidence/decision/assistant domains and docs dwarf Product docs.  
- **Impact:** Maintainability and commercial narrative skew.  
- **Recommendation:** Keep/freeze; document “future capability” clearly; no Batch 17.  
- **Priority:** Medium (governance already constrains)  

### Finding 7 — Prior realignment audit partially stale

- **Problem:** Still claims no SetWindowPos / no desktop arrangement restore.  
- **Impact:** Misleading if read alone.  
- **Recommendation:** Prefer **this** post-governance audit + DAF completion reports as current reality; optionally stamp the older audit “superseded in part by DAF-1a–1e”.  
- **Priority:** Low  

### Finding 8 — Linux / CI platform quirks

- **Problem:** Some DB tests `READONLY_DBMOVED` on Linux; full Win32 restore not verifiable here.  
- **Impact:** False “broken product” signals.  
- **Recommendation:** Keep platform notes in AGENTS.md; validate restore on Windows checkpoint.  
- **Priority:** Low  

---

## Section 6 — Maintainability Review

Can a new senior engineer find:

| Question | Answer today | Needs AI chat? |
|----------|--------------|----------------|
| Where does window control live? | `packages/windows-integration` (`WindowController`) + kernel desktop arrangement service | **No** — docs + module headers |
| Where do layouts live? | Canvas: `domain/layout`, `CanvasShell`; OS: `desktop_arrangement` | **No** if they read DAF-1c separation docs; **risk** if they only search “layout” |
| Where does UI belong? | `app/src/components`; Workspace stage in `App.tsx` | **No** |
| Where does Assistant belong? | Domain `workspace_assistant_*` + Assistant tab components | **No** for location; **Yes-ish** for which of 6 layers to edit without docs |
| Where do permissions belong? | `packages/kernel/src/security` + CommandPipeline | **No** |

### Still likely to require AI explanation without docs

- Navigating Programme IV evidence/decision graphs  
- Which of dozens of OperatorConsole sections is “real product” vs diagnostic  
- Full `handler.rs` command catalogue without IPC-SURFACE / architecture map  

Governance package (maintainability ≥ 8 target, magic numbers, black-box rules) is the correct control for **new** work. Existing hotspots need gradual improvement, not deletion.

---

## Section 7 — Recommended Roadmap

Priority order (binding with governance):

1. Core Workspace functionality  
2. User experience  
3. Stability  
4. Performance  
5. Assistant improvements  

### Milestone A — Product Apps + workspace switcher (next)

| Field | Content |
|-------|---------|
| **Purpose** | Make application management and workspace switching first-class on the Workspace stage |
| **Why now** | Highest usability gap on existing IPC/services; unlocks “manage my desktop” without new engines |
| **Files/systems** | Existing application/workspace IPC; `App.tsx` / new thin panels; possibly `list_workspaces` if missing |
| **Non-goals** | OS app discovery engine; AI launch autonomy; Operator deletion |
| **Validation** | Typecheck/tests; human visual checkpoint on Workspace tab |

### Milestone B — Work modes (productive ↔ focus)

| Field | Content |
|-------|---------|
| **Purpose** | User-driven density/mode switch over saved arrangements |
| **Why now** | Core concept-art requirement; builds on DAF-1e arrangements |
| **Files/systems** | Arrangement + UI chrome; possibly mode metadata on arrangements — **extend**, don’t fork layout |
| **Non-goals** | Auto-layout AI; all nine concept presets; Assistant-owned mode changes |
| **Validation** | Human visual + Windows restore smoke |

### Milestone C — Chrome realignment (Assistant sidecar)

| Field | Content |
|-------|---------|
| **Purpose** | Workspace stage primary; Assistant as persistent rail |
| **Why now** | Fixes inverted hierarchy after core UX exists to host the rail |
| **Files/systems** | `App.tsx`, CSS stage layout, Assistant panels relocated |
| **Non-goals** | New assistant engines; Programme IV expansion |
| **Validation** | **Required** human visual checkpoint |

### Milestone D — Stability & Windows verification

| Field | Content |
|-------|---------|
| **Purpose** | Prove restore/control on real Windows; harden IPC empty/error states |
| **Why now** | Commercial confidence; DAF-1d/1e path is otherwise unproven in production conditions |
| **Files/systems** | windows-integration Win32; arrangement UI; tests |
| **Non-goals** | New features |
| **Validation** | Windows manual checklist + existing cargo/frontend verifies |

### Milestone E — Performance (only with log)

| Field | Content |
|-------|---------|
| **Purpose** | Address measured bottlenecks (startup, large panels) |
| **Why now** | After UX loop exists; avoid blind optimisation |
| **Files/systems** | As measured; log in `docs/04-Operations/OPTIMISATION_LOG.md` |
| **Non-goals** | Speculative caches/embeddings |
| **Validation** | Perf budgets + maintainability score |

### Milestone F — Assistant improvements (last)

| Field | Content |
|-------|---------|
| **Purpose** | Better help/explain/retrieve **inside** sidecar |
| **Why now** | Only after Workspace is primary and usable |
| **Files/systems** | Existing assistant packages |
| **Non-goals** | New engines; window execution; Batch 17 |
| **Validation** | Product requirement + governance AI feature gate |

---

## Section 8 — Human Review Planning

Meaningful checkpoints only (per Human Review Policy + governance):

| Checkpoint | When | Why |
|------------|------|-----|
| DAF-1e Arrangement UI | Open / pending accept | First human-facing arrangement rail |
| Milestone A Apps + switcher | After UI lands | Core workflow visibility |
| Milestone B Work modes | After mode UI | Concept-critical interaction |
| Milestone C Assistant sidecar | After chrome change | Hierarchy / visual direction |
| Windows restore smoke | With Milestone D | Physical window behaviour |

**Do not** request review for documentation, tests, internal refactors, or governance-only changes.

Prefer live app + screenshots. Avoid unnecessary videos.

---

## Summary verdict

| Dimension | State |
|-----------|-------|
| Governance | Active — suitable control plane for next work |
| DAF foundation (observe → save → restore → UI) | **Complete enough to build on** |
| Original vision | **Partially aligned** — control path closed; modes/chrome/Apps UX open |
| AI balance | **Improving but still heavy** — freeze expansion; invest in core Workspace |
| Recommended next milestone | **Milestone A — Product Apps + workspace switcher** |

---

## Explicit confirmation

> Audit only. No Batch 17. No new AI engines. No Assistant architecture expansion.  
> Next value is core Workspace usability on top of DAF-1a–1e — not more intelligence layers.
