# Visual Review Checklist — Milestone B (Flow / Focus chrome)

| Field | Value |
|-------|-------|
| **Checkpoint** | Milestone B — chrome-density Flow ↔ Focus |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/milestone-b-flow-focus-chrome-34a5` |
| **Human review required?** | **Yes** |

## Verify

1. Chrome shows Flow / Focus segmented control near primary nav.
2. **Flow** on Layouts: dense app tiles + companion canvas visible.
3. **Focus** on Layouts: primary app emphasised; supporting chips available; canvas hidden with honest note.
4. Mode switch does **not** claim OS windows moved.
5. Arrangements rail still present; Assistant remains a tool, not mode owner.
6. Home reflects current presentation mode and updated “not available yet” list.

## Launch

```bash
cd app && pnpm exec vite
# http://localhost:1420
```

### Screenshots

- `/opt/cursor/artifacts/screenshots/milestone-b-flow-layouts.png`
- `/opt/cursor/artifacts/screenshots/milestone-b-focus-layouts.png`
- `/opt/cursor/artifacts/screenshots/milestone-b-mode-switch-chrome.png`

**Decision:** ☐ Accept  ☐ Changes requested  ☐ Defer
