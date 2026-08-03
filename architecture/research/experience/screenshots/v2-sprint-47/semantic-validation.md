# Sprint 47 — Semantic Workspace Validation

Authority: `architecture/40_Experience_Refoundation.md` · baseline commit `279e913` (Sprint 46)

## Machine checks

See `semantic-validation.json` — **pass: true**.

| Dimension | Evidence |
|---|---|
| Semantic clarity | `data-semantic="on"` stage; semantic-field nodes placed by score |
| Spatial meaning | No `home-satellite--N` grid; writing reshapes field distances |
| Relationship legibility | Band/proximity variation; restore panes ordered by importance |
| Workspace coherence | Reflection quiet (no new chrome); Guide on cluster, not destination expand |

## Screenshots

- `semantic-home.png`
- `writing-field.png`
- `restore-field.png`
- `reflection-influence.png`
- `contextual-guidance.png`

## Parity vs Sprint 46

| Metric | Sprint 46 | Sprint 47 | Δ |
|---|---|---|---|
| Overall mean | 8.84 | 9.06 | **+0.22** |
| Home | 9.05 | 9.23 | +0.18 |
| Save | 8.85 | 9.04 | +0.19 |
| Continue | 8.95 | 9.15 | +0.20 |
| Check-in | 8.70 | 8.91 | +0.21 |
| Guide | 8.65 | 8.98 | +0.33 |

New semantic dimension means: clarity 9.08 · spatial meaning 9.04 · relationship legibility 9.08 · coherence 9.04.

## Engineering notes

- Extended `lib/cognitive.ts` with `composeSemanticField` + `composeSemanticWindowField` (no second engine).
- Deleted obsolete satellite/pane grid slot heuristics replaced by semantic positioning.
- Guide annotations attach to the near semantic cluster.
