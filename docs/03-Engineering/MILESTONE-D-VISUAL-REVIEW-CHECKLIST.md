# Milestone D — Visual Review Checklist

| Field | Value |
|-------|-------|
| **Milestone** | D — Workspace Stage |
| **Branch** | `cursor/milestone-d-workspace-stage-34a5` |
| **Review package date** | 2026-07-30 |
| **References** | `docs/01-Product/references/workspace-concept-01.png`, `workspace-concept-02.png` |
| **Policy** | [HUMAN-REVIEW-POLICY.md](HUMAN-REVIEW-POLICY.md) |

## Review environment (runnable)

| Item | Value |
|------|-------|
| **Launch command** | `cd app && pnpm exec vite` |
| **Localhost URL** | http://localhost:1420 |
| **Also reachable as** | http://127.0.0.1:1420 |
| **Mode** | Frontend Vite browser preview (Tauri desktop shell not required for this visual checkpoint) |
| **Default landing verified** | **Yes** — opens on **Stage** (`aria-current="page"` on Stage; `useState<AppView>("layouts")`) |

### Launch notes

- Port **1420** (`strictPort`). If start fails with “Port 1420 is already in use”, an existing Vite process is already serving the app — open the URL above.
- Full `pnpm dev` / `tauri dev` is Windows-targeted and is **not** required for this Milestone D visual package.
- In browser preview, Tauri IPC is unavailable. A desktop-preview banner is expected; data actions (create workspace, live launch) need the native shell.

## Review goals

Confirm the UI reads as an **application-centric desktop workspace**, not an AI product.

## Checklist

1. **Landing** — App opens on Stage; primary nav leads with Stage.  
2. **Stage hero** — Applications dominate the Layouts/Stage view (large tiles, launch).  
3. **Canvas secondary** — Companion canvas is clearly optional / below the stage.  
4. **Home** — Apps-first; stage / “Open workspace stage” path is prominent.  
5. **Flow / Focus** — Chrome density still works; no OS window moves claimed.  
6. **Assistant** — Remains companion rail; does not own the stage.  
7. **Honesty** — No fake live window thumbnails; empty states clear.  
8. **Applications tab** — Still registry/launch; relates clearly to Stage.

## Fresh screenshots (review package)

Saved under `/opt/cursor/artifacts/screenshots/` (2026-07-30):

| File | View |
|------|------|
| `milestone-d-stage-landing.png` | Stage default landing (no workspace) |
| `milestone-d-home.png` | Home — apps-first + stage CTA cards |
| `milestone-d-applications.png` | Applications panel |
| `milestone-d-stage-focus.png` | Stage + Focus chrome |
| `milestone-d-review-stage-default.png` | Twin of stage landing |
| `milestone-d-review-home.png` | Twin of home |
| `milestone-d-review-applications.png` | Twin of applications |
| `milestone-d-review-stage-focus.png` | Twin of stage focus |

## Outcome

| Decision | ☐ |
|----------|---|
| Approve Milestone D | |
| Changes requested | |
| Block — product direction unclear | |

**Do not start Milestone F until approved.**
