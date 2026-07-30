# IM-1 — Stage Empty Spatial Calm — Completion Report

| Field | Value |
|-------|-------|
| **Status** | Complete — stop for human visual review |
| **Date** | 2026-07-30 |
| **Branch** | `cursor/im-1-stage-empty-spatial-calm-34a5` |
| **Contract** | [WORKSPACE-DESKTOP-INTERACTION-MODEL.md](../01-Product/WORKSPACE-DESKTOP-INTERACTION-MODEL.md) |
| **Audit** | [WORKSPACE-INTERACTION-MODEL-AUDIT.md](../01-Product/WORKSPACE-INTERACTION-MODEL-AUDIT.md) § IM-1 |

## Purpose

Make the Stage’s first impression a **calm spatial desktop plane**, not a text-heavy management empty state. Layout carries meaning; text supports in one short line.

## Ownership

Frontend product shell (`WorkspaceApplicationStage`, `stageDesktopUi`, Stage CSS).  
No domain/IPC ownership change. Reuses existing `get_workspace_state`.

## What changed

| Area | Change |
|------|--------|
| Empty Stage | Full-height desktop plane with a single status line |
| Copy | Removed paragraph empty states; `stageDesktopPlaneMessage` one-liners |
| Hero | Compact title + mode + Refresh (no long lede / canvas notes / hints) |
| Library | Collapsed `<details>`; quieter when plane is empty |
| Arrangements rail | Visually quieted via `:has(.stage-plane-calm)` when Stage is empty |
| Live Stage | Unchanged observation map when windows exist |

## Non-goals (honoured)

- IM-2+ (Assistant default, arrangements copy rewrite, Home, Workspaces, nav weight)  
- No fake windows  
- No new engines / AI  
- No OS geometry apply  

## Maintainability

- Empty messaging owned only by `stageDesktopUi.ts`  
- Extended existing Stage; no parallel Reality component  
- Library remains optional secondary; no duplicate observation store  

## Validation

- `pnpm typecheck` — pass  
- `pnpm test` — 125 pass  
- `pnpm build` — pass  
- `pnpm run verify:architecture-governance` — pass  
- `pnpm run verify:ipc-contract` — pass  
- `pnpm run verify:ui-experience-boundary` — pass  

## Screenshots

`/opt/cursor/artifacts/screenshots/im-1-*.png`

## Stop

**Do not begin IM-2** until human review of IM-1.
