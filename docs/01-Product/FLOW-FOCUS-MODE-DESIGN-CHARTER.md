# Flow / Focus Mode — Design Charter

| Field | Value |
|-------|-------|
| **Status** | Design only — **not implemented** |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/flow-focus-design-charter-34a5` |
| **References** | [`references/workspace-concept-01.png`](references/workspace-concept-01.png), [`references/workspace-concept-02.png`](references/workspace-concept-02.png) |
| **Visual north star** | [WORKSPACE-VISUAL-DIRECTION.md](WORKSPACE-VISUAL-DIRECTION.md) |
| **Related** | [WORKSPACE-PRODUCT-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-ALIGNMENT-AUDIT.md), [FLOW-FOCUS-ALIGNMENT-AUDIT.md](../03-Engineering/FLOW-FOCUS-ALIGNMENT-AUDIT.md) |

**This document does not authorise coding.**  
Human approval is required before any Flow/Focus implementation batch.

Images are **principles**, not pixel specs. Do not copy brands, wallpaper, or exact chrome.

---

## Product principles (binding)

| Principle | Meaning |
|-----------|---------|
| Workspace-first | Modes change how the **desktop workspace** is presented — not how an AI chat looks |
| Applications as spatial objects | Real apps (and later groups) occupy the stage; modes change density/placement, not product identity |
| User-controlled layouts | The user switches mode and restores arrangements; nothing auto-transforms without intent |
| Productivity environment | Flow = multitask density; Focus = immersive density of the **same working set** |
| Assistant as companion | Persistent supporting rail in **both** modes; never the mode switcher owner |

### Current product hierarchy (preserve)

```text
Workspace
 ├── Applications
 ├── Layouts
 ├── Desktop Arrangements
 ├── Future Controls
 └── Assistant Companion
```

### Systems to reuse (do not replace)

| System | Role in Flow/Focus |
|--------|--------------------|
| `DesktopArrangement` | Named saved window membership + bounds; restore pathway |
| `WindowController` | Apply bounds/focus via Permission Gateway |
| Application registry | Workspace assets; launch paths |
| Existing UI shell | Home / Workspaces / Applications / Layouts + tools group |
| Observation / WorkspaceState | Facts about running windows — not mode logic |

### Explicit non-goals

- AI planning, recommendation engines, autonomous mode changes  
- Hidden user modelling or “infer focus because it’s 10:30”  
- Workflow automation that moves windows without user intent  
- New arrangement engine or fork of canvas Layout into OS tiling  
- Implementing all nine concept-02 presets  
- Scenic wallpaper / phone mirroring as mode prerequisites  

---

## Mode relationship (conceptual)

```text
Same workspace + same running applications
        │
        ├─► Flow  — high density, multitasking presentation
        │
        └─► Focus — low density, immersive presentation

User-driven switch (toggle / segmented control / slider metaphor)
Assistant rail remains present; stage density changes
DesktopArrangement restore remains the persistence path for window geometry
```

**Interpretation of concept-01 “Slide & Release”:** user-driven mode switch. Implementation may be a segmented control, toggle, or slider — not a literal marketing widget.

**Apps stay open:** Mode change must not quit applications. Geometry may compress/expand via governed restore/apply paths when implemented.

---

## Flow Mode

### Purpose

Enable **high-density multitasking**: the user sees and works across multiple applications and utilities in one coherent workspace environment.

### User scenario

> “I’m writing code while checking docs, chat, and music. I need everything visible and arrangeable without hunting through the taskbar.”

Steps (target experience):

1. Open Workspace with an active workspace.  
2. Enter **Flow** (default productive mode).  
3. See applications as spatial stage elements; use arrangements to save/restore desktop geometry.  
4. Optionally ask Assistant for help without leaving the workspace stage.  

### Visual hierarchy

1. **Application stage** (dominant) — apps as spatial objects / tiles / windows  
2. **Workspace chrome** — Home / Apps / Layouts (or equivalent product nav)  
3. **Arrangement / layout controls** — save, restore, mode switch  
4. **Utilities strip** (future) — audio / system tools when scheduled  
5. **Assistant companion rail** (secondary, persistent)  

### Application behaviour

| Behaviour | Rule |
|-----------|------|
| Visibility | Multiple apps visible / available simultaneously |
| Spatial role | Treated as stage assets (registry +, when controlled, OS windows) |
| Launch | Existing governed launch path only |
| Grouping | Future; not required for first Flow ship |
| Closing on mode switch | **Forbidden** — apps remain running |

### Layout behaviour

| Behaviour | Rule |
|-----------|------|
| Density | High — more panels / app surfaces visible |
| Desktop arrangements | Primary persistence for OS window geometry |
| Companion canvas zones | May remain as board practice; must not be confused with OS tiling |
| Mode switch | User-initiated; may apply a **Flow-oriented** arrangement variant when implemented |
| Auto-layout AI | **Forbidden** |

### Assistant behaviour

| Behaviour | Rule |
|-----------|------|
| Placement | Right-side companion rail (concept intent); same rail in Focus |
| Role | Explain, answer, organise existing information |
| Mode control | **Must not** switch Flow/Focus autonomously |
| Window movement | **Must not** call WindowController except via existing permissioned user intent paths |

### Controls visible

- Current workspace identity  
- Mode control (Flow selected)  
- Applications / Layouts / arrangements actions  
- Assistant companion affordance  
- Honest empty states when apps/arrangements missing  

### What is intentionally hidden

- Diagnostics / Developer tooling (keep in tools group, not Flow chrome)  
- Programme IV / batch engineering labels  
- Nine concept-02 layout presets  
- Autonomous “suggestions that execute”  
- Fake live HWND tiles when observation/control unavailable  

---

## Focus Mode

### Purpose

Enable **low-density immersive work**: reduce visual competition while keeping the same applications available and the Assistant reachable.

### User scenario

> “I need to deep-work in one primary surface. Keep my apps alive, shrink the clutter, leave Assistant available if I ask.”

Steps (target experience):

1. From Flow (or Home), switch to **Focus**.  
2. Stage compresses: fewer simultaneous surfaces; primary work emphasised.  
3. Apps remain running; user can expand back to Flow without relaunching.  
4. Assistant rail stays present but does not dominate.  

### Visual hierarchy

1. **Primary application / work surface** (dominant)  
2. **Minimal stage chrome** — mode switch + essential workspace identity  
3. **Compressed app affordances** — dock / cluster / reduced tiles (implementation choice)  
4. **Assistant companion rail** (stable secondary)  
5. **Utilities** — minimal (e.g. future thin audio bar); not a full utility wall  

### Application behaviour

| Behaviour | Rule |
|-----------|------|
| Visibility | Fewer apps emphasised; others available but not competing for space |
| Stay open | **Required** — Focus is presentation density, not session teardown |
| Spatial role | Compressed cluster / dock / primary+satellites (principle, not pixel layout) |
| Launch | Same governed paths; Focus must not invent AI launch |

### Layout behaviour

| Behaviour | Rule |
|-----------|------|
| Density | Low — essential surfaces only |
| Desktop arrangements | May restore a **Focus-oriented** saved arrangement when implemented |
| Expand/compress | User returns to Flow to expand; relationship is reversible |
| Wallpaper / scenic chrome | Concept inspiration only; not a Focus prerequisite |
| Auto-hide without user control | Avoid; user must understand what is hidden |

### Assistant behaviour

| Behaviour | Rule |
|-----------|------|
| Placement | Same companion rail as Flow |
| Prominence | Secondary; quieter chrome acceptable, not removed |
| Autonomy | No auto Focus entry, no auto mute/layout without approval |

### Controls visible

- Mode control (Focus selected)  
- Path back to Flow  
- Essential workspace identity  
- Assistant companion  
- Arrangement restore when relevant  

### What is intentionally hidden

- Dense multitasking grids and utility walls  
- Diagnostic / Developer surfaces  
- High-noise Assistant status matrices (prefer calm companion)  
- Automation / Files / Settings chrome until those product areas exist  

---

## Transition contract (design)

| Rule | Detail |
|------|--------|
| User-driven | Explicit control only |
| Reversible | Flow ↔ Focus without data loss |
| Persistence | Prefer DesktopArrangement (and optional mode metadata on arrangements) — **extend, don’t fork** |
| Permissions | Any OS window apply goes through existing CommandPipeline → PermissionGateway → WindowController |
| Failure honesty | If restore gaps exist, show them (existing restore diagnostics) — do not invent windows |

---

## Implementation readiness (not a build plan)

Prerequisite foundations already in tree:

- DAF-1a–1e observe → save → restore → arrangement UI  
- Application registry + Layouts stage presence (Cycles A / A.1 / 1)  
- Assistant demoted to tools / companion presentation  

Still required before coding (human-approved Milestone B):

1. Mode model decision (workspace-level vs arrangement-linked)  
2. UI control choice (toggle vs segmented vs slider metaphor)  
3. Whether first ship is **chrome density only**, or also **OS geometry apply**  
4. Human visual review checkpoint  

---

## Explicit confirmation

> Design charter only. No Flow/Focus code in this batch.  
> No new AI engines. No replacement of DesktopArrangement.  
> Assistant remains subordinate. Await human approval before implementation.
