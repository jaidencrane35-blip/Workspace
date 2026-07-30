# Programme I — Implementation Contract 2  
# Product Workspace Composition

| Field | Value |
|-------|-------|
| **Authority** | Principal Architect |
| **Implementation agent** | Cursor |
| **Programme** | [Programme I](PROGRAMME-I-WORKSPACE-PRODUCT-CAPABILITY.md) |
| **Depends on** | [IC1 inventory](PROGRAMME-I-IC1-PRODUCT-SHELL-CAPABILITY-INVENTORY.md) |
| **Status** | Complete |
| **Date** | 2026-07-30 |

---

## Objective (satisfied)

Expose existing Workspace capability through a coherent product workflow by **composition**, not expansion.

No new runtime models, persistence models, desktop authorities, cognitive systems, or Assistant responsibilities were introduced.

---

## Unified Workspace workflow

Product language (user-facing):

```text
Workspace
    ↓
Profile
    ↓
Desktop
    ↓
Arrangement
    ↓
Restore
```

| Term | Means | Architecture underneath (not user-facing) |
|------|-------|-------------------------------------------|
| **Workspace** | The product | App shell |
| **Profile** | Named scope for saved library + Arrangements | Durable `Workspace` row + settings active id |
| **Desktop** | Running windows surface (primary tab) | Stage view id `layouts` + WorkspaceState |
| **Arrangement** | Saved layout of windows | `DesktopArrangement` |
| **Restore** | Bring saved layout back to the OS desktop | `restore_desktop_arrangement` via Permission Gateway → WindowController |

### How the user moves through it

1. Open **Workspace** (product shell).
2. Optionally choose or create a **Profile** (Profiles tab) — not required to see windows.
3. Open **Desktop** — observe and focus running windows (WorkspaceState).
4. Save an **Arrangement** (selection on Desktop, or Arrangements panel).
5. **Restore** that Arrangement from the Desktop Arrangement control (or Arrangements panel).

Assistant remains a sidecar for questions; it does not own Restore.

---

## Consistent terminology changes

| Before | After |
|--------|-------|
| Nav **Stage** | **Desktop** |
| Working set | **Arrangement** |
| Remember layout | **Arrangements** |
| Save selection | **Save Arrangement** |
| Open Stage (Apps/Profiles) | **Open Desktop** |

View id `layouts` unchanged (no navigation architecture rewrite).

---

## Composition delivered

1. **Unified workflow chrome** — Desktop nav label; Profile → Desktop → Arrangements line on Desktop hero; Profiles/Apps link to Desktop.
2. **Restore discoverability (IC1 G7)** — Desktop Arrangement row calls existing `restore_desktop_arrangement` (same Gateway path as Arrangements panel).
3. **Reduced conceptual duplication** — one product term “Arrangement” for saved layouts; Desktop Restores; Arrangements panel still saves/manages with the same IPC.
4. **Documentation** — this workflow document.

### Explicitly not added

- No new persistence / lock model
- No OS app discovery
- No Flow/Focus OS geometry apply
- No new WorkspaceState planes
- No new Assistant orchestration

---

## Validation evidence

Run on implementation tip:

- `pnpm typecheck`
- `pnpm test`
- `pnpm build`
- `verify:architecture-governance`
- `verify:ipc-contract`
- `verify:ui-experience-boundary`

Preservation claims:

- WorkspaceState remains authoritative for Desktop observation.
- Observation remains desktop fact.
- Permission Gateway remains on Restore (`desktop.restore`).
- No duplicate desktop model introduced.
- Existing Restore IPC reused; newly discoverable on Desktop.

---

## Stop condition

IC2 is complete: integrated workflow built entirely upon existing architectural capability; no additional product capability beyond composition.

Await Principal Architect before Implementation Contract 3.

---

## Key paths

- `app/src/App.tsx` — Desktop nav; wire Restore epoch
- `app/src/components/WorkspaceApplicationStage.tsx` — Arrangement + Restore
- `app/src/components/DesktopArrangementPanel.tsx` — Arrangements terminology
- `app/src/components/WorkspaceSwitcher.tsx` — Profile → Desktop link
- `app/src/lib/layoutsStageUi.ts` — workflow copy
- `docs/01-Product/PROGRAMME-I-IC2-PRODUCT-WORKSPACE-COMPOSITION.md` — this document
