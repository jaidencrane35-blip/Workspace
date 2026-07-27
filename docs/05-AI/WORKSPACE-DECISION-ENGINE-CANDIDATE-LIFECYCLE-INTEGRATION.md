# Decision Candidate Lifecycle Integration

| Field | Value |
|-------|-------|
| **Purpose** | Integrate native and recommendation-derived DecisionCandidates into the DE lifecycle without merging origins |
| **Status** | Projected — outcomes persist via existing `decision_engine_lifecycle` overlay |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionCandidate (native | recommendation_intake)
        ↓ derive_batch / try_apply_outcome
DecisionCandidateLifecycleIntegration
  integration_state =
    integrated | blocked | provenance_invalid
        ✗ no scoring / ranking
        ✗ no planner / Gateway / goals / intents / execution
        ✗ provenance immutable for recommendation_intake
        ✗ never mutates recommendation_lifecycle
```

## Origins

| Origin | Meaning |
|--------|---------|
| `native` | Synthesized DE candidate (attention / graph / goal / bootstrap) |
| `recommendation_intake` | Created from approved intake (`engine_decision:intake:*`) |

## Rules

| State | When |
|-------|------|
| `integrated` | Origin valid and provenance rules satisfied; may enter DE outcome lifecycle |
| `provenance_invalid` | Recommendation-intake missing/broken provenance, or native carrying intake provenance |
| `blocked` | Non-decision namespace / unsafe for lifecycle |

Lifecycle outcomes remain: `open | selected | dismissed | postponed | expired`.

## Provenance (recommendation_intake)

Retained and immutable across lifecycle transitions:

- `intake_candidate_id`
- `creation_request_id`
- `package_seal_digest`
- `recommendation_id` (package identity)

## Related

- [WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
