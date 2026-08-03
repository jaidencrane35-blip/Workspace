# Sprint 49 — Craftsmanship Audit

Authority: `architecture/40_Experience_Refoundation.md` · baseline `d856a80` (Sprint 48)

## Corrections (inconsistencies only)

| Area | Before | After |
|---|---|---|
| Motion | Hardcoded 1.15–1.4s field timings; orphan `--motion-lush` | Four purpose tokens: attention / continuity / memory / reconstruction |
| Springs | Five divergent spring recipes | Purpose springs + stable aliases |
| Material | Hero cyan radial recipe; blur 8/14/16/22/28px mix | Shared `--mat-blur` / `--mat-saturate`; object depth/edge |
| Radii | Hero `radius-lg` vs neighbour `radius-md` | Object surfaces unified to `radius-md` |
| Typography | Weights 540–680 | Cadence via 500/600 + letter-spacing/line-height |
| Surfaces | Continue panes custom gradient; dock alternate glass | `mat-object` / structural tokens |

## Machine checks

`craftsmanship-audit.json` — **pass: true**

## Screenshots

- `home.png` · `save.png` · `continue.png` · `checkin.png` · `guide.png`

## Parity vs Sprint 48

Overall **9.16 → 9.35 (+0.19)** — target ≥ 9.25 met. No destination decrease.
