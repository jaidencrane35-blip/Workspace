# Sprint 46 — Material audit

Authority: `architecture/40_Experience_Refoundation.md` · baseline `e999c81`

## Material roles

| Role | Surfaces | Treatment |
|---|---|---|
| Structural | shell stage, dock, menubar | Transparent / ink void |
| Object | primary Moment, surfaces | `--mat-object` + depth shadow |
| Environmental | atmosphere, ambient light | 2 lights, depth vignette |
| Ephemeral | affordances, inspect folds | Near-transparent until needed |

## Removed / consolidated

- Atmosphere glow C (and D stubs)
- Cyan UI bloom on hero / writing
- Screen-blend ambient spot
- Infinite empty-field halo loop
- Competing glass recipes on hero / sparse / write
- Focus motion scale (continuity = opacity)
- Check-in exit scale; restore pane blur filter

## Machine checks

`material-audit.json` — **PASS** (glow count 2, object depth, no UI bloom).

## Screenshots

- `idle-workspace.png`
- `focused-moment.png`
- `writing.png`
- `restore.png`
- `conversation.png`
