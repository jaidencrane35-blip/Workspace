# Sprint 45 — Cognitive behaviour validation

Authority: `architecture/40_Experience_Refoundation.md` · baseline `7398738`

## Consolidation

| Former | Now |
|---|---|
| `IntentEngine` provider | Facade → `CognitiveEngine` |
| `AttentionEngine` provider | Facade → `CognitiveEngine` |
| Scene weight multipliers in Attention + Object | `lib/cognitive.ts` single pipeline |

## Behaviour checks

| Check | Result |
|---|---|
| Cognitive stage marker | PASS |
| Neighbour field rebalances by relevance | PASS |
| Writing presence + tool focus | PASS |
| Restore windows cognitively ordered | PASS |
| Reflection feeds cognitive model | PASS (quiet) |
| Guide observational / confidence-gated | PASS |

Machine report: `cognitive-behaviour-validation.json`

## Screenshots

- `passive-workspace.png`
- `active-writing.png`
- `resumed-workspace.png`
- `reflection-influence.png`
- `contextual-guidance.png`
