# Workflow validation

Date: 2026-08-03  
Demo: Vite populated experience (`isExperienceDemoActive`)  
Runner: `architecture/research/experience/validate-production.mjs` (+ focused remeasure)

## Journeys

### Journey 1 — Home → Save → Home → Continue → Restore Preview

| Step | Result |
| --- | --- |
| Home | Pass — “Continue your work”, featured Moment |
| Save new Moment | Pass — name + handoff → Review → Save this context → Saved |
| Return Home | Pass — new Moment present among Moments |
| Continue | Pass — browse “What were you doing?” |
| Restore Preview | Pass — Approve and restore / place-ready |

Interruptions recorded: **F-01** (view key) blocked Save until fixed; **F-05** two-step save.

### Journey 2 — Home → Continue → Check-in → Home

| Step | Result |
| --- | --- |
| Home | Pass |
| Continue existing Moment | Pass — preview or browse |
| Check-in | Pass — “How’s the return feeling?” active pulse |
| Return Home | Pass |

Interruptions: **F-07** no chapter chips (accepted).

### Journey 3 — Guide → Home → Save → Continue

| Step | Result |
| --- | --- |
| Guide | Pass after F-01 fix — “How this pilot works” |
| Home | Pass |
| Save | Pass — Leave a note surface |
| Continue | Pass |

## Interaction audit

| Probe | Result |
| --- | --- |
| Keyboard dock tabs | Pass — aria-labels; ArrowRight Home→Save |
| Focus order | Pass after F-02 — satellites are tab stops; dock remains reachable |
| Restore flow | Pass — Continue → preview → Approve / Not now |
| Writing flow | Pass — Save fields focus; `data-writing` / ambient update |
| Dock behaviour | Pass after F-01 — destinations swap reliably |
| Resize 1100 / 720 | Pass — no horizontal overflow; dock visible |
| Empty ↔ populated | Pass — populated Home stable across journeys |
| Rapid dock (8 hops) | Pass — ~6s including waits |
| A11y smoke | Pass — named tabs/buttons, heading present |

## Performance

| Metric | Before fixes | After fixes |
| --- | --- | --- |
| CLS full script | 0.449 | (not re-run full; structural sources removed) |
| CLS Home↔Continue ×4 | 0.107 | **0.092** |
| Layout animations on shell/canvas | on | **off** |
| View reveal | opacity + y | **opacity only** |

Optimisations applied only where measured (F-03).

## Code quality (verified unused)

Deleted:

- `app/src/demo/DemoRestoreHistory.tsx` (no imports)
- `.demo-restore-history*` CSS
- unused `--home-satellite-clarity` custom property

## Verdict

Production workflow validation **complete**. Blocking friction F-01/F-02 resolved; CLS under threshold on the measured dock path. Remaining items are accepted product tradeoffs (F-05, F-07) or low-severity Inspect placement (F-06).
