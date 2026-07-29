# Workspace Product Vision Realignment Audit

| Field | Value |
|-------|-------|
| **Status** | Audit only — no major product implementation in this change |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/product-vision-realignment-audit-34a5` |
| **Visual north star** | Original Workspace concept art (switchable layouts + AI as sidecar) |
| **Audience** | Engineering leads, product, commercial review |

**This is not Programme IV Batch 17.**  
**Do not expand AI layers from this document.**

---

## 0. Product definition (authoritative for this audit)

Workspace is primarily a **desktop workspace management application**.

### Core product

- Managing **real** desktop applications
- Creating **switchable** workspace layouts
- Grouping applications together
- Moving and resizing **groups** of applications together
- Locking application arrangements
- Expanding and compressing workspace layouts (e.g. Flow ↔ Focus)
- Restoring saved workspace states
- Per-application control (including **audio** where supported)
- Creating a better desktop operating environment

### AI assistant (secondary)

- Assists, answers, provides context
- May help operate the workspace **in the future**
- Must **never** be the primary product surface
- Existing Programme III/IV infrastructure is **future capability** — keep, freeze expansion

Concept art confirms: the **stage** is apps + layouts; the **AI Assistant** is a persistent right-rail (~⅕ width) in both productive and focus modes.

---

## 1. Current product gap analysis

### 1.1 Visual north star vs today

| Concept element | Intended (art) | Today | Gap |
|---|---|---|---|
| Primary stage | Real app windows as tiles / cards | Zone rectangles on a free-form canvas | **Critical** — canvas ≠ OS windows |
| Layout modes | Flow (dense) ↔ Focus (immersive) via slider | Single free-form canvas; no mode engine | **Critical** |
| Apps stay open across transforms | Required | Launch + observe only; no rearrange | **Critical** |
| Group move / resize | Required | Informational “window groups” only | **Critical** |
| Lock arrangements | Required | Layout node `locked` flag (canvas drag only) | **Partial** |
| Save / restore workspace | Desktop window positions | SQLite **canvas** layout restore | **Partial / wrong target** |
| Audio mixer | Per-app volume strip | Not implemented | **Missing** |
| Bottom nav | Home / Apps / Layouts / Automation / Files / Settings | Top tabs: Canvas / Work / Assistant / Diagnostic | **Misaligned chrome** |
| Top chrome | Search, profile, notifications, clock | Brand + tablist | **Thin** |
| Quick launch | App icon grid | Manual app registry + Operator launch | **Partial** |
| Multi-monitor | Supported | Observed in capture; no layout manager | **Partial** |
| Phone mirroring | Managed window | Explicitly post-MVP / missing | **Deferred (OK)** |
| AI Assistant | Right sidebar, secondary | Full **Assistant** + **Work** tabs dominate navigation | **Inverted priority** |

### 1.2 What exists that *supports* the core product

| Capability | Location | Notes |
|---|---|---|
| Tauri Windows shell | `app/` | Correct runtime for product |
| Workspace / Zone / Application / Widget entities | `packages/domain`, kernel services | Foundation resources |
| Spatial **canvas** layout (DEC-009) | `layout` domain, `CanvasShell.tsx` | Useful as *shell* layout; **not** HWND tiling |
| Manual app registry + governed launch | `application`, `application_launch`, windows-integration launcher | Launch works on Windows; no installed-app discovery |
| Read-only desktop capture | `packages/windows-integration` (`EnumWindows`, monitors) | Strong **observation** base for future control |
| Observation → Environment / WorkspaceState | kernel + domain | Facts about live windows; **authority_effect: none** |
| Permission Gateway + Command Pipeline | kernel | Required for any future window-control commands |
| SQLite persistence | `packages/database` | Correct substrate for saved layouts |

### 1.3 What exists that is *not* the core product (keep / freeze)

| Family | Scale (approx.) | Role going forward |
|---|---|---|
| Programme IV evidence + assistant packages | Domains ~20k LOC; migrations 064–078 | Future assistant **capability**; freeze Batch 17+ |
| Programme III coherent runtime | Migrations 052–063 | Keep; freeze new isomorphic engines |
| Programme II cognitive | Migrations 045–051 | Keep; freeze |
| Recommendation / Decision Engine / Queue | Large domain + ~37 contract test files | Supporting suggestion path for MVP automation; freeze clone-slices |
| Work / Operator intelligence panels | ~7.5k UI LOC | Diagnostic + projection UI; demote in product chrome |
| `docs/05-AI` | **153** files vs **3** in `01-Product` | Doc imbalance mirrors engineering drift |

### 1.4 Explicit gaps vs MVP / vision docs

Even against committed `MVP-DEFINITION.md` and `PRODUCT-VISION.md`, core desktop gaps remain:

| MVP / vision item | Status |
|---|---|
| Discover installed apps | Missing |
| App grouping (basic) in shell | Missing as product behaviour |
| Window management (Phase 2 in MVP exclusions, but **central in concept art**) | Observation only — **no SetWindowPos / snap / restore** |
| Audio domain | Missing (listed Phase 2) |
| Overlay hosting apps (DEC-008) | Companion webview only |
| Suggest + approve automation | Machinery exists; not the primary UX of the art |

**Verdict:** Engineering effort inverted. The repo is a **strong governed-intelligence platform** with a **thin desktop-management shell**. Concept art requires the opposite emphasis.

---

## 2. Classification of the repository

### A. Core Workspace functionality (invest here)

1. Windows-integration **control** (move/resize/focus/show) — extend beyond capture/launch  
2. **Desktop layout** model: named layouts that bind registered apps → window geometries (and later groups)  
3. Layout **modes** / density transform (Flow ↔ Focus) — product feature, not another AI engine  
4. Save / restore **desktop** arrangements  
5. App discovery + quick launch  
6. Product chrome: Home / Layouts / Apps primary; utilities secondary  
7. Audio mixer (after window control baseline)

### B. Supporting infrastructure (preserve)

- Command pipeline, Permission Gateway, audit, IPC envelope  
- SQLite + repositories  
- Resource graph (Workspace → Zone/App/Widget)  
- Observation capture (feeds environment; later verifies restore)  
- Canvas layout (may remain as *companion* overview, not the OS tile engine)

### C. Future AI capabilities (keep; freeze expansion)

- Programme II–IV domains, services, migrations, projection helpers  
- Assistant Intelligence UI (sidecar candidate — do not delete)  
- Recommendation / Decision / Attention as suggestion infrastructure  
- Governed `ai_*` planning path (AssistantPanel legacy)

**Rule:** No new intelligence engines, evidence clones, or Programme batches without a **direct user-facing core-product requirement**.

### D. Technical debt

| Item | Action |
|---|---|
| Evidence/assistant clone inflation (~1.4–1.7k × N) | Consolidate **mechanically** later; do not add engines |
| RE/DE lifecycle slice farm | Freeze new slices |
| OperatorConsole kitchen sink | Keep diagnostic; do not grow as product UI |
| `main` / resilience compile debt (historical) | Verify on merge; fix if blocking |
| Linux DB TempDir tests | Platform; leave |
| Doc imbalance (153 AI vs 3 Product) | Rebalance with this audit + roadmap update |
| Naming collision: “layout” (canvas) vs desktop layout | Rename or namespace in next milestone (`CanvasLayout` vs `DesktopArrangement`) |

---

## 3. Architecture concerns

1. **Authority inversion risk** — Many projections are correctly non-executing. Window control must go through **Permission Gateway** with explicit user intent; do not route HWND moves through Assistant “compose” paths.  
2. **Two layout meanings** — Canvas `Layout` (DEC-009) vs OS window arrangements. Treat as separate bounded contexts; avoid bolting HWND IDs onto canvas nodes without a clear model.  
3. **Observation ≠ control** — Environment/Composition/Profiles must stay informational until a dedicated **WindowArrangement** (or equivalent) command family owns mutations.  
4. **Windows-only control surface** — Control APIs live only in `workspace-windows-integration`; stubs elsewhere. Tests must not pretend Linux can tile HWNDs.  
5. **UI ownership drift** — Work/Assistant tabs currently present intelligence as the product. Chrome must be realigned so Assistant is a **panel**, not a peer “product mode.”  
6. **Do not invent a seventh AI programme** to “drive layouts.” Layout transform is a **deterministic desktop feature**; AI may later *suggest* a layout switch.

---

## 4. Systems to freeze, simplify, or consolidate

| System | Decision |
|---|---|
| Programme IV Batch 17+ | **Frozen** |
| New evidence / cognitive / “hub” engines | **Frozen** |
| Assistant Intelligence stack (Batches 11–16) | **Keep**; treat as sidecar capability; no new layers |
| CanvasShell | **Keep**; clarify as companion spatial board; do not pretend it tiles Chrome/Discord |
| windows-integration capture/launch | **Extend** toward control (edit in place; no parallel crate) |
| WorkspaceIntelligencePanel / OperatorConsole | **Simplify product exposure**; keep code for diagnostics |
| Evidence-family LOC clones | **Consolidate later** (edit-in-place macros/helpers) — after desktop milestone lands |
| Product docs | **Elevate**; roadmap must re-centre on desktop management |

---

## 5. Recommended next engineering milestone

### Milestone name: **Desktop Arrangement Foundation (DAF-1)**

**Goal:** Make Workspace behave like a **workspace manager** for real Windows applications — not another intelligence surface.

### In scope (practical, sequential)

1. **Window control API** in `windows-integration`  
   - Move / resize / focus (and restore geometry) for enumerated HWNDs  
   - Capability-gated; audited  
2. **Desktop arrangement model** (domain + SQLite)  
   - Named arrangement: list of `{ app_ref or window identity fingerprint, bounds, monitor, z }`  
   - Save current observation → arrangement; restore arrangement → control API  
3. **Product UI v1 (chrome realignment)**  
   - Primary: **Home / Layouts (arrangements) / Apps**  
   - Secondary utility: Audio (placeholder), Automation  
   - **AI Assistant as right sidebar panel** (reuse Programme IV projections; no new engine)  
   - Demote Work/Diagnostic to advanced/diagnostic entry points  
4. **One density transform prototype**  
   - Two saved arrangements: “Flow” and “Focus”  
   - Explicit user control (slider or toggle) — apps remain running  
5. **Installed app discovery (basic)**  
   - Enough to populate Quick Launch without Operator-only registration  

### Explicitly out of scope for DAF-1

- Programme IV Batch 17 / new AI engines  
- Phone mirroring  
- Full nine layout concepts from the concept sheet (pick Flow + Focus only)  
- Production visual polish matching art 1:1  
- Audio mixer (track as DAF-2)  
- Multi-monitor advanced policies  

### Success statement

> “I opened Workspace, captured my running apps into a named layout, switched Flow ↔ Focus, quit and restored, and the real windows moved — while the Assistant stayed a sidebar that answered questions without owning the desktop.”

### Maintainability rules for DAF-1

- Prefer editing `windows-integration` + one new domain module over parallel abstractions  
- No wrapper crates “beside” existing layout  
- Clear ownership table: Control = windows-integration; Policy = kernel gateway; Model = domain arrangement; UI = presentation only  
- Human-readable names; avoid Programme-batch folder proliferation for desktop features  

---

## 6. Suggested roadmap after DAF-1

| Order | Milestone | Intent |
|---|---|---|
| DAF-2 | Groups + lock + expand/compress | Group move/resize; lock; richer density continuum |
| DAF-3 | Audio mixer | Per-app volume where Windows APIs allow |
| DAF-4 | Chrome polish + layout presets | Closer to concept art (dock, search, presets) |
| DAF-5 | Assistant operates workspace | Permission-gated “switch to Focus” / “load Work layout” using **existing** assistant stack |
| Later | Evidence mechanical consolidation | Shrink clone LOC without new engines |
| Never-by-default | Batch 17+ without product brief | AI expansion only on explicit user-facing need |

---

## 7. Immediate non-goals (reaffirmed)

- Do not delete Programme IV  
- Do not redesign Workspace into an AI assistant platform  
- Do not start Batch 17  
- Do not add intelligence engines to “catch up” to the concept art  
- Do not implement all nine layout concepts in one pass  

---

## 8. Summary scores (honest)

| Dimension | Score (1–5) | Note |
|---|---|---|
| Alignment with concept art | **1.5** | Observation + canvas; almost no OS arrangement product |
| Core desktop foundation readiness | **3.0** | Capture/launch/permissions are a good base |
| AI capability depth | **4.5** | Far ahead of product need |
| Product chrome alignment | **1.5** | Assistant/Work dominate |
| Maintainability for human desktop engineers | **2.5** | AI clone volume obscures the real product path |
| Recommended next spend | **DAF-1** | Window control + arrangement + chrome realignment |

---

## Explicit confirmation

> Workspace’s visual and product north star is **desktop workspace management**.  
> The AI assistant is a **secondary sidecar**.  
> Programme IV remains valuable **future infrastructure** and is **frozen for expansion**.  
> The next engineering milestone is **Desktop Arrangement Foundation (DAF-1)**, not another AI batch.
