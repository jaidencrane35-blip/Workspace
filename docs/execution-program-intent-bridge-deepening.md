# Execution Program: P1 Intent Bridge Deepening

| Field | Value |
| --- | --- |
| **Program** | Deepen intent bridge (named Moments, launch honesty) |
| **Date** | 2026-08-07 |
| **Prior milestone** | Conversational Operator Foundation (`ac6f997`) |
| **Handoff** | Awaiting Project Owner review |

---

## Implemented behaviour

| Utterance | Behaviour |
| --- | --- |
| `Restore Northwind` / `Continue Sprint board` | Lists saved Moments; unique name match focuses Continue on that Moment |
| Ambiguous / missing name | Opens Continue list; does **not** invent a restore |
| `Save this as Northwind` | Opens Save with naming guidance (capture still consent-gated) |
| `Open Cursor` / `Launch …` | Honest refusal — no fabricated launch |
| Existing Save / Continue / Expand / Health | Unchanged |

Matching: `app/src/lib/momentMatch.ts` (unique best score only).  
Lookup: existing `list_saved_contexts` IPC — no duplicate restore path.

---

## Validation

`pnpm typecheck` / `test` / `build`, launch Workspace — see project-health after sync.

---

## Known limitations

- Save-as does not prefill the Save form field yet (guidance only)
- Multi-match names fall back to Continue picker
- App launch by name remains unimplemented (by design for this program)

---

## Next recommended (after owner review)

Documentation authority convergence (Phase A / G3).
