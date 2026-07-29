# Milestone C — Persistent Assistant Companion Rail

| Field | Value |
|-------|-------|
| **Status** | Implemented (chrome placement) |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/controlled-optimisation-34a5` |
| **References** | `docs/01-Product/references/workspace-concept-01.png`, Flow/Focus charter |
| **Related** | [OPTIMISATION_LOG.md](../04-Operations/OPTIMISATION_LOG.md) cycles C1–C4 |

## What shipped

- Assistant is a **persistent right companion rail** on Home, Workspaces, Applications, and Layouts.
- Chrome **Assistant** control toggles the rail (`aria-pressed`, `aria-controls`); it is **not** a full-page peer product tab.
- Diagnostics and Developer remain engineering tool views (full width, no companion rail).
- Rail prefers ask-first help; evidence packages and advanced workflow stay folded until requested.
- Escape inside the rail hides the companion.
- When the rail is open, Desktop Arrangements stack under the stage so the concept right rail stays singular.

## Explicit non-goals (unchanged)

- No new AI engines or Assistant capability expansion
- No Permission Gateway / WindowController ownership changes
- No Desktop Arrangement geometry apply on Flow/Focus switch
- No autonomous mode or window actions

## Scores after C1–C4

| Dimension | Score |
|-----------|------:|
| Reference alignment | ~8.8 |
| Maintainability | ~8.7 |
| Commercial readiness | ~6.3 |
| Human readability | ~8.7 |

## Human verification

1. Open Home / Applications / Layouts — companion rail visible on the right.
2. Toggle Assistant in chrome — rail hides/shows; stage remains.
3. Focus mode — rail narrower; stage still primary.
4. Open Layouts with rail — arrangements appear under stage, not as a second right column.
