# IM-2 — Visual Review Checklist

| Field | Value |
|-------|-------|
| **Slice** | IM-2 Desktop Reality First |
| **Branch** | `cursor/im-2-desktop-reality-first-34a5` |
| **Launch** | `cd app && pnpm exec vite` → http://localhost:1420 |
| **Note** | Clear `localStorage` key `workspace.ui.assistant_rail_open` if an older session forced the rail open |

## Verify

1. **Stage first** — Opens on Stage; Assistant rail **not** open by default.  
2. **Ask later** — “Ask Assistant” control available; opening rail is optional.  
3. **No create-first Assistant empty** — Companion does not demand “create a workspace” before help.  
4. **Profiles, not setup product** — Nav/Profiles copy treats names as optional.  
5. **Arrangements** — Empty state says saving needs a profile; desktop already on Stage.  
6. **No fake apps** — Empty Stage plane still honest (IM-1).  

## Screenshots

- `im-2-stage-assistant-closed.png`  
- `im-2-profiles-optional.png`  
- `im-2-arrangements-remember.png`  

## Outcome

| Decision | ☐ |
|----------|---|
| Approve IM-2 | |
| Changes requested | |
| Block | |
