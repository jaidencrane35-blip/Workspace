# Programme I — Implementation Contract 1  
# Product Shell Capability Inventory

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme I — Workspace Product Capability](PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md) |
| **Contract** | Implementation Contract 1 |
| **Nature** | Inventory only — no feature implementation; no architecture change; no production behaviour change |
| **Audited tip** | Branch `cursor/programme-i-workspace-product-capability-34a5` (includes architectural evidence report) |
| **Related evidence** | [Architectural Evidence Report](../02-Architecture/ARCHITECTURAL-EVIDENCE-REPORT.md) |
| **Date** | 2026-07-30 |

---

## Contract objective (satisfied by this document)

Establish a complete **Product Shell Capability Inventory**.

The repository archaeology identified that the underlying architecture is substantially more mature than the product shell. Before new product capability is implemented, the Workspace product surface must be mapped against capabilities that already exist.

**No new runtime architecture is introduced by this contract.**

---

## Method

- Repository evidence only (product React shell, IPC registrations, kernel commands, domain models).
- Primary product surfaces: Stage (`layouts`), Apps (`applications`), Profiles (`workspaces`), Assistant rail, arrangements panel, Flow/Focus chrome.
- Diagnostic surfaces (`OperatorConsole`, `WorkspaceIntelligencePanel`) counted only when they are the sole exposure of a capability.
- Exposure classes for Programme I north-star mapping:
  - **Already exposed**
  - **Partially exposed**
  - **Hidden but implemented**
  - **Missing UI only**
  - **Missing backend**
  - **Not implemented**
- Gap limitation classes (exactly one per incomplete capability):
  - UI composition
  - Existing API not surfaced
  - Missing IPC exposure
  - Missing desktop integration
  - Missing persistence
  - Missing product workflow
  - Architectural constraint

---

# 1. Product Shell Capability Inventory

## 1.1 Product shell navigation (entry map)

| Chrome / path | View id / mount | Class |
|---------------|-----------------|-------|
| Stage | `layouts` → `WorkspaceApplicationStage` + `DesktopArrangementPanel` | Product |
| Apps | `applications` → `ApplicationsPanel` | Product |
| Profiles | `workspaces` → `WorkspaceSwitcher` | Product |
| Assistant | Rail → `AssistantCompanionRail` → `AssistantIntelligencePanel` | Product sidecar |
| Flow \| Focus | `WorkModeSwitch` (chrome) | Product chrome |
| Tools → Diagnostics | `diagnostics` → `OperatorConsole` | Diagnostic |
| Tools → Developer | `developer` → `WorkspaceIntelligencePanel` | Diagnostic |

Sources: `app/src/App.tsx`, `app/src/lib/productViews.ts`.

---

## 1.2 Inventory rows (user-facing capabilities)

### A. Workspace management

| Field | Evidence |
|-------|----------|
| **Capability** | Create / list / activate named workspace profiles |
| **UI entry point** | Profiles tab → `WorkspaceSwitcher` (list click; “Add profile”) |
| **Supporting backend** | Tauri `commands/workspace.rs` → `CommandHandler::{create,list,get}_workspace(s)`; settings for active id |
| **Runtime ownership** | Durable workspace rows + settings (`active_workspace_id`) |
| **WorkspaceState dependency** | None (desktop observation remains global) |
| **Permission Gateway** | Yes — `workspace.write` / `workspace.read`; `settings.write` / `settings.read` |
| **Status** | Fully exposed for profile CRUD/activation; does not rematerialise OS desktop geometry on switch |

### B. Desktop layouts — arrangements (remember / restore)

| Field | Evidence |
|-------|----------|
| **Capability** | Capture, list, restore named desktop arrangements |
| **UI entry point** | Stage → arrangements rail `DesktopArrangementPanel` (“Remember layout”) |
| **Supporting backend** | `DesktopArrangementService`; migrations `079`–`080`; WindowController on restore |
| **Runtime ownership** | Durable DesktopArrangement store; restore via OS control |
| **WorkspaceState dependency** | Capture uses observation; Stage re-reads WorkspaceState after restore epoch bump |
| **Permission Gateway** | Yes — `desktop.write` (capture), `desktop.read` (list), `desktop.restore` (restore) |
| **Status** | Fully exposed on arrangements rail |

### C. Desktop layouts — Stage working sets

| Field | Evidence |
|-------|----------|
| **Capability** | Save selection as arrangement subset; overlay working-set membership on Stage |
| **UI entry point** | Stage map — Working set `<select>` + “Save selection” |
| **Supporting backend** | Same arrangement IPC (`capture` with `memberHwnds`, `list`) |
| **Runtime ownership** | Arrangement rows + Stage UI selection state |
| **WorkspaceState dependency** | Tile keys from WorkspaceState windows |
| **Permission Gateway** | Yes — `desktop.write` / `desktop.read` |
| **Status** | Partially exposed — save/list/overlay only; **no Restore** on working-set control |

### D. Desktop layouts — canvas / zones

| Field | Evidence |
|-------|----------|
| **Capability** | Canvas layout graph and zones |
| **UI entry point** | **None** on product shell. Zone seed only on Diagnostics `OperatorConsole` |
| **Supporting backend** | `LayoutService` / zone commands; DB layout tables; orphan helper `app/src/lib/layoutPersistence.ts` (not imported by App) |
| **Runtime ownership** | Durable canvas Layout — distinct from DesktopArrangement |
| **WorkspaceState dependency** | None |
| **Permission Gateway** | Yes when invoked — `layout.read` / `layout.write`; zone caps |
| **Status** | Hidden but implemented (backend); diagnostic-only zone seed |

### E. Running applications — observe

| Field | Evidence |
|-------|----------|
| **Capability** | Observe running desktop applications/windows |
| **UI entry point** | Stage spatial map + Focus process dock |
| **Supporting backend** | Observation → `WorkspaceStateEngine` → `get_workspace_state` via `workspaceStateClient` |
| **Runtime ownership** | WorkspaceState (`windows`, `active_applications`, …) |
| **WorkspaceState dependency** | Direct |
| **Permission Gateway** | Yes on read path — `desktop.read` / observation gate |
| **Status** | Fully exposed on Stage |

### F. Running applications — library register / launch

| Field | Evidence |
|-------|----------|
| **Capability** | Manual application registry and governed launch |
| **UI entry point** | Apps tab (`ApplicationsPanel`); Stage secondary Library `<details>` |
| **Supporting backend** | Application repository; `launch_application` → ProcessLauncher |
| **Runtime ownership** | Durable applications scoped to profile |
| **WorkspaceState dependency** | Refresh shared WorkspaceState after launch |
| **Permission Gateway** | Yes — `application.read` / `application.write` / `application.launch` |
| **Status** | Fully exposed as library (not OS install discovery) |

### G. Window interaction — select / focus

| Field | Evidence |
|-------|----------|
| **Capability** | Select tiles; focus HWND via WindowController |
| **UI entry point** | Stage — click select; double-click / Enter / Focus button |
| **Supporting backend** | `focus_desktop_window` → DesktopArrangementService / WindowController |
| **Runtime ownership** | OS focus via windows-integration |
| **WorkspaceState dependency** | Tile model from WorkspaceState |
| **Permission Gateway** | Yes — `desktop.restore` |
| **Status** | Fully exposed for select/focus; no move/resize/minimize product UI |

### H. Window interaction — Flow / Focus organisation

| Field | Evidence |
|-------|----------|
| **Capability** | Flow (all windows + relationships) vs Focus (primary process map + dock) |
| **UI entry point** | Chrome `WorkModeSwitch`; Stage `organiseStageForWorkMode` / related keys |
| **Supporting backend** | No mode IPC — localStorage preference; organisation consumes WorkspaceState groups/semantics/attention |
| **Runtime ownership** | Frontend presentation over WorkspaceState |
| **WorkspaceState dependency** | Direct (`window_groups`, semantics, attention preferred keys) |
| **Permission Gateway** | No for mode toggle; yes if subsequent focus |
| **Status** | Fully exposed as Stage organisation; **not** OS geometry apply |

### I. Workspace switching

| Field | Evidence |
|-------|----------|
| **Capability** | Switch active named profile |
| **UI entry point** | Profiles list |
| **Supporting backend** | `update_settings` active workspace id; reload apps/zones |
| **Runtime ownership** | Settings + profile-scoped resources |
| **WorkspaceState dependency** | Desktop projection remains global across profile switch |
| **Permission Gateway** | Yes — `settings.write` |
| **Status** | Partially exposed relative to “switch desktop workspace” product meaning (profile switch ≠ arrangement restore) |

### J. Layout persistence (product meaning)

| Field | Evidence |
|-------|----------|
| **Capability** | Persist desktop organisation for later restore |
| **UI entry point** | Arrangements rail (primary); Stage working-set save (subset) |
| **Supporting backend** | DesktopArrangement persistence |
| **Runtime ownership** | Arrangement store |
| **WorkspaceState dependency** | Observation-backed capture |
| **Permission Gateway** | Yes — `desktop.write` / `desktop.read` |
| **Status** | Fully exposed via arrangements; canvas persistence not product-exposed |

### K. Arrangement restoration

| Field | Evidence |
|-------|----------|
| **Capability** | Restore saved arrangement bounds/focus through Gateway |
| **UI entry point** | Arrangements rail **Restore** |
| **Supporting backend** | `restore_desktop_arrangement` → set_bounds (+ focus) |
| **Runtime ownership** | WindowController |
| **WorkspaceState dependency** | Post-restore Stage refresh |
| **Permission Gateway** | Yes — `desktop.restore` |
| **Status** | Fully exposed on arrangements rail |

### L. Multi-monitor presentation

| Field | Evidence |
|-------|----------|
| **Capability** | Present windows across observed monitors |
| **UI entry point** | Stage map (monitor union plane; monitor labels/meta) |
| **Supporting backend** | Capture monitors → WorkspaceState.monitors |
| **Runtime ownership** | WorkspaceState |
| **WorkspaceState dependency** | Direct |
| **Permission Gateway** | Read path `desktop.read` |
| **Status** | Partially exposed — spatial presentation only; no per-monitor management UI |

### M. Assistant workspace interaction

| Field | Evidence |
|-------|----------|
| **Capability** | Ask about desktop / enrich compose from WorkspaceState |
| **UI entry point** | Assistant companion rail |
| **Supporting backend** | Local answers from WorkspaceState; else `compose_workspace_assistant_turn`; optional Programme IV package gets |
| **Runtime ownership** | Frontend companion + Programme IV assistant services |
| **WorkspaceState dependency** | Direct for local Q&A and enrich |
| **Permission Gateway** | Yes on compose/package IPC (`work_context.*`); local answers frontend-only |
| **Status** | Fully exposed as sidecar Ask; evidence packages optional advanced |

### N. Application grouping (user-visible)

| Field | Evidence |
|-------|----------|
| **Capability** | See/use groups of related windows/apps |
| **UI entry point** | Stage related-tile highlighting (Flow); Focus process groups; attention/semantic cues |
| **Supporting backend** | WorkspaceState `window_groups` + semantics relationships (projection) |
| **Runtime ownership** | WorkspaceState planes |
| **WorkspaceState dependency** | Direct |
| **Permission Gateway** | Read path |
| **Status** | Partially exposed — read-only visual grouping; no user-authored group edit/control UI |

### O. Lock workspace arrangements

| Field | Evidence |
|-------|----------|
| **Capability** | Lock arrangements against change |
| **UI entry point** | None |
| **Supporting backend** | None found |
| **Runtime ownership** | N/A |
| **WorkspaceState dependency** | N/A |
| **Permission Gateway** | N/A |
| **Status** | Not implemented |

### P. Discover installed applications (OS)

| Field | Evidence |
|-------|----------|
| **Capability** | Enumerate installed OS applications |
| **UI entry point** | None |
| **Supporting backend** | None (manual registry only) |
| **Runtime ownership** | N/A |
| **WorkspaceState dependency** | N/A |
| **Permission Gateway** | N/A |
| **Status** | Not implemented |

### Q. Organise windows (move / resize from product)

| Field | Evidence |
|-------|----------|
| **Capability** | User moves/resizes windows from Workspace UI (beyond restore) |
| **UI entry point** | None for interactive organise; restore applies bounds |
| **Supporting backend** | WindowController `set_bounds` exists (used by restore) |
| **Runtime ownership** | windows-integration |
| **WorkspaceState dependency** | Indirect |
| **Permission Gateway** | Would be `desktop.restore` / write path when used |
| **Status** | Hidden but implemented (control API via restore only) — interactive organise UI missing |

### R. Manage desktop from a single environment (coherence)

| Field | Evidence |
|-------|----------|
| **Capability** | Single coherent product shell for desktop management |
| **UI entry point** | Stage-centred shell with Apps/Profiles/Assistant |
| **Supporting backend** | Shared `workspaceStateClient` |
| **Runtime ownership** | Product shell |
| **WorkspaceState dependency** | Yes |
| **Permission Gateway** | Per-action |
| **Status** | Partially exposed — Stage owns running desktop; residual dual concepts (profile vs arrangement vs canvas); diagnostic mega-surfaces still present under Tools |

---

# 2. Capability Exposure Matrix

Programme I north-star capabilities mapped to exposure class (repository evidence).

| Programme I capability | Exposure class | Primary product surface | Notes |
|------------------------|----------------|-------------------------|-------|
| Discover applications | **Not implemented** (OS discovery) / library **Already exposed** (manual) | Apps / Stage library | Manual registry ≠ OS discovery |
| Observe running applications | **Already exposed** | Stage | Via WorkspaceState |
| View desktop windows | **Already exposed** | Stage | Spatial map |
| Create workspace layouts | **Partially exposed** | Arrangements rail + Stage working-set save | Capture/save exist; “layout” = arrangement not canvas; no interactive editor |
| Organise windows | **Partially exposed** | Stage Flow/Focus + focus; restore applies geometry | Interactive move/resize UI missing; `set_bounds` used by restore |
| Group applications | **Partially exposed** | Stage relatedness / Focus groups | Read-only projection; no user group authoring |
| Save layouts | **Already exposed** | Arrangements capture | Canvas save = Hidden but implemented |
| Restore layouts | **Already exposed** | Arrangements Restore | Working-set picker has no Restore |
| Switch between workspaces | **Partially exposed** | Profiles | Switches profile scope; not OS desktop rematerialisation |
| Lock workspace arrangements | **Not implemented** | — | No model/UI/IPC |
| Manage desktop from a single environment | **Partially exposed** | Stage-centred shell | Coherent Stage; remaining product/diagnostic/orphan surfaces |

### Supplementary exposure (supporting, not north-star line items)

| Capability | Exposure class |
|------------|----------------|
| Multi-monitor presentation | Partially exposed |
| Assistant workspace interaction | Already exposed (sidecar) |
| Canvas zone layouts | Hidden but implemented |
| Flow/Focus OS geometry apply | Not implemented (presentation only) |
| Arrangement working-set restore | Missing UI only (restore API exists; Stage picker lacks Restore) |

---

# 3. Product Surface Ownership Map

| Product surface | Owns (user-visible) | Must not own | Runtime authority consumed |
|-----------------|---------------------|--------------|----------------------------|
| **Stage** | Running desktop representation; select/focus; Flow/Focus organisation; working-set overlay; secondary library launch | OS APIs; inventing groups; canvas zones | WorkspaceState; focus/arrangement IPC |
| **Arrangements rail** | Capture/list/restore remembered layouts | Observation SoT; WindowController directly | Arrangement service via Gateway |
| **Apps** | Optional application library register/launch | Running-desktop observation (Stage owns) | Application IPC; WorkspaceState refresh after launch |
| **Profiles** | Named workspace profiles + active selection | Global desktop observation remapping | Workspace + settings IPC |
| **Flow/Focus chrome** | Presentation mode preference | OS geometry policy | localStorage + Stage consumers of WorkspaceState |
| **Assistant rail** | Ask/answer; explain WorkspaceState conclusions | Desktop control; parallel desktop model | WorkspaceState; compose IPC |
| **Shared state client** | Single frontend hold of WorkspaceState | Projection logic | `get_workspace_state` |
| **Diagnostics / Developer** | Operator/intelligence diagnostics | Product SoT for desktop | May call aggregators / raw diagnostics |
| **Canvas Layout stack** | (No product owner today) | HWND store (DAF) | Orphan relative to Stage |
| **windows-integration** | OS capture/control | Product UX | Called only via kernel |
| **Permission Gateway** | Privileged operation admission | Product presentation | All mutating/privileged IPC |
| **WorkspaceStateEngine** | Interpreted desktop runtime projection | Persistence of projection as durable SoT | Observation facts |

---

# 4. Capability Gap Inventory

Only capabilities that are **not fully exposed** as Programme I product meaning. Exactly one limiting factor each.

| ID | Capability | Current exposure | Limiting factor (exactly one) | Evidence of limitation |
|----|------------|------------------|-------------------------------|------------------------|
| G1 | Discover installed applications (OS) | Not implemented | **Missing desktop integration** | No Start Menu / App Paths enumerator; library is manual |
| G2 | Create/edit interactive desktop layouts | Partially exposed | **Missing product workflow** | Capture/restore exist; no interactive layout editor workflow on Stage |
| G3 | Organise windows (interactive move/resize) | Partially exposed / hidden control | **UI composition** | `set_bounds` exists for restore; no Stage drag/organise UI |
| G4 | User-authored application/window groups | Partially exposed | **Missing product workflow** | Groups projected on WorkspaceState; no create/edit group UI |
| G5 | Switch workspaces as desktop rematerialisation | Partially exposed | **Missing product workflow** | Profile switch scopes library/arrangements; does not auto-restore a layout |
| G6 | Lock workspace arrangements | Not implemented | **Missing persistence** | No lock field/API/UI in arrangement model |
| G7 | Working-set Restore on Stage | Partially exposed | **Existing API not surfaced** | `restore_desktop_arrangement` used by arrangements rail; Stage working-set control has no Restore |
| G8 | Multi-monitor management UI | Partially exposed | **UI composition** | Monitors on WorkspaceState and Stage map; no management/assignment surface |
| G9 | Canvas layout as product layout | Hidden but implemented | **Architectural constraint** | Desktop Reality / DAF separate canvas Layout from HWND arrangements; Stage does not own canvas |
| G10 | Flow/Focus applies OS geometry | Not implemented | **Architectural constraint** | Docs place OS apply in later milestone; current mode is Stage organisation only |
| G11 | Single-environment coherence (eliminate residual dual surfaces) | Partially exposed | **UI composition** | Stage-centred but Tools diagnostics + orphan layout helper + residual CSS remain |
| G12 | `get_desktop_arrangement` single-get | Hidden but implemented | **Existing API not surfaced** | Registered IPC; product uses list only |

**Not listed as gaps (fully exposed for Programme I product meaning):** observe running apps; view windows; save arrangements; restore arrangements (rail); library register/launch; select/focus; Assistant ask; profile create/list/activate.

---

# 5. Ordered list of implementation opportunities  
## (ranked by architectural leverage only)

Ranking criterion: **architectural leverage** = how much existing authoritative architecture becomes user-visible product capability per unit of change, without new engines or ownership duplication.

Do **not** treat this list as an approved backlog. Principal Architect authors Contract 2.

| Rank | Opportunity (description only) | Leverage rationale (evidence) | Related gaps |
|------|--------------------------------|-------------------------------|--------------|
| 1 | Surface existing arrangement **Restore** on Stage working-set control | Reuses `restore_desktop_arrangement` + Gateway + WindowController already product-proven on arrangements rail | G7 |
| 2 | Coherent “switch profile → optional restore remembered arrangement” product workflow using existing capture/restore + profile active id | Composes Profiles + Arrangement persistence already implemented; no new runtime | G5 |
| 3 | Stage/product UI that invokes existing `set_bounds` path for deliberate organise-from-Workspace (still via Gateway) | Control already exists for restore; missing is product UI composition | G3 |
| 4 | Product workflow to author/edit groups against existing `window_groups` / arrangement membership facts without new grouping engines | Projection and arrangement membership already exist | G4 |
| 5 | Product shell coherence pass: remove/hide orphan canvas helper paths from agent confusion; keep Stage as sole running-desktop surface | Ownership already decided in V15; residual UI/docs debt only | G11, G9 |
| 6 | Multi-monitor presentation enrichment from existing `WorkspaceState.monitors` | Data already on Stage plane | G8 |
| 7 | Interactive layout creation/editing workflow over DesktopArrangement (not canvas Layout) | Extends arrangement store; avoids resurrecting canvas HWND anti-pattern | G2 |
| 8 | OS installed-application discovery via windows-integration | Requires new desktop integration surface; higher platform cost | G1 |
| 9 | Arrangement **lock** model + persistence + UI | Requires new persistence field/API before UI | G6 |
| 10 | Flow/Focus OS geometry apply | Architectural constraint / later milestone; expands control policy beyond current Stage organisation | G10 |

---

## Stop condition

This Implementation Contract 1 is **complete**:

1. A deterministic Product Shell Capability Inventory has been produced (sections 1–5).
2. **No repository production behaviour has been changed** by this contract (documentation only).

Output is intended for the Principal Architect to author **Implementation Contract 2**.

---

## Appendix — Key evidence paths

- `app/src/App.tsx`, `app/src/lib/productViews.ts`
- `app/src/components/WorkspaceApplicationStage.tsx`
- `app/src/components/DesktopArrangementPanel.tsx`
- `app/src/components/ApplicationsPanel.tsx`
- `app/src/components/WorkspaceSwitcher.tsx`
- `app/src/components/AssistantIntelligencePanel.tsx`
- `app/src/lib/workspaceStateClient.ts`, `app/src/lib/stageDesktopUi.ts`, `app/src/lib/workMode.ts`
- `app/src/lib/layoutPersistence.ts` (orphan)
- `docs/03-Engineering/IPC-SURFACE.md`
- `docs/01-Product/PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md`
- `docs/02-Architecture/ARCHITECTURAL-EVIDENCE-REPORT.md`

---

*End of Implementation Contract 1 output.*
