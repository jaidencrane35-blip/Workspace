# DAF-1d — Desktop Arrangement Capture & Governed Restore

| Field | Value |
|-------|-------|
| **Purpose** | Explicit capture of observed windows into DesktopArrangement; user-requested restore through PermissionGateway → WindowController |
| **Batch** | DAF-1d |
| **Owner** | Kernel service + commands + IPC; domain restore planning; database geometry columns |
| **Status** | Implemented (foundation) |
| **Related** | [DAF-1A-WINDOW-CONTROLLER.md](DAF-1A-WINDOW-CONTROLLER.md), [DAF-1B-WINDOW-OBSERVATION.md](DAF-1B-WINDOW-OBSERVATION.md), [DAF-1C-DESKTOP-ARRANGEMENT.md](DAF-1C-DESKTOP-ARRANGEMENT.md) |

---

## 1. Product intent

Users save working environments and reopen them later. Workspace restores the chosen setup **only when the user requests it**. The user remains the authority; Workspace does not decide, invent, or auto-arrange.

---

## 2. Owned pathway

```text
Observed Windows
        |
        v
DesktopArrangement  (capture / persist)
        |
        v
Restore Request     (explicit user command)
        |
        v
CommandPipeline
        |
        v
PermissionGateway   (desktop.restore)
        |
        v
WindowController
        |
        v
Operating System
```

Never: Assistant → move windows. Never bypass CommandPipeline, PermissionGateway, or WindowController.

---

## 3. Capture behaviour

1. Use latest observation snapshot (optionally refresh capture first).
2. Record membership + optional bounds from observed facts.
3. Persist via `DesktopArrangementRepository` (overwrite only when the same `arrangement_id` is captured again — explicit).
4. Capability: `desktop.write`.

Must **not** infer importance, invent groups, or silently overwrite unrelated arrangements.

---

## 4. Restore behaviour

1. Load saved `DesktopArrangement`.
2. Query current observed windows.
3. Match saved identities (`stable_window_id`, then `hwnd`) — deterministic.
4. Produce restore diagnostics (available / missing / identity-known-missing).
5. For **Available** entries with stored bounds: `set_bounds` (optional `focus`) via WindowController.
6. Execute only after Gateway allows `desktop.restore`.

### Missing windows

Record missing identity, unavailable state, and restore gap. Do **not** launch replacements, guess apps, or omit diagnostics.

### Geometry

DAF-1c stored membership only. DAF-1d persists optional `x,y,width,height` at capture so restore can call `set_bounds` without inventing layout. Entries without bounds are diagnostic-only (no move).

### Partial apply / rollback posture

On mid-restore controller failure: stop further applies, record applied vs failed vs remaining gaps. Do not invent repairs or reverse earlier OS moves automatically (no silent “fix-up” automation).

---

## 5. Permission

| Capability | Purpose |
|------------|---------|
| `desktop.read` | Observe / diagnose / list / get |
| `desktop.write` | Capture / save arrangement |
| `desktop.restore` | Apply restore through WindowController |

Local user standard set includes write + restore. AI / automation actors start empty (default deny).

---

## 6. IPC (minimal)

| Command | Kind | Capability |
|---------|------|------------|
| `capture_desktop_arrangement` | mutation | `desktop.write` |
| `restore_desktop_arrangement` | mutation | `desktop.restore` |
| `get_desktop_arrangement` | query | `desktop.read` |
| `list_desktop_arrangements` | query | `desktop.read` |

---

## 7. Non-goals

Automatic layouts, snapping, DnD editing, AI suggestions, assistant execution, background auto-restore, audio, workspace modes, Canvas Layout merge.

---

## 8. Future UI integration points

Shell can expose Save / Restore actions that invoke the IPC above. No UI chrome is required for DAF-1d completion.
