# Decision Candidate Evaluation Resolution

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned result of evaluation eligibility — scoring-path admission without scoring |
| **Status** | Projected for awaiting/blocked; `accepted_for_scoring` / `rejected_for_scoring` persisted |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Why this exists

`DecisionCandidateEvaluationOriginContract.evaluated` acknowledges origin evaluation only.
It does **not** record whether DE admits the candidate into a future scoring path.

This mirrors intake `Evaluation → Disposition`: an explicit post-evaluation decision.

## Boundary

```
EvaluationOriginContract (evaluated)
        ↓ derive / try_resolve
DecisionCandidateEvaluationResolution
  awaiting_resolution | accepted_for_scoring | rejected_for_scoring | blocked
        ✗ accepted ≠ DecisionScore / ranking
        ✗ no planner / Gateway / goals / intents / execution
        ✗ never mutates recommendation_lifecycle
```

## Rules

| State | When |
|-------|------|
| `awaiting_resolution` | Evaluated, lifecycle/provenance valid, candidate active |
| `accepted_for_scoring` | Explicit DE admit to future scoring path |
| `rejected_for_scoring` | Explicit DE decline of scoring path |
| `blocked` | Invalid provenance, invalid lifecycle, withdrawn/invalidated source, or unevaluated |

## Persistence

Only `accepted_for_scoring` / `rejected_for_scoring` (non-derivable decisions).
Table: `decision_candidate_evaluation_resolution`.

## Related

- [WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-ORIGIN.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-ORIGIN.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
