# IM-2 — Desktop Reality First — Completion Report

| Field | Value |
|-------|-------|
| **Status** | Complete — stop for human visual review |
| **Date** | 2026-07-30 |
| **Branch** | `cursor/im-2-desktop-reality-first-34a5` |
| **Contract** | [WORKSPACE-DESKTOP-INTERACTION-MODEL.md](../01-Product/WORKSPACE-DESKTOP-INTERACTION-MODEL.md) |
| **Audit** | [WORKSPACE-INTERACTION-MODEL-AUDIT.md](../01-Product/WORKSPACE-INTERACTION-MODEL-AUDIT.md) |

## Purpose

Reduce work before Workspace is useful. Prefer **Observe → Represent → Organise → Ask**. Soften setup-first messaging; keep Assistant optional at launch so Stage desktop reality leads.

## Ownership

Frontend product shell preferences + copy. No new IPC, engines, or AI.

## What changed

| Area | Change |
|------|--------|
| Assistant rail | Default **closed** (`DEFAULT_ASSISTANT_RAIL_OPEN = false`) |
| Assistant empty copy | Desktop-first; no create-workspace CTA |
| Arrangements empty | “Saving needs a named profile” — Stage already shows desktop |
| Arrangements hero | “Remember this desktop” |
| Profiles tab | Nav label **Profiles**; switcher copy demotes create-as-product |
| Reopen chrome | “Ask Assistant” when rail collapsed |

## Non-goals (honoured)

- No fake application data  
- No AI behaviour  
- No grouping / arrangement editing / Milestone F  
- No Home redesign (IM-5)  
- No OS geometry apply  

## Maintainability

- Preference default owned in `assistantRail.ts`  
- Empty copy in pure helpers (`desktopArrangementUi`, `workspaceSwitcherUi`)  
- No duplicate models  

## Validation

- `pnpm typecheck` — pass  
- `pnpm test` — 125 pass  
- `pnpm build` — pass  
- `verify:architecture-governance` / `verify:ipc-contract` / `verify:ui-experience-boundary` — pass  

## Screenshots

`/opt/cursor/artifacts/screenshots/im-2-*.png`

## Stop

**Do not begin IM-3** until human review of IM-2.
