# DecisionCandidateProgressionRequest

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned non-executing request that a selected candidate be considered downstream |
| **Status** | Projected pending/blocked; `requested` / `cancelled` persisted |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Why this exists

`DecisionCandidateSelection.selected` records that DE chose a progression target.
It does **not** request downstream action.

`DecisionCandidateProgressionRequest` is the genuine next ownership boundary:
"DE wants this selected candidate considered for downstream progression" —
without planner handoff, Gateway, goals, or intents.

## Audit (selection → progression request)

| Question | Answer |
|----------|--------|
| Separate progression intent before planner? | **Yes** |
| Selection directly create planner handoff? | **No** — collapses decision vs request authority |
| Missing boundary selected → downstream request? | **Yes** — this artifact |
| Planner handoff now violate separation? | **Yes** — would skip this request |

Rejected: selection wrappers, selection permissions, selection readiness, scoring/ranking extensions.

## Boundary

```
DecisionCandidateSelection (selected)
        ↓ derive / try_request
DecisionCandidateProgressionRequest
  pending | requested | cancelled | blocked
        ✗ requested ≠ planner handoff / Command Pipeline
        ✗ no Gateway / goals / intents / execution
        ✗ never mutates recommendation_lifecycle or candidate outcome
```

## Rules

| State | When |
|-------|------|
| `pending` | Selected + valid provenance/lifecycle + active |
| `requested` | Explicit DE issue of progression request |
| `cancelled` | Explicit cancel of request path |
| `blocked` | Not selected, invalid provenance, invalid lifecycle, withdrawn |

## Persistence

`requested` / `cancelled` are non-derivable → DE-owned table
`decision_candidate_progression_request`. Pending/blocked remain projected.

## Related

- [WORKSPACE-DECISION-ENGINE-CANDIDATE-SELECTION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-SELECTION.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md](./WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md)
