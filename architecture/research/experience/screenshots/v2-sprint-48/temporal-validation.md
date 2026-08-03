# Sprint 48 — Temporal Workspace Validation

Authority: `architecture/40_Experience_Refoundation.md` · baseline `5fa77c1` (Sprint 47)

## Machine checks

See `temporal-validation.json` — **pass: true**.

| Dimension | Evidence |
|---|---|
| Temporal continuity | `data-temporal` on stage/nodes; phases drive proximity/opacity |
| Workspace memory | Permanence + intent memory; no badges/timelines/history panels |
| Evolution stability | Restore uses temporal confidence; dormant quiets without vanishing |
| Predictive relevance | Guide fades with `temporalCertainty`; resume panes ordered by unified score |

## Screenshots

- `fresh-workspace.png`
- `evolving-workspace.png`
- `resumed-workspace.png`
- `long-dormant-workspace.png`
- `contextual-guidance-over-time.png`

## Parity vs Sprint 47

| Metric | Sprint 47 | Sprint 48 | Δ |
|---|---|---|---|
| Overall mean | 9.06 | **9.16** | **+0.10** |
| Home | 9.23 | 9.29 | +0.06 |
| Save | 9.04 | 9.13 | +0.09 |
| Continue | 9.15 | 9.23 | +0.08 |
| Check-in | 8.91 | 9.05 | +0.14 |
| Guide | 8.98 | 9.09 | +0.11 |

New temporal dimension means: continuity 9.18 · memory 9.15 · evolution 9.16 · predictive 9.13.

## Engineering notes

- Unified `scoreMomentRelevance` = temporal × semantic (one weighting function).
- Added `resolveTemporalPhase`, permanence (`noteCapture`), intent memory.
- Removed parallel created_at recency sort; guide uses `temporalCertainty`.
