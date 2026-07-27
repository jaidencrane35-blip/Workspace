# DecisionCandidateSelection

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned progression decision after ranking |
| **Status** | Projected awaiting/withdrawn; `selected` / `rejected` persisted |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Why this exists

`DecisionCandidateRanking` is comparative ordering only. It does **not** decide
which candidate DE wishes to progress.

`DecisionCandidateSelection` is the genuine next ownership boundary: a selection
decision artifact — without execution, planner handoff, or Gateway.

## Audit (rank → selection)

| Question | Answer |
|----------|--------|
| Ranking final comparison before selection? | **Yes** |
| Selection needs its own DE contract? | **Yes** — ranking explicitly does not select |
| Selection inside ranking violate separation? | **Yes** — comparison ≠ progression authority |
| Missing prerequisite wrapper? | **No** — ranking + score + provenance are the gates |

Rejected: ranking wrappers, score wrappers, ranking permissions, readiness layers.

## Boundary

```
DecisionCandidateRanking (ordered entries)
        ↓ derive / try_select
DecisionCandidateSelection
  awaiting_selection | selected | rejected | withdrawn
        ✗ selected ≠ DecisionOutcome::Selected / planner handoff
        ✗ no execution / Gateway / goals / intents
        ✗ never mutates recommendation_lifecycle or candidate outcome
```

## Rules

| State | When |
|-------|------|
| `awaiting_selection` | Valid ranking entry + score + provenance + active |
| `selected` | Explicit DE choose to progress |
| `rejected` | Explicit DE choose not to progress |
| `withdrawn` | Invalid/missing gates or source withdrawn before decision |

## Persistence

`selected` / `rejected` are non-derivable → DE-owned table
`decision_candidate_selection`. Awaiting/withdrawn remain projected.

Progression request is a separate artifact — see
[WORKSPACE-DECISION-ENGINE-CANDIDATE-PROGRESSION-REQUEST.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-PROGRESSION-REQUEST.md).

## Related

- [WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md)
- [WORKSPACE-DECISION-ENGINE-CANDIDATE-PROGRESSION-REQUEST.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-PROGRESSION-REQUEST.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md](./WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md)
