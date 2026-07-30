# Programme I — Implementation Contract 3  
# Desktop Layout Editing Foundation

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme I](PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md) |
| **Depends on** | [IC1 inventory](PROGRAMME-I-IC1-PRODUCT-SHELL-CAPABILITY-INVENTORY.md), [IC2 composition](PROGRAMME-I-IC2-PRODUCT-WORKSPACE-COMPOSITION.md) |
| **Status** | Complete |
| **Date** | 2026-07-30 |

---

## Objective (satisfied)

Establish the **Desktop Layout Editing Foundation** by composing capabilities that already exist in Workspace architecture.

No new desktop model, runtime architecture, persistence model, IPC contract, or ownership boundary was introduced.

---

## Product capability delivered

A user can now:

1. **Enter layout editing mode** on Desktop for a selected Arrangement.
2. **See which Arrangement is being edited** (banner: `Editing · {name}`).
3. **Preview** saved layout geometry as ghost rectangles on the Desktop map before applying.
4. **Modify membership** by selecting windows, then **Update Arrangement** (existing `capture_desktop_arrangement` with `arrangementId`).
5. **Apply** changes to the OS desktop via the existing **Restore** pathway (`Apply` in edit mode).
6. **Update from desktop** from the Arrangements panel (same capture-with-id path).

### Unified editing workflow

```text
Desktop
  → select Arrangement
  → Edit layout
  → Preview (optional overlay)
  → select windows / organise on OS
  → Update Arrangement
  → Apply (Restore)
  → Done
```

Composition with IC2:

```text
Workspace → Profile → Desktop → Arrangement → Edit / Restore
```

---

## What was reused (not replaced)

| Capability | Reuse |
|------------|--------|
| WorkspaceState | Observation remains authoritative for live tiles |
| Arrangement persistence | Same `DesktopArrangement` / SQLite repository |
| Capture IPC | `capture_desktop_arrangement` with `arrangementId` + optional `memberHwnds` |
| Restore IPC | `restore_desktop_arrangement` (Apply = Restore) |
| Stage map | Existing Stage tiles + spatial plane |
| Gateway / WindowController | Restore still the only OS `set_bounds` product path |
| Arrangements panel | Same list/capture/restore; added Update from desktop |

### Explicitly not introduced

- No second arrangement model
- No canvas `Layout` as HWND store
- No new product IPC for per-window `set_bounds`
- No plan/preview-restore backend (preview is UI overlay of stored bounds)
- No new SQLite tables or durable edit sessions
- No Assistant ownership of layout editing

---

## Editing semantics

| Action | Meaning |
|--------|---------|
| **Edit layout** | Product UI mode over an existing Arrangement |
| **Preview** | Show stored entry bounds as ghosts on the map (does not move OS windows) |
| **Update Arrangement** | Re-capture observation into the same Arrangement id (selection → membership filter; empty selection → full desktop capture) |
| **Apply** | Existing Restore — Permission Gateway → WindowController |
| **Update from desktop** (Arrangements panel) | Same capture-with-`arrangementId`, all observed windows |

Geometry for Update always comes from **current observation** (user organises windows on the OS desktop, then records). Preview shows **previously saved** geometry before Apply.

---

## Architectural boundary validation

| Boundary | Status |
|----------|--------|
| WorkspaceState ownership unchanged | Live map still from observation only |
| Observation remains desktop fact | Edit mode does not invent windows |
| Desktop integration owns OS interaction | Apply/Restore only |
| Editing is a product capability | Stage + Arrangements UI composition |
| Assistant remains sidecar | Untouched |
| Existing arrangements compatible | Same schema; update reuses upsert |
| Existing Restore preserved | Same IPC path; Desktop still Restores outside edit mode |

---

## Files touched

- `app/src/lib/desktopLayoutEditing.ts` — preview projection + editing copy
- `app/src/components/WorkspaceApplicationStage.tsx` — edit mode UX
- `app/src/components/DesktopArrangementPanel.tsx` — Update from desktop
- `app/src/App.css` — edit banner + preview ghosts
- `tests/desktop-layout-editing.test.ts` — helper coverage
- This document

---

## Stop condition

Users can intentionally edit desktop layouts using existing Workspace architecture; architectural boundaries remain intact.

**Await further instruction from the Principal Architect before commencing Implementation Contract 4.**
