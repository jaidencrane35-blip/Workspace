# DAF-1c — DesktopArrangement Persistence Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Persist named relationships between Workspace and observed desktop windows |
| **Batch** | DAF-1c |
| **Owner** | `workspace-domain` (`desktop_arrangement`) + `workspace-database` repository |
| **Status** | Implemented (persistence foundation) |
| **Related** | [DAF-1B-WINDOW-OBSERVATION.md](DAF-1B-WINDOW-OBSERVATION.md), [DAF-ARCHITECTURE-AUDIT.md](DAF-ARCHITECTURE-AUDIT.md) |

---

## 1. Why this exists

> Workspace must be able to remember an arrangement before it can restore or manipulate one.

DAF-1a = control API. DAF-1b = observation/identity. DAF-1c = **durable “what belongs together”** records.

This is **not** a window manager, layout engine, or AI-assisted arrangement.

---

## 2. Problem it solves

- No place to save named desktop arrangements that reference real window identities
- Canvas `Layout` must not be overloaded with HWNDs
- Future restore needs a stable persistence contract and missing-window diagnostics

---

## 3. Responsibilities

* Desktop arrangement domain model
* Arrangement persistence contracts + SQLite migration
* Window identity references (stable id / hwnd / process facts)
* Arrangement metadata + lifecycle states
* Repository persistence
* Diagnostics for missing/unavailable referenced windows (against an observation snapshot)

---

## 4. Non-responsibilities

* Moving / resizing / focusing windows
* Restoring or applying arrangements
* UI layout controls / drag-and-drop
* AI recommendations, Assistant actions, automation
* Permission policy / Gateway wiring
* Window discovery (observation already owns that)

---

## 5. Canvas Layout vs Desktop Arrangement

| Concept | Meaning |
|---------|---------|
| **Canvas Layout** (`packages/domain/src/layout`, `007_layout.sql`) | Spatial board of **Zones** inside the Tauri companion UI |
| **Desktop Arrangement** (this batch) | Named set of **observed OS window identity references** |

Do **not** merge them. Do **not** store HWNDs on canvas layout nodes.

---

## 6. Architecture

```text
Operating System
        │
        ▼
windows-integration (capture)
        │
        ▼
ObservedWindow / ObservationWindowIdentity
        │
        ▼
DesktopArrangement Domain  ← DAF-1c
        │
        ▼
Future Apply/Restore (kernel + WindowController)  ← later
        │
        ▼
Workspace UI
```

### Model shape

* `DesktopArrangement` — id, workspace_id, name, description, lifecycle status, timestamps, `authority_effect: none`
* `DesktopArrangementEntry` — soft refs to `stable_window_id` and/or `hwnd`, optional process/title fingerprints, label, sort order
* **No geometry** in DAF-1c — “what belongs together,” not “move here”

### Missing windows

Given an observation snapshot, diagnose each entry:

* `Available` — identity/hwnd present in snapshot windows
* `IdentityKnownWindowMissing` — identity known historically but absent now
* `Unavailable` — no matching window or identity in snapshot

Do **not** recreate windows, guess replacements, or auto-repair.

---

## 7. Why a new model (not extending canvas Layout)

Canvas Layout owns zone presentation. Desktop Arrangement owns OS-window membership. Collapsing them would couple companion UI geometry to HWND lifecycle and break DEC-009 clarity.

---

## 8. Future restore pathway

Later batch (apply/restore):

1. Load `DesktopArrangement`
2. Capture / load observation snapshot
3. Diagnose availability
4. For available entries only, Permission Gateway → `WindowController` (geometry policy TBD)
5. Surface unavailable diagnostics to UI — never silent substitute

---

## 9. Human review

**Not required** ([HUMAN-REVIEW-POLICY.md](HUMAN-REVIEW-POLICY.md)).
