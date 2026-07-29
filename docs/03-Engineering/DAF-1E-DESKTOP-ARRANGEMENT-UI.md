# DAF-1e — Desktop Arrangement User Interface Foundation

| Field | Value |
|-------|-------|
| **Purpose** | First human-facing Desktop Arrangement experience over DAF-1d IPC |
| **Batch** | DAF-1e |
| **Owner** | React UI (`app/src/components`, types, helpers) |
| **Status** | Implemented (UI foundation) |
| **Related** | [DAF-1D-DESKTOP-ARRANGEMENT-RESTORE.md](DAF-1D-DESKTOP-ARRANGEMENT-RESTORE.md) |

---

## 1. Product intent

Workspace remembers the user’s setup. The UI exposes save / select / restore without becoming a layout engine or AI workspace manager.

Lead copy: **Your workspace can remember your setup.**  
Not: “AI can organise your workspace.”

---

## 2. User workflow

```text
User opens Workspace (primary tab)
        ↓
Desktop Arrangements rail (beside canvas stage)
        ↓
Create Arrangement  OR  Select Existing
        ↓
Review membership + diagnostics
        ↓
User presses Restore
        ↓
IPC → CommandPipeline → PermissionGateway → WindowController → OS
```

No automatic restore. No Assistant-controlled restore. No silent moves.

---

## 3. UI ownership

| Surface | Responsibility |
|---------|----------------|
| `DesktopArrangementPanel` | Orchestrates load / capture / restore; busy + error wiring |
| `DesktopArrangementList` | Browse saved arrangements |
| `DesktopArrangementDetails` | Selected membership + metadata |
| Restore action | Explicit user press only |
| `RestoreDiagnosticsView` | Gaps, unavailable windows, apply outcomes |

Logic and authority remain in domain/kernel. UI is a consumer of IPC contracts only.

---

## 4. Assistant separation

```text
Main workspace stage (canvas + arrangements rail)
        |
        +----------------+
        | Assistant tab  |  (supporting; not arrangement controller)
        +----------------+
```

Arrangement save/restore lives on the Workspace stage, not in Assistant panels.

---

## 5. Non-goals

Automatic layouts, AI suggestions, Assistant execution, background auto-restore, audio, grouping intelligence, UI-only arrangement storage.

---

## 6. Future expansion

- DAF-1f: Assistant as true side rail beside primary stage
- Richer empty / permission education
- Windows-native visual verification of restore against real HWNDs
