# Programme I — Implementation Contract 4  
# Desktop Layout Editing Experience Refinement

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme I](PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md) |
| **Depends on** | [IC3 Desktop Layout Editing Foundation](PROGRAMME-I-IC3-DESKTOP-LAYOUT-EDITING-FOUNDATION.md) |
| **Status** | Complete |
| **Date** | 2026-07-30 |

---

## Objective (satisfied)

Refine the Desktop Layout Editing experience established in IC3 so the workflow feels **complete and deliberate**, without changing architecture.

This was a **product refinement sprint** — not an architecture sprint.

---

## Editing session lifecycle

Lifecycle is **explicit** and **derived** from existing edit flags + Arrangement/observation comparison:

```text
Idle
  ↓ Enter Edit Layout
Editing
  ↓ meaningful delta detected
Changes Pending
  ↓ Update (capture with arrangementId)
Editing
  ↓ Done
Idle
```

| Phase | Derivation |
|-------|------------|
| **Idle** | Not in edit mode |
| **Editing** | Edit mode + no effective changes |
| **Changes pending** | Edit mode + membership/bounds delta vs saved Arrangement |

No duplicate runtime state. No duplicate persistence. Phase is UI derivation only (`data-layout-editing-phase`).

---

## Deliverables implemented

### 1. Editing session lifecycle
Phase helpers in `desktopLayoutEditing.ts`; Stage sets `data-layout-editing-phase` and phase-aware workflow copy.

### 2. Editing status surface
Banner shows:
- Arrangement name (`Editing · {name}`)
- Phase label (**No changes** / **Changes pending**)
- Preview on/off
- Last update timestamp (`Arrangement.updated_at`)
- Change summary when pending
- Last Update confirmation (ephemeral UI string after successful Update)

All derived from Arrangement + WorkspaceState + existing edit flags.

### 3. Change awareness
Pure field comparison (`diffLayoutEditingChanges`):
- window **added** / **removed** (identity: stable id, then hwnd)
- **bounds changed** (x/y/width/height)
- **z-order** reserved in the diff shape but unused until Arrangement entries persist z-order (no parallel store invented)
- **no effective changes** when none of the above apply

Proposed capture set matches Update semantics: selection if non-empty, else all observed windows.

### 4. Update confirmation
Before Update: `Update · N windows · X affected · Y unchanged`  
After Update: same line retained as **Last update** until Done/exit.  
Update disabled when there are no effective changes.

### 5. Preview refinement
Clearer ghost rendering (dashed hatch, **Saved ·** label), live tiles elevate over ghosts (`live-over-preview`). Preview remains visual only — never issues OS movement.

### 6. Exit behaviour
**Done** (or clearing Arrangement) clears edit mode, preview, and update confirmation. No residual editing artifacts.

### 7. UI polish
- Rename edit-mode **Apply** → **Restore** (Restore remains the OS path; Update is persist)
- Primary **Update** only when changes pending
- Preview / Restore / Done as secondary actions
- Pending banner accent when changes exist

---

## Architectural reasoning

IC3 already composed editing from Arrangement capture/restore and Stage observation. IC4 only makes that composition **legible**:

- Change detection compares saved Arrangement entries to current WorkspaceState windows — the same facts Update would persist.
- Preview continues to project stored entry bounds onto the Stage plane.
- Update still calls `capture_desktop_arrangement` with `arrangementId`.
- Restore still calls `restore_desktop_arrangement` (sole product `set_bounds` path).

---

## Preserved boundaries

| Constraint | Status |
|------------|--------|
| WorkspaceState sole runtime desktop model | Unchanged |
| No second desktop model | Unchanged |
| No second Arrangement persistence | Unchanged |
| No canvas-owned state | Unchanged |
| Restore sole OS positioning authority | Unchanged (edit-mode button restored to **Restore**) |
| Existing IPC ownership | Unchanged |
| Observation / Intelligence untouched | Unchanged |
| Editing composes existing foundations only | Unchanged |

---

## Validation

- `pnpm typecheck`
- `pnpm test`
- `pnpm build`
- Architecture / IPC / UI experience verifies as required by Programme I cycles
- Working tree clean after commit

---

## Files touched

- `app/src/lib/desktopLayoutEditing.ts` — lifecycle, diff, status, confirmation
- `app/src/components/WorkspaceApplicationStage.tsx` — status surface + polish
- `app/src/App.css` — banner / preview refinement
- `tests/desktop-layout-editing.test.ts` — lifecycle + change awareness
- This document; Programme I charter link

---

## Stop condition

IC4 complete. **Await Principal Architect review before further Programme I contracts.**
