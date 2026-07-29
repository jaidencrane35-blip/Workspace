# Visual Review Checklist — Milestone A.1

| Field | Value |
|-------|-------|
| **Checkpoint** | Milestone A.1 — Workspace identity UX hardening |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/milestone-a1-ux-hardening-34a5` |
| **Human review required?** | **Yes** |

## What to verify

1. Primary tabs (Home / Workspaces / Applications / Layouts) read as the product.
2. Assistant / Diagnostics / Developer sit in a separate tools group.
3. Assistant view looks like a companion rail, not the main app.
4. Applications use cards / monograms and mention Layouts relationship.
5. Home shows current workspace identity, belonging apps, available vs not-available.
6. Browser preview banner is informational (not a red “Error:” failure tone).

## Launch

```bash
cd app && pnpm exec vite
# http://localhost:1420
```

### Screenshots

- `/opt/cursor/artifacts/screenshots/milestone-a1-home.png`
- `/opt/cursor/artifacts/screenshots/milestone-a1-nav-tools.png`
- `/opt/cursor/artifacts/screenshots/milestone-a1-applications.png`
- `/opt/cursor/artifacts/screenshots/milestone-a1-assistant-companion.png`
- `/opt/cursor/artifacts/screenshots/milestone-a1-workspaces.png`

**Decision:** ☐ Accept  ☐ Changes requested  ☐ Defer