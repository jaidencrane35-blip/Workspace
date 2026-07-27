# Decision Engine Intake Evaluation

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned examination record for an active intake candidate |
| **Status** | Persisted (`decision_engine_intake_evaluation`) |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Boundary

```
DecisionEngineIntakeCandidate (active only)
        ↓ try_evaluate
DecisionEngineIntakeEvaluation
  evaluation_state = evaluated | rejected | deferred
        ✗ evaluate ≠ DecisionCandidate creation
        ✗ ≠ goal / intent / planner / Gateway / adapter / execution
        ✗ never mutates recommendation_lifecycle
```

## Rules

| Intake candidate lifecycle | May evaluate? |
|----------------------------|---------------|
| `active` | yes |
| `withdrawn` | no |
| `invalidated` | no |

Evaluation answers whether Decision Engine examined the intake candidate.
It does not authorize planning or execution.

## Related

- [WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md)
- [WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
