# DAF Architecture Audit (DAF-0)

| Field | Value |
|-------|-------|
| **Purpose** | Repository archaeology before Desktop Arrangement Foundation implementation |
| **Status** | Complete — audit only; **no DAF control code in this batch** |
| **Date** | 2026-07-29 |
| **Programme** | Desktop Arrangement Foundation (DAF) |
| **Related** | [ENGINEERING-GOVERNANCE.md](ENGINEERING-GOVERNANCE.md), [WORKSPACE-VISUAL-DIRECTION.md](../01-Product/WORKSPACE-VISUAL-DIRECTION.md), [PRODUCT-VISION-REALIGNMENT-AUDIT.md](../01-Product/PRODUCT-VISION-REALIGNMENT-AUDIT.md) |

---

## 0. Alignment check (DAF-0)

| # | Confirm | Pass |
|---|---------|------|
| 1 | Workspace only | Y |
| 2 | Desktop management vision | Y |
| 3 | Reuse existing foundations | Y |
| 4 | No foreign AI-context projects | Y |
| 5 | No new subsystem coded in DAF-0 | Y |

---

## 1. Audit scope

Inspected:

- `packages/windows-integration`
- Application discovery / launch / capture paths
- Workspace entities
- Zone / canvas layout system
- Persistence models (SQLite migrations)
- IPC commands (`app/src-tauri`)
- Permission Gateway boundaries

---

## 2. Existing capabilities

### 2.1 windows-integration (`packages/windows-integration`)

| Capability | Status | Notes |
|------------|--------|-------|
| `DesktopCapturer` | **Exists** (Win32 / stub) | Enumerate monitors + windows; geometry, focus, minimize |
| `WindowEnumerator` | **Exists** | Legacy snapshots from capture |
| `ProcessLauncher` | **Exists** | Spawn process from executable path (Windows); simulated on non-Windows |
| Move / resize / focus APIs | **Missing** | No `SetWindowPos` (or equivalent) control surface |
| Audio session control | **Missing** | Not in crate |
| Installed-app discovery | **Missing** | No Start Menu / AppX / registry scan |

**Ownership:** Only this crate may talk to OS window/process APIs (DEC-008).

### 2.2 Application model

| Capability | Status | Notes |
|------------|--------|-------|
| Manual application registry | **Exists** | Name, identifier, `executable_path` |
| `create_application` / get / delete IPC | **Exists** | `commands/resources.rs` |
| `launch_application` | **Exists** | Via Permission Gateway → launcher |
| OS installed-app discovery | **Missing** | MVP lists it; not implemented |
| Binding HWND ↔ Application | **Partial** | Environment model matches heuristically; observational only |

### 2.3 Capture / observation

| Capability | Status | Notes |
|------------|--------|-------|
| `capture_workspace_observation` | **Exists** | Persists desktop facts |
| WorkspaceState / Environment projections | **Exists** | Informational; `authority_effect: none` |
| Detect move/resize/focus deltas | **Exists** | Facts only — does not control |

### 2.4 Workspace / zone / canvas

| Capability | Status | Notes |
|------------|--------|-------|
| Workspace CRUD | **Exists** | Named containers; active id in settings |
| Zones | **Exists** | Graph children; canvas placement |
| Canvas `Layout` (DEC-009) | **Exists** | Viewport + nodes (bounds, z, locked/collapsed/hidden) |
| Canvas UI | **Exists** | `CanvasShell.tsx` drag/resize/pan/zoom; SQLite persist |
| Canvas = OS window tiling | **No** | Zones are shell rectangles, not HWNDs |

### 2.5 Persistence

| Area | Migrations (examples) | Role |
|------|----------------------|------|
| Settings / workspace | `001`, `002` | Active workspace, named workspaces |
| Graph | `006` | Contains edges |
| **Canvas layout** | `007_layout.sql` | Spatial shell layout — **not** desktop arrangement |
| Applications | `008_application_executable.sql` | Executable path |
| Observation | `020+` | Captured desktop facts |
| AI / intelligence | `010+`, `045–078` | Future capability infra — **out of DAF mutate scope** |

### 2.6 IPC

Relevant desktop-adjacent commands already registered:

- Workspace / zone / application / widget / layout CRUD
- `launch_application`
- `capture_workspace_observation`, `get_workspace_state`, environment/composition generators
- Status / health / settings

**Gap:** No IPC family for “save desktop arrangement” / “restore arrangement” / “set window bounds”.

### 2.7 Permission Gateway

| Item | Location |
|------|----------|
| `PermissionGateway::evaluate` | `packages/kernel/src/security/gateway.rs` |
| Gates | `PermissionGate`, `StandardGate`, `AllowAllPermissionGate` |
| Pattern | Commands request capability → gateway decides → service executes |

**Rule for DAF:** Any window **mutation** must be a capability-gated command. Assistant packages must not call Win32 or bypass the gateway.

---

## 3. Missing capabilities (vs product / visual direction)

| Missing | Why it matters |
|---------|----------------|
| Window control (move/resize/focus/show) | Core of “arrange real application windows” |
| Desktop arrangement persistence model | Save/restore of **OS** geometries |
| Work-mode switch (Flow ↔ Focus) | Concept-01 primary interaction |
| App groups + group transform | Concept + product definition |
| Installed app discovery | Quick launch / Apps surface |
| Product chrome realignment | Assistant must become sidecar, not primary tab peer |
| Audio mixer | Concept utility; schedule after control baseline |

---

## 4. Reusable foundations (do not reinvent)

| Foundation | Reuse how |
|------------|-----------|
| `windows-integration` traits | **Extend** with a `WindowController` (or equivalent) trait; Win32 impl + stub |
| Capture / observation | Feed “save arrangement” from latest capture; verify restore |
| Application entity + launch | Target apps in arrangements; launch if missing on restore (policy TBD) |
| Permission Gateway + audit | Gate `desktop.window.move` (names TBD) capabilities |
| Command pipeline + IPC envelope | Same path as existing commands |
| SQLite + migrations | New tables for arrangements — **separate** from `007_layout` |
| Canvas layout | Keep as companion shell board; do not overload as HWND store |
| Assistant Programme IV | Later: explain/suggest mode switch; **not** execute moves |

---

## 5. Systems to modify (DAF implementation batches)

| System | Change type |
|--------|-------------|
| `packages/windows-integration` | Add control API (edit in place) |
| `packages/domain` | New `desktop_arrangement` (name TBD) module — **not** a fork of canvas `layout` |
| `packages/database` | New migration(s) for arrangements |
| `packages/kernel` | Arrangement service + gated commands |
| `app/src-tauri` | Thin IPC adapters |
| `app/src` | Chrome realignment; Layouts/Apps surfaces; Assistant as panel |

---

## 6. Systems not to touch (freeze / leave alone)

| System | Reason |
|--------|--------|
| Programme II–IV evidence/assistant **engines** | Frozen expansion; keep as capability infra |
| Recommendation / Decision Engine clone slices | Not layout managers; do not extend for DAF |
| Environment / Composition / Profile generators | Stay observational; do not grant them HWND authority |
| OperatorConsole kitchen sink | Diagnostic only; do not grow as product UI |
| Canvas `Layout` schema | Do not silently store HWNDs in layout nodes without a new model |
| AI planning / memory / personalisation write paths | Out of DAF-1 scope |

---

## 7. Naming clarity (required)

| Term | Meaning |
|------|---------|
| **Canvas layout** (`packages/domain/src/layout`) | Spatial arrangement of **Zones** inside the Tauri companion UI |
| **Desktop arrangement** (DAF) | Saved arrangement of **real OS windows** (and later groups) |
| **Work mode** | User-facing density preset (e.g. Flow, Focus) implemented as arrangements + transform policy |
| **Assistant** | Supporting UI + projection stack; never desktop controller |

Human engineers must not confuse these.

---

## 8. Proposed DAF architecture

### 8.1 Principle

```text
UI (Layouts / Apps / mode switch)
  → Tauri IPC
    → Kernel command + Permission Gateway
      → DesktopArrangementService (domain rules + persistence)
        → windows-integration WindowController (OS mutations)
        → DesktopCapturer (read facts for save/verify)
```

Assistant (future):

```text
Assistant may: explain, suggest, draft a request
Assistant must not: call WindowController, bypass Gateway, invent geometries
```

### 8.2 Suggested layers (implementation order)

| Phase | Deliverable | Depends on |
|-------|-------------|------------|
| **DAF-0** (complete) | Docs, visual refs, governance, audit | — |
| **DAF-1a** (complete) | `WindowController` + stub/Win32 + tests | windows-integration |
| **DAF-1b** (complete) | Window observation & identity foundation (edit-in-place) | capture + observation |
| **DAF-1c** | `DesktopArrangement` model + SQLite | domain + database |
| **DAF-1d** | Save from capture / restore via controller | kernel + IPC |
| **DAF-1e** | Two work modes + explicit user switch | UI + arrangements |
| **DAF-1f** | Chrome realignment (Assistant sidecar) | frontend; **human visual checkpoint** |

Exact batch splits may adjust; do not skip Gateway or merge canvas layout with desktop arrangement.

### 8.3 Ownership table (proposed)

| Responsibility | Owner |
|----------------|-------|
| Win32 / OS window mutation | `workspace-windows-integration` |
| Arrangement schema + validation | `workspace-domain` |
| Persistence | `workspace-database` |
| Commands, permissions, orchestration | `workspace-kernel` |
| IPC adapters | `app/src-tauri` |
| Presentation / chrome | `app/src` |
| Assistant projections | Existing assistant packages (read/request only) |

### 8.4 Non-responsibilities (hard)

DAF architecture **deliberately does not**:

- Create an AI window manager
- Create a layout recommendation engine
- Create an autonomous workspace agent
- Allow Assistant to move windows directly
- Replace Windows as the OS
- Implement all nine concept layouts at once
- Delete Programme IV

---

## 9. Risk register

| Risk | Mitigation |
|------|------------|
| Colliding with canvas `Layout` | Separate module + docs; different IPC names |
| Control without permission | Mandatory Gateway capabilities + audit events |
| Linux CI pretending to tile | Stubs return clear unsupported; tests platform-gated |
| Scope creep into audio/phone | Park in DAF-2+ per visual direction |
| AI relapse (new engines) | Governance freeze + alignment check |

---

## 10. Gate to DAF-1

DAF-1 coding may start only when:

1. This audit is merged (or accepted) in-repo  
2. Visual direction + governance docs are in-repo  
3. Batch alignment check for DAF-1 is filled and passes  
4. First architecture doc for DAF-1a (`WindowController`) exists before large diffs  

---

## Explicit confirmation

> DAF builds desktop window arrangement on existing Tauri / IPC / Gateway / SQLite / windows-integration foundations.  
> Assistant remains a supporting layer.  
> No autonomous or AI-driven window control.
