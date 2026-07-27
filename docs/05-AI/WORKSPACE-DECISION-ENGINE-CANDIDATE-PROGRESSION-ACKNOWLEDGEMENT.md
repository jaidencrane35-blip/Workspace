# DecisionCandidateProgressionAcknowledgement

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned receipt that a progression request was received for a future workflow |
| **Status** | Projected awaiting/rejected/expired; `acknowledged` / `rejected` persisted |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Why this exists

`DecisionCandidateProgressionRequest.requested` asks for downstream consideration.
It does **not** record that the request was received/accepted for a future workflow.

`DecisionCandidateProgressionAcknowledgement` is the genuine next ownership
boundary — a DE-side receipt — without planner handoff, goals, intents, or Gateway.

## Audit (request → acknowledgement)

| Question | Answer |
|----------|--------|
| Next missing boundary planner handoff? | **No** |
| Missing DE receipt of progression request? | **Yes** — this artifact |
| Acknowledgement execute anything? | **No** |

Rejected: wrappers, duplicate gates, defensive contracts.

## Boundary

```
DecisionCandidateProgressionRequest (requested)
        ↓ derive / try_acknowledge
DecisionCandidateProgressionAcknowledgement
  awaiting_acknowledgement | acknowledged | rejected | expired
        ✗ acknowledged ≠ planner / goals / intents / Gateway
        ✗ never mutates recommendation_lifecycle or candidate outcome
```

## Rules

| State | When |
|-------|------|
| `awaiting_acknowledgement` | Valid requested progression + active lifecycle + provenance |
| `acknowledged` | Explicit DE receipt acceptance |
| `rejected` | Invalid request / gates |
| `expired` | Request no longer valid (withdrawn / inactive / provenance break) |

## Persistence

`acknowledged` / `rejected` are non-derivable → DE-owned table
`decision_candidate_progression_acknowledgement`. Awaiting/expired remain projected.

## Related

- [WORKSPACE-DECISION-ENGINE-CANDIDATE-PROGRESSION-REQUEST.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-PROGRESSION-REQUEST.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md](./WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md)
