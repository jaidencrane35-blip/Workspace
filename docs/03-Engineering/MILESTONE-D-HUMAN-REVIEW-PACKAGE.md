# Milestone D — Human Review Package

| Field | Value |
|-------|-------|
| **Purpose** | Runnable review environment + media for Milestone D acceptance |
| **Date** | 2026-07-30 |
| **Branch** | `cursor/milestone-d-workspace-stage-34a5` |
| **Scope** | Review deliverables only — no Milestone F, no behaviour/architecture changes |
| **Checklist** | [MILESTONE-D-VISUAL-REVIEW-CHECKLIST.md](MILESTONE-D-VISUAL-REVIEW-CHECKLIST.md) |

## Launch

```bash
cd app && pnpm exec vite
```

**URL:** http://localhost:1420

If port 1420 is already in use, Vite is already running — open the URL directly.

## Verified

| Check | Result |
|-------|--------|
| App serves on localhost:1420 | Pass (HTTP 200) |
| Stage is default landing | Pass (Stage `aria-current="page"` after load) |
| Nav order Stage → Applications → Workspaces → Home | Pass (visible in screenshots) |

## Screenshots produced

`/opt/cursor/artifacts/screenshots/`

- `milestone-d-stage-landing.png`
- `milestone-d-home.png`
- `milestone-d-applications.png`
- `milestone-d-stage-focus.png`
- `milestone-d-review-stage-default.png`
- `milestone-d-review-home.png`
- `milestone-d-review-applications.png`
- `milestone-d-review-stage-focus.png`

## Launch issues encountered

1. **Second Vite start failed** — `Port 1420 is already in use` because a prior `cd app && pnpm exec vite` was already healthy. **Resolution:** reuse the existing server; no code change.
2. **Tauri native shell** — not launched in this Linux review environment (Windows-targeted). Browser Vite preview is the supported review path here.

## Validation (review package turn)

| Command | Result |
|---------|--------|
| `curl` http://127.0.0.1:1420/ | 200 |
| Stage default after load | Confirmed |
| `pnpm typecheck` | Pass |

No application behaviour, architecture, or Milestone F work in this package.
