# Milestone D — Workspace Stage Completion Report

| Field | Value |
|-------|-------|
| **Status** | Complete — await human visual review |
| **Date** | 2026-07-30 |
| **Branch** | `cursor/milestone-d-workspace-stage-34a5` |
| **Roadmap** | [WORKSPACE-PRODUCT-DELIVERY-ROADMAP.md](../01-Product/WORKSPACE-PRODUCT-DELIVERY-ROADMAP.md) |
| **References** | `docs/01-Product/references/workspace-concept-01.png`, `workspace-concept-02.png` |

## Purpose delivered

Applications are the centrepiece of the product shell. Opening Workspace lands on the **Stage** (Layouts). Home leads with apps and a clear “Open workspace stage” path. Launch from stage tiles reuses the existing `launch_application` IPC.

## What changed

| Area | Change |
|------|--------|
| Default view | App opens on Stage (`layouts`), not Home |
| Primary nav | Order: Stage → Applications → Workspaces → Home |
| Stage tiles | Larger monograms; Launch via shared helper |
| Canvas | Demoted to labelled “optional board practice” under the stage |
| Home | Apps-first copy; stage preview tiles; Stage as primary action |
| Launch | `applicationLaunch.ts` shared by Applications panel + Stage |
| Copy | Apps-first / honest OS vs canvas wording |

## Explicitly not done (out of scope)

- Window grouping / lock  
- Flow/Focus OS geometry apply  
- Arrangement editing upgrades (Milestone F)  
- Audio  
- Assistant expansion  
- New engines or layers  

## Systems reused

Application registry + `launch_application`, `WorkspaceApplicationStage`, `ApplicationsPanel`, Desktop Arrangement rail (unchanged behaviour), Assistant companion rail (unchanged role), Flow/Focus chrome density (presentation only).

## Completion checklist

- [x] Layouts/Stage is the dominant product surface for workspace apps  
- [x] Empty / loading / runtime-unavailable states remain honest  
- [x] Launch still uses existing permissioned `launch_application` path  
- [x] Assistant remains companion rail, not stage owner  
- [ ] Human visual review against concept hierarchy (apps first) — **scheduled next**

## Validation

- `pnpm typecheck`  
- `pnpm test` (120)  
- `pnpm run verify:architecture-governance`  
- `pnpm run verify:ipc-contract`  
- `pnpm run verify:ui-experience-boundary`  
- `pnpm build`  

## Maintainability

| Question | Answer |
|----------|--------|
| Single responsibility? | Stage presents; launch helper only launches; copy helpers only copy |
| Ownership obvious? | Frontend product shell; no domain ownership change |
| Duplicate launch? | Removed — one `launchRegisteredApplication` |
| Hidden assumptions? | Stage shows registry assets, not live HWND tiles (documented in copy) |

## Screenshots

Fresh review package (2026-07-30). Browser Vite preview; desktop-preview banner expected.

**Launch:** `cd app && pnpm exec vite` → http://localhost:1420  
**Package doc:** [MILESTONE-D-HUMAN-REVIEW-PACKAGE.md](MILESTONE-D-HUMAN-REVIEW-PACKAGE.md)

| File | Contents |
|------|----------|
| `/opt/cursor/artifacts/screenshots/milestone-d-stage-landing.png` | Stage landing (no workspace) |
| `/opt/cursor/artifacts/screenshots/milestone-d-home.png` | Home apps-first + stage CTA |
| `/opt/cursor/artifacts/screenshots/milestone-d-applications.png` | Applications panel |
| `/opt/cursor/artifacts/screenshots/milestone-d-stage-focus.png` | Stage + Focus chrome |
| `/opt/cursor/artifacts/screenshots/milestone-d-review-*.png` | Review twins of the four views above |

## Stop

Milestone D implementation complete; **human review package ready**. **Do not start Milestone F** until human review and approval.
